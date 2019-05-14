use super::*;

pub(super) type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io {
        msg: String,
        error: std::io::Error,
    },
    Expected {
        got: String,
        expected: String,
    },
    UnknownTag {
        tag: u8,
    },
    InvalidMethodHandleKind {
        kind: u8,
    },
    InvalidString {
        error: std::str::Utf8Error,
    },
    ZeroIndex,
    OutOfRange {
        index: u16,
    },
    IndexInsideDoubleWidthConstant {
        index: u16,
    },
    InvalidAttributeType {
        attr: Constant,
    },
    UnknownAttributeType {
        attr: String,
    },
    InvalidStackFrameType {
        ty: u8,
    },
    InvalidVerificationType {
        ty: u8,
    },
    LengthMismatch {
        length: u32,
        actual: u32,
        ty: String,
    },
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io { error, .. } => Some(error),
            Error::InvalidString { error } => Some(error),
            _ => None,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Error::*;
        match self {
            Io { msg, error } => write!(f, "expected {}, got a read error: {}", msg, error),
            Expected { got, expected } => write!(f, "expected: {}, got {}", expected, got),
            UnknownTag { tag } => write!(f, "unknown tag: 0x{:02X}", tag),
            InvalidMethodHandleKind { kind } => {
                write!(f, "invalid method handle ref kind: 0x{:02X}", kind)
            }
            InvalidString { error } => write!(f, "invalid utf-8 string: {}", error),
            ZeroIndex => write!(f, "invalid index: zero index"),
            OutOfRange { index } => write!(f, "out of range: {}", index),
            IndexInsideDoubleWidthConstant { index } => {
                write!(f, "index inside of a double widht constant: {}", index)
            }
            InvalidAttributeType { attr } => write!(f, "invalid attribute type: {:?}", attr),
            UnknownAttributeType { attr } => write!(f, "unknown attribute type: {}", attr),

            InvalidStackFrameType { ty } => write!(f, "invalid stack frame type: {:#X?}", ty),
            InvalidVerificationType { ty } => write!(f, "invalid verification type: {:#X?}", ty),

            LengthMismatch { length, actual, ty } => write!(
                f,
                "length mismatch while parsing: `{}` got: {} wanted: {}",
                ty, actual, length
            ),
        }
    }
}
