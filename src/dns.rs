use super::parser::DnsBytePacketBuffer;
use super::Result;
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

impl From<u8> for ResponseCode {
    fn from(n: u8) -> Self {
        match n {
            1 => Self::FORMATERROR,
            2 => Self::SERVERFAILURE,
            3 => Self::NAMEERROR,
            4 => Self::NOTIMPLEMENTED,
            5 => Self::REFUSED,
            6..=15 => Self::FUTURE,
            0 | _ => Self::NOERROR,
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
    /// label sequence
    pub label: String,
    /// 2bytes record type
    pub r_type: QueryType,
    /// 2bytes record class always set to 1
    pub r_class: DnsClass,
    /// 4bytes Time-to-Live
    pub ttl: u32,
    /// 2bytes length of record type specific data
    pub rd_len: u16,
    /// record data
    pub r_data: RecordData,
}

impl DnsRecord {
    fn read(dbuf: &mut DnsBytePacketBuffer, entries: usize) -> Result<Vec<DnsRecord>> {
        let mut records: Vec<DnsRecord> = Vec::new();
        for _ in 1..=entries {
            let query: String = dbuf.read_label()?;
            let r_type: QueryType = dbuf.read_u16()?.into();
            let r_class: DnsClass = dbuf.read_u16()?.into();
            let ttl: u32 = dbuf.read_u32()?;
            let rd_len: u16 = dbuf.read_u16()?;
            let r_data: RecordData = RecordData::from(&r_type, &mut *dbuf)?;
            records.push(DnsRecord {
                label: query,
                r_type,
                r_class,
                ttl,
                rd_len,
                r_data,
            });
        }
        Ok(records)
    }
}

#[derive(Debug)]
pub enum RecordData {
    IPADDR(Ipv4Addr),
    UNKNOWN(u16),
}

impl Default for RecordData {
    fn default() -> Self {
        Self::UNKNOWN(0u16)
    }
}

impl RecordData {
    fn from(r_type: &QueryType, dbuf: &mut DnsBytePacketBuffer) -> Result<Self> {
        match *r_type {
            QueryType::A => {
                let ip_addr: Ipv4Addr =
                    Ipv4Addr::new(dbuf.read()?, dbuf.read()?, dbuf.read()?, dbuf.read()?);
                Ok(Self::IPADDR(ip_addr))
            }
            QueryType::UNKNOWN(x) => Ok(Self::UNKNOWN(x)),
        }
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

impl From<u16> for QueryType {
    fn from(num: u16) -> Self {
        match num {
            1 => QueryType::A,
            y => QueryType::UNKNOWN(y),
        }
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

impl From<u16> for DnsClass {
    fn from(num: u16) -> Self {
        match num {
            1 => Self::IN,
            2 => Self::CS,
            3 => Self::CH,
            4 => Self::HS,
            _ => Self::IN,
        }
    }
}

#[derive(Debug, Default)]
pub struct DnsQuestion {
    /// label sequence
    pub label: String,
    /// 2byte record type
    pub q_type: QueryType,
    /// 2byte class always set to 1
    pub q_class: DnsClass,
}

impl DnsQuestion {
    fn read(dbuf: &mut DnsBytePacketBuffer, entries: usize) -> Result<Vec<DnsQuestion>> {
        let mut questions: Vec<DnsQuestion> = Vec::new();
        for _ in 1..=entries {
            let query: String = dbuf.read_label()?;
            let q_type: QueryType = dbuf.read_u16()?.into();
            let q_class: DnsClass = dbuf.read_u16()?.into();
            questions.push(DnsQuestion {
                label: query,
                q_type,
                q_class,
            });
        }
        Ok(questions)
    }
}

#[derive(Debug, Default)]
pub struct DnsHeader {
    /// 16bits packet identifier
    pub id: u16,
    /// 1bit query response (0 if query, 1 if response)
    pub qr: bool,
    /// 4bits operation code
    pub op_code: u8,
    /// 1bit authoritative answer
    pub aa: bool,
    /// 1bit truncated message
    pub tc: bool,
    /// 1bit recursion desired
    pub rd: bool,
    /// 1bit recursion available
    pub ra: bool,
    /// 3bits reserved for future use must be 0 in all case
    pub z: bool,
    /// 4bits response code
    pub r_code: ResponseCode,
    /// 16bits question count
    pub qd_count: u16,
    /// 16bits answer count
    pub an_count: u16,
    /// 16bits authority count
    pub ns_count: u16,
    /// 16bits additional count
    pub ar_count: u16,
}

impl DnsHeader {
    fn read(dbuf: &mut DnsBytePacketBuffer) -> Result<Self> {
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
        header.r_code = (b & 0x0F).into();
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
    pub fn new(file_name: &str) -> Result<Self> {
        let mut dbuf: DnsBytePacketBuffer = DnsBytePacketBuffer::load(file_name)?;
        let mut packet: DnsPacket = Self::default();
        packet.header = DnsHeader::read(&mut dbuf)?;
        packet.questions = DnsQuestion::read(&mut dbuf, packet.header.qd_count as usize)?;
        if packet.header.qr && packet.header.an_count > 0 {
            packet.answers = DnsRecord::read(&mut dbuf, packet.header.an_count as usize)?;
        }
        if packet.header.ns_count > 0 {
            packet.answers = DnsRecord::read(&mut dbuf, packet.header.ns_count as usize)?;
        }
        if packet.header.ar_count > 0 {
            packet.answers = DnsRecord::read(&mut dbuf, packet.header.ar_count as usize)?;
        }
        Ok(packet)
    }
}
