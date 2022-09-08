use std::{fmt::Display, sync::MutexGuard};

///Extension trait for errors to quickly do things
pub trait ErrorExt<T> {
    ///If `Err` write to [`warn!`]
    fn warn(self);
    ///If `Err` write to [`error!`]
    fn error(self);
    ///If `Err` write to [`error!`] and [`std::process::exit`] with code 1
    fn error_exit(self);
    ///If `Err` write to [`eprintln!`] and [`std::process::exit`] with code 1
    fn eprint_exit(self);
    ///If `Err` write to [`error!`] and [`std::process::exit`] with code 1, else return `Ok` value
    fn unwrap_log_error(self) -> T;
}

///Utility trait for Mutexes
pub trait MutexExt<T> {
    ///Function to unlock or panic using `error!`
    fn lock_panic<C: Display + Send + Sync + 'static>(&self, msg: C) -> MutexGuard<T>;
}
