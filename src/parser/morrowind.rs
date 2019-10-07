//! Parsers for archive files introduced by **TES III: Morrowind**.

use crate::{archive::MAGIC_MORROWIND, Archive};
use nom::{bytes::complete::tag, combinator::map};

/// Root parser.
pub(super) fn root<'input, Input, Error>(input: Input) -> nom::IResult<Input, Archive, Error>
where
    Input: 'input + Clone + nom::InputTake + nom::Compare<&'static [u8; 4]>,
    Error: nom::error::ParseError<Input>,
{
    map(magic, |_| Archive::Morrowind)(input)
}

/// Magic number parser.
fn magic<'input, Input, Error>(input: Input) -> nom::IResult<Input, Input, Error>
where
    Input: 'input + Clone + nom::InputTake + nom::Compare<&'static [u8; 4]>,
    Error: nom::error::ParseError<Input>,
{
    tag(&MAGIC_MORROWIND)(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck_macros::quickcheck;

    fn parse_magic(input: &[u8]) {
        let result = magic::<_, ()>(input);

        match result {
            Ok((_, magic)) => {
                assert_eq!(input, MAGIC_MORROWIND);
                assert_eq!(magic, MAGIC_MORROWIND);
            }

            Err(_) => {
                assert_ne!(input, MAGIC_MORROWIND);
            }
        }
    }

    #[test]
    fn test_parse_magic_valid() {
        parse_magic(&MAGIC_MORROWIND);
    }

    #[quickcheck]
    fn quickcheck_parse_magic(input: Vec<u8>) {
        parse_magic(&input);
    }
}
