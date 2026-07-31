use rust_unit_test_only_functions::Device;

fn main() {
    let mut device = Device::new();
    device.write(42);
    println!("device response: {}", device.read());

    // Uncommenting this call makes `cargo run` fail to compile: the library is
    // built without `cfg(test)`, so `mock_read_response` does not exist (and
    // the private driver module is not application API).
    // rust_unit_test_only_functions::driver::mock_read_response(&mut device, 99);
}
