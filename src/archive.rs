//! Archive interface.
//!
//! This module exposes the [`Archive`] structure, which defines the crate's primary interface for
//! managing Bethesda archive data.
//!
//! [`Archive`]: ./struct.Archive.html

use std::io::Read;

/// A Bethesda Softworks archive.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Archive;

impl Archive {
    /// Read an archive from an input stream.
    pub fn new<Reader: Read>(reader: Reader) -> crate::Result<Self> {
        crate::parser::parse(reader)
    }
}
