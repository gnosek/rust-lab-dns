use super::qclass::QClass;
use super::qtype::QType;

#[derive(Debug, PartialEq, Eq)]
pub struct Query {
    pub qname: String,
    pub qtype: QType,
    pub qclass: QClass,
}
