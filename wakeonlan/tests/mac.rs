use std::str::FromStr;
use wakeonlan::Mac;

#[test]
fn test_parse_mac_from_str() {
    let mac = Mac::from_str("01:23:45:67:89:AB").expect("Failed to parse MAC address");
    assert_eq!(mac.0, [0x01, 0x23, 0x45, 0x67, 0x89, 0xAB]);
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
