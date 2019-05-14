use super::*;

pub(super) type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(String, std::io::Error),
    Expected(String, String),
    UnknownTag(u8),
    InvalidMethodHandleKind(u8),
    InvalidString(std::str::Utf8Error),
    ZeroIndex,
    OutOfRange(u16),
    IndexInsideDoubleWidthConstant(u16),
    InvalidAttributeType(Constant),
    UnknownAttributeType(String),
    InvalidStackFrameType(u8),
    InvalidVerificationType(u8),
    LengthMismatch {
        length: u32,
        actual: u32,
        ty: String,
    },
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(_, err) => Some(err),
            Error::InvalidString(err) => Some(err),
            _ => None,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(msg, err) => write!(f, "expected {}, got a read error: {}", msg, err),
            Error::Expected(left, right) => write!(f, "expected: {}, got {}", right, left),
            Error::UnknownTag(d) => write!(f, "unknown tag: 0x{:02X}", d),
            Error::InvalidMethodHandleKind(d) => {
                write!(f, "invalid method handle ref kind: 0x{:02X}", d)
            }
            Error::InvalidString(err) => write!(f, "invalid utf-8 string: {}", err),
            Error::ZeroIndex => write!(f, "invalid index: zero index"),
            Error::OutOfRange(d) => write!(f, "out of range: {}", d),
            Error::IndexInsideDoubleWidthConstant(d) => {
                write!(f, "index inside of a double widht constant: {}", d)
            }
            Error::InvalidAttributeType(d) => write!(f, "invalid attribute type: {:?}", d),
            Error::InvalidStackFrameType(d) => write!(f, "invalid stack frame type: {:#X?}", d),
            Error::InvalidVerificationType(d) => write!(f, "invalid verification type: {:#X?}", d),
            Error::UnknownAttributeType(s) => write!(f, "unknown attribute type: {}", s),
            Error::LengthMismatch { length, actual, ty } => write!(
                f,
                "length mismatch while parsing: `{}` got: {} wanted: {}",
                ty, actual, length
            ),
        }
    }
}
