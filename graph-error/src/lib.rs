//! Various errors used to implement the Graph API and OAuth clients for the graph-rs project
//! See the project on [GitHub](https://github.com/sreeise/graph-rs).

#[macro_use]
extern crate serde;

mod error;
mod graph_failure;
mod headers;
mod internal;

pub use error::*;
pub use graph_failure::*;
pub use headers::*;
pub use internal::*;

pub type GraphResult<T> = Result<T, GraphFailure>;
