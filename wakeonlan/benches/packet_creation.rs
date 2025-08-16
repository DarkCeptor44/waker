// Copyright (C) 2025 DarkCeptor44
//
// This file is part of wakeonlan.
//
// wakeonlan is free software: you can redistribute it and/or modify
// it under theterms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// wakeonlan is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with wakeonlan.  If not, see <https://www.gnu.org/licenses/>.

use divan::black_box;
use wakeonlan as wol;

const MAC_BYTES: [u8; 6] = [0x01, 0x23, 0x45, 0x67, 0x89, 0xAB];
const MAC_STRING: &str = "01:23:45:67:89:AB";

fn main() {
    divan::main();
}

#[divan::bench]
fn create_magic_packet_from_bytes() {
    let res = wol::create_magic_packet(black_box(MAC_BYTES));
    black_box(res.expect("Failed to create magic packet"));
}

#[divan::bench]
fn create_magic_packet_from_string() {
    let res = wol::create_magic_packet(black_box(MAC_STRING));
    black_box(res.expect("Failed to create magic packet"));
}