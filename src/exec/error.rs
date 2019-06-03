pub(super) type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Parse(crate::parse::Error),
    MissingMainClass,
    MissingEntryPoint,
    EmptyStack,
    StackType(&'static str),
    VariableType(&'static str, usize),
    VariableOutOfScope,
    GenericError(String),
}

impl From<crate::parse::Error> for Error {
    fn from(err: crate::parse::Error) -> Self {
        Error::Parse(err)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Parse(err) => Some(err),
            _ => None,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Parse(err) => write!(f, "{}", err),
            Error::MissingMainClass => write!(f, "main class is missing"),
            Error::MissingEntryPoint => write!(f, "entry point is missing"),
            Error::EmptyStack => write!(f, "empty stack"),
            Error::StackType(expected) => write!(f, "expected {} in stack", expected),
            Error::VariableType(expected, offset) => {
                write!(f, "expected {} at offset {}", expected, offset)
            }
            Error::VariableOutOfScope => write!(f, "variable is out of scope"),
            Error::GenericError(msg) => write!(f, "{}", msg),
        }
    }
}

#[macro_export]
macro_rules! generic_error {
    ($f:expr, $($args:tt),* $(,)?) => {
        generic_error!(format_args!($f, $($args),*))
    };
    ($msg:expr) => {
        return Err(Error::GenericError(format!("{}", $msg)))
    };
}
