//! Archive interface.
//!
//! This module exposes the [`Archive`] structure, which defines the crate's primary interface for
//! managing Bethesda archive data.
//!
//! [`Archive`]: ./struct.Archive.html

use std::io::Read;

/// Archive type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Type {
    /// `.bsa` archive.
    Bsa,

    /// `.ba2` archive.
    Ba2,
}

#[cfg(test)]
impl quickcheck::Arbitrary for Type {
    fn arbitrary<Gen: quickcheck::Gen>(gen: &mut Gen) -> Self {
        use rand::Rng;

        let values = &[Type::Bsa, Type::Ba2];

        values[gen.gen_range(0, values.len())]
    }
}

/// A Bethesda Softworks archive.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Archive;

impl Archive {
    /// Read an archive from an input stream.
    pub fn new<Reader: Read>(reader: Reader) -> crate::Result<Self> {
        crate::parser::parse(reader)
    }
}
