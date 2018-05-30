#[cfg(not(feature = "std"))]
use core::hash::Hasher;
#[cfg(feature = "std")]
use std::hash::Hasher;

pub use util::make_table_crc64 as make_table;

include!(concat!(env!("OUT_DIR"), "/crc64_constants.rs"));

/// Structure that holds all of the important values for calculating a CRC
///
/// # Definitions
/// * **table:** Holds the table values based on the supplied polynomial for the fast CRC calculations
/// * **initial:** The initial input value. AKA *reflect_in*
/// * **value:** Holds the current value of the CRC
/// * **reflect:** Chooses whether or not the CRC math is normal or reflected
/// * **final_xor:** Final value to XOR with when calling Digest::sum64
pub struct Digest {
    table: [u64; 256],
    initial: u64,
    value: u64,
    reflect: bool,
    final_xor: u64,
}

pub trait Hasher64 {
    fn reset(&mut self);
    fn write(&mut self, bytes: &[u8]);
    fn sum64(&self) -> u64;
}

/// Caclulate the CRC of the byte string of values.
///
/// Updates the current CRC *value* using the CRC table *table* using the byte array *bytes*.
/// The parameter *rfl* will reflect the data.  *rfl=false* will calculate the CRC MSB first.
/// *rfl=true* will calculate the CRC LSB first.
///
/// # Usage
///
/// call using Digest::write(&bytes)
pub fn update(mut value: u64, table: &[u64; 256], bytes: &[u8], rfl: bool) -> u64 {
    if rfl {
        value = bytes.iter().fold(value, |acc, &x| {
            (acc >> 8) ^ (table[((acc ^ (u64::from(x))) & 0xFF) as usize])
        });
    } else {
        value = bytes.iter().fold(value, |acc, &x| {
            (acc << 8) ^ (table[((u64::from(x)) ^ (acc >> 56)) as usize])
        });
    }
    value
}

/// Generates a generic ECMA-188 64 bit CRC (AKA CRC-64-ECMA)
pub fn checksum_ecma(bytes: &[u8]) -> u64 {
    return update(0xFFFFFFFFFFFFFFFF, &ECMA_TABLE, bytes, true) ^ 0xFFFFFFFFFFFFFFFF;
}

/// Generates a generic ISO 3309 32 bit CRC (AKA CRC-64-ISO)
pub fn checksum_iso(bytes: &[u8]) -> u64 {
    return update(0xFFFFFFFFFFFFFFFF, &ISO_TABLE, bytes, true) ^ 0xFFFFFFFFFFFFFFFF;
}

impl Digest {
    /// Creates a new table from the supplied polynomial and reflect parameter
    ///
    /// # Example
    ///
    /// ```rust
    /// use crc::{crc64, Hasher64};
    /// let mut digest = crc64::Digest::new(crc64::ECMA);
    /// digest.write(b"123456789");
    /// assert_eq!(digest.sum64(), 0x995dc9bbdf1939fa);;
    /// ```
    pub fn new(poly: u64) -> Digest {
        Digest {
            table: make_table(poly, true),
            initial: 0xFFFFFFFFFFFFFFFF,
            value: 0xFFFFFFFFFFFFFFFF,
            reflect: true,
            final_xor: 0xFFFFFFFFFFFFFFFF,
        }
    }

    /// Creates a new table from the supplied polynomial, reflect parameter, and an initial value
    ///
    /// # Example
    ///
    /// ```rust
    /// use crc::{crc64, Hasher64};
    /// let mut digest = crc64::Digest::new_with_initial(crc64::ECMA, 0xFFFFFFFFFFFFFFFF);
    /// digest.write(b"123456789");
    /// assert_eq!(digest.sum64(), 0x995dc9bbdf1939fa);
    /// ```
    pub fn new_with_initial(poly: u64, initial: u64) -> Digest {
        Digest {
            table: make_table(poly, true),
            initial: initial,
            value: initial,
            reflect: true,
            final_xor: 0xFFFFFFFFFFFFFFFF,
        }
    }

    /// Creates a new table from the supplied polynomial, reflect parameter, initial value, and final XOR value
    ///
    /// This should be the dafault way to generate a custom CRC64.  See default values here: *http://crccalc.com/*
    /// The example will generate a standard CRC64 table.
    ///
    /// # Example
    ///
    /// ```rust
    /// use crc::{crc64, Hasher64};
    /// let mut digest = crc64::Digest::new_custom(crc64::ECMA, 0xFFFFFFFFFFFFFFFF, true, 0xFFFFFFFFFFFFFFFF);
    /// digest.write(b"123456789");
    /// assert_eq!(digest.sum64(), 0x995dc9bbdf1939fa);
    /// ```
    pub fn new_custom(poly: u64, initial: u64, reflect: bool, final_xor: u64) -> Digest {
        Digest {
            table: make_table(poly, reflect),
            initial: initial,
            value: initial,
            reflect: reflect,
            final_xor: final_xor,
        }
    }
}

impl Hasher64 for Digest {
    /// Resets the current CRC to the initial value
    fn reset(&mut self) {
        self.value = self.initial;
    }

    /// Takes in a byte array and updates the CRC from based on the Digest::reflect field
    fn write(&mut self, bytes: &[u8]) {
        self.value = update(self.value, &self.table, bytes, self.reflect);
    }

    /// Returns the current CRC after being XOR'd with the final XOR value
    fn sum64(&self) -> u64 {
        self.value ^ self.final_xor
    }
}

/// Implementation of std::hash::Hasher so that types which #[derive(Hash)] can hash with Digest.
impl Hasher for Digest {
    fn write(&mut self, bytes: &[u8]) {
        Hasher64::write(self, bytes);
    }

    fn finish(&self) -> u64 {
        self.sum64() as u64
    }
}
