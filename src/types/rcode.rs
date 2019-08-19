#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rcode {
    NoError,
    FormatError,
    ServerFailure,
    NameError,
    NotImplemented,
    Refused,
    YxDomain,
    YxRrSet,
    NxRrSet,
    NotAuth,
    NotZone,
    Reserved(u8),
}

impl From<u8> for Rcode {
    fn from(bits: u8) -> Rcode {
        match bits {
            0 => Rcode::NoError,
            1 => Rcode::FormatError,
            2 => Rcode::ServerFailure,
            3 => Rcode::NameError,
            4 => Rcode::NotImplemented,
            5 => Rcode::Refused,
            6 => Rcode::YxDomain,
            7 => Rcode::YxRrSet,
            8 => Rcode::NxRrSet,
            9 => Rcode::NotAuth,
            10 => Rcode::NotZone,
            other => Rcode::Reserved(other),
        }
    }
}

impl Into<u8> for Rcode {
    fn into(self) -> u8 {
        match self {
            Rcode::NoError => 0,
            Rcode::FormatError => 1,
            Rcode::ServerFailure => 2,
            Rcode::NameError => 3,
            Rcode::NotImplemented => 4,
            Rcode::Refused => 5,
            Rcode::YxDomain => 6,
            Rcode::YxRrSet => 7,
            Rcode::NxRrSet => 8,
            Rcode::NotAuth => 9,
            Rcode::NotZone => 10,
            Rcode::Reserved(other) => other,
        }
    }
}
