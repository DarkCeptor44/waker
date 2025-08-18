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

use divan::{black_box, Bencher};
use std::{net::UdpSocket, time::Duration};

const MAC_BYTES: [u8; 6] = [0x01, 0x23, 0x45, 0x67, 0x89, 0xAB];

fn main() {
    divan::main();
}

#[divan::bench]
fn send_magic_packet(b: Bencher) {
    let rec_socket = UdpSocket::bind("127.0.0.1:0").expect("Failed to bind receiving socket");
    rec_socket
        .set_read_timeout(Some(Duration::from_millis(100)))
        .expect("Failed to set read timeout");
    let rec_addr = rec_socket
        .local_addr()
        .expect("Failed to get local address");
    let packet = waker::create_magic_packet(MAC_BYTES).expect("Failed to create magic packet");

    b.bench(|| {
        waker::send_magic_packet_to_broadcast_address(
            black_box(&packet),
            black_box(rec_addr.to_string()),
        )
        .expect("Failed to send magic packet");
        black_box(());
    });
}
