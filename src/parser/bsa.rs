//! `.bsa` parser functions.
//!
//! This module provides the parsing logic specific to `.bsa` archives.

use super::magic;
use crate::{Archive, ArchiveType};
use nom::combinator::map;

pub const MAGIC: &[u8] = b"BSA\0";

/// Root parser.
pub fn root<'input, Error>(input: &'input [u8]) -> nom::IResult<&'input [u8], Archive, Error>
where
    Error: nom::error::ParseError<&'input [u8]>,
{
    map(magic(ArchiveType::Bsa), |_| Archive)(input)
}
