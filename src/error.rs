use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum Error {
    Message(String),
    Error(Box<dyn std::error::Error>),
    IoError {
        error: std::io::Error,
        filepath: Option<PathBuf>,
    },
    RegexError(regex::Error),
    KeyNotFound(String),
    ConstantNotFound {
        constant: String,
        command: String,
    },
    CommandError {
        command: String,
        code: Option<i32>,
    },
    Unreachable(String),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use self::Error::*;

        match self {
            Message(msg) => write!(f, "[Message] {}", msg),
            Error(err) => write!(f, "[Error] {}", err),
            IoError {
                error,
                filepath: None,
            } => write!(f, "[IoError] {}", error),
            IoError {
                error,
                filepath: Some(path),
            } => write!(f, "[IoError] for file: {:?}\n{}", path, error),
            RegexError(err) => write!(f, "[RegexError] {}", err),
            KeyNotFound(key) => {
                write!(f, "[KeyNotFound] No command found for key '{}'", key)
            }
            ConstantNotFound {
                constant,
                command,
            } => write!(f, "[ConstantNotFound] Referenced undefined constant '{}' in command:\n{}", constant, command),
            CommandError {
                command,
                code: None,
            } => write!(f, "[CommandError] command: {}\nexited with unknown exit code", command),
            CommandError {
                command,
                code: Some(code),
            } => write!(f, "[CommandError] command: {}\nexit code: {}", command, code),
            Unreachable(msg) => write!(f, "[Unreachable] Ohoh something's wrong ¯\\_(ツ)_/¯\n{}", msg),
        }
    }
}

impl From<regex::Error> for Error {
    fn from(error: regex::Error) -> Self {
        Self::RegexError(error)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::IoError {
            error,
            filepath: None,
        }
    }
}

impl<P: AsRef<Path>> From<(std::io::Error, P)> for Error {
    fn from((error, path): (std::io::Error, P)) -> Self {
        Self::IoError {
            error,
            filepath: Some(path.as_ref().into()),
        }
    }
}
