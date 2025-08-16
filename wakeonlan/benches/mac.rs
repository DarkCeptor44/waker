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
