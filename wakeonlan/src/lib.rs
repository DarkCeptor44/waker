//! # wakeonlan
//!
//! A Rust library for creating and sending Wake-on-LAN (WoL) magic packets over the network.

#![forbid(unsafe_code)]
#![warn(clippy::pedantic, missing_debug_implementations, missing_docs)]
#![allow(clippy::doc_markdown)]

mod errors;
mod types;

use anyhow::{Context, Result};
use std::net::UdpSocket;

pub use errors::MacAddressError;
pub use types::{AsMacBytes, Mac, MagicPacket};

/// Creates a Wake-on-LAN magic packet for the given MAC address
///
/// ## Arguments
///
/// * `mac_address` - A type that can be converted into a [`Mac`] struct, like:
///   - A string slice: `"01:23:45:67:89:AB"`
///   - A byte array of length 6: `[0x01, 0x23, 0x45, 0x67, 0x89, 0xAB]`
///   - A byte slice: `&[0x01, 0x23, 0x45, 0x67, 0x89, 0xAB]`
///
/// ## Returns
///
/// A [`Result`] containing the [`MagicPacket`] on success, on an error if the MAC address is invalid
///
/// ## Errors
///
/// Returns an error if the MAC address is invalid
///
/// ## Examples
///
/// Create a magic packet from a MAC address string (separated by either `:`, `-`, or `.`):
///
/// ```rust
/// use wakeonlan::create_magic_packet;
///
/// let _ = create_magic_packet("01:23:45:67:89:AB").unwrap();
/// ```
///
/// Create a magic packet from a byte array of length 6:
///
/// ```rust
/// use wakeonlan::create_magic_packet;
///
/// let _ = create_magic_packet([0x01, 0x23, 0x45, 0x67, 0x89, 0xAB]).unwrap();
/// ```
///
/// Create a magic packet from a byte slice of any length, as long as it can be converted to a 6-byte array:
///
/// ```rust
/// use wakeonlan::create_magic_packet;
///
/// let _ = create_magic_packet(&[0x01, 0x23, 0x45, 0x67, 0x89, 0xAB][..]).unwrap();
/// ```
///
/// Create a magic packet from a [`Mac`] struct:
///
/// ```rust
/// use std::str::FromStr;
/// use wakeonlan::{create_magic_packet, Mac};
///
/// let _ = create_magic_packet(Mac([0x01, 0x23, 0x45, 0x67, 0x89, 0xAB])).unwrap();
/// // or
/// let _ = create_magic_packet(Mac::from_str("01:23:45:67:89:AB").unwrap()).unwrap();
/// ```
#[allow(clippy::needless_pass_by_value)]
pub fn create_magic_packet<T>(mac_address: T) -> Result<MagicPacket, T::Error>
where
    T: AsMacBytes,
{
    let mac_bytes = mac_address.as_mac_bytes()?;

    Ok(create_magic_packet_impl(mac_bytes))
}

/// Creates a Wake-on-LAN magic packet from a 6-byte MAC address array
fn create_magic_packet_impl(addr: [u8; 6]) -> MagicPacket {
    let mut packet: Vec<u8> = vec![0xFF; 6];
    packet.reserve(96);

    for _ in 0..16 {
        packet.extend_from_slice(&addr);
    }

    MagicPacket(packet)
}

/// Sends a Wake-on-LAN magic packet to the default broadcast address (`255.255.255.255:9`)
///
/// ## Arguments
///
/// * `packet` - A reference to a [`MagicPacket`] that you want to send
///
/// ## Returns
///
/// A [`Result`] indicating success or failure of the operation
///
/// ## Errors
///
/// Returns an error if the UDP socket cannot be bound, if the broadcast option cannot be set, or if sending the packet fails
///
/// ## Example
///
/// Create a magic packet and send it to the default broadcast address (`255.255.255.255:9`):
///
/// ```rust,no_run
/// use wakeonlan::{create_magic_packet, send_magic_packet};
///
/// let packet = create_magic_packet("01:23:45:67:89:AB").unwrap();
///
/// send_magic_packet(&packet).unwrap();
/// ```
pub fn send_magic_packet(packet: &MagicPacket) -> Result<()> {
    send_magic_packet_impl(packet, "255.255.255.255:9")
}

/// Sends a Wake-on-LAN magic packet to the specified broadcast address.
///
/// This function is for advanced users, for most cases you should use [`send_magic_packet`] instead.
///
/// ## Arguments
///
/// * `packet` - A reference to a [`MagicPacket`] that you want to send
/// * `broadcast_address` - A string slice representing the broadcast address and port, e.g., `"192.168.0.255:9"`
///
/// ## Returns
///
/// A [`Result`] indicating success or failure of the operation
///
/// ## Errors
///
/// Returns an error if the UDP socket cannot be bound, if the broadcast option cannot be set, or if sending the packet fails
///
/// /// ## Example
///
/// Create a magic packet and send it to a specific broadcast address:
///
/// ```rust,no_run
/// use wakeonlan::{create_magic_packet, send_magic_packet_to_broadcast_address};
///
/// let packet = create_magic_packet("01:23:45:67:89:AB").unwrap();
/// let addr = "192.168.0.255:9"; // Replace with your broadcast address and port
///
/// send_magic_packet_to_broadcast_address(&packet, addr).unwrap();
/// ```
pub fn send_magic_packet_to_broadcast_address<S>(
    packet: &MagicPacket,
    broadcast_address: S,
) -> Result<()>
where
    S: AsRef<str>,
{
    send_magic_packet_impl(packet, broadcast_address.as_ref())
}

/// Sends a Wake-on-LAN magic packet to the specified address
fn send_magic_packet_impl(packet: &MagicPacket, addr: &str) -> Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0").context("Failed to bind UDP socket")?;

    socket
        .set_broadcast(true)
        .context("Failed to set socket to broadcast")?;
    socket
        .send_to(&packet.0, addr)
        .context("Failed to send magic packet")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    const EXPECTED_PACKET: [u8; 102] = [
        255, 255, 255, 255, 255, 255, 1, 35, 69, 103, 137, 171, 1, 35, 69, 103, 137, 171, 1, 35,
        69, 103, 137, 171, 1, 35, 69, 103, 137, 171, 1, 35, 69, 103, 137, 171, 1, 35, 69, 103, 137,
        171, 1, 35, 69, 103, 137, 171, 1, 35, 69, 103, 137, 171, 1, 35, 69, 103, 137, 171, 1, 35,
        69, 103, 137, 171, 1, 35, 69, 103, 137, 171, 1, 35, 69, 103, 137, 171, 1, 35, 69, 103, 137,
        171, 1, 35, 69, 103, 137, 171, 1, 35, 69, 103, 137, 171, 1, 35, 69, 103, 137, 171,
    ];

    #[test]
    fn test_create_magic_packet_from_str() {
        let packet = create_magic_packet("01:23:45:67:89:AB").unwrap();

        assert_eq!(packet.0, EXPECTED_PACKET);
    }

    #[test]
    fn test_create_magic_packet_from_bytes() {
        let mac_bytes: [u8; 6] = [0x01, 0x23, 0x45, 0x67, 0x89, 0xAB];
        let packet = create_magic_packet(mac_bytes).unwrap();

        assert_eq!(packet.0, EXPECTED_PACKET);
    }

    #[test]
    #[should_panic(expected = "InvalidLength(5)")]
    fn test_invalid_mac_length() {
        let mac_bytes: [u8; 5] = [0x01, 0x23, 0x45, 0x67, 0x89];

        create_magic_packet(&mac_bytes[..]).unwrap();
    }

    #[test]
    #[should_panic(expected = r#"InvalidMacAddress("01:23:45:67:89")"#)]
    fn test_invalid_mac_str() {
        create_magic_packet("01:23:45:67:89").unwrap();
    }

    #[test]
    fn test_send_magic_packet() {
        let rec_socket = UdpSocket::bind("127.0.0.1:0").expect("Failed to bind receiving socket");
        rec_socket
            .set_read_timeout(Some(Duration::from_millis(100)))
            .expect("Failed to set read timeout");
        let rec_addr = rec_socket
            .local_addr()
            .expect("Failed to get local address");

        let mac = Mac([0x01, 0x23, 0x45, 0x67, 0x89, 0xAB]);
        let packet = create_magic_packet(mac).expect("Failed to create magic packet");

        send_magic_packet_to_broadcast_address(&packet, rec_addr.to_string())
            .expect("Failed to send magic packet");

        let mut buffer = [0u8; 102];
        rec_socket
            .recv_from(&mut buffer)
            .expect("Failed to receive magic packet");

        assert_eq!(buffer, EXPECTED_PACKET);
    }
}
