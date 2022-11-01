//! Crate to hold lots of utilities for the author. I often end up using these in most projects.
//!
//! ## Error Extension
//! Includes a bunch of stuff for handling errors. For example `anyhow::Result::Err(5).warn();` will print "Warning: 5" to stderr, or via `tracing` means.
//!
//! Also includes utilities for hard to handle error types, and unwrapping them with tracing logs, or converting them to anyhow results.
//!
//! ## Either
//! An enumeration for representing an object with can be either A or B, with utility methods to convert to one or the other.
//!
//! ## Memcache
//! A circular queue structure backed by a [`Vec`] *(it used to be a [`std::mem::MaybeUninit`] array but a [`Vec`] added 10x speedups lol)*, which can optionally hold a `DoOnInterval` in order to only add things on an interval.
//!
//! ## Macros
//! I've found that I love Rust Type States, for which it is very repetitive and easiest to use a macro like the one provided. See the module-level docs for more examples
//!
//! ## Time Based Structs
//! ### Do On Interval
//! This struct is useful if we want to do anything on an interval, like sending a network request or logging an average.
//!
//! ### Scoped Timer
//! I love this for logging! It starts a timer when you make the object, and on [`std::ops::Drop`] it logs out the time since the timer started, making for very conventient logging.
//!
//! ## Coords
//! Some nice 2D Coordinates, with support for maximum positions, and generic types using the `num_traits`.
//!
//! ## Piston Cache
//! NB: Only enabled if you have the relevant feature enabled.
//!
//! A nice struct for managing `Piston2D` image textures, if you want to cache them over reading them in to individual bindings.
//!
//! ## 2D Array
//! A struct for a grid array, which can be indexed using Coordinates or a usize pair.
#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::missing_docs_in_private_items
)]

#[macro_use]
extern crate anyhow;

pub mod either;
pub mod macros;
pub mod memcache;
pub mod time_based_structs;

pub mod coords;
pub mod error_ext;
#[cfg(feature = "piston_cacher")]
pub mod piston_cache;
pub mod twod_array;

///Private to crate
mod crate_private {
    ///Trait which cannot be externally implemented
    pub trait Sealed {}
}
