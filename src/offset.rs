use std::fmt::Debug;

pub const OFFSET_SIZE: usize = 8;

/// The data struct representing the offset in data file.
#[derive(Eq, PartialEq, Clone, Copy)]
pub struct Offset(u64);

impl Offset {
    pub fn new(n: u64) -> Self {
        Self(n)
    }

    pub fn to_bytes(&self) -> [u8; OFFSET_SIZE] {
        let mut bytes = [0u8; 8];
        for i in 0..8 {
            bytes[i] = (self.0 >> (i * 8)) as u8;
        }
        bytes
    }

    pub fn from_bytes(bytes: [u8; OFFSET_SIZE]) -> Self {
        let mut n = 0;
        for i in 0..OFFSET_SIZE {
            n |= (bytes[i] as u64) << (i * 8);
        }
        Self(n)
    }

    pub fn to_u64(&self) -> u64 {
        self.0
    }
}

impl Debug for Offset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Offset(0x{:02x} = {})", self.0, self.0)?;
        Ok(())
    }
}
