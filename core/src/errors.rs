// Copyright (C) 2024 Dirk Strauss
//
// This file is part of Nachtwacht.
//
// Nachtwacht is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Nachtwacht is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use protobuf_json_mapping::ParseError;
use reqwest::Error;
use std::io;
use std::path::Path;

use nachtwacht_models::generated::longhorn::ErrorBlock;
use thiserror::Error;

/// This enum contains all known errors so far.
#[derive(Error, Debug)]
pub enum AgentErrors {
    /// Error to indicate a read error for the given file.
    #[error("Could not access local file data from {0}")]
    FailedToGetLocalInfo(String),
    #[error("A required kube config at {0} was not found! Perhaps you need to configure this?")]
    KubeConfigNotFound(Box<Path>),
    #[error("data store disconnected")]
    Disconnect(#[from] io::Error),
    /// Error if something is not yet implemented.
    #[error("This flow is not (yet) implemented!")]
    NotImplemented(),
}

#[derive(Error, Debug)]
pub enum K8sErrors {
    #[error("An internal Longhorn error occurred: {0}")]
    LhServerError(ErrorBlock),
    #[error("A network error occurred!")]
    ReqwestNetworkError(#[from] Error),
    #[error("Could not parse the given json into object!")]
    JsonParseError(#[from] ParseError),
    /// Error if something is not yet implemented.
    #[error("This flow is not (yet) implemented!")]
    NotImplemented(),
}
