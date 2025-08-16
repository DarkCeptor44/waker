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
