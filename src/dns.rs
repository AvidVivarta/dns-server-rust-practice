use super::parser::DnsBytePacketBuffer;
use super::Result;
use std::net::Ipv4Addr;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
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
    pub fn write(&self, buffer: &mut DnsBytePacketBuffer) -> Result<usize> {
        let start_pos = buffer.get_pos();

        buffer.write_label(&self.label)?;
        buffer.write_u16(QueryType::A.into())?;
        buffer.write_u16(1)?;
        buffer.write_u32(self.ttl)?;
        buffer.write_u16(4)?;

        match &self.r_data {
            RecordData::IPADDR(addr) => {
                let octets = addr.octets();
                buffer.write_u8(octets[0])?;
                buffer.write_u8(octets[1])?;
                buffer.write_u8(octets[2])?;
                buffer.write_u8(octets[3])?;
            }
            RecordData::UNKNOWN { .. } => {
                println!("Skipping record: {:?}", self);
            }
        }

        Ok(buffer.get_pos() - start_pos)
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
            _ => Ok(Self::UNKNOWN(17)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u16)]
pub enum QueryType {
    /// 1 a host address
    A = 1,
    /// 2 an authoritative name server
    NS = 2,
    /// 3 a mail destination (Obsolete - use MX)
    MD = 3,

    /// 4 a mail forwarder (Obsolete - use MX)
    MF = 4,
    /// 5 the canonical name for an alias
    CNAME = 5,
    /// 6 marks the start of a zone of authority
    SOA = 6,
    /// 7 a mailbox domain name (EXPERIMENTAL)
    MB = 7,
    /// 8 a mail group member (EXPERIMENTAL)
    MG = 8,
    /// 9 a mail rename domain name (EXPERIMENTAL)
    MR = 9,
    /// 10 a null RR (EXPERIMENTAL)
    NULL = 10,
    /// 11 a well known service description
    WKS = 11,
    /// 12 a domain name pointer
    PTR = 12,
    /// 13 host information
    HINFO = 13,
    /// 14 mailbox or mail list information
    MINFO = 14,
    /// 15 mail exchange
    MX = 15,
    /// 16 text strings
    TXT = 16,
    UNKNOWN(u16) = 17,
}

impl Default for QueryType {
    fn default() -> Self {
        Self::A
    }
}

impl From<QueryType> for u16 {
    fn from(num: QueryType) -> Self {
        match num {
            QueryType::A => 1,
            QueryType::NS => 2,
            QueryType::MD => 3,
            QueryType::MF => 4,
            QueryType::CNAME => 5,
            QueryType::SOA => 6,
            QueryType::MB => 7,
            QueryType::MG => 8,
            QueryType::MR => 9,
            QueryType::NULL => 10,
            QueryType::WKS => 11,
            QueryType::PTR => 12,
            QueryType::HINFO => 13,
            QueryType::MINFO => 14,
            QueryType::MX => 15,
            QueryType::TXT => 16,
            QueryType::UNKNOWN(y) => y,
        }
    }
}

impl From<u16> for QueryType {
    fn from(num: u16) -> Self {
        match num {
            1 => QueryType::A,
            2 => QueryType::NS,
            3 => QueryType::MD,
            4 => QueryType::MF,
            5 => QueryType::CNAME,
            6 => QueryType::SOA,
            7 => QueryType::MB,
            8 => QueryType::MG,
            9 => QueryType::MR,
            10 => QueryType::NULL,
            11 => QueryType::WKS,
            12 => QueryType::PTR,
            13 => QueryType::HINFO,
            14 => QueryType::MINFO,
            15 => QueryType::MX,
            16 => QueryType::TXT,
            y => QueryType::UNKNOWN(y),
        }
    }
}

#[derive(Debug)]
#[repr(u16)]
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
    pub fn new(label: String, q_type: QueryType) -> Self {
        Self {
            label,
            q_type,
            q_class: DnsClass::IN,
        }
    }
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
    pub fn write(&self, buffer: &mut DnsBytePacketBuffer) -> Result<()> {
        buffer.write_label(&self.label)?;
        buffer.write_u16(self.q_type.into())?;
        buffer.write_u16(1)?;

        Ok(())
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
    pub fn new() -> Self {
        Self {
            id: 0,
            qr: false,
            op_code: 0,
            aa: false,
            tc: false,
            rd: false,
            ra: false,
            z: false,
            r_code: ResponseCode::NOERROR,
            qd_count: 0,
            an_count: 0,
            ns_count: 0,
            ar_count: 0,
        }
    }

    pub fn read(dbuf: &mut DnsBytePacketBuffer) -> Result<Self> {
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
    pub fn write(&self, buffer: &mut DnsBytePacketBuffer) -> Result<()> {
        buffer.write_u16(self.id)?;

        buffer.write_u8(
            (self.rd as u8)
                | ((self.tc as u8) << 1)
                | ((self.aa as u8) << 2)
                | (self.op_code << 3)
                | ((self.qr as u8) << 7) as u8,
        )?;

        buffer.write_u8(
            (self.r_code as u8)
            // | ((self.checking_disabled as u8) << 4)
            // | ((self.authed_data as u8) << 5)
            | ((self.z as u8) << 6)
            | ((self.ra as u8) << 7),
        )?;

        buffer.write_u16(self.qd_count)?;
        buffer.write_u16(self.an_count)?;
        buffer.write_u16(self.ns_count)?;
        buffer.write_u16(self.ar_count)?;

        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct DnsPacket {
    pub header: DnsHeader,
    pub questions: Vec<DnsQuestion>,
    pub answers: Vec<DnsRecord>,
    pub authorities: Vec<DnsRecord>,
    pub additionals: Vec<DnsRecord>,
}

impl DnsPacket {
    pub fn new() -> Self {
        Self {
            header: DnsHeader::new(),
            questions: Vec::new(),
            answers: Vec::new(),
            authorities: Vec::new(),
            additionals: Vec::new(),
        }
    }

    pub fn from_buffer(dbuf: &mut DnsBytePacketBuffer) -> Result<Self> {
        let mut packet: DnsPacket = Self::default();
        packet.header = DnsHeader::read(dbuf)?;
        packet.questions = DnsQuestion::read(dbuf, packet.header.qd_count as usize)?;
        if packet.header.qr && packet.header.an_count > 0 {
            packet.answers = DnsRecord::read(dbuf, packet.header.an_count as usize)?;
        }
        if packet.header.ns_count > 0 {
            packet.answers = DnsRecord::read(dbuf, packet.header.ns_count as usize)?;
        }
        if packet.header.ar_count > 0 {
            packet.answers = DnsRecord::read(dbuf, packet.header.ar_count as usize)?;
        }
        Ok(packet)
    }

    pub fn read(file_name: &str) -> Result<Self> {
        let mut dbuf: DnsBytePacketBuffer = DnsBytePacketBuffer::load(file_name)?;
        Self::from_buffer(&mut dbuf)
    }
    pub fn write(&mut self, buffer: &mut DnsBytePacketBuffer) -> Result<()> {
        self.header.qd_count = self.questions.len() as u16;
        self.header.an_count = self.answers.len() as u16;
        self.header.ns_count = self.authorities.len() as u16;
        self.header.ar_count = self.additionals.len() as u16;

        self.header.write(buffer)?;

        for question in &self.questions {
            question.write(buffer)?;
        }
        for rec in &self.answers {
            rec.write(buffer)?;
        }
        for rec in &self.authorities {
            rec.write(buffer)?;
        }
        for rec in &self.additionals {
            rec.write(buffer)?;
        }

        Ok(())
    }
}
