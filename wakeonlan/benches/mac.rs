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
use std::str::FromStr;
use wakeonlan::Mac;

const MAC_STRING: &str = "01:23:45:67:89:AB";

fn main() {
    divan::main();
}

#[divan::bench]
fn create_mac_from_string() {
    let res = Mac::from_str(black_box(MAC_STRING));
    black_box(res.expect("Failed to create MAC"));
}

#[divan::bench]
fn hex_val() {
    black_box(wakeonlan::hex_val(black_box('0')).expect("Failed to create MAC"));
}

#[divan::bench]
fn u8_from_str_radix() {
    black_box(u8::from_str_radix(black_box("0"), 16).expect("Failed to create MAC"));
}