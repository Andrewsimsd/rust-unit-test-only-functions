# Unit-test-only memory-map access in Rust

This project demonstrates how a unit test can mock a memory-mapped device
response without exposing that capability to application code.

`Device` contains a 16-byte simulated memory map. Its public API does not
accept addresses:

- `Device::write(value)` always writes to a private driver write address.
- `Device::read()` always reads from a separate private driver read address.

The addresses and raw memory access are contained in the private `driver`
module. Production callers therefore cannot choose an arbitrary address.

The driver also defines this unit-test-only function:

```rust
#[cfg(test)]
pub(super) fn mock_read_response(device: &mut Device, value: u8) {
    device.memory[READ_ADDRESS] = value;
}
```

It simulates hardware writing a response to the hidden read address. Because
of `#[cfg(test)]`, the function is only compiled when the library's unit-test
harness is built. The unit test calls it directly through the private driver:

```rust
driver::mock_read_response(&mut device, 0b1010_0101);
assert_eq!(device.read(), 0b1010_0101);
```

The test module can access `driver` because it is a child module of the crate
root. No separate test-support module is necessary for this small example.

## Running the example

Run the unit test and application with:

```sh
cargo test
cargo run
```

The application writes `42` to the hidden write address and reads the untouched
hidden read address, so `cargo run` prints:

```text
device response: 0
```

`src/main.rs` contains a commented-out call to `mock_read_response`.
Uncommenting it makes `cargo run` fail to compile: the driver is private, and
the mock function is absent from a non-test library build.

## Integration-test caveat

Tests in `tests/` are integration tests.
For those, Cargo builds this library as a normal dependency, so `cfg(test)` is
not enabled for the library. If integration tests must use the mock, use an
explicit, non-default Cargo feature instead:

```toml
[features]
test-support = []
```

Then replace `#[cfg(test)]` with
`#[cfg(any(test, feature = "test-support"))]`, expose the required API, and run
`cargo test --features test-support`.

A Cargo feature can also be enabled by downstream users, so it is not an
access-control or security boundary. For hardware access that must remain
strictly unavailable to consumers, keep tests inside the crate or place test
fakes in a separate development-only crate.
