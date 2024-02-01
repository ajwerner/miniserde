use crate::error::Result;
use crate::json::{Array, Number, Object};
use crate::ser::{Fragment, Serialize};
use alloc::borrow::{Cow, ToOwned};
use alloc::boxed::Box;
use alloc::string::String;
use core::fmt::{self, Debug};
use core::mem;
use core::str;

/// Any valid JSON value.
///
/// This type has a non-recursive drop implementation so it is safe to build
/// arbitrarily deeply nested instances.
///
/// ```rust
/// use miniserde::json::{Array, Value};
///
/// let mut value = Value::Null;
#[cfg_attr(not(miri), doc = "for _ in 0..100000 {")]
#[cfg_attr(miri, doc = "for _ in 0..40 {")]
///     let mut array = Array::new();
///     array.push(value);
///     value = Value::Array(array);
/// }
/// // no stack overflow when `value` goes out of scope
/// ```
#[derive(Clone)]
pub enum Value {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Array(Array),
    Object(Object),
}

impl Default for Value {
    /// The default value is null.
    fn default() -> Self {
        Value::Null
    }
}

impl Debug for Value {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Null => formatter.write_str("Null"),
            Value::Bool(boolean) => write!(formatter, "Bool({})", boolean),
            Value::Number(number) => write!(formatter, "Number({})", number),
            Value::String(string) => write!(formatter, "String({:?})", string),
            Value::Array(array) => Debug::fmt(array, formatter),
            Value::Object(object) => Debug::fmt(object, formatter),
        }
    }
}

impl Serialize for Value {
    fn begin(&self) -> Fragment {
        match self {
            Value::Null => Fragment::Null,
            Value::Bool(b) => Fragment::Bool(*b),
            Value::Number(number) => Serialize::begin(number),
            Value::String(s) => Fragment::Str(Cow::Borrowed(s)),
            Value::Array(array) => Serialize::begin(array),
            Value::Object(object) => Serialize::begin(object),
        }
    }
}
