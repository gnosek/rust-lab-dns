use super::error::*;
use super::traits::ResolveRefLabels;
use crate::types::{Label, LabelSet};
use std::convert::TryInto;

impl<'a> ResolveRefLabels<'a> for Label<'a> {
    type Output = Vec<Label<'a>>;

    fn resolve_ref_labels(self, orig_packet: &'a [u8]) -> Result<Self::Output, ParseError> {
        match self {
            Label::Reference(offset) => {
                let target = &orig_packet[offset as usize..];
                let (_, target_label) = parse_label_list(target)?;
                Ok(target_label.resolve_ref_labels(orig_packet)?.0)
            }
            other => Ok(vec![other]),
        }
    }
}

impl<'a> TryInto<String> for Label<'a> {
    type Error = DnsParseError<'a>;

    fn try_into(self) -> Result<String, Self::Error> {
        match self {
            Label::Data(bytes) => Ok(std::str::from_utf8(bytes)?.to_string()),
            _ => Err(DnsParseError::InvalidLabel),
        }
    }
}

impl<'a> ResolveRefLabels<'a> for LabelSet<'a> {
    type Output = Self;

    fn resolve_ref_labels(self, orig_packet: &'a [u8]) -> Result<Self, ParseError> {
        let resolved_labels = self
            .0
            .into_iter()
            .map(|label| label.resolve_ref_labels(orig_packet));
        let resolved_labels: Result<Vec<_>, _> = resolved_labels.collect();

        Ok(LabelSet(resolved_labels?.into_iter().flatten().collect()))
    }
}

impl<'a> TryInto<Vec<String>> for LabelSet<'a> {
    type Error = DnsParseError<'a>;

    fn try_into(self) -> Result<Vec<String>, Self::Error> {
        self.0.into_iter().map(TryInto::try_into).collect()
    }
}

fn parse_direct_label(input: &[u8]) -> nom::IResult<&[u8], Label> {
    let (input, (_, len)): (&[u8], (u8, u8)) =
        nom::bits::bits(nom::sequence::pair::<_, _, _, BitsError, _, _>(
            nom::bits::streaming::tag(0, 2usize),
            nom::bits::streaming::take(6usize),
        ))(input)?;
    let (input, label) = nom::bytes::streaming::take(len)(input)?;
    Ok((input, Label::Data(label)))
}

fn parse_empty_label(input: &[u8]) -> nom::IResult<&[u8], Label> {
    let (input, _) = nom::bytes::streaming::tag([0u8])(input)?;
    Ok((input, Label::End))
}

fn parse_label_ref(input: &[u8]) -> nom::IResult<&[u8], Label> {
    let (input, (_, offset)): (&[u8], (u8, u16)) =
        nom::bits::bits(nom::sequence::pair::<_, _, _, BitsError, _, _>(
            nom::bits::streaming::tag(0x3, 2usize),
            nom::bits::streaming::take(14usize),
        ))(input)?;

    Ok((input, Label::Reference(offset)))
}

fn parse_label_end(input: &[u8]) -> nom::IResult<&[u8], Label> {
    nom::branch::alt((parse_empty_label, parse_label_ref))(input)
}

pub(crate) fn parse_label_list(input: &[u8]) -> nom::IResult<&[u8], LabelSet> {
    let (input, (mut labels, last)) = nom::multi::many_till(parse_direct_label, parse_label_end)(input)?;
    match last {
        Label::End => (),
        label => labels.push(label),
    }
    Ok((input, LabelSet(labels)))
}
