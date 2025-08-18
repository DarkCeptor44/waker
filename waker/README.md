# waker

A Rust library for creating and sending Wake-on-LAN (WoL) magic packets over the network.

## Installation

You can add this library as a dependency with the following command:

```bash
cargo add waker
```

Or you can add this to your `Cargo.toml` file:

```toml
[dependencies]
waker = "^0.1"
```

## Features

- `serde`: Enables serialization and deserialization of the `Mac` and `MagicPacket` types.

## MSRV

The Minimum Supported Rust Version (MSRV) for `waker` is **1.78**.

## Usage

To wake a machine you will need the MAC address (it can also be called physical or hardware address) for its network interface, then you just need to create a magic packet and send it to the broadcast address, by default it's usually `255.255.255.255:9` so you can just use `wake_device`, if you want to send it to a specific broadcast address you can pass a `WakeOptions` struct.

The easiest way to create a magic packet is to use `create_magic_packet`:

```rust
use waker::create_magic_packet;

let packet = create_magic_packet("01:23:45:67:89:AB").unwrap();
```

The MAC address can be passed as either `&str`, `String`, a byte array of length 6 (`[u8; 6]`) or a byte slice (`&[u8]`). Currently the string MAC address must have its bytes separated but `:`, `.` or `-` are all supported as separators.

The magic packet can then be sent using `wake_device`:

```rust
use waker::{create_magic_packet, wake_device};

let packet = create_magic_packet("01:23:45:67:89:AB").unwrap();
wake_device(&packet).unwrap();
```

To send the packet to a specific broadcast address you have to pass a `WakeOptions` struct with the address (note that it must be in the format `IP:PORT`):

```rust
use waker::{create_magic_packet, wake_device, WakeOptions};

let packet = create_magic_packet("01:23:45:67:89:AB").unwrap();
wake_device(WakeOptions::new(&packet).broadcast_address("192.168.0.255:9")).unwrap();
```

You can also specify the address to bind the UDP socket to (default is `0.0.0.0:0`):

```rust
use waker::{create_magic_packet, wake_device, WakeOptions};

let packet = create_magic_packet("01:23:45:67:89:AB").unwrap();
wake_device(WakeOptions::new(&packet).bind_address("127.0.0.1:0")).unwrap();
```

## Benchmarks

### MAC-related

```text
Timer precision: 100 ns
mac                        fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ create_mac_from_string  15.18 ns      │ 21.43 ns      │ 15.28 ns      │ 15.6 ns       │ 100     │ 102400
├─ hex_val                 1.981 ns      │ 8.5 ns        │ 1.993 ns      │ 2.144 ns      │ 100     │ 819200
╰─ u8_from_str_radix       1.542 ns      │ 1.554 ns      │ 1.554 ns      │ 1.549 ns      │ 100     │ 819200
```

### Packet Creation

```text
Timer precision: 100 ns
packet_creation                     fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ create_magic_packet_from_bytes   114.6 ns      │ 138 ns        │ 118.5 ns      │ 118.8 ns      │ 100     │ 12800
╰─ create_magic_packet_from_string  132.5 ns      │ 186.4 ns      │ 133.3 ns      │ 135.7 ns      │ 100     │ 12800
```

### Packet Send

```text
Timer precision: 100 ns
packet_send     fastest       │ slowest       │ median        │ mean          │ samples │ iters
╰─ wake_device  78.49 µs      │ 268 µs        │ 81.09 µs      │ 84.5 µs       │ 100     │ 100
```

## License

This project is licensed under the [GNU Lesser General Public License v3.0](https://www.gnu.org/licenses/lgpl-3.0.en.html).
