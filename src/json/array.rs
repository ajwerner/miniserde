use crate::error::Result;
use crate::json::{drop, Value};
use crate::private;
use crate::ser::{Fragment, Serialize};
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::fmt::{self, Debug};
use core::mem::{self, ManuallyDrop};
use core::ops::{Deref, DerefMut};
use core::ptr;

/// A `Vec<Value>` with a non-recursive drop impl.
#[derive(Default)]
pub struct Array {
    inner: Vec<Value>,
}

impl Drop for Array {
    fn drop(&mut self) {
        self.inner.drain(..).for_each(drop::safely);
    }
}

fn take(array: Array) -> Vec<Value> {
    let array = ManuallyDrop::new(array);
    unsafe { ptr::read(&array.inner) }
}

impl Array {
    pub fn new() -> Self {
        Array { inner: Vec::new() }
    }
}

impl Deref for Array {
    type Target = Vec<Value>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Array {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Clone for Array {
    fn clone(&self) -> Self {
        Array {
            inner: self.inner.clone(),
        }
    }

    fn clone_from(&mut self, other: &Self) {
        self.inner.clone_from(&other.inner);
    }
}

impl IntoIterator for Array {
    type Item = Value;
    type IntoIter = <Vec<Value> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        take(self).into_iter()
    }
}

impl<'a> IntoIterator for &'a Array {
    type Item = &'a Value;
    type IntoIter = <&'a Vec<Value> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut Array {
    type Item = &'a mut Value;
    type IntoIter = <&'a mut Vec<Value> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl FromIterator<Value> for Array {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Value>,
    {
        Array {
            inner: Vec::from_iter(iter),
        }
    }
}

impl Debug for Array {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Array ")?;
        formatter.debug_list().entries(self).finish()
    }
}

impl Serialize for Array {
    fn begin(&self) -> Fragment {
        private::stream_slice(self)
    }
}
