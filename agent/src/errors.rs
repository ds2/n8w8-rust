use std::io;

use thiserror::Error;

/// This enum contains all known errors so far.
#[derive(Error, Debug)]
pub enum AgentErrors {
    #[error("Could not access local file data from {0}")]
    FailedToGetLocalInfo(String),
    #[error("data store disconnected")]
    Disconnect(#[from] io::Error),
}
