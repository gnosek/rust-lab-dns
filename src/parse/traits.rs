use super::error::DnsParseError;

pub(crate) trait ResolveRefLabels<'a>: Sized {
    type Output;

    fn resolve_ref_labels(self, orig_packet: &'a [u8]) -> Result<Self::Output, DnsParseError<'a>>;
}
