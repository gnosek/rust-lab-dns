#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QType {
    A,
    AAAA,
    SOA,
    OPT,
    Reserved(u16),
}

impl From<u16> for QType {
    fn from(bits: u16) -> QType {
        match bits {
            1 => QType::A,
            6 => QType::SOA,
            28 => QType::AAAA,
            41 => QType::OPT,
            other => QType::Reserved(other),
        }
    }
}

impl Into<u16> for QType {
    fn into(self) -> u16 {
        match self {
            QType::A => 1,
            QType::SOA => 6,
            QType::AAAA => 28,
            QType::OPT => 41,
            QType::Reserved(other) => other,
        }
    }
}
