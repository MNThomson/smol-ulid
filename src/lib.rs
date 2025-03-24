use rand::Rng;
use std::time::{Duration, SystemTime};

#[derive(PartialOrd, Ord, PartialEq, Eq, Hash, Clone, Copy)]
pub struct SUlid(u64);

impl SUlid {
    #[allow(dead_code)]
    const TIME_BITS: u8 = 43;
    const TIME_BITMASK: u64 = 0b1111111111111111111111111111111111111111111;

    const RAND_BITS: u8 = 21;
    const RAND_BITMASK: u64 = 0b111111111111111111111;

    /// Create a Ulid with the current time and random bits
    pub fn new() -> Self {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or(Duration::ZERO) // TODO: How to handle this?
            .as_millis();

        let val = ((timestamp as u64 & Self::TIME_BITMASK) << Self::RAND_BITS)
            | (rand::rng().random::<u64>() & Self::TIME_BITMASK);

        Self(val)
    }

    /// Get the SystemTime the Ulid was created at
    pub fn datetime(self) -> SystemTime {
        SystemTime::UNIX_EPOCH + Duration::from_millis(self.0 >> Self::RAND_BITS)
    }

    /// Get the random bits in the Ulid
    pub fn random(self) -> u64 {
        self.0 & Self::RAND_BITMASK
    }
}

impl Default for SUlid {
    fn default() -> Self {
        Self::new()
    }
}
