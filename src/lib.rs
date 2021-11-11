//! # quickjs_runtime
//! This crate is made up of two main parts:
//! * thread-safe utils and wrappers
//!   you can call these from any thread, all logic is directed to a single worker-thread which talks to the quickjs API
//! * quickjs bindings and utils
//!   these talk to the quickjs API directly and need to run in the same thread as the Runtime
//!
//! ## Noteworthy structs
//!
//! These are the structs you'll use the most
//!
//! | Thread safe | Abstracted in hirofa_utils as | Runtime Thread-local | Abstracted in hirofa_utils as |
//! | --- | --- | --- |
//! | [QuickjsRuntimeFacade](facades/struct.QuickjsRuntimeFacade.html) the 'starting point' | [JsRuntimeFacade](https://hirofa.github.io/utils/hirofa_utils/js_utils/facades/trait.JsRuntimeFacade.html) | [QuickJsRuntimeAdapter](quickjsruntimeadapter/struct.QuickJsRuntimeAdapter.html) the wrapper for all things quickjs | [JsRuntimeAdapter](https://hirofa.github.io/utils/hirofa_utils/js_utils/adapters/trait.JsRuntimeAdapter.html) |
//! | - | - | [QuickJsRealmAdapter](quickjsrealmadapter/struct.QuickJsRealmAdapter.html) a realm or context | [JsRealmAdapter](https://hirofa.github.io/utils/hirofa_utils/js_utils/adapters/trait.JsRealmAdapter.html) |
//! | - | [JsValueFacade](https://hirofa.github.io/utils/hirofa_utils/js_utils/facades/values/enum.JsValueFacade.html) copy of or reference to a value in the Runtime | [JSValueRef](valueref/struct.JSValueRef.html) reference counting pointer to a Value | [JsValueAdapter](https://hirofa.github.io/utils/hirofa_utils/js_utils/adapters/trait.JsValueAdapter.html) |
//!
//! ## Doing something in the runtime worker thread
//!
//! You always start with building a new [EsRuntime](esruntime/struct.EsRuntime.html)
//!
//! ```dontrun
//! use quickjs_runtime::esruntimebuilder::EsRuntimeBuilder;
//! let rt: EsRuntime = EsRuntimeBuilder::new().build();
//! ```
//!
//! [EsRuntime](https://hirofa.github.io/quickjs_es_runtime/quickjs_runtime/esruntime/struct.EsRuntime.html) has plenty public methods you can check out but one of the things you'll need to understand is how to communicate with the QuickJsRuntime
//! This is done by adding a job to the [EventQueue](utils/single_threaded_event_queue/struct.SingleThreadedEventQueue.html) of the [EsRuntime](esruntime/struct.EsRuntime.html)
//!
//! ```dontrun
//! use quickjs_runtime::quickjsruntime::QuickJsRuntime;
//! let res = rt.add_to_event_queue(|q_js_rt: &QuickJsRuntime| {
//!    // this will run in the Worker thread, here we can use the quickjs API
//!    return true;
//! }).await;
//! ```
//! All the non-sync functions return a Future so you can .await them from async functions.
//!
//! In order to do something and get the result synchronously you can use the sync variant
//! ```dontrun
//! use quickjs_runtime::quickjsruntime::QuickJsRuntime;
//! let res = rt.add_to_event_queue_sync(|q_js_rt: &QuickJsRuntime| {
//!    // this will run in the Worker thread, here we can use the quickjs API
//!    return 1;
//! });
//! ```
//!
//! For more details and examples please explore the packages below

#[macro_use]
extern crate lazy_static;

#[macro_export]
macro_rules! es_args {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec: Vec<crate::esvalue::EsValueFacade> = Vec::new();
            $(
                temp_vec.push(crate::esvalue::EsValueConvertible::to_es_value_facade($x));
            )*
            temp_vec
        }
    };
}

pub mod builder;
pub mod esvalue;
pub mod facades;
pub mod features;
pub mod quickjs_utils;
pub mod quickjsrealmadapter;
pub mod quickjsruntimeadapter;
pub mod reflection;
pub mod runtimefacade_utils;
pub mod valueref;

#[cfg(test)]
pub mod tests {

    #[test]
    fn test_macro() {
        let _args = es_args!(1, 2i32, true, "sdf".to_string());
    }
}
