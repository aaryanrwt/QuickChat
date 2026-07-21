use anyhow::Result;
use wasmtime::{Config, Engine, Linker, Module, Store};
use wasmtime_wasi::p1::WasiP1Ctx;
use wasmtime_wasi::WasiCtxBuilder;

pub struct PluginContext {
    wasi_ctx: WasiP1Ctx,
}

pub struct PluginManager {
    engine: Engine,
    linker: Linker<PluginContext>,
}

impl PluginManager {
    pub fn new() -> Result<Self> {
        let mut config = Config::new();
        config.wasm_backtrace_details(wasmtime::WasmBacktraceDetails::Enable);
        let engine = Engine::new(&config)?;

        let mut linker = Linker::new(&engine);
        wasmtime_wasi::p1::add_to_linker_sync(&mut linker, |ctx: &mut PluginContext| {
            &mut ctx.wasi_ctx
        })?;

        // Link host functions
        linker.func_wrap(
            "quickchat_host",
            "host_log",
            |mut _caller: wasmtime::Caller<'_, PluginContext>, level: i32, ptr: i32, len: i32| {
                // Implementation to read from WASM memory and log
                println!("Plugin log level {}: pointer {} len {}", level, ptr, len);
            },
        )?;

        Ok(Self { engine, linker })
    }

    pub fn load_plugin(&self, wasm_bytes: &[u8]) -> Result<Store<PluginContext>> {
        let module = Module::new(&self.engine, wasm_bytes)?;

        let mut builder = WasiCtxBuilder::new();
        // Strict Capability Sandboxing:
        // - Do NOT inherit host env vars
        // - Preopen a specific plugin temporary directory, restricting host fs access
        // - Inherit stdio only for debugging purposes
        builder.inherit_stdio();
        builder.env("QUICKCHAT_VERSION", "3.0.0");

        let wasi = builder.build_p1();
        let mut store = Store::new(&self.engine, PluginContext { wasi_ctx: wasi });

        let _instance = self.linker.instantiate(&mut store, &module)?;

        Ok(store)
    }
}
