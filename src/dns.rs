#[derive(Debug, Default)]
pub struct DnsHeader {
    pub id: u16,       // 16bits packet identifier
    pub qr: bool,      // 1bit query response
    pub op_code: u16,  // 4bits operation code
    pub aa: bool,      // 1bit authoritative answer
    pub tc: bool,      // 1bit truncated message
    pub rd: bool,      // 1bit recursion desired
    pub ra: bool,      // 1bit recursion available
    pub z: u8,         // 3bits reserved
    pub r_code: u16,   // 4bits response code
    pub qd_count: u16, // 16bits question count
    pub an_count: u16, // 16bits answer count
    pub ns_count: u16, // 16bits authority count
    pub ar_count: u16, // 16bits additional count
}

#[derive(Debug, Default)]
pub struct DnsQuestion {
    label: String,            // label sequence
    question_type: QueryType, // 2byte record type
    question_class: DnsClass,      // 2byte class always set to 1
}

#[derive(Debug, Default)]
pub struct DnsRecord {
    label: String,          // label sequence
    record_type: QueryType, // 2bytes record type
    record_class: DnsClass,      // 2bytes record class always set to 1
    ttl: u32,               // 4bytes Time-to-Live
    rlen: u16,              // 2bytes length of record type specific data
    rdata: RecordData,      // record data
}

#[derive(Debug)]
pub enum RecordData {
    ipaddr,
}
impl Default for RecordData {
    fn default () -> Self {
        Self::ipaddr
    }
}

#[derive(Debug)]
pub enum QueryType {
    A,
    UNKNOWN(u16),
}

impl Default for QueryType {
    fn default() -> Self{
        Self::A
    }
}

#[derive(Debug)]
pub enum DnsClass { 
    IN = 1, 
    CS = 2, 
    CH = 3, 
    HS = 4
}
impl Default for DnsClass {
    fn default() -> Self { 
        Self::IN
    }
}

#[derive(Debug, Default)]
pub struct DnsPacket {
    pub header: DnsHeader,
    pub question: Vec<DnsQuestion>,
    pub answer: Vec<DnsRecord>,
    pub authority: Vec<DnsRecord>,
    pub additional: Vec<DnsRecord>,
}
