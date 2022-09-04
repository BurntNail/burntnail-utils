use std::error::Error;
use super::ErrorExt;

impl<T> ErrorExt<T> for Result<T, Error> {
    fn warn(self) {
        if let Err(e) = self {
            eprintln!("Warning: {e:?}");
        }
    }

    fn error(self) {
        if let Err(e) = self {
            eprintln!("Error: {e:?}");
        }
    }

    fn error_exit(self) {
        self.eprint_exit()
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
                eprintln!("Fatal Error on unwrap: {e:?}");
                std::process::exit(1);
            }
        }
    }
}