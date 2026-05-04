// This Source Code Form is subject to the terms of the
// Mozilla Public License, v. 2.0. If a copy of the MPL was not distributed
// with this file, You can obtain one at http://mozilla.org/MPL/2.0/.

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
