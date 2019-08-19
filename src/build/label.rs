use super::traits::Serialize;
use crate::types::{Label, LabelSet};
use byteorder::{NetworkEndian, WriteBytesExt};
use std::io::Write;

impl<'a> Serialize for Label<'a> {
    fn serialize_to(&self, buf: &mut Vec<u8>) -> Result<(), std::io::Error> {
        match self {
            Label::Reference(offset) => {
                let offset = *offset | 0xc000;
                buf.write_u16::<NetworkEndian>(offset)?;
            }
            Label::Data(label) => {
                buf.write_u8(label.len() as u8)?;
                buf.write_all(label)?;
            }
            Label::End => {
                buf.write_u8(0)?;
            }
        }
        Ok(())
    }
}

impl<'a> Serialize for LabelSet<'a> {
    fn serialize_to(&self, buf: &mut Vec<u8>) -> Result<(), std::io::Error> {
        for label in &self.0 {
            label.serialize_to(buf)?;
        }
        Ok(())
    }
}
