use std::fmt;

#[derive(Debug, Clone)]
pub enum EmuError {
    LoadError { err: LoadError },
}

impl fmt::Display for EmuError {
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

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ProgramTooLarge { maximum_size } => write!(
                f,
                "program is too large (exceeds maximum size of {maximum_size} bytes)"
            ),
            Self::FileOpenFailed { message } => {
                write!(f, "failed to open program file: {message}")
            }
            Self::MissingHeader => {
                write!(f, "the program header is missing")
            }
        }
    }
}
