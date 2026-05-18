/// Result type used by the safe Accelerate wrappers.
pub type Result<T> = core::result::Result<T, Error>;

/// Errors returned by the safe Accelerate wrappers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    InvalidLength { expected: usize, actual: usize },
    InvalidValue(&'static str),
    OperationFailed(&'static str),
    BnnsStatus(i32),
    LapackInfo(i32),
    QuadratureStatus(i32),
    SparseStatus(i32),
    VImageError(isize),
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidLength { expected, actual } => {
                write!(f, "invalid length: expected {expected}, got {actual}")
            }
            Self::InvalidValue(message) | Self::OperationFailed(message) => f.write_str(message),
            Self::BnnsStatus(status) => write!(f, "BNNS operation failed with status {status}"),
            Self::LapackInfo(info) => write!(f, "LAPACK operation failed with info {info}"),
            Self::QuadratureStatus(status) => {
                write!(f, "Quadrature integration failed with status {status}")
            }
            Self::SparseStatus(status) => write!(f, "Sparse operation failed with status {status}"),
            Self::VImageError(code) => write!(f, "vImage operation failed with status {code}"),
        }
    }
}

impl std::error::Error for Error {}
