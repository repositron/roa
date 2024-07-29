use std::fmt;

#[derive(Debug)]
pub enum InfraError {
    InternalServerError,
    NotFound,
    UniqueViolation,
}

pub fn adapt_infra_error<T: Error>(error: T) -> InfraError {
    error.as_infra_error()
}

impl fmt::Display for InfraError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InfraError::NotFound => write!(f, "Not found"),
            InfraError::InternalServerError => write!(f, "Internal server error"),
            InfraError::UniqueViolation => write!(f, "UniqueViolation error")
        }
    }
}

pub trait Error {
    fn as_infra_error(&self) -> InfraError;
}

impl Error for diesel::result::DatabaseErrorKind {
    fn as_infra_error(&self) -> InfraError {
        match self {
            diesel::result::DatabaseErrorKind::UniqueViolation => InfraError::UniqueViolation,
            _ => InfraError::InternalServerError,
        }
    }
}

impl Error for diesel::result::Error {
    fn as_infra_error(&self) -> InfraError {
        match self {
            diesel::result::Error::NotFound => InfraError::NotFound,
            _ => InfraError::InternalServerError,
        }
    }
}

/*impl Error for deadpool_diesel::PoolError {
    fn as_infra_error(&self) -> InfraError {
        InfraError::InternalServerError
    }
}

impl Error for InteractError {
    fn as_infra_error(&self) -> InfraError {
        InfraError::InternalServerError
    }
}
*/