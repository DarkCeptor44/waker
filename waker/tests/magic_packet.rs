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

use std::{net::UdpSocket, time::Duration};
use waker::{create_magic_packet, send_magic_packet_to_broadcast_address, Mac};

const MAC_BYTES: [u8; 6] = [0x01, 0x23, 0x45, 0x67, 0x89, 0xAB];
const EXPECTED_PACKET: [u8; 102] = [
    255, 255, 255, 255, 255, 255, 1, 35, 69, 103, 137, 171, 1, 35, 69, 103, 137, 171, 1, 35, 69,
    103, 137, 171, 1, 35, 69, 103, 137, 171, 1, 35, 69, 103, 137, 171, 1, 35, 69, 103, 137, 171, 1,
    35, 69, 103, 137, 171, 1, 35, 69, 103, 137, 171, 1, 35, 69, 103, 137, 171, 1, 35, 69, 103, 137,
    171, 1, 35, 69, 103, 137, 171, 1, 35, 69, 103, 137, 171, 1, 35, 69, 103, 137, 171, 1, 35, 69,
    103, 137, 171, 1, 35, 69, 103, 137, 171, 1, 35, 69, 103, 137, 171,
];

#[test]
fn test_create_magic_packet_from_str() {
    let packet = create_magic_packet("01:23:45:67:89:AB").unwrap();
    assert_eq!(packet.0, EXPECTED_PACKET);
}

#[test]
fn test_create_magic_packet_from_bytes() {
    let packet = create_magic_packet(MAC_BYTES).unwrap();
    assert_eq!(packet.0, EXPECTED_PACKET);
}

#[test]
#[should_panic(expected = "InvalidLength(5)")]
fn test_create_magic_packet_panics_on_invalid_mac_length() {
    let mac_bytes: [u8; 5] = [0x01, 0x23, 0x45, 0x67, 0x89];
    create_magic_packet(&mac_bytes[..]).unwrap();
}

#[test]
#[should_panic(expected = "InvalidLength(14)")]
fn test_create_magic_packet_panics_on_invalid_mac_str() {
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

    let mac = Mac(MAC_BYTES);
    let packet = create_magic_packet(mac).expect("Failed to create magic packet");

    send_magic_packet_to_broadcast_address(&packet, rec_addr.to_string())
        .expect("Failed to send magic packet");

    let mut buffer = [0u8; 102];
    rec_socket
        .recv_from(&mut buffer)
        .expect("Failed to receive magic packet");

    assert_eq!(buffer, EXPECTED_PACKET);
}

#[test]
#[cfg(feature = "serde")]
fn test_magic_packet_serde_serialize() {
    use waker::MagicPacket;

    let packet = MagicPacket(EXPECTED_PACKET.to_vec());
    let s = serde_json::to_string(&packet).expect("Failed to serialize magic packet");
    assert_eq!(s, format!("{:?}", EXPECTED_PACKET).replace(" ", ""));
}

#[test]
#[cfg(feature = "serde")]
fn test_magic_packet_serde_deserialize() {
    use waker::MagicPacket;

    let s = format!("{:?}", EXPECTED_PACKET).replace(" ", "");
    let packet: MagicPacket = serde_json::from_str(&s).expect("Failed to deserialize magic packet");
    assert_eq!(packet.0, EXPECTED_PACKET);
}
