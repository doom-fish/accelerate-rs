pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    InvalidLength { expected: usize, actual: usize },
    OperationFailed(&'static str),
    VImageError(isize),
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidLength { expected, actual } => {
                write!(f, "invalid length: expected {expected}, got {actual}")
            }
            Self::OperationFailed(message) => f.write_str(message),
            Self::VImageError(code) => write!(f, "vImage operation failed with status {code}"),
        }
    }
}

impl std::error::Error for Error {}
