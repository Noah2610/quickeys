#[derive(Debug)]
pub enum Error {
    Message(String),
    Error(Box<dyn std::error::Error>),
    KeyNotFound(String),
    ConstantNotFound { constant: String, command: String },
    Unreachable(String),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Message(msg) => write!(f, "[Message] {}", msg),
            Self::Error(err) => write!(f, "[Error] {}", err),
            Self::KeyNotFound(key) => {
                write!(f, "[KeyNotFound] No command found for key '{}'", key)
            }
            Self::ConstantNotFound { constant, command } => write!(f, "[ConstantNotFound] Referenced undefined constant '{}' in command:\n{}", constant, command),
            Self::Unreachable(msg) => write!(f, "[Unreachable] Ohoh something's wrong ¯\\_(ツ)_/¯\n{}", msg),
        }
    }
}

impl From<regex::Error> for Error {
    fn from(error: regex::Error) -> Self {
        Self::Error(Box::new(error))
    }
}
