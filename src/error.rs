use std::fmt;
use std::io;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    Io(io::ErrorKind),
    Serialize,
    Deserialize,
}

#[derive(Debug, Clone)]
pub struct Error {
    kind: ErrorKind,
    message: String,
}

impl Error {
    pub(crate) fn deserialize<S: Into<String>>(message: S) -> Error {
        Error {
            kind: ErrorKind::Deserialize,
            message: message.into(),
        }
    }

    pub fn kind(&self) -> ErrorKind {
        self.kind
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}: {}",
            ::std::error::Error::description(self),
            self.message
        )
    }
}

impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        match self.kind {
            ErrorKind::Io(_) => "i/o error",
            ErrorKind::Serialize => "serialize error",
            ErrorKind::Deserialize => "deserialize error",
        }
    }
}

impl ::serde::de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Error {
        Error {
            kind: ErrorKind::Deserialize,
            message: msg.to_string(),
        }
    }
}

impl ::serde::ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Error {
        Error {
            kind: ErrorKind::Serialize,
            message: msg.to_string(),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error {
            kind: ErrorKind::Io(err.kind()),
            message: err.to_string(),
        }
    }
}
