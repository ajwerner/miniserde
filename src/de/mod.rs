//! Deserialization traits.
//!
//! Deserialization in miniserde works by returning a "place" into which data
//! may be written through the methods of the `Visitor` trait object.
//!
//! Use the `make_place!` macro to acquire a "place" type. A library may use a
//! single place type across all of its Deserialize impls, or each impl or each
//! module may use a private place type. There is no difference.
//!
//! A place is simply:
//!
//! ```rust
//! struct Place<T> {
//!     out: Option<T>,
//! }
//! ```
//!
//! Upon successful deserialization the output object is written as `Some(T)`
//! into the `out` field of the place.
//!
//! ## Deserializing a primitive
//!
//! The Visitor trait has a method corresponding to each supported primitive
//! type.
//!

mod impls;

use crate::error::{Error, Result};
use alloc::boxed::Box;

/// Trait for data structures that can be deserialized from a JSON string.
///
/// [Refer to the module documentation for examples.][crate::de]
pub trait Deserialize: Sized {
    /// The only correct implementation of this method is:
    ///
    fn begin(out: &mut Option<Self>) -> &mut dyn Visitor;

    // Not public API. This method is only intended for Option<T>, should not
    // need to be implemented outside of this crate.
    #[doc(hidden)]
    #[inline]
    fn default() -> Option<Self> {
        None
    }
}

/// Trait that can write data into an output place.
///
/// [Refer to the module documentation for examples.][crate::de]
pub trait Visitor {
    fn null(&mut self) -> Result<()> {
        Err(Error)
    }

    fn boolean(&mut self, b: bool) -> Result<()> {
        let _ = b;
        Err(Error)
    }

    fn string(&mut self, s: &str) -> Result<()> {
        let _ = s;
        Err(Error)
    }

    fn negative(&mut self, n: i64) -> Result<()> {
        let _ = n;
        Err(Error)
    }

    fn nonnegative(&mut self, n: u64) -> Result<()> {
        let _ = n;
        Err(Error)
    }

    fn float(&mut self, n: f64) -> Result<()> {
        let _ = n;
        Err(Error)
    }

    fn seq(&mut self) -> Result<Box<dyn Seq + '_>> {
        Err(Error)
    }

    fn map(&mut self) -> Result<Box<dyn Map + '_>> {
        Err(Error)
    }
}

/// Trait that can hand out places to write sequence elements.
///
/// [Refer to the module documentation for examples.][crate::de]
pub trait Seq {
    fn element(&mut self) -> Result<&mut dyn Visitor>;
    fn finish(&mut self) -> Result<()>;
}

/// Trait that can hand out places to write values of a map.
///
/// [Refer to the module documentation for examples.][crate::de]
pub trait Map {
    fn key(&mut self, k: &str) -> Result<&mut dyn Visitor>;
    fn finish(&mut self) -> Result<()>;
}
