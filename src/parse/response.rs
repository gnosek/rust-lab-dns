use super::error::*;
use super::label::parse_label_list;
use super::resource_record::{parse_record, RawResourceRecord};
use super::traits::ResolveRefLabels;
use crate::types::{LabelSet, QClass, QType, Response};
use std::convert::TryInto;

#[derive(Debug)]
pub(crate) struct RawResponse<'a> {
    qname: LabelSet<'a>,
    ttl: u32,
    rdata: RawResourceRecord<'a>,
}

impl<'a> ResolveRefLabels<'a> for RawResponse<'a> {
    type Output = Self;

    fn resolve_ref_labels(mut self, orig_packet: &'a [u8]) -> Result<Self, ParseError> {
        self.qname = self.qname.resolve_ref_labels(orig_packet)?;
        self.rdata = self.rdata.resolve_ref_labels(orig_packet)?;
        Ok(self)
    }
}
impl<'a> TryInto<Response> for RawResponse<'a> {
    type Error = DnsParseError<'a>;

    fn try_into(self) -> Result<Response, Self::Error> {
        let labels: Vec<_> = self.qname.try_into()?;

        Ok(Response {
            qname: labels.join("."),
            ttl: self.ttl,
            rdata: self.rdata.try_into()?,
        })
    }
}

pub(crate) fn parse_response(input: &[u8]) -> nom::IResult<&[u8], RawResponse> {
    let (input, labels) = parse_label_list(input)?;
    let (input, qtype) = nom::number::streaming::be_u16(input)?;
    let (input, qclass) = nom::number::streaming::be_u16(input)?;
    let (input, ttl) = nom::number::streaming::be_u32(input)?;
    let (input, rdlen) = nom::number::streaming::be_u16(input)?;
    let (input, rdata) = nom::bytes::streaming::take(rdlen)(input)?;

    let qtype: QType = qtype.into();
    let qclass: QClass = qclass.into();

    let rdata = parse_record(rdata, qtype, qclass)?.1;

    Ok((
        input,
        RawResponse {
            qname: labels,
            ttl,
            rdata,
        },
    ))
}
