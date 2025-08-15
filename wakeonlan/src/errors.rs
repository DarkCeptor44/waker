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
