use super::traits::Serialize;
use crate::types::Header;
use byteorder::{NetworkEndian, WriteBytesExt};
use std::io::Write;

impl Serialize for Header {
    fn serialize_to(&self, buf: &mut Vec<u8>) -> Result<(), std::io::Error> {
        buf.write_u16::<NetworkEndian>(self.seq)?;
        let flags: Vec<u8> = Vec::new();
        let mut bits = bitbit::BitWriter::new(flags);

        let opcode: u8 = self.opcode.into();
        let rcode: u8 = self.rcode.into();

        bits.write_bit(self.is_response)?;
        bits.write_bits(opcode.into(), 4)?;
        bits.write_bit(self.authoritative)?;
        bits.write_bit(self.truncated)?;
        bits.write_bit(self.recursion_requested)?;
        bits.write_bit(self.recursion_available)?;
        bits.write_bits(0, 3)?;
        bits.write_bits(rcode.into(), 4)?;
        buf.write_all(bits.get_ref())?;

        Ok(())
    }
}
