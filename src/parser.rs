//! Parser backend.
//!
//! This module provides the private backend implementation for the archive parser logic.

use crate::{Archive, Result};

use nom::bytes::complete::tag;
use std::io::Read;

/// Magic number used by `.bsa` archives.
const BSA_MAGIC: &[u8; 4] = b"BSA\0";

/// Parse the magic number field of a `.bsa` file.
///
/// The magic number field is what identifies a file as a Bethesda archive. In valid `.bsa`
/// archives, the magic number is always `BSA\0`.
fn magic<'input, Error>(input: &'input [u8]) -> nom::IResult<&'input [u8], &'input [u8], Error>
where
    Error: nom::error::ParseError<&'input [u8]>,
{
    tag(BSA_MAGIC)(input)
}

/// Parse a Bethesda archive file.
pub(crate) fn parse<Reader: Read>(mut reader: Reader) -> Result<Archive> {
    let mut buffer = Vec::new();

    // TODO: Use the streaming parser. This is waiting on a fix for the higher rank trait bounds
    //       issue found here: https://github.com/rust-lang/rust/issues/64437
    let _ = reader.read_to_end(&mut buffer)?;

    let (_, _) = magic::<nom::error::VerboseError<_>>(&buffer)?;

    Ok(Archive)
}

#[cfg(test)]
mod test {
    use super::*;

    use quickcheck_macros::quickcheck;

    fn parse_magic(input: &[u8]) {
        let result = magic::<()>(&input);

        match result {
            Ok((_, magic)) => {
                assert_eq!(input, BSA_MAGIC);
                assert_eq!(magic, BSA_MAGIC);
            }

            Err(_) => {
                assert_ne!(input, BSA_MAGIC);
            }
        }
    }

    #[test]
    fn test_parse_magic_valid() {
        parse_magic(BSA_MAGIC);
    }

    #[quickcheck]
    fn quickcheck_parse_magic(input: Vec<u8>) {
        parse_magic(&input);
    }
}
