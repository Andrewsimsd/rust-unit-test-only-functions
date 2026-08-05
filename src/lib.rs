//! Consumer-side device API. The memory driver lives in a separate crate.

use memory_driver::{DriverError, MemoryDriver, Region};

const COMMAND_ADDRESS: usize = 3;
const RESPONSE_ADDRESS: usize = 11;

pub struct Device {
    driver: MemoryDriver,
}

impl Device {
    pub fn new() -> Self {
        Self {
            driver: MemoryDriver::new(
                16,
                vec![
                    Region::write_only(0..4),
                    Region::read_write(4..8),
                    Region::read_only(8..12),
                ],
            )
            .expect("the static device memory map must be valid"),
        }
    }

    pub fn read_response(&self) -> Result<u8, DriverError> {
        self.driver.read(RESPONSE_ADDRESS)
    }

    pub fn write_command(&mut self, value: u8) -> Result<(), DriverError> {
        self.driver.write(COMMAND_ADDRESS, value)
    }

    pub fn driver(&self) -> &MemoryDriver {
        &self.driver
    }

    pub fn driver_mut(&mut self) -> &mut MemoryDriver {
        &mut self.driver
    }

    // uncomment this to see that .test_write is not accessible in this scope
    // although it is acesible below in the test mod
    //pub fn use_test_write(&mut self) {
    //    self.driver_mut().test_write(RESPONSE_ADDRESS, 0xa5).unwrap();
    //}
}

impl Default for Device {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::{Device, RESPONSE_ADDRESS};
    use memory_driver::DriverError;

    #[test]
    fn consumer_obeys_the_driver_memory_map() {
        let mut device = Device::new();
        device.write_command(42).unwrap();
        assert_eq!(device.read_response(), Ok(0));
        assert_eq!(
            device.driver_mut().write(11, 99),
            Err(DriverError::NotWritable { address: 11 })
        );
        assert_eq!(
            device.driver().read(3),
            Err(DriverError::NotReadable { address: 3 })
        );
    }

    #[test]
    fn consumer_test_can_mock_a_future_hardware_write() {
        let mut device = Device::new();

        // Available here because this crate enables test-support only on its
        // dev-dependency. It bypasses write permission to emulate hardware.
        device.driver_mut().test_write(RESPONSE_ADDRESS, 0xa5).unwrap();

        assert_eq!(device.read_response(), Ok(0xa5));
    }
}
