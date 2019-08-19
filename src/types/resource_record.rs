use super::qclass::QClass;
use super::qtype::QType;
use std::net::{Ipv4Addr, Ipv6Addr};

#[derive(Debug, PartialEq, Eq)]
pub enum ResourceRecord {
    A(Ipv4Addr),
    AAAA(Ipv6Addr),
    SOA {
        mname: String,
        rname: String,
        serial: u32,
        refresh: u32,
        retry: u32,
        expire: u32,
        min_ttl: u32,
    },
    Unknown(QType, QClass, Vec<u8>),
}

impl ResourceRecord {
    pub fn qtype(&self) -> QType {
        match self {
            ResourceRecord::A(_) => QType::A,
            ResourceRecord::AAAA(_) => QType::AAAA,
            ResourceRecord::SOA { .. } => QType::SOA,
            ResourceRecord::Unknown(qtype, _, _) => *qtype,
        }
    }

    pub fn qclass(&self) -> QClass {
        match self {
            ResourceRecord::A(_) => QClass::IN,
            ResourceRecord::AAAA(_) => QClass::IN,
            ResourceRecord::SOA { .. } => QClass::IN,
            ResourceRecord::Unknown(_, qclass, _) => *qclass,
        }
    }
}
