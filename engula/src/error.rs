use std::fmt::Debug;

use parquet::errors::ParquetError;
use thiserror::Error;
use tokio::sync::{mpsc, oneshot, watch};

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("io error: `{0}`")]
    Io(String),
    #[error("rpc error: `{0}`")]
    Rpc(String),
    #[error("task error: `{0}`")]
    Task(String),
    #[error("timeout error: `{0}`")]
    Timeout(String),
    #[error("channel error: `{0}`")]
    Channel(String),
    #[error("transport error: `{0}`")]
    Transport(String),
    #[error("aws error: `{0}`")]
    AwsSdk(String),
    #[error("parquet error: `{0}`")]
    Parquet(String),
    #[error("url parse error: `{0}`")]
    UrlParse(String),
    #[error("not found: `{0}`")]
    NotFound(String),
    #[error("invalid argument: `{0}`")]
    InvalidArgument(String),
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Error {
        Error::Io(error.to_string())
    }
}

impl From<tonic::Status> for Error {
    fn from(status: tonic::Status) -> Error {
        Error::Rpc(status.to_string())
    }
}

impl From<tokio::task::JoinError> for Error {
    fn from(error: tokio::task::JoinError) -> Error {
        Error::Task(error.to_string())
    }
}

impl<T> From<mpsc::error::SendError<T>> for Error {
    fn from(error: mpsc::error::SendError<T>) -> Error {
        Error::Channel(error.to_string())
    }
}

impl<T> From<watch::error::SendError<T>> for Error {
    fn from(_: watch::error::SendError<T>) -> Error {
        Error::Channel("watch send error".to_owned())
    }
}

impl From<oneshot::error::RecvError> for Error {
    fn from(error: oneshot::error::RecvError) -> Error {
        Error::Channel(error.to_string())
    }
}

impl From<tokio::time::error::Elapsed> for Error {
    fn from(error: tokio::time::error::Elapsed) -> Error {
        Error::Timeout(error.to_string())
    }
}

impl From<tonic::transport::Error> for Error {
    fn from(error: tonic::transport::Error) -> Error {
        Error::Transport(error.to_string())
    }
}

impl<E: std::error::Error> From<aws_sdk_s3::SdkError<E>> for Error {
    fn from(error: aws_sdk_s3::SdkError<E>) -> Error {
        Error::AwsSdk(error.to_string())
    }
}

impl From<ParquetError> for Error {
    fn from(error: ParquetError) -> Error {
        Error::Parquet(error.to_string())
    }
}

impl From<url::ParseError> for Error {
    fn from(error: url::ParseError) -> Error {
        Error::UrlParse(error.to_string())
    }
}

impl From<Error> for tonic::Status {
    fn from(error: Error) -> tonic::Status {
        let code = match &error {
            Error::NotFound(_) => tonic::Code::NotFound,
            Error::InvalidArgument(_) => tonic::Code::InvalidArgument,
            _ => tonic::Code::Internal,
        };
        tonic::Status::new(code, error.to_string())
    }
}

pub type Result<T> = std::result::Result<T, Error>;
