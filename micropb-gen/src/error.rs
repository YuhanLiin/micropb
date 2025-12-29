use std::{fmt::Display, io};

/// Error encountered while processing proto files and generating Rust code
#[derive(Debug)]
pub enum Error {
    /// IO error
    Io(io::Error),
    /// Protoc invocation error
    Protoc(String),
    /// Protobuf field error
    Field {
        /// Protobuf package
        package: String,
        /// Protobuf message
        message: String,
        /// Protobuf field
        field: String,
        /// Error text
        text: String,
    },
    /// Protobuf message error
    Message {
        /// Protobuf package
        package: String,
        /// Protobuf message
        message: String,
        /// Error text
        text: String,
    },
    /// Protobuf package error
    Package {
        /// Protobuf package
        package: String,
        /// Error text
        text: String,
    },
    #[cfg(feature = "config-file")]
    /// Config file parsing error
    ConfigFile {
        /// File name
        file_name: std::path::PathBuf,
        /// TOML parsing error
        err: toml::de::Error,
    },
}

/// Result alias
pub type Result<T> = std::result::Result<T, Error>;

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(error) => error.fmt(f),
            Error::Protoc(error) => write!(f, "protoc failed: {error}"),
            Error::Field {
                package,
                message,
                field,
                text,
            } => {
                let dot = if package.is_empty() { "" } else { "." };
                write!(f, "({dot}{package}.{message}.{field}) {text}")
            }
            Error::Message {
                package,
                message,
                text,
            } => {
                let dot = if package.is_empty() { "" } else { "." };
                write!(f, "({dot}{package}.{message}) {text}")
            }
            Error::Package { package, text } => {
                let dot = if package.is_empty() { "" } else { "." };
                write!(f, "({dot}{package}) {text}")
            }
            #[cfg(feature = "config-file")]
            Error::ConfigFile { file_name, err } => {
                write!(f, "Failed to parse {}: {err}", file_name.display())
            }
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(error) => Some(error),
            #[cfg(feature = "config-file")]
            Error::ConfigFile { err, .. } => Some(err),
            Error::Field { .. }
            | Error::Message { .. }
            | Error::Package { .. }
            | Error::Protoc(_) => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

pub(crate) fn field_error(
    pkg: &str,
    msg_name: &str,
    field_name: &str,
    err_text: impl Display,
) -> Error {
    Error::Field {
        package: pkg.to_owned(),
        message: msg_name.to_owned(),
        field: field_name.to_owned(),
        text: err_text.to_string(),
    }
}

pub(crate) fn msg_error(pkg: &str, msg_name: &str, err_text: impl Display) -> Error {
    Error::Message {
        package: pkg.to_owned(),
        message: msg_name.to_owned(),
        text: err_text.to_string(),
    }
}

pub(crate) fn pkg_error(pkg: &str, err_text: impl Display) -> Error {
    Error::Package {
        package: pkg.to_owned(),
        text: err_text.to_string(),
    }
}
