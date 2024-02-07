use std::fmt;

pub enum ErrorKind {
    IOError(std::io::Error),
    NotFound,
    AlreadyRunning,
    NotRunning,
    NotSupported,
    FailedToReadMemory,
    FailedToWriteMemory,
    InvalidInput,
    NoGameFound,
    Error(String),
}

pub type Result<T> = std::result::Result<T, ErrorKind>;

impl From<std::io::Error> for ErrorKind {
    fn from(err: std::io::Error) -> ErrorKind {
        ErrorKind::IOError(err)
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorKind::IOError(err) => write!(f, "IO Error: {}", err),
            ErrorKind::NotFound => write!(f, "Not found"),
            ErrorKind::AlreadyRunning => write!(f, "Already running"),
            ErrorKind::NotRunning => write!(f, "Not running"),
            ErrorKind::NotSupported => write!(f, "Not supported"),
            ErrorKind::FailedToReadMemory => write!(f, "Failed to read memory"),
            ErrorKind::FailedToWriteMemory => write!(f, "Failed to write memory"),
            ErrorKind::InvalidInput => write!(f, "Invalid input"),
            ErrorKind::NoGameFound => write!(f, "No games are found! Please run the game first!"),
            ErrorKind::Error(err) => write!(f, "{}", err),
        }
    }
}