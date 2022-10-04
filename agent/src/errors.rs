use std::io;

use thiserror::Error;

/// This enum contains all known errors so far.
#[derive(Error, Debug)]
pub enum AgentErrors {
    #[error("Could not access local file data from /proc")]
    FailedToGetLocalInfo(),
    #[error("data store disconnected")]
    Disconnect(#[from] io::Error),
    // #[error("the data for key `{0}` is not available")]
    // Redaction(String),
    // #[error("invalid header (expected {expected:?}, found {found:?})")]
    // InvalidHeader { expected: String, found: String },
    /// Simple unknown error. Usually something to avoid.
    #[error("unknown data store error")]
    Unknown,
}
