use std::{net::UdpSocket, time::Duration};
use wakeonlan::{create_magic_packet, send_magic_packet_to_broadcast_address, Mac};

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
    let mac_bytes: [u8; 6] = [0x01, 0x23, 0x45, 0x67, 0x89, 0xAB];
    let packet = create_magic_packet(mac_bytes).unwrap();

    assert_eq!(packet.0, EXPECTED_PACKET);
}

#[test]
#[should_panic(expected = "InvalidLength(5)")]
fn test_create_magic_packet_panics_on_invalid_mac_length() {
    let mac_bytes: [u8; 5] = [0x01, 0x23, 0x45, 0x67, 0x89];

    create_magic_packet(&mac_bytes[..]).unwrap();
}

#[test]
#[should_panic(expected = r#"InvalidMacAddress("01:23:45:67:89")"#)]
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

#[test]
fn test_mac_display_lower() {
    let mac = Mac([0x01, 0x23, 0x45, 0x67, 0x89, 0xAB]);

    assert_eq!(mac.to_string(), "01:23:45:67:89:ab");
}

#[test]
fn test_mac_display_upper() {
    let mac = Mac([0x01, 0x23, 0x45, 0x67, 0x89, 0xAB]);

    assert_eq!(format!("{mac:X}"), "01:23:45:67:89:AB");
}
