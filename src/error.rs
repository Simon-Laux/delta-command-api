use serde::Serialize;

#[derive(Debug)]
pub struct ErrorInstance {
    pub kind: ErrorType,
    pub message: String,
}

#[derive(Serialize, Debug)]
pub enum ErrorType {
    CommandIdMissing,
    CommandNotFound,
    CommandNotImplementedYet,
    CommandParseFailure,
    NoContext,
    /** the command threw an Error */
    Generic,
    DeltaChatSQLError,
    AnyhowError,
}

impl From<anyhow::Error> for ErrorInstance {
    fn from(err: anyhow::Error) -> ErrorInstance {
        ErrorInstance {
            kind: ErrorType::AnyhowError,
            message: format!("{:?}", err),
        }
    }
}

impl From<deltachat::sql::Error> for ErrorInstance {
    fn from(err: deltachat::sql::Error) -> ErrorInstance {
        ErrorInstance {
            kind: ErrorType::DeltaChatSQLError,
            message: format!("SQL error: {:?}", err),
        }
    }
}

#[macro_export]
macro_rules! genericError {
    ($err:expr) => {
        ErrorInstance {
            kind: ErrorType::Generic,
            message: $err.to_owned(),
        }
    };
}
