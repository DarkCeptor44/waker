// Copyright (C) 2025 DarkCeptor44
//
// This file is part of wakeonlan.
//
// wakeonlan is free software: you can redistribute it and/or modify
// it under theterms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// wakeonlan is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with wakeonlan.  If not, see <https://www.gnu.org/licenses/>.

use thiserror::Error;

/// Represents errors that can occur when working with MAC addresses
#[derive(Debug, Error, PartialEq, Eq)]
pub enum MacAddressError {
    /// This happens when the MAC address byte cannot be parsed as a hexadecimal number
    #[error("Invalid byte in MAC address: {0}")]
    InvalidByteInMac(String),

    /// This happens when the MAC address string is not 6 bytes long or has an invalid format
    #[error("Invalid MAC address: {0}")]
    InvalidMacAddress(String),

    /// This happens when the MAC address byte slice is not 6 bytes long
    #[error("Invalid MAC address length: expected 6 bytes, got {0}")]
    InvalidLength(usize),
}
