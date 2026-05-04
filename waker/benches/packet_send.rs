// This Source Code Form is subject to the terms of the
// Mozilla Public License, v. 2.0. If a copy of the MPL was not distributed
// with this file, You can obtain one at http://mozilla.org/MPL/2.0/.

use divan::{Bencher, black_box};
use std::{net::UdpSocket, time::Duration};
use waker::WakeOptions;

const MAC_BYTES: [u8; 6] = [0x01, 0x23, 0x45, 0x67, 0x89, 0xAB];

fn main() {
    divan::main();
}

#[divan::bench]
fn wake_device(b: Bencher) {
    let rec_socket = UdpSocket::bind("127.0.0.1:0").expect("Failed to bind receiving socket");
    rec_socket
        .set_read_timeout(Some(Duration::from_millis(100)))
        .expect("Failed to set read timeout");
    let rec_addr = rec_socket
        .local_addr()
        .expect("Failed to get local address");
    let packet = waker::create_magic_packet(MAC_BYTES).expect("Failed to create magic packet");

    b.bench(|| {
        waker::wake_device(black_box(
            WakeOptions::new(black_box(&packet)).broadcast_address(black_box(rec_addr.to_string())),
        ))
        .expect("Failed to send magic packet");
        black_box(());
    });
}
