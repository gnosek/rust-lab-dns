use super::error::*;
use crate::types::{Header, Opcode, Rcode};

type RawHeaderFlags = (u8, u8, u8, u8, u8, u8, u8, u8);

pub(crate) fn parse_header_flags(input: &[u8]) -> nom::IResult<&[u8], Header> {
    let (input, seq) = nom::number::streaming::be_u16(input)?;
    let (input, flag_bytes) = nom::bytes::streaming::take(2usize)(input)?;
    let (_, raw_flags): (&[u8], RawHeaderFlags) =
        nom::bits::bits(nom::sequence::tuple::<_, _, BitsError<_>, _>((
            nom::bits::complete::take(1usize),
            nom::bits::complete::take(4usize),
            nom::bits::complete::take(1usize),
            nom::bits::complete::take(1usize),
            nom::bits::complete::take(1usize),
            nom::bits::complete::take(1usize),
            nom::bits::complete::take(3usize),
            nom::bits::complete::take(4usize),
        )))(flag_bytes)?;

    Ok((
        input,
        Header {
            seq,
            is_response: raw_flags.0 != 0,
            opcode: Opcode::from(raw_flags.1),
            authoritative: raw_flags.2 != 0,
            truncated: raw_flags.3 != 0,
            recursion_requested: raw_flags.4 != 0,
            recursion_available: raw_flags.5 != 0,
            rcode: Rcode::from(raw_flags.7),
        },
    ))
}
