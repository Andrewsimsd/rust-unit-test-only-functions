# Unit-test-only Rust helpers

Run the example with:

```sh
cargo test
cargo run
```

The raw memory mock in `src/lib.rs` uses `#[cfg(test)]`, so Rust only compiles
it while building this crate's **unit-test harness**. The unit tests can access
the private `driver` module because they are a child module of the crate root.

There is one important distinction: tests in `tests/` are integration tests.
For those, Cargo builds this library as a normal dependency, so `cfg(test)` is
not enabled for the library. If integration tests must use the helpers, use an
explicit, non-default Cargo feature instead:

```toml
[features]
test-support = []
```

Then replace `#[cfg(test)]` with
`#[cfg(any(test, feature = "test-support"))]`, make the required helpers
public, and run `cargo test --features test-support`. A feature can be enabled
by downstream users, so it is not a security boundary; for genuinely unsafe
hardware access, consider putting integration-test fakes in a separate crate.
