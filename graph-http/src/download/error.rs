use crate::iotools;
use graph_error::GraphError;

#[derive(Debug, thiserror::Error)]
#[allow(clippy::large_enum_variant)]
pub enum BlockingDownloadError {
    #[error(transparent)]
    Io(#[from] iotools::ThreadedIoError),

    #[error("target directory does not exist {0}")]
    TargetDoesNotExist(String),

    #[error("request error: {0}")]
    Request(#[from] reqwest::Error),

    #[error("graph error: {0}")]
    Graph(#[from] GraphError),

    #[error(
        "file name is too long (max {} chars)",
        super::client::MAX_FILE_NAME_LEN
    )]
    FileNameTooLong,

    #[error("could not determine file name")]
    NoFileName,

    #[error(
        "Download file already exists: {0}. \
        If you want to over write this file then use overwrite_existing_file(true)"
    )]
    FileExists(String),
}

impl From<std::io::Error> for BlockingDownloadError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(iotools::ThreadedIoError::Std(err))
    }
}

#[derive(Debug, thiserror::Error)]
#[allow(clippy::large_enum_variant)]
pub enum AsyncDownloadError {
    #[error(transparent)]
    Io(#[from] iotools::AsyncIoError),

    #[error("target directory does not exist {0}")]
    TargetDoesNotExist(String),

    #[error("request error: {0}")]
    Request(#[from] reqwest::Error),

    #[error("graph error: {0}")]
    Graph(#[from] GraphError),

    #[error(
        "file name is too long (max {} chars)",
        super::client::MAX_FILE_NAME_LEN
    )]
    FileNameTooLong,

    #[error("could not determine file name")]
    NoFileName,

    #[error(
        "Download file already exists: {0}. \
        If you want to over write this file then use overwrite_existing_file(true)"
    )]
    FileExists(String),
}

impl From<std::io::Error> for AsyncDownloadError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(iotools::AsyncIoError::Std(err))
    }
}
