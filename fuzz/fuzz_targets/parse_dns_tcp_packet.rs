#![no_main]
use libfuzzer_sys::fuzz_target;

use dns::parse::parse_dns_tcp_packet;

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    parse_dns_tcp_packet(data).ok();
});
