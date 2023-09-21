use super::parser::DnsBytePacketBuffer;
use anyhow::Result;
use std::net::Ipv4Addr;

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

impl ResponseCode {
    fn get_r_code(n: u8) -> ResponseCode {
        match n {
            1 => ResponseCode::FORMATERROR,
            2 => ResponseCode::SERVERFAILURE,
            3 => ResponseCode::NAMEERROR,
            4 => ResponseCode::NOTIMPLEMENTED,
            5 => ResponseCode::REFUSED,
            6..=15 => ResponseCode::FUTURE,
            0 | _ => ResponseCode::NOERROR,
        }
    }
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

impl DnsQuestion {
    fn read(dbuf: &mut DnsBytePacketBuffer, entries: usize) -> Result<Vec<DnsQuestion>> {
        Ok(Vec::new())
    }
}

#[derive(Debug, Default)]
pub struct DnsHeader {
    pub id: u16,              // 16bits packet identifier
    pub qr: bool,             // 1bit query response
    pub op_code: u8,          // 4bits operation code
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

impl DnsHeader {
    fn read(dbuf: &mut DnsBytePacketBuffer) -> Result<Self, Box<dyn std::error::Error>> {
        let mut header: DnsHeader = DnsHeader::default();
        header.id = dbuf.read_u16()?;
        let a: u8 = dbuf.read()?;
        let b: u8 = dbuf.read()?;
        header.qr = (a >> 7) > 0;
        header.op_code = (a >> 3) & 0x0F;
        header.aa = ((a >> 2) & 1) > 0;
        header.tc = ((a >> 1) & 1) > 0;
        header.rd = (a & 1) > 0;
        header.ra = (b >> 7) > 0;
        header.z = false;
        header.r_code = ResponseCode::get_r_code(b & 0x0F);
        header.qd_count = dbuf.read_u16()?;
        header.an_count = dbuf.read_u16()?;
        header.ns_count = dbuf.read_u16()?;
        header.ar_count = dbuf.read_u16()?;
        Ok(header)
    }
}

#[derive(Debug, Default)]
pub struct DnsPacket {
    pub header: DnsHeader,
    pub questions: Vec<DnsQuestion>,
    pub answers: Vec<DnsRecord>,
    pub authoritys: Vec<DnsRecord>,
    pub additionals: Vec<DnsRecord>,
}

impl DnsPacket {
    pub fn new(file_name: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut dbuf: DnsBytePacketBuffer = DnsBytePacketBuffer::load(file_name)?;
        let mut packet: DnsPacket = Self::default();
        packet.header = DnsHeader::read(&mut dbuf)?;
        packet.questions = DnsQuestion::read(&mut dbuf, packet.header.qd_count as usize)?;
        Ok(packet)
    }
}
