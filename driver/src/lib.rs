//! A small memory driver with region-based access control.

use std::{error::Error, fmt, ops::Range};

/// Permissions and address range for one contiguous memory region.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Region {
    range: Range<usize>,
    readable: bool,
    writable: bool,
}

impl Region {
    pub fn new(range: Range<usize>, readable: bool, writable: bool) -> Self {
        Self {
            range,
            readable,
            writable,
        }
    }

    pub fn read_only(range: Range<usize>) -> Self {
        Self::new(range, true, false)
    }

    pub fn write_only(range: Range<usize>) -> Self {
        Self::new(range, false, true)
    }

    pub fn read_write(range: Range<usize>) -> Self {
        Self::new(range, true, true)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DriverError {
    InvalidRegion(Range<usize>),
    OverlappingRegions,
    AddressOutOfBounds { address: usize },
    NotReadable { address: usize },
    NotWritable { address: usize },
}

impl fmt::Display for DriverError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRegion(range) => write!(f, "invalid memory region {range:?}"),
            Self::OverlappingRegions => write!(f, "memory regions overlap"),
            Self::AddressOutOfBounds { address } => {
                write!(f, "address {address} is outside the driver's memory")
            }
            Self::NotReadable { address } => write!(f, "address {address} is not readable"),
            Self::NotWritable { address } => write!(f, "address {address} is not writable"),
        }
    }
}

impl Error for DriverError {}

/// Memory whose accesses are checked against a fixed region map.
pub struct MemoryDriver {
    memory: Vec<u8>,
    regions: Vec<Region>,
}

impl MemoryDriver {
    pub fn new(size: usize, mut regions: Vec<Region>) -> Result<Self, DriverError> {
        if let Some(region) = regions
            .iter()
            .find(|region| region.range.start >= region.range.end || region.range.end > size)
        {
            return Err(DriverError::InvalidRegion(region.range.clone()));
        }

        regions.sort_by_key(|region| region.range.start);
        if regions
            .windows(2)
            .any(|pair| pair[0].range.end > pair[1].range.start)
        {
            return Err(DriverError::OverlappingRegions);
        }

        Ok(Self {
            memory: vec![0; size],
            regions,
        })
    }

    pub fn read(&self, address: usize) -> Result<u8, DriverError> {
        self.check_bounds(address)?;
        let region = self
            .region(address)
            .ok_or(DriverError::NotReadable { address })?;
        if !region.readable {
            return Err(DriverError::NotReadable { address });
        }
        Ok(self.memory[address])
    }

    pub fn write(&mut self, address: usize, value: u8) -> Result<(), DriverError> {
        self.check_bounds(address)?;
        let region = self
            .region(address)
            .ok_or(DriverError::NotWritable { address })?;
        if !region.writable {
            return Err(DriverError::NotWritable { address });
        }
        self.memory[address] = value;
        Ok(())
    }

    fn check_bounds(&self, address: usize) -> Result<(), DriverError> {
        if address >= self.memory.len() {
            Err(DriverError::AddressOutOfBounds { address })
        } else {
            Ok(())
        }
    }

    fn region(&self, address: usize) -> Option<&Region> {
        self.regions
            .iter()
            .find(|region| region.range.contains(&address))
    }

    /// Injects a value as if hardware will have written it before a later read.
    ///
    /// This deliberately bypasses region write permissions. Normal users do not
    /// compile this method; test consumers must opt in through a dev-dependency.
    #[cfg(any(test, feature = "test-support"))]
    pub fn test_write(&mut self, address: usize, value: u8) -> Result<(), DriverError> {
        self.check_bounds(address)?;
        self.memory[address] = value;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{DriverError, MemoryDriver, Region};

    fn driver() -> MemoryDriver {
        MemoryDriver::new(16, vec![Region::write_only(0..4), Region::read_only(8..12)]).unwrap()
    }

    #[test]
    fn enforces_region_permissions() {
        let mut driver = driver();
        assert_eq!(driver.read(0), Err(DriverError::NotReadable { address: 0 }));
        assert_eq!(
            driver.write(8, 1),
            Err(DriverError::NotWritable { address: 8 })
        );
        assert_eq!(driver.read(6), Err(DriverError::NotReadable { address: 6 }));
        assert_eq!(
            driver.write(16, 1),
            Err(DriverError::AddressOutOfBounds { address: 16 })
        );
    }

    #[test]
    fn test_write_mocks_a_future_hardware_write() {
        let mut driver = driver();
        driver.test_write(8, 0xa5).unwrap();
        assert_eq!(driver.read(8), Ok(0xa5));
    }
}
