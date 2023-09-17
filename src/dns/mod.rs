use std::net::Ipv4Addr;
mod parser;

#[derive(Debug)]
pub enum ResponseCode {
    NOERROR = 0,
    FORMATERROR = 1,
    SERVERFAILURE = 2,
    NAMEERROR = 3,
    NOTIMPLEMENTED = 4,
    REFUSED = 5,
    FUTURE,
}

impl Default for ResponseCode {
    fn default() -> Self {
        Self::NOERROR
    }
}

#[derive(Debug, Default)]
pub struct DnsRecord {
    pub label: String,      // label sequence
    pub r_type: QueryType,  // 2bytes record type
    pub r_class: DnsClass,  // 2bytes record class always set to 1
    pub ttl: u32,           // 4bytes Time-to-Live
    pub rd_len: u16,        // 2bytes length of record type specific data
    pub r_data: RecordData, // record data
}

#[derive(Debug)]
pub enum RecordData {
    IPADDR(Ipv4Addr),
    UNKNOWN,
}

impl Default for RecordData {
    fn default() -> Self {
        Self::UNKNOWN
    }
}

#[derive(Debug)]
pub enum QueryType {
    A,
    UNKNOWN(u16),
}

impl Default for QueryType {
    fn default() -> Self {
        Self::A
    }
}

#[derive(Debug)]
pub enum DnsClass {
    IN = 1,
    CS = 2,
    CH = 3,
    HS = 4,
}

impl Default for DnsClass {
    fn default() -> Self {
        Self::IN
    }
}

#[derive(Debug, Default)]
pub struct DnsQuestion {
    pub label: String,     // label sequence
    pub q_type: QueryType, // 2byte record type
    pub q_class: DnsClass, // 2byte class always set to 1
}

impl DnsQuestion {}

#[derive(Debug, Default)]
pub struct DnsHeader {
    pub id: u16,              // 16bits packet identifier
    pub qr: bool,             // 1bit query response
    pub op_code: u16,         // 4bits operation code
    pub aa: bool,             // 1bit authoritative answer
    pub tc: bool,             // 1bit truncated message
    pub rd: bool,             // 1bit recursion desired
    pub ra: bool,             // 1bit recursion available
    pub z: bool,              // 3bits reserved for future use must be 0 in all case
    pub r_code: ResponseCode, // 4bits response code
    pub qd_count: u16,        // 16bits question count
    pub an_count: u16,        // 16bits answer count
    pub ns_count: u16,        // 16bits authority count
    pub ar_count: u16,        // 16bits additional count
}

impl DnsHeader {}

#[derive(Debug, Default)]
pub struct DnsPacket {
    pub header: DnsHeader,
    pub question: Vec<DnsQuestion>,
    pub answer: Vec<DnsRecord>,
    pub authority: Vec<DnsRecord>,
    pub additional: Vec<DnsRecord>,
}

impl DnsPacket {}
