// Copyright (C) 2025 DarkCeptor44
//
// This file is part of waker.
//
// waker is free software: you can redistribute it and/or modify
// it under theterms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// waker is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with waker.  If not, see <https://www.gnu.org/licenses/>.

//! # waker
//!
//! A Rust library for creating and sending Wake-on-LAN (WoL) magic packets over the network.
//!
//! ## Installation
//!
//! You can add this library as a dependency with the following command:
//!
//! ```bash
//! cargo add waker
//! ```
//!
//! Or you can add this to your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! waker = "^0.1"
//! ```
//!
//! ## Features
//!
//! - `serde`: Enables serialization and deserialization of the [`Mac`] and [`MagicPacket`] types.
//!
//! ## Usage
//!
//! To wake a machine you will need the MAC address (it can also be called physical or hardware address) for its network interface, then you just need to create a magic packet and send it to the broadcast address, by default it's usually `255.255.255.255:9` so you can just use [`send_magic_packet`], if you want to send it to a specific broadcast address you can use [`send_magic_packet_to_broadcast_address`].
//!
//! The easiest way to create a magic packet is to use [`create_magic_packet`]:
//!
//! ```rust
//! use waker::create_magic_packet;
//!
//! let packet = create_magic_packet("01:23:45:67:89:AB").unwrap();
//! ```
//!
//! The MAC address can be passed as either [`&str`](str), [`String`], a byte array of length 6 ([`[u8; 6]`](u8)) or a byte slice ([`&[u8]`](u8)). Currently the string MAC address must have its bytes separated but `:`, `.` or `-` are all supported as separators.
//!
//! The magic packet can then be sent using [`send_magic_packet`]:
//!
//! ```rust,no_run
//! use waker::{create_magic_packet, send_magic_packet};
//!
//! let packet = create_magic_packet("01:23:45:67:89:AB").unwrap();
//!
//! send_magic_packet(&packet).unwrap();
//! ```
//!
//! To send the packet to a specific broadcast address you can use [`send_magic_packet_to_broadcast_address`] (note that the address must be in the format `IP:PORT`):
//!
//! ```rust,no_run
//! use waker::{create_magic_packet, send_magic_packet_to_broadcast_address};
//!
//! let packet = create_magic_packet("01:23:45:67:89:AB").unwrap();
//!
//! send_magic_packet_to_broadcast_address(&packet, "192.168.0.255:9").unwrap();
//! ```
//!
//! ## Benchmarks
//!
//! ### MAC-related
//!
//! ```text
//! Timer precision: 100 ns
//! mac                        fastest       │ slowest       │ median        │ mean          │ samples │ iters
//! ├─ create_mac_from_string  14.01 ns      │ 16.45 ns      │ 14.11 ns      │ 14.19 ns      │ 100     │ 102400
//! ├─ hex_val                 1.969 ns      │ 2.701 ns      │ 1.981 ns      │ 1.998 ns      │ 100     │ 819200
//! ╰─ u8_from_str_radix       1.542 ns      │ 1.81 ns       │ 1.554 ns      │ 1.552 ns      │ 100     │ 819200
//! ```
//!
//! ### Packet Creation
//!
//! ```text
//! Timer precision: 100 ns
//! packet_creation                     fastest       │ slowest       │ median        │ mean          │ samples │ iters
//! ├─ create_magic_packet_from_bytes   123.1 ns      │ 137.2 ns      │ 123.9 ns      │ 124.3 ns      │ 100     │ 12800
//! ╰─ create_magic_packet_from_string  131 ns        │ 152.1 ns      │ 132.5 ns      │ 132.6 ns      │ 100     │ 12800
//! ```
//!
//! ### Packet Send
//!
//! ```text
//! Timer precision: 100 ns
//! packet_send           fastest       │ slowest       │ median        │ mean          │ samples │ iters
//! ╰─ send_magic_packet  77.59 µs      │ 232.6 µs      │ 81.64 µs      │ 83.85 µs      │ 100     │ 100
//! ```
//!
//! ## MSRV
//!
//! The Minimum Supported Rust Version (MSRV) for `waker` is **1.78**.
//!
//! ## License
//!
//! This project is licensed under the [Lesser GNU Public License v3.0](https://www.gnu.org/licenses/lgpl-3.0.en.html).

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
/// use waker::create_magic_packet;
///
/// let _ = create_magic_packet("01:23:45:67:89:AB").unwrap();
/// ```
///
/// Create a magic packet from a byte array of length 6:
///
/// ```rust
/// use waker::create_magic_packet;
///
/// let _ = create_magic_packet([0x01, 0x23, 0x45, 0x67, 0x89, 0xAB]).unwrap();
/// ```
///
/// Create a magic packet from a byte slice of any length, as long as it can be converted to a 6-byte array:
///
/// ```rust
/// use waker::create_magic_packet;
///
/// let _ = create_magic_packet(&[0x01, 0x23, 0x45, 0x67, 0x89, 0xAB][..]).unwrap();
/// ```
///
/// Create a magic packet from a [`Mac`] struct:
///
/// ```rust
/// use std::str::FromStr;
/// use waker::{create_magic_packet, Mac};
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

/// Converts a character to a hexadecimal value.
///
/// This performs around 0.5ms slower than [`u8::from_str_radix`] but avoids allocations
///
/// ## Arguments
///
/// * `c` - A character to convert to a hexadecimal value
///
/// ## Returns
///
/// A [`Result`] containing the hexadecimal value on success, or an error if the character is not a valid hexadecimal character
///
/// ## Errors
///
/// Returns an error if the character is not a valid hexadecimal character
pub fn hex_val(c: char) -> Result<u8, MacAddressError> {
    match c {
        '0'..='9' => Ok(c as u8 - b'0'),
        'a'..='f' => Ok(c as u8 - b'a' + 10),
        'A'..='F' => Ok(c as u8 - b'A' + 10),
        _ => Err(MacAddressError::InvalidByteInMac(c.to_string())),
    }
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
/// use waker::{create_magic_packet, send_magic_packet};
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
/// use waker::{create_magic_packet, send_magic_packet_to_broadcast_address};
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
