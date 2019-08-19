use super::traits::Serialize;
use crate::types::{LabelSet, ResourceRecord};
use byteorder::{NetworkEndian, WriteBytesExt};
use std::io::Write;

impl Serialize for ResourceRecord {
    fn serialize_to(&self, buf: &mut Vec<u8>) -> Result<(), std::io::Error> {
        match self {
            ResourceRecord::A(ip) => {
                buf.write_u16::<NetworkEndian>(4u16)?;
                buf.write_all(&ip.octets())?;
            }
            ResourceRecord::AAAA(ip) => {
                buf.write_u16::<NetworkEndian>(16u16)?;
                for seg in &ip.segments() {
                    buf.write_u16::<NetworkEndian>(*seg)?;
                }
            }
            ResourceRecord::SOA {
                mname,
                rname,
                serial,
                refresh,
                retry,
                expire,
                min_ttl,
            } => {
                let mut soa_buf: Vec<u8> = Vec::new();
                let mname = LabelSet::from(mname.as_str());
                mname.serialize_to(&mut soa_buf)?;
                let rname = LabelSet::from(rname.as_str());
                rname.serialize_to(&mut soa_buf)?;

                soa_buf.write_u32::<NetworkEndian>(*serial)?;
                soa_buf.write_u32::<NetworkEndian>(*refresh)?;
                soa_buf.write_u32::<NetworkEndian>(*retry)?;
                soa_buf.write_u32::<NetworkEndian>(*expire)?;
                soa_buf.write_u32::<NetworkEndian>(*min_ttl)?;

                buf.write_u16::<NetworkEndian>(soa_buf.len() as u16)?;
                buf.write_all(&soa_buf)?;
            }
            ResourceRecord::Unknown(_, _, data) => {
                buf.write_u16::<NetworkEndian>(data.len() as u16)?;
                buf.write_all(data)?;
            }
        }
        Ok(())
    }
}
