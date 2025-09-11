use std::fmt;

#[derive(Debug, Clone)]
pub enum Error {
    LoadError { err: LoadError },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::LoadError { err } => write!(f, "{err}"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum LoadError {
    ProgramTooLarge { maximum_size: usize },
    FileOpenFailed { message: String },
    MissingHeader,
}
