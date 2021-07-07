use from_as::*;
use std::convert::TryFrom;
use std::io::{Read, Write};

/// Describes the type of action this request will perform. In some instances
/// the task described is just the return type for the request.
///
/// The RequestTask is mainly used to tell what macro will be used to write out
/// the actual request.
///
/// For instance, an upload session must perform certain operations such as
/// reading a file from a drive while a simple GET request just returns json.
/// Both the upload session and GET request will have a separate and
/// specific macro that is used to generate the method for the api clients.
#[derive(Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub enum RequestTask {
    /// 204 no-content response.
    NoContent,

    /// JSON in the response body.
    Json,

    /// The request is an upload of an item such as uploading
    /// a file to OneDrive.
    Upload,

    /// The request is an upload session and will perform a series
    /// of requests to upload an item.
    UploadSession,

    /// Bytes or [`Vec<u8>`] in the response body.
    Bytes,

    /// The request is a download of an item from the graph api
    /// such as a file from OneDrive.
    Download,

    /// The request is an asynchronous download of an item from the graph api
    /// such as a file from OneDrive.
    AsyncDownload,

    /// The request is a delta operation where multiple items have been requested
    /// and the responses will come in a series of responses typically through
    /// [mpsc](https://doc.rust-lang.org/std/sync/mpsc/index.html) channels.
    Delta,
}

impl RequestTask {
    /// Each RequestTask can be represented as a return type in the sdk client methods:
    /// [`IntoResponse<'a, T, Client>`](graph_http::IntoResponse) is the same as
    /// [`IntoResponse<'a, NoContent, Client>`](graph_http::IntoResponse)
    pub fn type_name(&self) -> &'static str {
        match self {
            RequestTask::NoContent
            | RequestTask::Upload => "NoContent",
            RequestTask::Json => "serde_json::Value",
            RequestTask::UploadSession => "UploadSessionClient<Client>",
            RequestTask::Bytes => "Vec<u8>",
            RequestTask::Download => "BlockingDownload",
            RequestTask::AsyncDownload => "AsyncDownload",
            RequestTask::Delta => "DeltaPhantom<serde_json::Value>",
        }
    }

    /// Imports required for the task to compile and perform properly.
    /// Not all request tasks will have required imports.
    pub fn imports(&self) -> Vec<&str> {
        match self {
            RequestTask::Json
            | RequestTask::Bytes => vec![],
            RequestTask::NoContent
            | RequestTask::Upload => vec!["graph_http::types::NoContent"],
            RequestTask::UploadSession => vec![
                "std::path::Path",
                "graph_error::GraphFailure",
                "graph_http::UploadSessionClient",
            ],
            RequestTask::Download => vec![
                "std::path::Path",
                "graph_error::GraphFailure",
                "graph_http::{BlockingDownload, BlockingHttpClient}",
            ],
            RequestTask::AsyncDownload => vec![
                "std::path::Path",
                "graph_error::GraphFailure",
                "graph_http::{AsyncDownload, AsyncHttpClient}",
            ],
            RequestTask::Delta => vec!["graph_http::types::DeltaPhantom"],
        }
    }
}

impl Default for RequestTask {
    fn default() -> Self {
        RequestTask::Json
    }
}
