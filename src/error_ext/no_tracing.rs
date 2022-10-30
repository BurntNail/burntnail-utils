use crate::error_ext::ErrorExt;
use anyhow::Error;

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

    ///Just panics
    fn error_exit(self) {
        if let Err(e) = self {
            panic!("Fatal Error: {e:?}");
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
                panic!("Fatal Error unwrapping: {e:?}");
            }
        }
    }
}
