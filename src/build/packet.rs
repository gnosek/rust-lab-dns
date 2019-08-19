use super::traits::Serialize;
use crate::types::DnsPacket;
use byteorder::{NetworkEndian, WriteBytesExt};
use std::io::Write;

impl Serialize for DnsPacket {
    fn serialize_to(&self, buf: &mut Vec<u8>) -> Result<(), std::io::Error> {
        self.header.serialize_to(buf)?;
        buf.write_u16::<NetworkEndian>(self.queries.len() as u16)?;
        buf.write_u16::<NetworkEndian>(self.answers.len() as u16)?;
        buf.write_u16::<NetworkEndian>(self.authority.len() as u16)?;
        buf.write_u16::<NetworkEndian>(self.additional.len() as u16)?;

        for item in &self.queries {
            item.serialize_to(buf)?;
        }

        for item in &self.answers {
            item.serialize_to(buf)?;
        }

        for item in &self.authority {
            item.serialize_to(buf)?;
        }

        for item in &self.additional {
            item.serialize_to(buf)?;
        }

        Ok(())
    }
}

impl DnsPacket {
    pub fn serialize_tcp_to(&self, buf: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let mut serialized = Vec::new();
        self.serialize_to(&mut serialized)?;

        buf.write_u16::<NetworkEndian>(serialized.len() as u16)?;
        buf.write_all(&serialized)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::build::traits::Serialize;
    use crate::parse::parse_dns_packet;
    use crate::types::{DnsPacket, Header, Opcode, QClass, QType, Query, Rcode};

    #[test]
    fn serialize_query() {
        let packet = DnsPacket {
            header: Header {
                seq: 0xcafe,
                is_response: false,
                opcode: Opcode::StandardQuery,
                authoritative: false,
                truncated: false,
                recursion_requested: true,
                recursion_available: false,
                rcode: Rcode::NoError,
            },

            queries: vec![Query {
                qname: String::from("example.com"),
                qtype: QType::A,
                qclass: QClass::IN,
            }],
            answers: Vec::new(),
            authority: Vec::new(),
            additional: Vec::new(),
        };

        let mut buf = Vec::new();
        packet.serialize_to(&mut buf).unwrap();

        let (tail, roundtripped) = parse_dns_packet(&buf).unwrap();
        assert_eq!(tail, []);
        assert_eq!(roundtripped, packet);

        assert_eq!(
            buf.as_slice(),
            &[
                0xca, 0xfe, 0x01, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x07, 0x65,
                0x78, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x03, 0x63, 0x6f, 0x6d, 0x00, 0x00, 0x01, 0x00,
                0x01
            ]
        );
    }
}
