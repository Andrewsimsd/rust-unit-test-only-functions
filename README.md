# Region-restricted memory driver

This workspace contains two crates:

- `memory-driver` owns simulated memory and checks every read and write against
  non-overlapping regions with read-only, write-only, or read/write permission.
- `rust-unit-test-only-functions` is a consumer of that driver. It defines the
  device's memory map and propagates `DriverError` when access is forbidden.

Addresses outside memory return `AddressOutOfBounds`. Addresses inside memory
but outside a permitted region (or used with the wrong operation) return
`NotReadable` or `NotWritable`.

The driver exposes `test_write` only when its `test-support` feature (or its own
unit-test configuration) is active. The consumer enables that feature solely
on its dev-dependency, so its unit tests can simulate hardware populating a
read-only response region before a future read. An ordinary `cargo build` does
not compile the helper.

This is an API-visibility convention, not a security boundary: a downstream
crate could explicitly enable the public Cargo feature. Keeping it on a
dev-dependency prevents accidental use in this consumer's production builds.

Run everything with:

```sh
cargo test --workspace
cargo run
```
