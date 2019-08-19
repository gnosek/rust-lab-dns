use super::error::*;
use super::label::parse_label_list;
use super::traits::ResolveRefLabels;
use crate::types::{LabelSet, QClass, QType, Query};
use std::convert::TryInto;

#[derive(Debug)]
pub(crate) struct RawQuery<'a> {
    qname: LabelSet<'a>,
    qtype: QType,
    qclass: QClass,
}

impl<'a> ResolveRefLabels<'a> for RawQuery<'a> {
    type Output = Self;

    fn resolve_ref_labels(mut self, orig_packet: &'a [u8]) -> Result<Self, ParseError> {
        self.qname = self.qname.resolve_ref_labels(orig_packet)?;
        Ok(self)
    }
}

impl<'a> TryInto<Query> for RawQuery<'a> {
    type Error = DnsParseError<'a>;

    fn try_into(self) -> Result<Query, Self::Error> {
        let labels: Vec<_> = self.qname.try_into()?;

        Ok(Query {
            qname: labels.join("."),
            qtype: self.qtype,
            qclass: self.qclass,
        })
    }
}

pub(crate) fn parse_query(input: &[u8]) -> nom::IResult<&[u8], RawQuery> {
    let (input, labels) = parse_label_list(input)?;
    let (input, qtype) = nom::number::streaming::be_u16(input)?;
    let (input, qclass) = nom::number::streaming::be_u16(input)?;

    Ok((
        input,
        RawQuery {
            qname: labels,
            qtype: qtype.into(),
            qclass: qclass.into(),
        },
    ))
}
