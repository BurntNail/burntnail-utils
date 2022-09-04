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