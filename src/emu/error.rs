use std::fmt;

#[derive(Debug, Clone)]
pub enum Error {
    FileError { err: FileError },
    CartridgeError { err: CartridgeError },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::FileError { err } => write!(f, "{err}"),
            Self::CartridgeError { err } => write!(f, "{err}"),
        }
    }
}

impl From<FileError> for Error {
    fn from(err: FileError) -> Self {
        Error::FileError { err }
    }
}

impl From<CartridgeError> for Error {
    fn from(err: CartridgeError) -> Self {
        Error::CartridgeError { err }
    }
}

#[derive(Debug, Clone)]
pub enum FileError {
    FileOpenFailed { message: String },
}

impl fmt::Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let failure = "file load failed";
        match self {
            Self::FileOpenFailed { message } => {
                write!(f, "{failure}: {message}")
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum CartridgeError {
    MissingHeader,
    InvalidHeader { message: String },
    NotSupported { message: String },
}

impl fmt::Display for CartridgeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let failure = "cartridge load failed";
        match self {
            Self::MissingHeader => {
                write!(f, "{failure}: missing header")
            }
            Self::InvalidHeader { message } => {
                write!(f, "{failure}: {message}")
            }
            Self::NotSupported { message } => {
                write!(f, "{failure}: {message}")
            }
        }
    }
}
