//! Parsers for archive files introduced by **Fallout 4**.

use crate::{
    archive::{MAGIC_FALLOUT4, VERSION_FALLOUT4},
    Archive, ArchiveVersion,
};

use nom::{
    bytes::complete::tag, combinator::map_opt, number::complete::le_u32, sequence::preceded,
};

/// Root parser.
pub(super) fn root<'input, Error>(input: &'input [u8]) -> nom::IResult<&'input [u8], Archive, Error>
where
    Error: nom::error::ParseError<&'input [u8]>,
{
    preceded(magic, header)(input)
}

/// Magic number parser.
fn magic<'input, Input, Error>(input: Input) -> nom::IResult<Input, Input, Error>
where
    Input: 'input + Clone + nom::InputTake + nom::Compare<&'static [u8; 4]>,
    Error: nom::error::ParseError<Input>,
{
    tag(&MAGIC_FALLOUT4)(input)
}

/// Header parser.
fn header<'input, Error>(input: &'input [u8]) -> nom::IResult<&'input [u8], Archive, Error>
where
    Error: nom::error::ParseError<&'input [u8]>,
{
    map_opt(version, |version| match version {
        ArchiveVersion::Fallout4 => Some(Archive::Fallout4),

        _ => None,
    })(input)
}

/// Version number field parser.
fn version<'input, Error>(input: &'input [u8]) -> nom::IResult<&'input [u8], ArchiveVersion, Error>
where
    Error: nom::error::ParseError<&'input [u8]>,
{
    map_opt(le_u32, |value| match value {
        VERSION_FALLOUT4 => Some(ArchiveVersion::Fallout4),

        _ => None,
    })(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use byteorder::{ByteOrder, LittleEndian};
    use quickcheck_macros::quickcheck;

    fn parse_magic(input: &[u8]) {
        let result = magic::<_, ()>(input);

        match result {
            Ok((_, magic)) => assert_eq!(magic, MAGIC_FALLOUT4),
            Err(_) => assert_ne!(input, MAGIC_FALLOUT4),
        }
    }

    #[test]
    fn test_parse_magic_valid() {
        parse_magic(&MAGIC_FALLOUT4);
    }

    #[allow(clippy::needless_pass_by_value)]
    #[quickcheck]
    fn quickcheck_parse_magic(input: Vec<u8>) {
        parse_magic(&input);
    }

    fn parse_version(input: u32) {
        let mut buffer = [0; 4];

        LittleEndian::write_u32(&mut buffer, input);

        let result = version::<()>(&buffer);

        if let Ok((_, ArchiveVersion::Fallout4)) = result {
            assert_eq!(input, VERSION_FALLOUT4);
        } else {
            assert_ne!(input, VERSION_FALLOUT4);
        }
    }

    #[test]
    fn test_parse_version_fo4_valid() {
        parse_version(VERSION_FALLOUT4);
    }

    #[quickcheck]
    fn quickcheck_parse_version(input: u32) {
        parse_version(input);
    }
}
