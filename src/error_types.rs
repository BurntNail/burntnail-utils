//! Library module for anyhow vs color-eyre
//!
//! The expectation is that you will use this for all error types.
#![allow(clippy::use_self)]

use std::fmt::Display;

///Trait for providing context to an error
pub trait Contextable<RES = Self> {
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
    pub type BResult<T> = color_eyre::Result<T>;
    ///Color-eyre error type
    pub type BError = color_eyre::Report;

    impl<T> Contextable for BResult<T> {
        fn context<C>(self, context: C) -> BResult<T>
        where
            C: Display + Send + Sync + 'static,
        {
            WrapErr::context(self, context)
        }

        fn with_context<C, F>(self, f: F) -> BResult<T>
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

#[cfg(feature = "ah")]
///Anyhow stuff
mod anyhow_mod {
    use super::Contextable;
    use anyhow::Context;
    use std::fmt::Display;

    ///Anyhow result type
    pub type BResult<T> = anyhow::Result<T>;
    ///Anyhow error type
    pub type BError = anyhow::Error;

    impl<T> Contextable for BResult<T> {
        fn context<C>(self, context: C) -> BResult<T>
        where
            C: Display + Send + Sync + 'static,
        {
            Context::context(self, context)
        }

        fn with_context<C, F>(self, f: F) -> BResult<T>
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

#[cfg(not(any(feature = "ah", feature = "eyre")))]
mod std_mod {
    use std::error::Error;

    pub struct BError {
        inner: Box<dyn Error>,
        contexts: Vec<String>,
    }

    impl<T: Error> From<T> for BError {
        fn from (e: T) -> Self {
            Self {
                inner: Box::new(e),
                contexts: Vec::new()
            }
        }
    }

    pub type BResult<T> = Result<T, BError>;

    impl<T> Contextable for BResult<T> {
        fn context<C>(self, context: C) -> BResult<T>
        where
            C: Display + Send + Sync + 'static,
        {
            let mut s = self;
            s.contexts.push(context.to_string());
            s
        }

        ///NB: Not lazily evaluated
        fn with_context<C, F>(self, f: F) -> BResult<T>
        where
            C: Display + Send + Sync + 'static,
            F: FnOnce() -> C,
        {
            self.context(f());
        }
    }
}

#[cfg(feature = "ah")]
pub use anyhow_mod::*;
#[cfg(feature = "eyre")]
pub use eyre_mod::*;
#[cfg(not(any(feature = "ah", feature = "eyre")))]
pub use std_mod::*;
