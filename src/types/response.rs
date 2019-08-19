use super::resource_record::ResourceRecord;

#[derive(Debug, PartialEq, Eq)]
pub struct Response {
    pub qname: String,
    pub ttl: u32,
    pub rdata: ResourceRecord,
}
