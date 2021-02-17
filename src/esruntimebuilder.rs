use crate::esruntime::{EsRuntime, FetchResponseProvider};
use crate::esscript::EsScript;
use crate::features::fetch::request::FetchRequest;
use crate::features::fetch::response::FetchResponse;
use crate::quickjscontext::QuickJsContext;
use crate::quickjsruntime::{ModuleScriptLoader, NativeModuleLoader};
use std::sync::Arc;
use std::time::Duration;

/// the EsRuntimeBuilder is used to init an EsRuntime
/// # Example
/// ```rust
/// use quickjs_runtime::esruntimebuilder::EsRuntimeBuilder;
/// // init a rt which may use 16MB of memory
/// let rt = EsRuntimeBuilder::new()
/// .memory_limit(1024*1024*16)
/// .build();
/// ```
pub struct EsRuntimeBuilder {
    pub(crate) opt_module_script_loader: Option<Box<ModuleScriptLoader>>,
    pub(crate) opt_native_module_loader: Option<Box<dyn NativeModuleLoader + Send>>,
    pub(crate) opt_fetch_response_provider: Option<Box<FetchResponseProvider>>,
    pub(crate) opt_memory_limit_bytes: Option<u64>,
    pub(crate) opt_gc_threshold: Option<u64>,
    pub(crate) opt_max_stack_size: Option<u64>,
    pub(crate) opt_gc_interval: Option<Duration>,
}

impl EsRuntimeBuilder {
    /// build an EsRuntime
    pub fn build(self) -> Arc<EsRuntime> {
        EsRuntime::new(self)
    }

    /// init a new EsRuntimeBuilder
    pub fn new() -> Self {
        Self {
            opt_module_script_loader: None,
            opt_native_module_loader: None,
            opt_fetch_response_provider: None,
            opt_memory_limit_bytes: None,
            opt_gc_threshold: None,
            opt_max_stack_size: None,
            opt_gc_interval: None,
        }
    }

    /// add a script loaders which will be used to load modules when they are imported from script
    /// # Example
    /// ```rust
    /// use quickjs_runtime::esscript::EsScript;
    /// use quickjs_runtime::esruntimebuilder::EsRuntimeBuilder;
    /// use quickjs_runtime::quickjscontext::QuickJsContext;
    /// fn load_module(_q_ctx: &QuickJsContext, base: &str, name: &str) -> Option<EsScript> {
    ///     // you should load your modules from files here
    ///     // please note that you need to return the name as absolute_path in the returned script struct
    ///     // return None if module is not found
    ///     use quickjs_runtime::quickjscontext::QuickJsContext;
    /// Some(EsScript::new(name, "export const foo = 12;"))
    /// }
    /// fn main(){
    ///     let rt = EsRuntimeBuilder::new()
    ///         .module_script_loader(load_module)
    ///         .build();
    /// }
    /// ```
    pub fn module_script_loader<M>(mut self, loader: M) -> Self
    where
        M: Fn(&QuickJsContext, &str, &str) -> Option<EsScript> + Send + Sync + 'static,
    {
        self.opt_module_script_loader = Some(Box::new(loader));
        self
    }

    /// add a module loader which can load native functions and proxy classes
    /// # Example
    /// ```rust
    /// use quickjs_runtime::esruntimebuilder::EsRuntimeBuilder;
    /// use quickjs_runtime::quickjsruntime::NativeModuleLoader;
    /// use quickjs_runtime::valueref::JSValueRef;
    /// use quickjs_runtime::quickjscontext::QuickJsContext;
    /// use quickjs_runtime::quickjs_utils::functions;
    /// use quickjs_runtime::quickjs_utils::primitives::{from_bool, from_i32};
    /// use quickjs_runtime::reflection::Proxy;
    /// use quickjs_runtime::esscript::EsScript;
    ///
    /// struct MyModuleLoader{}
    /// impl NativeModuleLoader for MyModuleLoader {
    ///     fn has_module(&self, _q_ctx: &QuickJsContext,module_name: &str) -> bool {
    ///         module_name.eq("my_module")
    ///     }
    ///
    ///     fn get_module_export_names(&self, _q_ctx: &QuickJsContext, _module_name: &str) -> Vec<&str> {
    ///         vec!["someVal", "someFunc", "SomeClass"]
    ///     }
    ///
    ///     fn get_module_exports(&self, q_ctx: &QuickJsContext, _module_name: &str) -> Vec<(&str, JSValueRef)> {
    ///         
    ///         let js_val = from_i32(1470);
    ///         let js_func = functions::new_function_q(
    ///             q_ctx,
    ///             "someFunc", |_q_ctx, _this, _args| {
    ///                 return Ok(from_i32(432));
    ///             }, 0)
    ///             .ok().unwrap();
    ///         let js_class = Proxy::new()
    ///             .name("SomeClass")
    ///             .static_method("doIt", |_q_ctx, _args|{
    ///                 return Ok(from_i32(185));
    ///             })
    ///             .install(q_ctx, false)
    ///             .ok().unwrap();
    ///
    ///         vec![("someVal", js_val), ("someFunc", js_func), ("SomeClass", js_class)]
    ///     }
    /// }
    ///
    /// let rt = EsRuntimeBuilder::new()
    /// .native_module_loader(MyModuleLoader{})
    /// .build();
    ///
    /// rt.eval_module_sync(EsScript::new("test_native_mod.es", "import {someVal, someFunc, SomeClass} from 'my_module';\nlet i = (someVal + someFunc() + SomeClass.doIt());\nif (i !== 2087){throw Error('i was not 2087');}")).ok().expect("script failed");
    /// ```
    pub fn native_module_loader<M: NativeModuleLoader + Send + 'static>(
        mut self,
        loader: M,
    ) -> Self {
        self.opt_native_module_loader = Some(Box::new(loader));
        self
    }

    /// Provide a fetch response provider in order to make the fetch api work in the EsRuntime
    /// # Example
    /// ```rust
    ///
    /// use quickjs_runtime::esruntimebuilder::EsRuntimeBuilder;
    /// use quickjs_runtime::features::fetch::response::FetchResponse;
    /// use quickjs_runtime::features::fetch::request::FetchRequest;
    /// use quickjs_runtime::esscript::EsScript;
    /// use std::time::Duration;   
    ///
    /// struct SimpleResponse{
    ///     read_done: bool
    /// }
    ///
    /// impl SimpleResponse {
    ///     fn new(_req: &FetchRequest) -> Self {
    ///         Self{read_done:false}
    ///     }
    /// }
    ///
    /// impl FetchResponse for SimpleResponse {
    ///     fn get_http_status(&self) -> u16 {
    ///         200
    ///     }
    ///
    ///     fn get_header(&self,name: &str) -> Option<&str> {
    ///         unimplemented!()
    ///     }
    ///
    ///     fn read(&mut self) -> Option<Vec<u8>> {
    ///         if self.read_done {
    ///             None
    ///         } else {
    ///             self.read_done = true;      
    ///             Some("Hello world".as_bytes().to_vec())
    ///         }
    ///     }
    /// }
    ///
    /// let rt = EsRuntimeBuilder::new()
    /// .fetch_response_provider(|req| {Box::new(SimpleResponse::new(req))})
    /// .build();
    ///
    /// let res_prom = rt.eval_sync(EsScript::new("test_fetch.es", "(fetch('something')).then((fetchRes) => {return fetchRes.text();});")).ok().expect("script failed");
    /// let res = res_prom.get_promise_result_sync(Duration::from_secs(1)).ok().expect("promise timed out");
    /// let str_esvf = res.ok().expect("promise did not resolve ok");
    /// assert_eq!(str_esvf.get_str(), "Hello world");
    /// ```
    pub fn fetch_response_provider<P>(mut self, provider: P) -> Self
    where
        P: Fn(&FetchRequest) -> Box<dyn FetchResponse + Send> + Send + Sync + 'static,
    {
        self.opt_fetch_response_provider = Some(Box::new(provider));
        self
    }

    /// set max memory the runtime may use
    pub fn memory_limit(mut self, bytes: u64) -> Self {
        self.opt_memory_limit_bytes = Some(bytes);
        self
    }

    /// number of allocations before gc is run
    pub fn gc_threshold(mut self, size: u64) -> Self {
        self.opt_gc_threshold = Some(size);
        self
    }

    /// set a max stack size
    pub fn max_stack_size(mut self, size: u64) -> Self {
        self.opt_max_stack_size = Some(size);
        self
    }

    pub fn gc_interval(mut self, interval: Duration) -> Self {
        self.opt_gc_interval = Some(interval);
        self
    }
}

impl Default for EsRuntimeBuilder {
    fn default() -> Self {
        EsRuntimeBuilder::new()
    }
}
