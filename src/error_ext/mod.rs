#[cfg(all(not(feature = "tracing"), not(feature = "anyhow")))]
mod nothing;
#[cfg(all(not(feature = "tracing"), not(feature = "anyhow")))]
pub use self::nothing::*;

#[cfg(all(feature = "anyhow", feature = "tracing"))]
mod both;
#[cfg(all(feature = "anyhow", feature = "tracing"))]
pub use both::*;

#[cfg(all(not(feature = "tracing"), feature = "anyhow"))]
mod not_anyhow_just_tracing;
#[cfg(all(not(feature = "tracing"), feature = "anyhow"))]
pub use not_anyhow_just_tracing::*;


#[cfg(all(not(feature = "anyhow"), feature = "tracing"))]
mod not_tracing_just_anyhow;
#[cfg(all(not(feature = "anyhow"), feature = "tracing"))]
pub use not_tracing_just_anyhow::*;


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