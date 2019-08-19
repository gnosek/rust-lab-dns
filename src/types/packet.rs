use super::header::Header;
use super::query::Query;
use super::response::Response;

#[derive(Debug, PartialEq, Eq)]
pub struct DnsPacket {
    pub header: Header,

    pub queries: Vec<Query>,
    pub answers: Vec<Response>,
    pub authority: Vec<Response>,
    pub additional: Vec<Response>,
}
