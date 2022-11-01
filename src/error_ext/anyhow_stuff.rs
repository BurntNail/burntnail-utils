use anyhow::{Context, Result};
use std::{
    any::Any,
    fmt::Display,
    sync::{LockResult, Mutex, MutexGuard},
};

///Extension trait for errors to quickly do things
pub trait ErrorExt<T> {
    ///If `Err` write to `warn!`
    fn warn(self);
    ///If `Err` write to `error!`
    fn error(self);
    ///If `Err` write to error out if not using stdlib logging (eg. `error!`) and then to [`panic!`] with the error.
    fn error_exit(self);
    ///If `Err` write to error (eg. `error!` and [`eprintln!`]) and then to [`std::process::exit`] with code 1.
    fn eprint_exit(self);
    ///If `Err` write to `error!`/[`eprintln!`] and [`std::process::exit`] with code 1, else return `Ok` value
    fn unwrap_log_error(self) -> T;
}

///Utility trait for Mutexes
pub trait MutexExt<T> {
    ///Function to unlock or panic using `error!`
    fn lock_panic<C: Display + Send + Sync + 'static>(&self, msg: C) -> MutexGuard<T>;
}

///Creates a trait with a function `ae(self) -> anyhow::Result`
macro_rules! to_anyhow_trait {
    ($($name:ident => $doc:expr),+) => {
        $(
            #[doc=$doc]
            pub trait $name<T> {
                ///Converter function to [`anyhow::Result`]
                #[allow(clippy::missing_errors_doc)]
                fn ae (self) -> anyhow::Result<T>;

                ///Function that is the same as [`ErrorExt::unwrap_log_error`] only this includes an easy way to get context
                fn unwrap_log_error_with_context<C: Display + Send + Sync + 'static, F: FnOnce() -> C> (self, f: F) -> T;
                ///Function that is the same as [`ErrorExt::unwrap_log_error`] only this includes an easy way to get context
                fn unwap_log_error_context<C: Display + Send + Sync + 'static> (self, c: C) -> T;
            }
        )+
    };
}
to_anyhow_trait!(
    ToAnyhowErr => "Trait to turn [`std::error::Error`] to [`anyhow::Error`]",
    ToAnyhowNotErr => "Trait to turn non-errors (like [`Option`]) to [`anyhow::Error`]",
    ToAnyhowPoisonErr => "Trait to turn `Box<dyn Any + Send + 'static>` to [`anyhow::Error`]",
    ToAnyhowThreadErr => "Trait to turn [`std::sync::LockResult`] to [`anyhow::Result`]"
);
//To avoid overlapping trait bounds

impl<T> ToAnyhowNotErr<T> for Option<T> {
    fn ae(self) -> Result<T> {
        match self {
            Some(s) => Ok(s),
            None => Err(anyhow!("empty option")),
        }
    }

    fn unwrap_log_error_with_context<C: Display + Send + Sync + 'static, F: FnOnce() -> C>(
        self,
        f: F,
    ) -> T {
        self.ae().with_context(f).unwrap_log_error()
    }

    fn unwap_log_error_context<C: Display + Send + Sync + 'static>(self, c: C) -> T {
        self.ae().context(c).unwrap_log_error()
    }
}

impl<T, E: std::error::Error + Send + Sync + 'static> ToAnyhowErr<T> for std::result::Result<T, E> {
    fn ae(self) -> Result<T> {
        self.map_err(|e| anyhow::Error::new(e))
    }

    fn unwrap_log_error_with_context<C: Display + Send + Sync + 'static, F: FnOnce() -> C>(
        self,
        f: F,
    ) -> T {
        self.ae().with_context(f).unwrap_log_error()
    }

    fn unwap_log_error_context<C: Display + Send + Sync + 'static>(self, c: C) -> T {
        self.ae().context(c).unwrap_log_error()
    }
}
impl<T> ToAnyhowThreadErr<T> for std::result::Result<T, Box<dyn Any + Send + 'static>> {
    fn ae(self) -> Result<T> {
        self.map_err(|_| anyhow!("Error joining thread"))
    }

    fn unwrap_log_error_with_context<C: Display + Send + Sync + 'static, F: FnOnce() -> C>(
        self,
        f: F,
    ) -> T {
        self.ae().with_context(f).unwrap_log_error()
    }

    fn unwap_log_error_context<C: Display + Send + Sync + 'static>(self, c: C) -> T {
        self.ae().context(c).unwrap_log_error()
    }
}
impl<T> ToAnyhowPoisonErr<T> for LockResult<T> {
    fn ae(self) -> Result<T> {
        self.map_err(|e| anyhow!("{}", e))
    }

    fn unwrap_log_error_with_context<C: Display + Send + Sync + 'static, F: FnOnce() -> C>(
        self,
        f: F,
    ) -> T {
        self.ae().with_context(f).unwrap_log_error()
    }

    fn unwap_log_error_context<C: Display + Send + Sync + 'static>(self, c: C) -> T {
        self.ae().context(c).unwrap_log_error()
    }
}

impl<T> MutexExt<T> for Mutex<T> {
    fn lock_panic<C: Display + Send + Sync + 'static>(&self, msg: C) -> MutexGuard<T> {
        self.lock().ae().context(msg).unwrap_log_error()
    }
}
