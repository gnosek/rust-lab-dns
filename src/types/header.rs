use super::opcode::Opcode;
use super::rcode::Rcode;

#[derive(Debug, PartialEq, Eq)]
pub struct Header {
    pub seq: u16,
    pub is_response: bool,
    pub opcode: Opcode,
    pub authoritative: bool,
    pub truncated: bool,
    pub recursion_requested: bool,
    pub recursion_available: bool,
    pub rcode: Rcode,
}
