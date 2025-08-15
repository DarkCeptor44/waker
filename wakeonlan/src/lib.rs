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
