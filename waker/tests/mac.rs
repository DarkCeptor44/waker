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

use std::str::FromStr;
use waker::Mac;

const MAC_BYTES: [u8; 6] = [0x01, 0x23, 0x45, 0x67, 0x89, 0xAB];

#[test]
fn test_parse_mac_from_str() {
    let mac = Mac::from_str("01:23:45:67:89:AB").expect("Failed to parse MAC address");
    assert_eq!(mac.0, MAC_BYTES);
}

#[test]
fn test_mac_display_lower() {
    let mac = Mac(MAC_BYTES);
    assert_eq!(mac.to_string(), "01:23:45:67:89:ab");
}

#[test]
fn test_mac_display_upper() {
    let mac = Mac(MAC_BYTES);
    assert_eq!(format!("{mac:X}"), "01:23:45:67:89:AB");
}

#[test]
#[cfg(feature = "serde")]
fn test_mac_serde_serialize() {
    let mac = Mac(MAC_BYTES);
    let s = serde_json::to_string(&mac).expect("Failed to serialize MAC address");
    assert_eq!(s, format!("\"{mac}\""));
}

#[test]
#[cfg(feature = "serde")]
fn test_mac_serde_deserialize() {
    let s = "\"01:23:45:67:89:AB\"";
    let mac: Mac = serde_json::from_str(s).expect("Failed to deserialize MAC address");
    assert_eq!(mac, Mac(MAC_BYTES));
}
