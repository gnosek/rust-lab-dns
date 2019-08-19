#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QClass {
    IN,
    Reserved(u16),
}

impl From<u16> for QClass {
    fn from(bits: u16) -> QClass {
        match bits {
            1 => QClass::IN,
            other => QClass::Reserved(other),
        }
    }
}

impl Into<u16> for QClass {
    fn into(self) -> u16 {
        match self {
            QClass::IN => 1,
            QClass::Reserved(other) => other,
        }
    }
}
