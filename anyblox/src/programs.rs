use std::collections::HashMap;
use self::native::NativeProgram;
use self::wasm::WasmProgram;
use crate::config::Config;
use lru::LruCache;
use std::sync::{Mutex, RwLock};
use thiserror::Error;
use tracing::instrument;
pub mod native;
pub mod sinks;
pub mod wasm;

pub struct WasmProgramCache {
    lru: LruCache<blake3::Hash, WasmProgram>,
    total_size: usize,
    size_limit: usize,
}

pub struct Programs {
    engine: wasmtime::Engine,
    cache: Mutex<WasmProgramCache>,
}

impl Programs {
    pub fn new(engine: wasmtime::Engine, config: &Config) -> Result<Self, Error> {
        Ok(Self {
            engine,
            cache: Mutex::new(WasmProgramCache::new(config.wasm_cache_limit())),
        })
    }

    pub fn get_native(&self, name: &str) -> Result<native::NativeProgram, Error> {
        let name = name.to_lowercase();
        match &*name {
            "urn:anyblox:rle" => Ok(NativeProgram::rle()),
            "urn:anyblox:rle_simd" => Ok(NativeProgram::rle_simd()),
            "urn:anyblox:rle_simd_avx2" => Ok(NativeProgram::rle_simd_avx2()),
            "urn:anyblox:rle_simd_sse2" => Ok(NativeProgram::rle_simd_sse2()),
            "urn:anyblox:rle_simd_stateless" => Ok(NativeProgram::rle_simd_stateless()),
            //"trunc8" => Ok(NativeProgram::trunc8()),
            "urn:anyblox:taxpayer_fsst" => Ok(NativeProgram::taxpayer_fsst()),
            "urn:anyblox:taxpayer_libfsst" => Ok(NativeProgram::taxpayer_libfsst()),
            "urn:anyblox:tpch_vortex" => Ok(NativeProgram::tpch_vortex()),
            "urn:anyblox:rle_linestatus" => Ok(NativeProgram::rle_linestatus()),
            _ => Err(Error::NoNativeImpl(name)),
        }
    }

    #[instrument(skip(self, code))]
    pub fn load_wasm_blocking(&self, code: &[u8], name: &str) -> Result<wasm::WasmProgram, Error> {
        get_or_else(&self.cache, code, || {
            WasmProgram::new(code, &self.engine)
                .map_err(|err| Error::WasmCompilationError(err.to_string(), name.to_string()))
        })
    }
}

impl WasmProgramCache {
    pub fn new(size_limit: usize) -> Self {
        Self {
            lru: LruCache::unbounded(),
            total_size: 0,
            size_limit,
        }
    }
}

#[instrument(skip(cache, code, f))]
pub fn get_or_else<F>(cache: &Mutex<WasmProgramCache>, code: &[u8], f: F) -> Result<wasm::WasmProgram, Error>
where
    F: FnOnce() -> Result<wasm::WasmProgram, Error>,
{
    let hash = blake3::hash(code);
    let mut cache = cache.lock().expect("wasm lru lock");
    tracing::debug!("wasm program lookup by hash: {}", hash);
    if let Some(program) = cache.lru.get(&hash) {
        tracing::debug!("cache hit");
        Ok(program.clone())
    } else {
        tracing::info_span!("wasm cache miss").in_scope(|| {
            let program = f()?;
            tracing::info_span!("wasm cache miss - lru manipulation").in_scope(|| {
                if program.exe_size() > cache.size_limit {
                    return Err(Error::CompiledWasmTooLarge(program.exe_size(), cache.size_limit));
                }
                cache.total_size += program.exe_size();
                tracing::debug!("new size of cache: {}", cache.total_size);
                while cache.total_size > cache.size_limit {
                    let (hash, dropped_program) = cache.lru.pop_lru().unwrap();
                    tracing::debug!("dropping program with hash {}", hash);
                    cache.total_size -= dropped_program.exe_size();
                }
                cache.lru.put(hash, program.clone());
                Ok(())
            })?;
            Ok(program)
        })
    }
}
#[derive(Debug, Error, Clone)]
pub enum Error {
    #[error("no native program named '{0}' found")]
    NoNativeImpl(String),
    #[error("name needs to be explicitly specified for wasm programs")]
    NameRequired,
    #[error("error compiling wasm program '{1}': {0}")]
    WasmCompilationError(String, String),
    #[error("compiled wasm size ({0}B) exceeds the max cache limit ({1}B)")]
    CompiledWasmTooLarge(usize, usize),
}
