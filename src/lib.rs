//! A small example of exposing privileged memory-map operations to unit tests.

/// A simplified memory-mapped device.
pub struct Device {
    memory: [u8; 16],
}

impl Device {
    pub fn new() -> Self {
        Self { memory: [0; 16] }
    }

    /// Reads a response from the driver's hidden read address.
    pub fn read(&self) -> u8 {
        driver::read(self)
    }

    /// Writes a command to the driver's hidden write address.
    pub fn write(&mut self, value: u8) {
        driver::write(self, value);
    }
}

impl Default for Device {
    fn default() -> Self {
        Self::new()
    }
}

/// The driver layer owns unrestricted memory access in production code.
mod driver {
    use super::Device;

    // Only the driver knows the memory-map layout in production code.
    const WRITE_ADDRESS: usize = 3;
    const READ_ADDRESS: usize = 11;

    pub(super) fn write(device: &mut Device, value: u8) {
        device.memory[WRITE_ADDRESS] = value;
    }

    pub(super) fn read(device: &Device) -> u8 {
        device.memory[READ_ADDRESS]
    }

    /// Simulates the hardware placing a response at its read address.
    #[cfg(test)]
    pub(super) fn mock_read_response(device: &mut Device, value: u8) {
        device.memory[READ_ADDRESS] = value;
    }
}

#[cfg(test)]
mod tests {
    use super::{Device, driver};

    #[test]
    fn reads_a_response_mocked_at_the_hidden_read_address() {
        let mut device = Device::new();

        // Pretend the hardware produced this response. Application code cannot
        // access the hidden address or compile this helper.
        driver::mock_read_response(&mut device, 0b1010_0101);

        assert_eq!(device.read(), 0b1010_0101);
    }
}
