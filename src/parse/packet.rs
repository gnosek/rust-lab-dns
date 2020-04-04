use super::error::*;
use super::header::parse_header_flags;
use super::query::{parse_query, RawQuery};
use super::response::{parse_response, RawResponse};
use super::traits::ResolveRefLabels;
use crate::types::{DnsPacket, Header};
use std::convert::TryInto;

#[derive(Debug)]
struct RawDnsPacket<'a> {
    header: Header,

    queries: Vec<RawQuery<'a>>,
    answers: Vec<RawResponse<'a>>,
    authority: Vec<RawResponse<'a>>,
    additional: Vec<RawResponse<'a>>,
}

impl<'a> ResolveRefLabels<'a> for RawDnsPacket<'a> {
    type Output = Self;

    fn resolve_ref_labels(mut self, orig_packet: &'a [u8]) -> Result<Self::Output, DnsParseError> {
        let queries: Result<Vec<_>, _> = self
            .queries
            .into_iter()
            .map(|q| q.resolve_ref_labels(orig_packet))
            .collect();
        self.queries = queries?;

        let answers: Result<Vec<_>, _> = self
            .answers
            .into_iter()
            .map(|q| q.resolve_ref_labels(orig_packet))
            .collect();
        self.answers = answers?;

        let authority: Result<Vec<_>, _> = self
            .authority
            .into_iter()
            .map(|q| q.resolve_ref_labels(orig_packet))
            .collect();
        self.authority = authority?;

        let additional: Result<Vec<_>, _> = self
            .additional
            .into_iter()
            .map(|q| q.resolve_ref_labels(orig_packet))
            .collect();
        self.additional = additional?;

        Ok(self)
    }
}

impl<'a> TryInto<DnsPacket> for RawDnsPacket<'a> {
    type Error = DnsParseError<'a>;

    fn try_into(self) -> Result<DnsPacket, Self::Error> {
        let queries: Result<Vec<_>, _> = self.queries.into_iter().map(|q| q.try_into()).collect();
        let answers: Result<Vec<_>, _> = self.answers.into_iter().map(|q| q.try_into()).collect();
        let authority: Result<Vec<_>, _> =
            self.authority.into_iter().map(|q| q.try_into()).collect();
        let additional: Result<Vec<_>, _> =
            self.additional.into_iter().map(|q| q.try_into()).collect();

        Ok(DnsPacket {
            header: self.header,
            queries: queries?,
            answers: answers?,
            authority: authority?,
            additional: additional?,
        })
    }
}

fn parse_raw_dns_packet(input: &[u8]) -> Result<(&[u8], RawDnsPacket), DnsParseError> {
    let orig_packet = input;

    let (input, header) = parse_header_flags(input)?;
    let (input, qd_count) = nom::number::streaming::be_u16(input)?;
    let (input, an_count) = nom::number::streaming::be_u16(input)?;
    let (input, ns_count) = nom::number::streaming::be_u16(input)?;
    let (input, ar_count) = nom::number::streaming::be_u16(input)?;

    let (input, queries) = nom::multi::count(parse_query, qd_count as usize)(input)?;
    let (input, answers) = nom::multi::count(parse_response, an_count as usize)(input)?;
    let (input, authority) = nom::multi::count(parse_response, ns_count as usize)(input)?;
    let (input, additional) = nom::multi::count(parse_response, ar_count as usize)(input)?;

    let packet = RawDnsPacket {
        header,
        queries,
        answers,
        authority,
        additional,
    };

    Ok((input, packet.resolve_ref_labels(orig_packet)?))
}

pub fn parse_dns_packet(input: &[u8]) -> Result<(&[u8], DnsPacket), DnsParseError> {
    let (input, packet) = parse_raw_dns_packet(input)?;
    Ok((input, packet.try_into()?))
}

pub fn parse_dns_tcp_packet(input: &[u8]) -> Result<(&[u8], DnsPacket), DnsParseError> {
    let (packet, length) = nom::number::streaming::be_i16(input)?;
    let packet = &packet.get(..length as usize).ok_or(DnsParseError::InvalidLabel)?;

    parse_dns_packet(packet)
}
