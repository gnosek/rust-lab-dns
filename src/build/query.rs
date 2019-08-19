use super::traits::Serialize;
use crate::types::{LabelSet, Query};
use byteorder::{NetworkEndian, WriteBytesExt};

impl Serialize for Query {
    fn serialize_to(&self, buf: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let label = LabelSet::from(self.qname.as_str());
        label.serialize_to(buf)?;

        let qtype = self.qtype.into();
        let qclass = self.qclass.into();

        buf.write_u16::<NetworkEndian>(qtype)?;
        buf.write_u16::<NetworkEndian>(qclass)?;

        Ok(())
    }
}
