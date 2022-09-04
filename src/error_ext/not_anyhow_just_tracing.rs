impl<T> ErrorExt<T> for Result<T, Error> {
    fn warn(self) {
        if let Err(e) = self {
            warn!(?e);
        }
    }

    fn error(self) {
        if let Err(e) = self {
            error!(?e);
        }
    }

    fn error_exit(self) {
        if let Err(e) = self {
            error!(?e, "Fatal Error");
            std::process::exit(1);
        }
    }

    fn eprint_exit(self) {
        if let Err(e) = self {
            eprintln!("Fatal Error: {e:?}");
            std::process::exit(1);
        }
    }

    fn unwrap_log_error(self) -> T {
        match self {
            Ok(o) => o,
            Err(e) => {
                error!(?e, "Fatal Error on unwrap");
                std::process::exit(1);
            }
        }
    }
}
