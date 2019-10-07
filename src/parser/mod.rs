//! Parser backend.

use crate::{Archive, Result};
use nom::branch::alt;
use std::io::Read;

mod fallout4;
mod morrowind;
mod oblivion;

/// Root parser.
fn root<'input, Error>(input: &'input [u8]) -> nom::IResult<&'input [u8], Archive, Error>
where
    Error: nom::error::ParseError<&'input [u8]>,
{
    alt((morrowind::root, oblivion::root, fallout4::root))(input)
}

/// Parse a Bethesda archive file.
pub(crate) fn parse<Reader: Read>(mut reader: Reader) -> Result<Archive> {
    let mut buffer = Vec::new();

    // TODO: Use the streaming parser. This is waiting on a fix for the higher rank trait bounds
    //       issue found here: https://github.com/rust-lang/rust/issues/64437
    let _ = reader.read_to_end(&mut buffer)?;

    let (_, archive) = root::<()>(&buffer[..])?;

    Ok(archive)
}
