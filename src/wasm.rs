use super::*;
use wasmer::{Instance, Module, Store};
use wasmer_cache::{Cache, FileSystemCache, Hash};
use wasmer_wasix::{WasiEnv, WasiFunctionEnv};

pub struct Engine {
    store: wasmer::Store,
    wasi_env: WasiFunctionEnv,
}

impl Engine {
    pub fn new() -> Result<Self> {
        // Setup engine
        let mut store = Store::default();
        let wasi_env = WasiEnv::builder("cloud").finalize(&mut store)?;
        let engine = Self { store, wasi_env };
        Ok(engine)
    }

    fn get_or_compile(&self, bytes: &[u8]) -> Result<Module> {
        // Setup cache
        let mut cache = FileSystemCache::new("target")?;
        let hash = Hash::generate(bytes);

        // Check if exists, otherwise compile it
        let module = match unsafe { cache.load(&self.store, hash) } {
            Ok(module) => module,
            Err(_) => {
                let module = Module::new(&self.store, bytes)?;
                cache.store(hash, &module)?;
                module
            }
        };
        Ok(module)
    }

    pub fn run(&mut self, path: &str, params: &[wasmer::Value]) -> Result<Box<[wasmer::Value]>> {
        // Compile wasm
        let bytes = std::fs::read(path)?;
        let module = self.get_or_compile(&bytes)?;

        // Setup wasix
        let import_object = self.wasi_env.import_object(&mut self.store, &module)?;
        let instance = Instance::new(&mut self.store, &module, &import_object)?;
        self.wasi_env
            .initialize(&mut self.store, instance.clone())?;

        // Get main function and call it with params
        let function = instance.exports.get_function("_start")?;
        let result = function.call(&mut self.store, params)?;
        self.wasi_env.cleanup(&mut self.store, None);

        Ok(result)
    }
}
