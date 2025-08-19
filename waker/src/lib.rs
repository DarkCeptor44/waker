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
//!
//! # this also works
//! waker = { version = "^0.1", git = "https://github.com/DarkCeptor44/waker" }
//! ```
//!
//! ## Features
//!
//! - `serde`: Enables serialization and deserialization of the [`Mac`] and [`MagicPacket`] types.
//!
//! ## Usage
//!
//! To wake a machine you will need the MAC address (it can also be called physical or hardware address) for its network interface, then you just need to create a magic packet and send it to the broadcast address, by default it's usually `255.255.255.255:9` so you can just use [`wake_device`], if you want to send it to a specific broadcast address you can pass a [`WakeOptions`] struct.
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
//! The magic packet can then be sent using [`wake_device`]:
//!
//! ```rust,no_run
//! use waker::{create_magic_packet, wake_device};
//!
//! let packet = create_magic_packet("01:23:45:67:89:AB").unwrap();
//! wake_device(&packet).unwrap();
//! ```
//!
//! To send the packet to a specific broadcast address you have to pass a [`WakeOptions`] struct with the address (note that it must be in the format `IP:PORT`):
//!
//! ```rust,no_run
//! use waker::{create_magic_packet, wake_device, WakeOptions};
//!
//! let packet = create_magic_packet("01:23:45:67:89:AB").unwrap();
//! wake_device(WakeOptions::new(&packet).broadcast_address("192.168.0.255:9")).unwrap();
//! ```
//!
//! You can also specify the address to bind the UDP socket to (default is `0.0.0.0:0`):
//!
//! ```rust,no_run
//! use waker::{create_magic_packet, wake_device, WakeOptions};
//!
//! let packet = create_magic_packet("01:23:45:67:89:AB").unwrap();
//! wake_device(WakeOptions::new(&packet).bind_address("127.0.0.1:0")).unwrap();
//! ```
//!
//! ## Audits
//!
//! No vulnerabilities found according to [cargo-audit](https://crates.io/crates/cargo-audit/)
//!
//! ## Benchmarks
//!
//! ### MAC-related
//!
//! ```text
//! Timer precision: 100 ns
//! mac                        fastest       │ slowest       │ median        │ mean          │ samples │ iters
//! ├─ create_mac_from_string  15.18 ns      │ 21.43 ns      │ 15.28 ns      │ 15.6 ns       │ 100     │ 102400
//! ├─ hex_val                 1.981 ns      │ 8.5 ns        │ 1.993 ns      │ 2.144 ns      │ 100     │ 819200
//! ╰─ u8_from_str_radix       1.542 ns      │ 1.554 ns      │ 1.554 ns      │ 1.549 ns      │ 100     │ 819200
//! ```
//!
//! ### Packet Creation
//!
//! ```text
//! Timer precision: 100 ns
//! packet_creation                     fastest       │ slowest       │ median        │ mean          │ samples │ iters
//! ├─ create_magic_packet_from_bytes   114.6 ns      │ 138 ns        │ 118.5 ns      │ 118.8 ns      │ 100     │ 12800
//! ╰─ create_magic_packet_from_string  132.5 ns      │ 186.4 ns      │ 133.3 ns      │ 135.7 ns      │ 100     │ 12800
//! ```
//!
//! ### Packet Send
//!
//! ```text
//! Timer precision: 100 ns
//! packet_send     fastest       │ slowest       │ median        │ mean          │ samples │ iters
//! ╰─ wake_device  78.49 µs      │ 268 µs        │ 81.09 µs      │ 84.5 µs       │ 100     │ 100
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
pub use types::{AsMacBytes, Mac, MagicPacket, WakeOptions};

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

/// Sends a Wake-on-LAN magic packet to a broadcast address for waking up a specific device
///
/// ## Arguments
///
/// * `options` - A [`WakeOptions`] struct containing the magic packet, broadcast address, and bind address
///
/// ## Returns
///
/// A [`Result`] indicating success or failure of the operation
///
/// ## Errors
///
/// Returns an error if the UDP socket cannot be bound, if the broadcast option cannot be set, or if sending the packet fails
///
/// ## Examples
///
/// Create a magic packet and send it to the default broadcast address (`255.255.255.255:9`):
///
/// ```rust,no_run
/// use waker::{create_magic_packet, wake_device, WakeOptions};
///
/// let packet = create_magic_packet("01:23:45:67:89:AB").unwrap();
/// wake_device(&packet).unwrap();
/// ```
///
/// Create a magic packet and send it to a specific broadcast address:
///
/// ```rust,no_run
/// use waker::{create_magic_packet, wake_device, WakeOptions};
///
/// let packet = create_magic_packet("01:23:45:67:89:AB").unwrap();
/// let addr = "192.168.0.255:9"; // Replace with your broadcast address and port
/// wake_device(WakeOptions::new(&packet).broadcast_address(addr)).unwrap();
/// ```
///
/// Create a magic packet and send it to a specific bind address:
///
/// ```rust,no_run
/// use waker::{create_magic_packet, wake_device, WakeOptions};
///
/// let packet = create_magic_packet("01:23:45:67:89:AB").unwrap();
/// let addr = "0.0.0.0:0"; // Replace with your bind address and port
/// wake_device(WakeOptions::new(&packet).bind_address(addr)).unwrap();
/// ```
pub fn wake_device<'a, O>(options: O) -> Result<()>
where
    O: Into<WakeOptions<'a>>,
{
    wake_device_impl(options.into())
}

/// Sends a Wake-on-LAN magic packet to a broadcast address for waking up a specific device
#[allow(clippy::needless_pass_by_value)]
fn wake_device_impl(options: WakeOptions) -> Result<()> {
    let socket = UdpSocket::bind(&*options.bind_address).context("Failed to bind UDP socket")?;

    // TODO implement secure_on

    socket
        .set_broadcast(true)
        .context("Failed to set socket to broadcast")?;
    socket
        .send_to(&options.packet.0, &*options.broadcast_address)
        .context("Failed to send magic packet")?;

    Ok(())
}
