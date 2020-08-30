use crate::quickjsruntime::{OwnedValueRef, TAG_NULL, TAG_UNDEFINED};

pub(crate) mod arrays;
pub(crate) mod bigints;
pub(crate) mod functions;
pub(crate) mod modules;
pub(crate) mod objects;
pub(crate) mod primitives;
pub(crate) mod promises;
pub(crate) mod reflection;
pub(crate) mod typedarrays;

/// todo
/// runtime and context in thread_local here
/// all function (where applicable) get an Option<QuickJSRuntime> which if None will be gotten from the thread_local
/// every function which returns a q::JSValue will return a OwnedValueRef to ensure values are freed on drop

pub fn new_undefined() -> OwnedValueRef {
    OwnedValueRef::new(q::JSValue {
        u: q::JSValueUnion { int32: 0 },
        tag: TAG_UNDEFINED,
    })
}

pub fn new_null() -> OwnedValueRef {
    OwnedValueRef::new(q::JSValue {
        u: q::JSValueUnion { int32: 0 },
        tag: TAG_NULL,
    })
}

pub fn is_null(value_ref: &OwnedValueRef) -> bool {
    value_ref.value.tag == TAG_NULL
}

pub fn is_undefined(value_ref: &OwnedValueRef) -> bool {
    value_ref.value.tag == TAG_UNDEFINED
}

pub fn is_null_or_undefined(value_ref: &OwnedValueRef) -> bool {
    is_null(value_ref) || is_undefined(value_ref)
}
