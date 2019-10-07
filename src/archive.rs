//! Archive interface.
//!
//! This module exposes the [`Archive`] structure, which defines the crate's primary interface for
//! managing Bethesda archive data.
//!
//! [`Archive`]: ./struct.Archive.html

use std::io::Read;

/// Magic number used by archives since **TES III: Morrowind**.
pub(crate) const MAGIC_MORROWIND: [u8; 4] = [0x00, 0x01, 0x00, 0x00];

/// Magic number used by archives since **TES IV: Oblivion**.
pub(crate) const MAGIC_OBLIVION: [u8; 4] = [0x42, 0x53, 0x41, 0x00];

/// Magic number used by archives since **Fallout 4**.
pub(crate) const MAGIC_FALLOUT4: [u8; 4] = [0x42, 0x54, 0x44, 0x58];

/// Version number used by archives since **TES IV: Oblivion**.
pub(crate) const VERSION_OBLIVION: u32 = 0x67;

/// Version number used by archives since **Fallout 3**.
pub(crate) const VERSION_FALLOUT3: u32 = 0x68;

/// Version number used by archives since **TES V: Skyrim Special Edition**.
pub(crate) const VERSION_SKYRIMSE: u32 = 0x69;

/// Version number used by archives since **Fallout 4**.
pub(crate) const VERSION_FALLOUT4: u32 = 0x01;

/// Archive file format versions.
///
/// These version names don't strictly represent the games that they are compatible with, but rather
/// the game that a particular update to the archive file format was introduced in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Version {
    /// **BSA** archive used by **TES III: Morrowind**.
    Morrowind,

    /// **BSA** archive used by **TES IV: Oblivion**.
    Oblivion,

    /// **BSA** archive used by **Fallout 3**, **Fallout: New Vegas** and **TES V: Skyrim**.
    Fallout3,

    /// **BSA** archive used by **TES V: Skyrim SE** and **TES V: Skyrim VR**.
    SkyrimSe,

    /// **BA2** archive used by **Fallout 4**.
    Fallout4,
}

#[cfg(test)]
impl quickcheck::Arbitrary for Version {
    fn arbitrary<Gen: quickcheck::Gen>(gen: &mut Gen) -> Self {
        use rand::Rng;

        let values = &[
            Version::Morrowind,
            Version::Oblivion,
            Version::Fallout3,
            Version::SkyrimSe,
            Version::Fallout4,
        ];

        values[gen.gen_range(0, values.len())]
    }
}

/// A Bethesda Softworks archive.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Archive {
    /// **BSA** archive used by **TES III: Morrowind**.
    Morrowind,

    /// **BSA** archive used by **TES IV: Oblivion**.
    Oblivion,

    /// **BSA** archive used by **Fallout 3**, **Fallout: New Vegas** and **TES V: Skyrim**.
    Fallout3,

    /// **BSA** archive used by **TES V: Skyrim SE** and **TES V: Skyrim VR**.
    SkyrimSe,

    /// **BA2** archive used by **Fallout 4**.
    Fallout4,
}

impl Archive {
    /// Read an archive from an input stream.
    pub fn new<Reader: Read>(reader: Reader) -> crate::Result<Self> {
        crate::parser::parse(reader)
    }
}
