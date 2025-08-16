use crate::{hex_val, MacAddressError};
use std::{convert::Infallible, fmt, str::FromStr};

/// A trait for types that can be converted into a MAC address byte array
pub trait AsMacBytes {
    /// The error type returned by the conversion
    type Error;

    /// Converts the implementing type into a MAC address byte array
    ///
    /// ## Returns
    ///
    /// A [`Result`] containing the MAC address as a byte array on success, on an error if the conversion fails
    ///
    /// ## Errors
    ///
    /// Returns an error if the conversion fails
    fn as_mac_bytes(&self) -> Result<[u8; 6], Self::Error>;
}

impl AsMacBytes for Mac {
    type Error = Infallible;

    fn as_mac_bytes(&self) -> Result<[u8; 6], Self::Error> {
        Ok(self.0)
    }
}

impl AsMacBytes for &[u8] {
    type Error = MacAddressError;

    fn as_mac_bytes(&self) -> Result<[u8; 6], Self::Error> {
        if self.len() != 6 {
            return Err(MacAddressError::InvalidLength(self.len()));
        }

        let mut mac_bytes = [0u8; 6];
        mac_bytes.copy_from_slice(&self[0..6]);

        Ok(mac_bytes)
    }
}

impl AsMacBytes for [u8; 6] {
    type Error = Infallible;

    fn as_mac_bytes(&self) -> Result<[u8; 6], Self::Error> {
        Ok(*self)
    }
}

impl AsMacBytes for &str {
    type Error = MacAddressError;

    fn as_mac_bytes(&self) -> Result<[u8; 6], Self::Error> {
        let mac_addr = Mac::from_str(self)?;

        Ok(mac_addr.0)
    }
}

impl AsMacBytes for String {
    type Error = MacAddressError;

    fn as_mac_bytes(&self) -> Result<[u8; 6], Self::Error> {
        let mac_addr = Mac::from_str(self.as_str())?;

        Ok(mac_addr.0)
    }
}

impl AsMacBytes for &String {
    type Error = MacAddressError;

    fn as_mac_bytes(&self) -> Result<[u8; 6], Self::Error> {
        let mac_addr = Mac::from_str(self.as_str())?;

        Ok(mac_addr.0)
    }
}

/// Represents a Wake-on-LAN magic packet
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MagicPacket(pub Vec<u8>);

impl AsRef<[u8]> for MagicPacket {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

/// Represents a MAC address
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Mac(pub [u8; 6]);

impl From<[u8; 6]> for Mac {
    fn from(value: [u8; 6]) -> Self {
        Self(value)
    }
}

impl TryFrom<&[u8]> for Mac {
    type Error = MacAddressError;

    fn try_from(value: &[u8]) -> std::result::Result<Self, Self::Error> {
        if value.len() != 6 {
            return Err(MacAddressError::InvalidLength(value.len()));
        }

        let mut bytes = [0u8; 6];
        bytes.copy_from_slice(value);

        Ok(Self(bytes))
    }
}

impl TryFrom<&str> for Mac {
    type Error = MacAddressError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl FromStr for Mac {
    type Err = MacAddressError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let s = s.trim();
        let mut bytes = [0u8; 6];
        let mut s_chars = s.chars().peekable();

        for (i, byte_ref) in bytes.iter_mut().enumerate() {
            let c1 = s_chars
                .next()
                .ok_or(MacAddressError::InvalidLength(s.len()))?;
            let c2 = s_chars
                .next()
                .ok_or(MacAddressError::InvalidLength(s.len()))?;

            let val = (hex_val(c1)? << 4) | hex_val(c2)?;
            *byte_ref = val;

            if i < 5 {
                match s_chars.next() {
                    Some(c) if c == ':' || c == '-' || c == '_' || c == '.' => {}
                    Some(_) => return Err(MacAddressError::InvalidMacAddress(s.to_string())),
                    None => return Err(MacAddressError::InvalidLength(s.len())),
                }
            }
        }

        if s_chars.next().is_some() {
            return Err(MacAddressError::InvalidLength(s.len()));
        }

        Ok(Self(bytes))
    }
}

impl fmt::Display for Mac {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(self, f)
    }
}

impl fmt::LowerHex for Mac {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5]
        )
    }
}

impl fmt::UpperHex for Mac {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5]
        )
    }
}
