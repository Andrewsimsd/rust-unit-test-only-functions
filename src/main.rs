use rust_unit_test_only_functions::Device;

fn main() {
    let mut device = Device::new();
    device
        .write_command(42)
        .expect("command address is writable");
    println!(
        "device response: {}",
        device
            .read_response()
            .expect("response address is readable")
    );
}
