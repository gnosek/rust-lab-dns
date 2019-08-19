use super::error::*;
use super::label::parse_label_list;
use super::traits::ResolveRefLabels;
use crate::types::{LabelSet, QClass, QType, ResourceRecord};
use std::convert::TryInto;
use std::net::{Ipv4Addr, Ipv6Addr};

#[derive(Debug)]
pub(crate) enum RawResourceRecord<'a> {
    A(Ipv4Addr),
    AAAA(Ipv6Addr),
    SOA {
        mname: LabelSet<'a>,
        rname: LabelSet<'a>,
        serial: u32,
        refresh: u32,
        retry: u32,
        expire: u32,
        min_ttl: u32,
    },
    Unknown(QType, QClass, &'a [u8]),
}

impl<'a> ResolveRefLabels<'a> for RawResourceRecord<'a> {
    type Output = Self;

    fn resolve_ref_labels(self, orig_packet: &'a [u8]) -> Result<Self, ParseError> {
        match self {
            RawResourceRecord::SOA {
                mname,
                rname,
                serial,
                refresh,
                retry,
                expire,
                min_ttl,
            } => Ok(RawResourceRecord::SOA {
                mname: mname.resolve_ref_labels(orig_packet)?,
                rname: rname.resolve_ref_labels(orig_packet)?,
                serial,
                refresh,
                retry,
                expire,
                min_ttl,
            }),
            other => Ok(other),
        }
    }
}

impl<'a> TryInto<ResourceRecord> for RawResourceRecord<'a> {
    type Error = ParseError<'a>;

    fn try_into(self) -> Result<ResourceRecord, Self::Error> {
        match self {
            RawResourceRecord::A(ip) => Ok(ResourceRecord::A(ip)),
            RawResourceRecord::AAAA(ip) => Ok(ResourceRecord::AAAA(ip)),
            RawResourceRecord::SOA {
                mname,
                rname,
                serial,
                refresh,
                retry,
                expire,
                min_ttl,
            } => Ok(ResourceRecord::SOA {
                mname: mname.to_string(),
                rname: rname.to_string(),
                serial,
                refresh,
                retry,
                expire,
                min_ttl,
            }),
            RawResourceRecord::Unknown(qtype, qclass, data) => {
                Ok(ResourceRecord::Unknown(qtype, qclass, data.to_vec()))
            }
        }
    }
}

fn parse_a_record(input: &[u8]) -> nom::IResult<&[u8], RawResourceRecord> {
    let (input, addr) = nom::bytes::streaming::take(4usize)(input)?;
    let addr = Ipv4Addr::new(addr[0], addr[1], addr[2], addr[3]);

    Ok((input, RawResourceRecord::A(addr)))
}

fn parse_aaaa_record(input: &[u8]) -> nom::IResult<&[u8], RawResourceRecord> {
    let (input, addr) = nom::multi::count(nom::number::streaming::be_u16, 8usize)(input)?;
    let addr = Ipv6Addr::new(
        addr[0], addr[1], addr[2], addr[3], addr[4], addr[5], addr[6], addr[7],
    );

    Ok((input, RawResourceRecord::AAAA(addr)))
}

fn parse_soa_record(input: &[u8]) -> nom::IResult<&[u8], RawResourceRecord> {
    let (input, mname) = parse_label_list(input)?;
    let (input, rname) = parse_label_list(input)?;
    let (input, serial) = nom::number::streaming::be_u32(input)?;
    let (input, refresh) = nom::number::streaming::be_u32(input)?;
    let (input, retry) = nom::number::streaming::be_u32(input)?;
    let (input, expire) = nom::number::streaming::be_u32(input)?;
    let (input, min_ttl) = nom::number::streaming::be_u32(input)?;

    Ok((
        input,
        RawResourceRecord::SOA {
            mname,
            rname,
            serial,
            refresh,
            retry,
            expire,
            min_ttl,
        },
    ))
}

pub(crate) fn parse_record(
    input: &[u8],
    qtype: QType,
    qclass: QClass,
) -> nom::IResult<&[u8], RawResourceRecord> {
    match (qtype, qclass) {
        (QType::A, QClass::IN) => parse_a_record(input),
        (QType::AAAA, QClass::IN) => parse_aaaa_record(input),
        (QType::SOA, QClass::IN) => parse_soa_record(input),
        (qtype, qclass) => {
            let tail = &input[input.len()..];
            Ok((tail, RawResourceRecord::Unknown(qtype, qclass, input)))
        }
    }
}
