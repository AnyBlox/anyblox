#[repr(C)]
#[derive(Debug, Clone)]
pub struct WasmString {
    pub ptr: WasmPtr,
    pub len: u32,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub enum WasmOption<T> {
    Some(T),
    None,
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WasmPtr(pub(crate) u32);

impl WasmString {
    pub fn from_static_str(value: &'static str) -> Self {
        Self {
            ptr: WasmPtr(value.as_ptr() as u32),
            len: value.len() as u32,
        }
    }
}

impl<T> WasmOption<T> {
    pub fn from_option(value: Option<T>) -> Self {
        match value {
            Some(x) => Self::Some(x),
            None => Self::None,
        }
    }

    pub fn into_option(self) -> Option<T> {
        match self {
            Self::Some(x) => Some(x),
            Self::None => None,
        }
    }

    pub fn as_option(&self) -> Option<&T> {
        match self {
            Self::Some(x) => Some(x),
            Self::None => None,
        }
    }
}

impl WasmPtr {
    pub const NULL: WasmPtr = WasmPtr(0);

    pub const fn new(ptr: u32) -> Self {
        Self(ptr)
    }

    pub fn is_null(&self) -> bool {
        self.0 == 0
    }

    pub unsafe fn as_native(&self, base_ptr: *const u8) -> *const u8 {
        base_ptr.add(self.0 as usize)
    }

    pub unsafe fn as_native_mut(&self, base_ptr: *mut u8) -> *mut u8 {
        base_ptr.add(self.0 as usize)
    }
}

impl From<&'static str> for WasmString {
    fn from(value: &'static str) -> Self {
        Self::from_static_str(value)
    }
}

impl<T> From<Option<T>> for WasmOption<T> {
    fn from(value: Option<T>) -> Self {
        Self::from_option(value)
    }
}

impl<T> From<WasmOption<T>> for Option<T> {
    fn from(value: WasmOption<T>) -> Self {
        value.into_option()
    }
}

impl From<u32> for WasmPtr {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl<T> From<*const T> for WasmPtr {
    fn from(value: *const T) -> Self {
        Self(value as u32)
    }
}

impl<T> From<*mut T> for WasmPtr {
    fn from(value: *mut T) -> Self {
        Self(value as u32)
    }
}
