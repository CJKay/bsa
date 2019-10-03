//! Parser backend.
//!
//! This module provides the private backend implementation for the archive parser logic.

mod ba2;
mod bsa;

use crate::{Archive, ArchiveType, Result};
use nom::{branch::alt, bytes::complete::tag};
use std::io::Read;

/// Magic number parser.
pub fn magic<'input, Input: 'input, Error: nom::error::ParseError<Input>>(
    archive_type: ArchiveType,
) -> impl Fn(Input) -> nom::IResult<Input, Input, Error>
where
    Input: nom::InputTake + nom::Compare<&'static [u8]>,
{
    match archive_type {
        ArchiveType::Bsa => tag(bsa::MAGIC),
        ArchiveType::Ba2 => tag(ba2::MAGIC),
    }
}

/// Parse a Bethesda archive file.
pub(crate) fn parse<Reader: Read>(mut reader: Reader) -> Result<Archive> {
    let mut buffer = Vec::new();

    // TODO: Use the streaming parser. This is waiting on a fix for the higher rank trait bounds
    //       issue found here: https://github.com/rust-lang/rust/issues/64437
    let _ = reader.read_to_end(&mut buffer)?;

    let (_, archive) = alt::<_, _, (), _>((bsa::root, ba2::root))(&buffer)?;

    Ok(archive)
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck_macros::quickcheck;

    fn parse_magic(archive_type: ArchiveType, input: &[u8]) {
        let result = magic::<_, ()>(archive_type)(input);

        match (archive_type, result) {
            (ArchiveType::Bsa, Ok((_, magic))) => {
                assert_eq!(input, bsa::MAGIC);
                assert_eq!(magic, bsa::MAGIC);
            }

            (ArchiveType::Ba2, Ok((_, magic))) => {
                assert_eq!(input, ba2::MAGIC);
                assert_eq!(magic, ba2::MAGIC);
            }

            (_, Err(_)) => {
                assert_ne!(input, bsa::MAGIC);
                assert_ne!(input, ba2::MAGIC);
            }
        }
    }

    #[test]
    fn test_parse_magic_bsa_valid() {
        parse_magic(ArchiveType::Bsa, bsa::MAGIC);
    }

    #[test]
    fn test_parse_magic_ba2_valid() {
        parse_magic(ArchiveType::Ba2, ba2::MAGIC);
    }

    #[quickcheck]
    fn quickcheck_parse_magic(archive_type: ArchiveType, input: Vec<u8>) {
        parse_magic(archive_type, &input);
    }
}
