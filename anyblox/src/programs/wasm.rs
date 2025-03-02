use std::{rc::Rc, sync::Arc};

use crate::{
    data_bufs::Dataset,
    wasm::mem::{MmapMemory, WasmMemoryManager},
};
use decoder_lib::ffi_utils::WasmPtr;
use wasmtime::*;

#[derive(Debug, Clone)]
pub struct WasmProgram {
    module: Module,
    exe_size: usize,
}

/// Signature is:
/// ```
/// (
///     data_ptr: u32 /* interpreted as *const TData by consumer */,
///     data_len: u32,
///     start_tuple: u32,
///     tuple_count: u32,
///     state_ptr: u32 /* interpreted as *mut TState by consumer */,
///     projection_mask_1: u32,
///     projection_mask_2: u32 /* interpreted as the ColumnProjection struct*/
/// ) -> u32 /* interpreted as *const RecordBatch by the runtime */
/// ```
type DecodeFunc = TypedFunc<(u32, u32, u32, u32, u32, u32, u32), u32>;

pub struct ActiveWasmProgram {
    instance: Instance,
    decode_f: DecodeFunc,
    store: Store<()>,
    _linker: Linker<()>,
    schema: Arc<arrow::datatypes::Schema>,
    memory_manager: Arc<WasmMemoryManager>,
    memory_instance: Rc<MmapMemory>,
    forced_offset: u32,
    peak_memory_usage: usize,
}

impl WasmProgram {
    pub fn new(bytes: &[u8], engine: &Engine) -> Result<Self, wasmtime::Error> {
        let init_start = std::time::Instant::now();
        let module = Module::new(engine, bytes)?;

        let init_time = init_start.elapsed();
        tracing::info!("Wasm compilation elapsed: {: >8} ms", init_time.as_secs_f32() * 1_000.0);
        let res = module.resources_required();
        tracing::info!(
            "Module resources data: {} {:?} {} {:?}",
            res.num_memories,
            res.max_initial_memory_size,
            res.num_tables,
            res.max_initial_table_size
        );
        Ok(Self {
            module,
            exe_size: bytes.len(),
        })
    }

    pub fn exe_size(&self) -> usize {
        self.exe_size
    }

    pub fn prepare(
        &self,
        engine: &Engine,
        dataset: Dataset,
        schema: Arc<arrow::datatypes::Schema>,
        memory_manager: Arc<WasmMemoryManager>,
    ) -> Result<ActiveWasmProgram> {
        let mut store = Store::new(engine, ());
        let mut linker = Linker::new(engine);
        let init_start = std::time::Instant::now();
        let init_span = tracing::info_span!("store init");
        linker.func_wrap("host", "log", |mut caller: Caller<'_, _>, ptr: u32, len: u32| {
            let mem = match caller.get_export("memory") {
                Some(Extern::Memory(mem)) => mem,
                _ => anyhow::bail!("failed to find host memory"),
            };
            let start = ptr as usize;
            let end = (ptr + len) as usize;
            let mem_data = mem.data(&mut caller);
            let str = std::str::from_utf8(&mem_data[start..end])?;
            {
                let _ = tracing::debug_span!("wasm_log").enter();
                tracing::debug!("{str}");
            };

            Ok(())
        })?;
        linker.func_wrap(
            "host",
            "panic",
            |mut caller: Caller<'_, _>, ptr: u32, len: u32| -> Result<(), _> {
                let mem = match caller.get_export("memory") {
                    Some(Extern::Memory(mem)) => mem,
                    _ => anyhow::bail!("failed to find host memory"),
                };
                let str = if ptr > 0 {
                    let start = ptr as usize;
                    let end = (ptr + len) as usize;
                    let mem_data = mem.data(&mut caller);
                    std::str::from_utf8(&mem_data[start..end])?
                } else {
                    "[NULL]"
                };
                {
                    let _ = tracing::error_span!("wasm_panic").enter();
                    tracing::error!("WASM module panicked with: {str}");
                };

                anyhow::bail!("Wasm module panicked!")
            },
        )?;
        drop(init_span);
        let init_time = init_start.elapsed();

        tracing::info!("Wasm init elapsed: {: >8} ms", init_time.as_secs_f32() * 1_000.0);

        let load_start = std::time::Instant::now();
        let (instance, memory) =
            memory_manager.create_memory_scope(dataset, || linker.instantiate(&mut store, &self.module))?;
        let decode_f = instance.get_typed_func(&mut store, "anyblox_decode")?;
        tracing::info!(
            "Wasm load elapsed: {: >8} ms",
            load_start.elapsed().as_secs_f32() * 1_000.0
        );

        Ok(ActiveWasmProgram {
            instance,
            decode_f,
            store,
            _linker: linker,
            schema,
            memory_instance: memory,
            memory_manager,
            forced_offset: 0,
            peak_memory_usage: 0,
        })
    }
}

impl ActiveWasmProgram {
    pub fn base_ptr(&mut self) -> *const u8 {
        self.get_memory().data_ptr(&self.store)
    }

    fn get_memory(&mut self) -> Memory {
        self.instance.get_memory(&mut self.store, "memory").unwrap()
    }

    pub fn schema(&self) -> Arc<arrow::datatypes::Schema> {
        self.schema.clone()
    }

    pub fn set_forced_offset(&mut self, offset: u32) {
        self.forced_offset = offset;
    }

    pub fn peak_memory_usage(&self) -> usize {
        self.peak_memory_usage
    }

    pub fn decode_batch(
        &mut self,
        start_tuple: usize,
        tuple_count: usize,
        projection: decoder_lib::column_projection::ColumnProjection,
    ) -> Result<&decoder_lib::arrow::ArrowArray> {
        let batch_ptr: u32 = self.decode_f.call(
            &mut self.store,
            (
                self.memory_instance.wasm_data_ptr() + self.forced_offset,
                self.memory_instance.wasm_data_len() - self.forced_offset,
                start_tuple as u32,
                tuple_count as u32,
                self.memory_instance.wasm_state_ptr(),
                projection.mask_1(),
                projection.mask_2(),
            ),
        )?;
        let base_ptr = self.base_ptr();
        let batch_wasm_ptr = unsafe { WasmPtr::from(batch_ptr).as_native(base_ptr) };
        let batch_ptr = batch_wasm_ptr.cast::<decoder_lib::arrow::ArrowArray>();
        let batch_ref = unsafe { &*batch_ptr };
        self.peak_memory_usage = std::cmp::max(
            self.peak_memory_usage,
            self.memory_instance.accessible_pages() * crate::wasm::PAGE_SIZE,
        );

        Ok(batch_ref)
    }
}

impl Drop for ActiveWasmProgram {
    fn drop(&mut self) {
        self.memory_manager
            .notify_memory_drop(self.memory_instance.as_ref())
            .expect("memory drop for active wasm program");
    }
}
