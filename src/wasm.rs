use super::*;
use byteorder::{LittleEndian, ReadBytesExt};
use wasmer::{Instance, Memory, MemoryView, Module, Store, Value, ValueType, WasmSlice};
use wasmer_cache::{Cache, FileSystemCache, Hash};
use wasmer_wasix::{WasiEnv, WasiFunctionEnv};

pub struct Engine {
    store: wasmer::Store,
    wasi_env: WasiFunctionEnv,
}

fn read<T: ValueType>(view: &MemoryView<'_>, offset: u64, length: u64) -> Result<Vec<T>> {
    Ok(WasmSlice::new(view, offset, length)?.read_to_vec()?)
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

    pub fn run(&mut self, bytes: &[u8], input: &serde_json::Value) -> Result<serde_json::Value> {
        // Compile wasm
        let module = self.get_or_compile(&bytes)?;

        // Setup wasix
        let import_object = self.wasi_env.import_object(&mut self.store, &module)?;
        let instance = Instance::new(&mut self.store, &module, &import_object)?;
        self.wasi_env
            .initialize(&mut self.store, instance.clone())?;

        // Serialize data
        let serialized = rmp_serde::to_vec(input)?;
        let input_len = (serialized.len() as u32).to_le_bytes();
        let input_bytes = [&input_len[..], &serialized].concat();

        // Get memory from instance
        let heap_start = match instance
            .exports
            .get::<wasmer::Global>("__heap_base")
            .map(|it| it.get(&mut self.store))
        {
            Ok(Value::I32(heap_start)) => heap_start,
            _ => 0x100000,
        };

        // Grow memory
        let memory = instance.exports.get::<Memory>("memory")?;
        let pages = (input_bytes.len() / wasmer::WASM_PAGE_SIZE) + 1;
        memory.grow(&mut self.store, pages as u32)?;

        // Write bytes into memory
        {
            let view = memory.view(&self.store);
            view.write(heap_start as u64, &input_bytes)?;
        }

        // Call module and pass pointer
        let function = instance.exports.get_function("main")?;
        let values = function.call(&mut self.store, &[Value::I32(heap_start)])?;
        self.wasi_env.cleanup(&mut self.store, None);

        // Deserialize data from pointer
        match &values[..] {
            [Value::I32(pointer)] => {
                let view = memory.view(&self.store);
                let output_len = {
                    let bytes = read::<u8>(&view, *pointer as u64, 4)?;
                    bytes.as_slice().read_u32::<LittleEndian>()?
                };
                let output_ptr = {
                    let bytes = read::<u8>(&view, *pointer as u64 + 4, 4)?;
                    bytes.as_slice().read_u32::<LittleEndian>()?
                };
                let output_bytes = read::<u8>(&view, output_ptr as u64, output_len as u64)?;
                let output = rmp_serde::from_read(output_bytes.as_slice())?;
                unwrap_output(output)
            }
            _ => Err(error!(
                "Expected pointer to serialized data, got {values:#?}"
            ))?,
        }
    }
}

fn unwrap_output(output: serde_json::Value) -> Result<serde_json::Value> {
    match output.as_object() {
        Some(object) if object.contains_key("Err") => {
            Err(error!(format!("{}", object["Err"]["description"])))?
        }
        Some(object) if object.contains_key("Ok") => Ok(output["Ok"].clone()),
        _ => Ok(output),
    }
}
