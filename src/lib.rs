//!Crate to hold lots of utilities for the author.
//!
//!Includes a basic `Either`, some macros, a circular cache, some stuff for time and error extensions

#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::missing_docs_in_private_items
)]

#[macro_use]
extern crate anyhow;

///Module to hold [`either::Either`]
pub mod either;
///Module to hold commonly-used and widely applicable macros
pub mod macros;
///Module to hold circular list cache
pub mod memcache;
///Module to hold a 2d array struct
pub mod multid_array;
///Module to hold structs which deal with time
pub mod time_based_structs;

#[cfg(feature = "piston_cacher")]
///Utility cacher for `G2dTexture` objects in a set directory
pub mod cacher;
///Module to hold coordinates
pub mod coords;
///Module to hold Error Extension traits. Can appear empty if not many features are enabled
pub mod error_ext;

///Private to crate
mod crate_private {
    ///Trait which cannot be externally implemented
    pub trait Sealed {}
}
