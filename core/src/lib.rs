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

/// This crate contains the business logic for N8w8.

/// Some common errors that may occur.
pub mod errors;
/// Module to load data from /proc/loadavg.
pub mod proc_loadavg;
/// Module to read /proc/meminfo.
pub mod proc_meminfo;
/// Module to contain reader methods for /proc/stat.
pub mod proc_stat;
