use crate::{
    config::Config,
    data_bufs::{Dataset, DatasetDescriptor},
    units::*,
    wasm::{MAX_PAGE_COUNT, PAGE_SIZE},
};
use rustix::{
    mm::{madvise, mmap, mmap_anonymous, mprotect, Advice, MapFlags, MprotectFlags, ProtFlags},
    param::page_size,
};
use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    ffi,
    fmt::Debug,
    ptr::{self, NonNull},
    rc::Rc,
    sync::Arc,
};
use thiserror::Error;
use tracing::instrument;

/// The memory map backed linear memory for wasm modules.
///
/// We create a mapping containing all 8GiB of addressable wasm memory
/// (8GiB because wasm allows a load ptr+offset, where ptr and offset are u32).
/// We reserve memory for a bunch of initial pages based on how much the module needs
/// (that's where the code and static data go).
/// Then we hook the dataset to the first page after the initial segment.
/// Right after the data there is the dedicated state page.
/// Any further allocations go after the entire data chunk.
///
/// Assuming we have I initial pages, a data `buf` of length B pages, and N pages allocated from the guest module
/// (P is a single wasm page, 64KB):
/// wasm addr.    0            I*P            (I+B)*P         (I+B+1)*P       (I+B+1+N)*P          4GiB        8GiB
///               |             |                |                |               |                 |           |
/// purpose       |<==initial==>|<===buf data===>|<==state page==>|<==guest mem==>|<--unallocated-->|<--guard-->|
/// memory kind   | reserved mem|  mmap(PRIVATE) |  reserved mem  | reserved mem  |     virtual     |  virtual  |
/// memory prot   |     R+W     |       R        |      R+W       |      R+W      |      NONE       |    NONE   |
/// Memory is initially guaranteed to be zeroed everywhere except for the buf data segment.
/// All growth happens in wasm-page-sized chunks.
#[derive(Debug)]
pub struct MmapMemory {
    main_mmap: NonNull<ffi::c_void>,
    initial_pages: Cell<usize>,
    hook_mmap: Cell<*mut ffi::c_void>,
    hook_mmap_len: Cell<usize>,
    hook_wasm_ptr: Cell<u32>,
    hook_dataset_desc: Cell<DatasetDescriptor>,
    accessible_pages: Cell<usize>,
    is_tied: Cell<bool>,
    rank: MemoryRank,
}

#[derive(Debug)]
pub struct WasmMmapMemory(Rc<MmapMemory>);

impl WasmMmapMemory {
    pub fn new(inner: Rc<MmapMemory>) -> Self {
        Self(inner)
    }
}

pub struct WasmMemoryManager {
    per_thread_cache_limit: usize,
    memory_creator: Arc<WasmMemoryCreator>,
}

std::thread_local! {
    static LOCAL_MEMORY_MANAGER: RefCell<ThreadLocalMemoryManager> = RefCell::new(ThreadLocalMemoryManager::new());
}

#[derive(Debug)]
struct ThreadLocalMemoryManager {
    memory_count_limit: usize,
    current_create_memory_context: MemoryInitContext,
    next_memory_rank: MemoryRank,
    memories: HashMap<NonNull<ffi::c_void>, Rc<MmapMemory>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct MemoryRank(u64);

impl MemoryRank {
    pub const MIN: Self = MemoryRank(0);

    pub fn next(self) -> Self {
        MemoryRank(self.0.wrapping_add(1))
    }
}

pub struct WasmMemoryCreator {}

pub struct MemoryInitContext {
    state: Cell<MemoryInitContextState>,
}

impl Debug for MemoryInitContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state = self.state.take();
        let result = f.debug_struct("MemoryInitContext").field("state", &state).finish();
        self.state.set(state);
        result
    }
}

#[derive(Debug)]
enum MemoryInitContextState {
    New(Dataset),
    Created(Rc<MmapMemory>),
    Reused(Rc<MmapMemory>),
    None,
}

impl Default for MemoryInitContextState {
    fn default() -> Self {
        Self::None
    }
}

impl MemoryInitContext {
    fn new(dataset: Dataset) -> Self {
        Self {
            state: Cell::new(MemoryInitContextState::New(dataset)),
        }
    }

    fn none() -> Self {
        Self {
            state: Cell::new(MemoryInitContextState::None),
        }
    }

    fn take_request(&mut self) -> Dataset {
        let state = self.state.take();

        match state {
            MemoryInitContextState::New(dataset) => dataset,
            _ => panic!("invalid context state: {:?}, must be New to take request", state),
        }
    }

    pub(crate) fn get_memory_instance(&self) -> Rc<MmapMemory> {
        let state = self.state.take();
        match &state {
            MemoryInitContextState::Created(memory) => {
                let result = memory.clone();
                self.state.set(state);
                result
            }
            MemoryInitContextState::Reused(memory) => {
                let result = memory.clone();
                self.state.set(state);
                result
            }
            _ => panic!(
                "invalid context state: {:?}, get_memory_instance only possible after memory request was served",
                state
            ),
        }
    }
}

unsafe impl Send for WasmMmapMemory {}
unsafe impl Sync for WasmMmapMemory {}

impl WasmMemoryManager {
    pub fn new(config: &Config) -> Self {
        let creator = Arc::new(WasmMemoryCreator::new());
        let per_thread_cache_limit = config.thread_virtual_memory_limit();
        Self {
            per_thread_cache_limit,
            memory_creator: creator,
        }
    }

    pub fn memory_creator(&self) -> &Arc<WasmMemoryCreator> {
        &self.memory_creator
    }

    pub fn create_memory_scope<F, T, E>(&self, dataset: Dataset, f: F) -> Result<(T, Rc<MmapMemory>), E>
    where
        F: FnOnce() -> Result<T, E>,
    {
        LOCAL_MEMORY_MANAGER.with_borrow_mut(|mgr| {
            // This only needs to be set once, but this is easier than intervening in the thread-local initialization.
            mgr.set_memory_cache_limit(self.per_thread_cache_limit);
            mgr.current_create_memory_context = MemoryInitContext::new(dataset);
        });
        let t = f()?;
        let mem = LOCAL_MEMORY_MANAGER.with_borrow(|mgr| mgr.current_create_memory_context.get_memory_instance());
        Ok((t, mem))
    }

    #[instrument(skip(self, wasm_ctx))]
    pub fn get_tied_memory<C: wasmtime::AsContext>(
        &self,
        memory: wasmtime::Memory,
        wasm_ctx: C,
    ) -> Result<Rc<MmapMemory>, Error> {
        let base_host_ptr = memory.data_ptr(&wasm_ctx);
        let non_null_ptr = NonNull::new(base_host_ptr.cast()).expect("wasm memory ptr cannot be null");
        LOCAL_MEMORY_MANAGER.with_borrow_mut(|mgr| mgr.get_tied_memory(non_null_ptr))
    }

    #[instrument(skip(self))]
    pub fn notify_memory_drop(&self, memory_instance: &MmapMemory) -> Result<(), Error> {
        memory_instance.is_tied.set(false);
        Ok(())
    }
}

impl WasmMemoryCreator {
    fn new() -> Self {
        Self {}
    }
}

impl ThreadLocalMemoryManager {
    fn new() -> Self {
        Self {
            memory_count_limit: 0,
            current_create_memory_context: MemoryInitContext::none(),
            next_memory_rank: MemoryRank::MIN.next(),
            memories: HashMap::new(),
        }
    }

    fn set_memory_cache_limit(&mut self, cache_limit: usize) {
        self.memory_count_limit = cache_limit / MmapMemory::TOTAL_LEN;
    }

    fn can_create_more_memories(&self) -> bool {
        self.memory_count_limit > self.memories.len()
    }

    #[instrument]
    fn get_tied_memory(&mut self, ptr: NonNull<ffi::c_void>) -> Result<Rc<MmapMemory>, Error> {
        use std::collections::hash_map::Entry;

        match self.memories.entry(ptr) {
            Entry::Occupied(occupied_entry) => Ok(occupied_entry.get().clone()),
            Entry::Vacant(_) => Err(Error::CrossThreadMemoryRequest(ptr)),
        }
    }

    #[instrument]
    fn serve_memory_request(&mut self, initial_pages: usize) -> Result<Rc<MmapMemory>, Error> {
        // Memory class in order of "badness", we want to find an unused memory with possibly
        // greatest (best) class.
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
        enum MemoryClass {
            None,
            NeedsReinit,
            CorrectInitial,
            ExactFit,
        }
        #[derive(Debug)]
        struct MemoryDesc<'a> {
            memory: Option<&'a Rc<MmapMemory>>,
            class: MemoryClass,
            rank: MemoryRank,
        }
        impl PartialEq for MemoryDesc<'_> {
            fn eq(&self, other: &Self) -> bool {
                self.class == other.class && self.rank == other.rank
            }
        }
        impl Eq for MemoryDesc<'_> {}
        impl Ord for MemoryDesc<'_> {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.class.cmp(&other.class).then(self.rank.cmp(&other.rank).reverse())
            }
        }
        impl PartialOrd for MemoryDesc<'_> {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        // Assert we are in the correct context and extract the dataset.
        let dataset = self.current_create_memory_context.take_request();
        let dataset_descriptor = dataset.descriptor();
        let can_create_new = self.can_create_more_memories();

        // Classify all unused memories.
        let mut best_memory = MemoryDesc {
            memory: None,
            class: MemoryClass::None,
            rank: MemoryRank::MIN,
        };
        {
            let _ = tracing::debug_span!("scan memories").entered();
            for memory in self.memories.values() {
                if !memory.is_tied.get() {
                    let mut this_memory = MemoryDesc {
                        memory: Some(memory),
                        class: MemoryClass::NeedsReinit,
                        rank: memory.rank,
                    };

                    if memory.initial_pages() >= initial_pages {
                        this_memory.class = MemoryClass::CorrectInitial;
                        if memory.hook_dataset_desc.get() == dataset_descriptor {
                            this_memory.class = MemoryClass::ExactFit;
                        }
                    }
                    best_memory = std::cmp::max(best_memory, this_memory);
                }
            }
        }
        tracing::debug!("best memory fit: {best_memory:?}");
        if let Some(memory) = best_memory.memory {
            let _ = tracing::debug_span!("reuse old memory").entered();
            if best_memory.class == MemoryClass::ExactFit {
                // We just need a refresh, i.e. clearing excess allocated memory.
                memory.refresh()?;
                memory.is_tied.set(true);
                self.current_create_memory_context
                    .state
                    .set(MemoryInitContextState::Reused(memory.clone()));
                return Ok(memory.clone());
            } else if !can_create_new {
                // We cannot create new memory so we need to reset an old one.
                memory.reset(initial_pages, &dataset)?;
                memory.is_tied.set(true);
                self.current_create_memory_context
                    .state
                    .set(MemoryInitContextState::Reused(memory.clone()));
                return Ok(memory.clone());
            }
        }
        if can_create_new {
            let _ = tracing::debug_span!("create new memory").entered();
            let rank = self.next_memory_rank;
            self.next_memory_rank = rank.next();
            let new_memory = Rc::new(MmapMemory::new(rank, initial_pages, &dataset)?);
            self.memories.insert(new_memory.base_ptr(), new_memory.clone());
            new_memory.is_tied.set(true);
            self.current_create_memory_context
                .state
                .set(MemoryInitContextState::Created(new_memory.clone()));
            return Ok(new_memory);
        }

        // There is no way to satisfy this request as all existing memories are in use and we cannot allocate. Bail.
        return Err(Error::VirtualMemoryLimitExceeded);
    }
}

impl MmapMemory {
    const TOTAL_LEN: usize = 8 * GIB;

    #[instrument]
    pub fn new(rank: MemoryRank, initial_pages: usize, dataset: &Dataset) -> Result<Self, Error> {
        tracing::debug!("Creating new MmapMemory");
        let initial_pages = std::cmp::max(super::DEFAULT_INITIAL_RESERVATION_IN_PAGES, initial_pages);
        let initial_bytes = initial_pages * PAGE_SIZE;

        // Create the full mapping, do not reserve memory and make it protected.
        let full_memory = unsafe {
            mmap_anonymous(
                std::ptr::null_mut(),
                Self::TOTAL_LEN,
                ProtFlags::empty(), // PROT_NONE
                MapFlags::PRIVATE | MapFlags::NORESERVE,
            )
        }?;
        tracing::debug!(
            "Main memory map created at {full_memory:?} with len {}",
            crate::fmt::WasmPageDisplay(Self::TOTAL_LEN / PAGE_SIZE)
        );

        // Make the initial chunk accessible.
        unsafe {
            mprotect(full_memory, initial_bytes, MprotectFlags::READ | MprotectFlags::WRITE)?;
        }
        tracing::debug!("Reserved {} with mprotect", crate::fmt::WasmPageDisplay(initial_pages));
        unsafe {
            madvise(full_memory, initial_bytes, Advice::LinuxPopulateWrite)?;
        }

        let main_mmap = NonNull::new(full_memory).expect("mmap_anonymous succeeded but returned NULL");
        // Create the dataset hook.
        let this = Self {
            main_mmap,
            initial_pages: Cell::new(initial_pages),
            accessible_pages: Cell::new(initial_pages),
            hook_mmap: Cell::new(ptr::null_mut()),
            hook_mmap_len: Cell::new(0),
            hook_wasm_ptr: Cell::new(0),
            hook_dataset_desc: Cell::new(dataset.descriptor()),
            is_tied: Cell::new(false),
            rank,
        };

        // On error `this` is going to be dropped and deallocate the main map.
        // Correctness relies on `hook` not setting the hook-relevant fields of self
        // until it successfully performs all the work, or else the drop might try to destroy a hook
        // that doesn't exist.
        this.hook(dataset, 0)?;

        Ok(this)
    }

    pub fn base_ptr(&self) -> NonNull<ffi::c_void> {
        self.main_mmap
    }

    pub fn initial_pages(&self) -> usize {
        self.initial_pages.get()
    }

    pub fn dataset_pages(&self) -> usize {
        self.hook_mmap_len.get().div_ceil(PAGE_SIZE)
    }

    pub fn accessible_pages(&self) -> usize {
        self.accessible_pages.get()
    }

    pub fn max_reserved_bytes(&self) -> usize {
        4 * GIB
    }

    pub fn wasm_data_ptr(&self) -> u32 {
        self.hook_wasm_ptr.get()
    }

    pub fn wasm_data_len(&self) -> u32 {
        self.hook_dataset_desc.get().len() as u32
    }

    pub fn wasm_state_ptr(&self) -> u32 {
        ((self.initial_pages() + self.dataset_pages()) * PAGE_SIZE) as u32
    }

    #[instrument]
    fn grow_accessible_to(&self, new_accessible_bytes: usize) -> Result<(), Error> {
        let new_accessible_pages = new_accessible_bytes.div_ceil(PAGE_SIZE);
        let accessible_pages = self.accessible_pages.get();

        tracing::debug!(
            "request to grow {:?} to {}; current size is {}",
            self.main_mmap,
            crate::fmt::WasmPageDisplay(new_accessible_pages),
            crate::fmt::WasmPageDisplay(accessible_pages),
        );
        assert!(new_accessible_pages > accessible_pages);
        assert!(new_accessible_bytes <= self.max_reserved_bytes());
        let start = unsafe { self.main_mmap.as_ptr().add(accessible_pages * PAGE_SIZE) };
        let len = (new_accessible_pages - accessible_pages) * PAGE_SIZE;
        unsafe { mprotect(start, len, MprotectFlags::READ | MprotectFlags::WRITE)? };
        tracing::debug!("Reserved {len} more bytes with mprotect starting at {start:?}");

        self.accessible_pages.set(new_accessible_pages);

        Ok(())
    }

    #[instrument]
    fn refresh(&self) -> Result<(), Error> {
        use rustix::mm::{madvise, mprotect, Advice, MprotectFlags};
        assert!(!self.hook_mmap.get().is_null());
        let old_accessible = self.accessible_pages();
        let initial_pages = self.initial_pages();
        let dataset_pages = self.dataset_pages();
        // Zero out the initial pages.
        unsafe { self.main_mmap.write_bytes(0, initial_pages * PAGE_SIZE) };
        tracing::debug!(
            "Zeroed out initial pages at {:?}, len {}",
            self.main_mmap,
            crate::fmt::WasmPageDisplay(initial_pages)
        );
        // Remove all allocated memory except for the state page.
        let new_accessible = initial_pages + dataset_pages + 1;
        assert!(new_accessible <= old_accessible);

        unsafe {
            let dealloc_ptr = self.main_mmap.add(new_accessible * PAGE_SIZE).as_ptr();
            let dealloc_len = (old_accessible - new_accessible) * PAGE_SIZE;
            tracing::debug!(
                "Deallocating {} at {:?}",
                crate::fmt::WasmPageDisplay(old_accessible - new_accessible),
                dealloc_ptr,
            );
            if dealloc_len > 0 {
                madvise(dealloc_ptr, dealloc_len, Advice::DontNeed)?;
                mprotect(dealloc_ptr, dealloc_len, MprotectFlags::empty())?; // PROT_NONE
            }
        }
        // Zero out the state page.
        unsafe {
            let state_ptr = self.main_mmap.add((initial_pages + dataset_pages) * PAGE_SIZE);
            state_ptr.write_bytes(0, PAGE_SIZE);
            tracing::debug!(
                "Zeroed out the state page at {:?}, len {}",
                state_ptr,
                crate::fmt::WasmPageDisplay(1)
            );
        }

        self.accessible_pages.set(new_accessible);

        Ok(())
    }

    #[instrument]
    fn reset(&self, initial_pages: usize, dataset: &Dataset) -> Result<(), Error> {
        use rustix::mm::{mprotect, MprotectFlags};
        // Unhook.
        let old_hook_len = self.hook_mmap_len.get();
        self.destroy_hook()?;
        // Now grow the initial pages if necessary.
        let mut current_pages = self.initial_pages.get();
        if current_pages < initial_pages {
            unsafe {
                mprotect(
                    self.main_mmap.as_ptr(),
                    initial_pages * PAGE_SIZE,
                    MprotectFlags::READ | MprotectFlags::WRITE,
                )?;
            }
            tracing::debug!(
                "Reserved {} with mprotect starting at {:?}",
                crate::fmt::WasmPageDisplay(initial_pages),
                self.main_mmap.as_ptr()
            );
            current_pages = initial_pages;
        }
        // ... and zero them out.
        unsafe { self.main_mmap.write_bytes(0, current_pages * PAGE_SIZE) };
        self.initial_pages.set(current_pages);
        self.accessible_pages.set(current_pages);
        // Rehook.
        self.hook(dataset, old_hook_len)?;

        Ok(())
    }

    #[instrument]
    fn hook(&self, dataset: &Dataset, possible_hole: usize) -> Result<(), Error> {
        use rustix::fd::AsRawFd;
        let initial_pages = self.initial_pages.get();
        tracing::debug!(
            "hooking dataset (fd {}) at memory map {:?} (initial {} allocd)",
            dataset.as_raw_fd(),
            self.main_mmap,
            crate::fmt::WasmPageDisplay(initial_pages),
        );
        assert!(self.hook_mmap.get().is_null());

        // The offset within the fd for mmap has to be a multiple of page size, but Dataset cannot guarantee that.
        // We obviously cannot push the offset *forward* or we'd skip data.
        // We therefore align the offset to the *previous* page boundary. To avoid wasm reading garbage
        // we internally push the wasm_ptr forward to point to the actual start of the hooked dataset.
        //
        // This means that the actual pointer won't match with the requested wasm_offset and will not be wasm-paged-aligned.
        // However, it is guaranteed to be aligned to the same boundary the original dataset was.
        let dataset_pages = dataset.len().div_ceil(PAGE_SIZE);
        tracing::debug!(
            "requested dataset len is {}, which is {} in full pages",
            dataset.len(),
            crate::fmt::WasmPageDisplay(dataset_pages)
        );
        let closest_valid_offset = dataset.offset() & !(page_size() - 1);
        let additional_wasm_offset = dataset.offset() - closest_valid_offset;
        let hook_map_len = dataset.len() + additional_wasm_offset;
        tracing::debug!(
            "requested dataset offset is {}, 
        while closest page-aligned offset is: {closest_valid_offset}, 
        making the additional wasm offset: {additional_wasm_offset}
        and increasing the map len to {hook_map_len}",
            dataset.offset()
        );
        let hook_mmap = unsafe {
            mmap(
                self.main_mmap.as_ptr().add(initial_pages * PAGE_SIZE),
                hook_map_len,
                ProtFlags::READ,
                MapFlags::PRIVATE | MapFlags::FIXED,
                dataset,
                closest_valid_offset as u64,
            )
        }?;
        tracing::debug!("memory mapped {} bytes at memory address {hook_mmap:?}", hook_map_len,);

        // Register the hook internally *now* - in case of any errors we need to have this
        // locked in for the Drop to deallocate the mapping from above!
        self.hook_mmap.set(hook_mmap);
        self.hook_mmap_len.set(hook_map_len);
        self.hook_wasm_ptr
            .set((initial_pages * PAGE_SIZE) as u32 + additional_wasm_offset as u32);
        self.hook_dataset_desc.set(dataset.descriptor());

        unsafe {
            madvise(hook_mmap, hook_map_len, Advice::Sequential)?;
        };

        // Plug in the hole.
        // After unmapping a dataset *all* of the mappings in that area are destroyed.
        // We mapped a new one, but if it is shorter than the previous one then there will be
        // an unmapped hole right after.
        // This only happens if the previous hook was larger.
        if possible_hole > hook_map_len {
            let new_hook_full_os_pages = hook_map_len.div_ceil(page_size());
            let old_hook_full_os_pages = possible_hole.div_ceil(page_size());
            let hole_pages = old_hook_full_os_pages - new_hook_full_os_pages;
            let hole_start = unsafe { hook_mmap.add(new_hook_full_os_pages * page_size()) };
            tracing::debug!("patching a hole of {hole_pages} OS pages starting at {hole_start:?} due to len difference (old {possible_hole}, new {hook_map_len})");
            unsafe {
                mmap_anonymous(
                    hole_start,
                    hole_pages * page_size(),
                    ProtFlags::empty(), // PROT_NONE
                    MapFlags::PRIVATE | MapFlags::NORESERVE | MapFlags::FIXED,
                )
            }?;
        };

        // Initialize the state page.
        // Dataset pages might have changed since the offset was added!
        let dataset_pages = hook_map_len.div_ceil(PAGE_SIZE);
        tracing::debug!(
            "state starts at offset: {}",
            (initial_pages + dataset_pages) * PAGE_SIZE
        );
        let state_start = unsafe { hook_mmap.add(dataset_pages * PAGE_SIZE) };
        tracing::debug!("raw address to mprotect: {state_start:?}");
        let state_len = PAGE_SIZE;
        unsafe {
            mprotect(state_start, state_len, MprotectFlags::READ | MprotectFlags::WRITE)?;
            state_start.write_bytes(0, state_len);
        };

        let total_accessible_pages = initial_pages + dataset_pages + 1;
        self.accessible_pages.set(total_accessible_pages);

        Ok(())
    }

    #[instrument]
    fn destroy_hook(&self) -> Result<(), Error> {
        use rustix::mm::munmap;
        // We need to deallocate all memory after the dataset.
        let hook_ptr = self.hook_mmap.get();
        assert!(!hook_ptr.is_null());
        tracing::debug!("unmapping the dataset at {:?}", hook_ptr);
        let old_accessible = self.accessible_pages.get();
        let initial_pages = self.initial_pages.get();
        // Remove all allocated memory and the dataset.
        assert!(initial_pages <= old_accessible);

        unsafe {
            let dealloc_len = (old_accessible - initial_pages) * PAGE_SIZE;
            tracing::debug!(
                "Deallocating {} at {:?}",
                crate::fmt::WasmPageDisplay(old_accessible - initial_pages),
                hook_ptr,
            );
            if dealloc_len > 0 {
                madvise(hook_ptr, dealloc_len, Advice::DontNeed)?;
                // PROT_NONE
                mprotect(hook_ptr, dealloc_len, MprotectFlags::empty())?;
            }
        }
        // Remove the dataset memory map.
        unsafe {
            munmap(self.hook_mmap.get(), self.hook_mmap_len.get())?;
        }
        self.hook_mmap.set(ptr::null_mut());
        // The rest of the hook state does not matter.
        self.accessible_pages.set(initial_pages);
        Ok(())
    }
}

impl Drop for MmapMemory {
    fn drop(&mut self) {
        use rustix::mm::munmap;
        let hook_ptr = self.hook_mmap.get();
        if !hook_ptr.is_null() {
            tracing::debug!("unmapping the dataset at {:?}", hook_ptr);
            unsafe {
                munmap(hook_ptr, self.hook_mmap_len.get()).expect("dataset munmap failed");
            }
        }
        unsafe {
            munmap(self.main_mmap.as_ptr(), Self::TOTAL_LEN).expect("main munmap failed");
        }
    }
}

unsafe impl wasmtime::LinearMemory for WasmMmapMemory {
    #[instrument(level = "debug")]
    fn byte_size(&self) -> usize {
        self.0.accessible_pages() * PAGE_SIZE
    }

    #[instrument(level = "debug")]
    fn maximum_byte_size(&self) -> Option<usize> {
        Some(self.0.max_reserved_bytes())
    }

    #[instrument(level = "info")]
    fn grow_to(&mut self, new_size: usize) -> anyhow::Result<()> {
        if new_size > self.0.max_reserved_bytes() {
            Err(wasmtime::Error::new(std::io::Error::new(
                std::io::ErrorKind::OutOfMemory,
                "out of assigned wasm memory",
            )))
        } else if new_size <= self.byte_size() {
            Ok(())
        } else {
            self.0.grow_accessible_to(new_size)?;
            Ok(())
        }
    }

    #[instrument(level = "debug")]
    fn as_ptr(&self) -> *mut u8 {
        self.0.main_mmap.as_ptr().cast()
    }

    #[instrument(level = "debug")]
    fn wasm_accessible(&self) -> std::ops::Range<usize> {
        let start = self.as_ptr();
        let end = unsafe { start.add(MmapMemory::TOTAL_LEN) };
        start as usize..end as usize
    }
}

unsafe impl wasmtime::MemoryCreator for WasmMemoryCreator {
    fn new_memory(
        &self,
        ty: wasmtime::MemoryType,
        minimum: usize,
        maximum: Option<usize>,
        reserved_size_in_bytes: Option<usize>,
        guard_size_in_bytes: usize,
    ) -> anyhow::Result<Box<dyn wasmtime::LinearMemory>, String> {
        tracing::debug!("Got the following memory creation request: {ty:?}, min:{minimum}, max:{maximum:?}, reserved:{reserved_size_in_bytes:?}, guard:{guard_size_in_bytes}");
        if ty.is_64() {
            return Err("64-bit wasm is not supported".to_string());
        }
        if ty.is_shared() {
            return Err("Shared memory is not supported".to_string());
        }
        if ty.minimum() as usize > MAX_PAGE_COUNT {
            return Err(format!("Request for at least {} pages is unreasonable", ty.minimum()));
        }

        let raw_memory = LOCAL_MEMORY_MANAGER
            .with_borrow_mut(|mgr| mgr.serve_memory_request(ty.minimum() as usize))
            .map_err(|err| err.to_string())?;

        Ok(Box::new(WasmMmapMemory::new(raw_memory)))
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    SyscallError(#[from] rustix::io::Errno),
    #[error("memory at address 0x{:x} is not managed by this thread", .0.as_ptr() as usize)]
    CrossThreadMemoryRequest(NonNull<ffi::c_void>),
    #[error(
        "wasm memory request failed: all existing memories are in use and the virtual memory limit has been reached"
    )]
    VirtualMemoryLimitExceeded,
}

// There is no actual access to the memory in the error,
// we just use it for printing debug messages.
unsafe impl Send for Error {}
unsafe impl Sync for Error {}
