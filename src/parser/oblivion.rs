//! Parsers for archive files introduced by **TES IV: Oblivion**.

use crate::{
    archive::{MAGIC_OBLIVION, VERSION_FALLOUT3, VERSION_OBLIVION, VERSION_SKYRIMSE},
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
    tag(&MAGIC_OBLIVION)(input)
}

/// Header parser.
fn header<'input, Error>(input: &'input [u8]) -> nom::IResult<&'input [u8], Archive, Error>
where
    Error: nom::error::ParseError<&'input [u8]>,
{
    map_opt(version, |version| match version {
        ArchiveVersion::Oblivion => Some(Archive::Oblivion),
        ArchiveVersion::Fallout3 => Some(Archive::Fallout3),
        ArchiveVersion::SkyrimSe => Some(Archive::SkyrimSe),

        _ => None,
    })(input)
}

/// Version number field parser.
fn version<'input, Error>(input: &'input [u8]) -> nom::IResult<&'input [u8], ArchiveVersion, Error>
where
    Error: nom::error::ParseError<&'input [u8]>,
{
    map_opt(le_u32, |value| match value {
        VERSION_OBLIVION => Some(ArchiveVersion::Oblivion),
        VERSION_FALLOUT3 => Some(ArchiveVersion::Fallout3),
        VERSION_SKYRIMSE => Some(ArchiveVersion::SkyrimSe),

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
            Ok((_, magic)) => assert_eq!(magic, MAGIC_OBLIVION),
            Err(_) => assert_ne!(input, MAGIC_OBLIVION),
        }
    }

    #[test]
    fn test_parse_magic_valid() {
        parse_magic(&MAGIC_OBLIVION);
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

        match result {
            Ok((_, ArchiveVersion::Oblivion)) => {
                assert_eq!(input, VERSION_OBLIVION);
            }

            Ok((_, ArchiveVersion::Fallout3)) => {
                assert_eq!(input, VERSION_FALLOUT3);
            }

            Ok((_, ArchiveVersion::SkyrimSe)) => {
                assert_eq!(input, VERSION_SKYRIMSE);
            }

            _ => {
                assert_ne!(input, VERSION_OBLIVION);
                assert_ne!(input, VERSION_FALLOUT3);
                assert_ne!(input, VERSION_SKYRIMSE);
            }
        }
    }

    #[test]
    fn test_parse_version_ob_valid() {
        parse_version(VERSION_OBLIVION);
    }

    #[test]
    fn test_parse_version_fo3_valid() {
        parse_version(VERSION_FALLOUT3);
    }

    #[test]
    fn test_parse_version_sse_valid() {
        parse_version(VERSION_SKYRIMSE);
    }

    #[quickcheck]
    fn quickcheck_parse_version(input: u32) {
        parse_version(input);
    }
}
