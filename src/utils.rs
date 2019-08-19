use crate::types::{DnsPacket, Header, Rcode, ResourceRecord, Response, QType, QClass};
use std::net::SocketAddr;

pub fn error(orig_packet: DnsPacket, error: Rcode) -> DnsPacket {
    DnsPacket {
        header: Header {
            seq: orig_packet.header.seq,
            is_response: true,
            opcode: orig_packet.header.opcode,
            truncated: false,
            authoritative: true,
            recursion_requested: orig_packet.header.recursion_requested,
            recursion_available: false,
            rcode: error,
        },

        queries: orig_packet.queries,
        answers: Vec::new(),
        authority: Vec::new(),
        additional: Vec::new(),
    }
}

pub fn ok(orig_packet: DnsPacket, answers: Vec<Response>) -> DnsPacket {
    DnsPacket {
        header: Header {
            seq: orig_packet.header.seq,
            is_response: true,
            opcode: orig_packet.header.opcode,
            truncated: false,
            authoritative: true,
            recursion_requested: orig_packet.header.recursion_requested,
            recursion_available: false,
            rcode: Rcode::NoError,
        },

        queries: orig_packet.queries,
        answers,
        authority: Vec::new(),
        additional: Vec::new(),
    }
}

fn addr_to_record(addr: &SocketAddr) -> ResourceRecord {
    match addr {
        SocketAddr::V4(addr) => ResourceRecord::A(addr.ip().clone()),
        SocketAddr::V6(addr) => ResourceRecord::AAAA(addr.ip().clone()),
    }
}

pub fn respond(query: DnsPacket, src: &SocketAddr) -> DnsPacket {
    if query.header.is_response {
        return error(query, Rcode::Refused);
    }

    if query.queries.len() != 1 {
        return error(query, Rcode::ServerFailure);
    }

    let q = &query.queries[0];
    dbg!(q);
    match (q.qtype, q.qclass) {
        (QType::A, QClass::IN) | (QType::AAAA, QClass::IN) => {
            let resp = Response {
                qname: q.qname.clone(),
                ttl: 12345,
                rdata: addr_to_record(src),
            };

            ok(query, vec![resp])
        }
        _ => ok(query, Vec::new()),
    }
}
