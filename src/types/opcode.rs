#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
    StandardQuery,
    InverseQuery,
    ServerStatusRequest,
    Notify,
    Update,
    Reserved(u8),
}

impl From<u8> for Opcode {
    fn from(bits: u8) -> Opcode {
        match bits {
            0 => Opcode::StandardQuery,
            1 => Opcode::InverseQuery,
            2 => Opcode::ServerStatusRequest,
            4 => Opcode::Notify,
            5 => Opcode::Update,
            other => Opcode::Reserved(other),
        }
    }
}

impl Into<u8> for Opcode {
    fn into(self) -> u8 {
        match self {
            Opcode::StandardQuery => 0,
            Opcode::InverseQuery => 1,
            Opcode::ServerStatusRequest => 2,
            Opcode::Notify => 4,
            Opcode::Update => 5,
            Opcode::Reserved(other) => other,
        }
    }
}
