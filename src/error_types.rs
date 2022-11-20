//! Library module for anyhow vs color-eyre
//!
//! The expectation is that you will use this for all error types.
#![allow(clippy::use_self)]

///Trait for providing context to an error
pub trait Contextable<RES> {
    /// Wrap the error value with additional context
    fn context<C>(self, context: C) -> RES
    where
        C: Display + Send + Sync + 'static;

    /// Wrap the error value with additional context that is evaluated lazily
    /// only once an error does occur.
    fn with_context<C, F>(self, f: F) -> RES
    where
        C: Display + Send + Sync + 'static,
        F: FnOnce() -> C;
}

#[cfg(feature = "eyre")]
///Eyre stuff
mod eyre_mod {
    use super::Contextable;
    use color_eyre::eyre::WrapErr;
    use std::fmt::Display;

    ///Color-eyre result type
    pub type Result<T> = color_eyre::Result<T>;
    ///Color-eyre error type
    pub type Error = color_eyre::Report;

    impl<T> Contextable<Result<T>> for Result<T> {
        fn context<C>(self, context: C) -> Result<T>
        where
            C: Display + Send + Sync + 'static,
        {
            WrapErr::context(self, context)
        }

        fn with_context<C, F>(self, f: F) -> Result<T>
        where
            C: Display + Send + Sync + 'static,
            F: FnOnce() -> C,
        {
            WrapErr::with_context(self, f)
        }
    }

    ///Everything from the `color_eyre` crate
    mod native {
        pub use color_eyre::*;
    }
}

#[cfg(not(feature = "eyre"))]
///Anyhow stuff
mod anyhow_mod {
    use super::Contextable;
    use anyhow::Context;
    use std::fmt::Display;

    ///Anyhow result type
    pub type Result<T> = anyhow::Result<T>;
    ///Anyhow error type
    pub type Error = anyhow::Error;

    impl<T> Contextable<Result<T>> for Result<T> {
        fn context<C>(self, context: C) -> Result<T>
        where
            C: Display + Send + Sync + 'static,
        {
            Context::context(self, context)
        }

        fn with_context<C, F>(self, f: F) -> Result<T>
        where
            C: Display + Send + Sync + 'static,
            F: FnOnce() -> C,
        {
            Context::with_context(self, f)
        }
    }

    ///Everything from the `anyhow` crate
    pub mod native {
        pub use anyhow::*;
    }
}

#[cfg(not(feature = "eyre"))]
pub use anyhow_mod::*;
#[cfg(feature = "eyre")]
pub use eyre_mod::*;
use std::fmt::Display;
