mod error;
mod header;
mod label;
mod packet;
mod query;
mod resource_record;
mod response;
mod traits;

pub use error::*;
pub use packet::{parse_dns_packet, parse_dns_tcp_packet};

#[cfg(test)]
mod tests {
    use crate::parse::packet::parse_dns_packet;
    use crate::types::Opcode;
    use crate::types::Rcode;
    use crate::types::ResourceRecord;
    use std::net::Ipv4Addr;
    use crate::parse::DnsParseError::ParseError;

    #[test]
    fn parse_request() {
        let req = vec![
            0xea, 0x3f, 0x01, 0x20, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x07, 0x65,
            0x78, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x03, 0x63, 0x6f, 0x6d, 0x00, 0x00, 0x01, 0x00,
            0x01, 0x00, 0x00, 0x29, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        let res = parse_dns_packet(&req);
        let (tail, packet) = res.unwrap();
        assert_eq!(packet.header.seq, 0xea3f);
        assert_eq!(packet.header.is_response, false);
        assert_eq!(packet.header.opcode, Opcode::StandardQuery);
        assert_eq!(packet.header.authoritative, false);
        assert_eq!(packet.header.truncated, false);
        assert_eq!(packet.header.recursion_requested, true);
        assert_eq!(packet.header.recursion_available, false);
        assert_eq!(packet.header.rcode, Rcode::NoError);

        assert_eq!(packet.queries.len(), 1);
        let qname = packet.queries[0].qname.as_str();
        assert_eq!(qname, "example.com");

        assert!(tail.is_empty());
    }

    #[test]
    fn parse_request_truncated() {
        let req = vec![
            0xea, 0x3f, 0x01, 0x20, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x07, 0x65,
            0x78, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x03, 0x63, 0x6f, 0x6d, 0x00, 0x00, 0x01, 0x00,
        ];

        let res = parse_dns_packet(&req);
        match res {
            Err(ParseError(nom::Err::Incomplete(_))) => (),
            _ => assert!(false),
        }
    }

    #[test]
    fn parse_response() {
        let req = vec![
            0xea, 0x3f, 0x81, 0x80, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x07, 0x65,
            0x78, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x03, 0x63, 0x6f, 0x6d, 0x00, 0x00, 0x01, 0x00,
            0x01, 0xc0, 0x0c, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x3e, 0x5e, 0x00, 0x04, 0x5d,
            0xb8, 0xd8, 0x22, 0x00, 0x00, 0x29, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        let res = parse_dns_packet(&req);
        let (tail, packet) = res.unwrap();
        assert_eq!(packet.header.seq, 0xea3f);
        assert_eq!(packet.header.is_response, true);
        assert_eq!(packet.header.opcode, Opcode::StandardQuery);
        assert_eq!(packet.header.authoritative, false);
        assert_eq!(packet.header.truncated, false);
        assert_eq!(packet.header.recursion_requested, true);
        assert_eq!(packet.header.recursion_available, true);
        assert_eq!(packet.header.rcode, Rcode::NoError);

        assert_eq!(packet.queries.len(), 1);
        let qname = packet.queries[0].qname.as_str();
        assert_eq!(qname, "example.com");

        assert_eq!(packet.answers.len(), 1);
        let qname = packet.answers[0].qname.as_str();
        assert_eq!(qname, "example.com");
        match &packet.answers[0].rdata {
            ResourceRecord::A(ip) => assert_eq!(ip, &Ipv4Addr::new(93, 184, 216, 34)),
            rr => panic!("Got unexpected RR: {:?}", rr),
        }

        assert!(tail.is_empty());
    }
}
