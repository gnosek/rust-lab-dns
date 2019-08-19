use super::traits::Serialize;
use crate::types::{LabelSet, Response};
use byteorder::{NetworkEndian, WriteBytesExt};

impl Serialize for Response {
    fn serialize_to(&self, buf: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let label = LabelSet::from(self.qname.as_str());
        label.serialize_to(buf)?;

        let qtype: u16 = self.rdata.qtype().into();
        let qclass: u16 = self.rdata.qclass().into();

        buf.write_u16::<NetworkEndian>(qtype)?;
        buf.write_u16::<NetworkEndian>(qclass)?;

        buf.write_u32::<NetworkEndian>(self.ttl)?;

        self.rdata.serialize_to(buf)?;
        Ok(())
    }
}
