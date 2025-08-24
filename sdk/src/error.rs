use std::io::{Error, ErrorKind};

/// Creates a `std::io::Error` of kind `InvalidData` that contains the given message.
pub fn error_invalid_data<E>(msg: E) -> Error
where
    E: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    Error::new(ErrorKind::InvalidData, msg)
}
