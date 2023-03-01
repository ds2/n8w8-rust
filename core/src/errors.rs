// Copyright (C) 2023 Dirk Strauss
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

use std::io;

use thiserror::Error;

/// This enum contains all known errors so far.
#[derive(Error, Debug)]
pub enum AgentErrors {
    /// Error to indicate a read error for the given file.
    #[error("Could not access local file data from {0}")]
    FailedToGetLocalInfo(String),
    /// No idea.
    #[error("data store disconnected")]
    Disconnect(#[from] io::Error),
    /// Error if something is not yet implemented.
    #[error("This flow is not (yet) implemented!")]
    NotImplemented(),
}
