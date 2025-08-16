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
