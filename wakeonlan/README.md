# wakeonlan

A Rust library for creating and sending Wake-on-LAN (WoL) magic packets over the network.

## Installation

You can add this library as a dependency with the following command:

```bash
cargo add wakeonlan
```

Or you can add this to your `Cargo.toml` file:

```toml
[dependencies]
wakeonlan = "^0.1"
```

## MSRV

The Minimum Supported Rust Version (MSRV) for `wakeonlan` is **1.78**.

## Usage

To wake a machine you will need the MAC address (it can also be called physical or hardware address) for its network interface, then you just need to create a magic packet and send it to the broadcast address, by default it's usually `255.255.255.255:9` so you can just use `send_magic_packet`, if you want to send it to a specific broadcast address you can use `send_magic_packet_to_broadcast_address`.

The easiest way to create a magic packet is to use `create_magic_packet`:

```rust
use wakeonlan::create_magic_packet;

let packet = create_magic_packet("01:23:45:67:89:AB").unwrap();
```

The MAC address can be passed as either `&str`, `String`, a byte array of length 6 (`[u8; 6]`) or a byte slice (`&[u8]`). Currently the string MAC address must have its bytes separated but `:`, `.` or `-` are all supported as separators.

The magic packet can then be sent using `send_magic_packet`:

```rust,no_run
use wakeonlan::{create_magic_packet, send_magic_packet};

let packet = create_magic_packet("01:23:45:67:89:AB").unwrap();

send_magic_packet(&packet).unwrap();
```

To send the packet to a specific broadcast address you can use `send_magic_packet_to_broadcast_address` (note that the address must be in the format `IP:PORT`):

```rust,no_run
use wakeonlan::{create_magic_packet, send_magic_packet_to_broadcast_address};

let packet = create_magic_packet("01:23:45:67:89:AB").unwrap();

send_magic_packet_to_broadcast_address(&packet, "192.168.0.255:9").unwrap();
```

## Benchmarks

### MAC-related

```text
Timer precision: 100 ns
mac                        fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ create_mac_from_string  14.01 ns      │ 16.45 ns      │ 14.11 ns      │ 14.19 ns      │ 100     │ 102400
├─ hex_val                 1.969 ns      │ 2.701 ns      │ 1.981 ns      │ 1.998 ns      │ 100     │ 819200
╰─ u8_from_str_radix       1.542 ns      │ 1.81 ns       │ 1.554 ns      │ 1.552 ns      │ 100     │ 819200
```

### Packet Creation

```text
Timer precision: 100 ns
packet_creation                     fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ create_magic_packet_from_bytes   123.1 ns      │ 137.2 ns      │ 123.9 ns      │ 124.3 ns      │ 100     │ 12800
╰─ create_magic_packet_from_string  131 ns        │ 152.1 ns      │ 132.5 ns      │ 132.6 ns      │ 100     │ 12800
```

### Packet Send

```text
Timer precision: 100 ns
packet_send           fastest       │ slowest       │ median        │ mean          │ samples │ iters
╰─ send_magic_packet  77.59 µs      │ 232.6 µs      │ 81.64 µs      │ 83.85 µs      │ 100     │ 100
```

## License

This project is licensed under the [GNU Lesser General Public License v3.0](https://www.gnu.org/licenses/lgpl-3.0.en.html).
