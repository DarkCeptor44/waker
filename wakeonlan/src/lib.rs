//! # wol
//!
//! A Rust library for creating and sending Wake-on-LAN (WoL) magic packets over the network.

#![forbid(unsafe_code)]
#![warn(clippy::pedantic, missing_debug_implementations, missing_docs)]
#![allow(clippy::doc_markdown)]

mod types;

use anyhow::{anyhow, Result};

pub use types::MagicPacket;

/// Creates a Wake-on-LAN magic packet for the given MAC address
///
/// ## Arguments
///
/// * `mac_address` - A byte slice representing the MAC address. It must be exactly 6 bytes long
///
/// ## Returns
///
/// A [`MagicPacket`] containing the Wake-on-LAN magic packet data
///
/// ## Errors
///
/// Returns an error if the MAC address is not exactly 6 bytes long
pub fn create_magic_packet<T>(mac_address: T) -> Result<MagicPacket>
where
    T: AsRef<[u8]>,
{
    create_magic_packet_impl(mac_address.as_ref())
}

fn create_magic_packet_impl(addr: &[u8]) -> Result<MagicPacket> {
    if addr.len() != 6 {
        return Err(anyhow!(
            "MAC address must be 6 bytes long, got {}",
            addr.len()
        ));
    }

    let mut packet: Vec<u8> = vec![0xFF; 6];
    packet.reserve(96);

    for _ in 0..16 {
        packet.extend_from_slice(addr);
    }

    Ok(MagicPacket { data: packet })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_magic_packet_from_bytes() {
        let mac_bytes: [u8; 6] = [0x01, 0x23, 0x45, 0x67, 0x89, 0xAB];
        let packet = create_magic_packet(mac_bytes).unwrap();

        assert_eq!(
            packet.data,
            vec![
                255, 255, 255, 255, 255, 255, 1, 35, 69, 103, 137, 171, 1, 35, 69, 103, 137, 171,
                1, 35, 69, 103, 137, 171, 1, 35, 69, 103, 137, 171, 1, 35, 69, 103, 137, 171, 1,
                35, 69, 103, 137, 171, 1, 35, 69, 103, 137, 171, 1, 35, 69, 103, 137, 171, 1, 35,
                69, 103, 137, 171, 1, 35, 69, 103, 137, 171, 1, 35, 69, 103, 137, 171, 1, 35, 69,
                103, 137, 171, 1, 35, 69, 103, 137, 171, 1, 35, 69, 103, 137, 171, 1, 35, 69, 103,
                137, 171, 1, 35, 69, 103, 137, 171
            ]
        );
    }

    #[test]
    #[should_panic(expected = "MAC address must be 6 bytes long, got 5")]
    fn test_invalid_mac_length() {
        let mac_bytes: [u8; 5] = [0x01, 0x23, 0x45, 0x67, 0x89];

        create_magic_packet(mac_bytes).unwrap();
    }
}
