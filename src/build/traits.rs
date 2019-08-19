pub trait Serialize {
    fn serialize_to(&self, buf: &mut Vec<u8>) -> Result<(), std::io::Error>;
}
