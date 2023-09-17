#[derive(Debug,Default)]
pub struct DnsHeader {
    pub id: u16, // 16bits packet identifier
    pub qr: bool, // 1bit query response
    pub op_code: u16, // 4bits operation code
    pub aa: bool, // 1bit authoritative answer
    pub tc: bool, // 1bit truncated message
    pub rd: bool, // 1bit recursion desired
    pub ra: bool, // 1bit recursion available
    pub z: u8, // 3bits reserved
    pub r_code: u16, // 4bits response code
    pub qd_count: u16, // 16bits question count
    pub an_count: u16, // 16bits answer count
    pub ns_count: u16, // 16bits authority count 
    pub ar_count: u16, // 16bits additional count
}

// impl Default for DnsHeader { 
//     fn default() -> Self { 
//         DnsHeader {
//             id: 0, 
//             qr: false, 
//             op_code: 0, 
//             aa: false, 
//             tc: false, 
//             rd: false, 
//             ra: false, 
//             z: 0, 
//             r_code: 0, 
//             qd_count: 0, 
//             an_count: 0, 
//             ns_count:0 ,
//             ar_count: 0
//         }
//     }
// }

#[derive(Debug, Default)]
pub struct DnsQuestion;
#[derive(Debug, Default)]
pub struct DnsAnswer;
#[derive(Debug, Default)]
pub struct DnsAuthority;
#[derive(Debug, Default)]
pub struct DnsAdditional;

#[derive(Debug, Default)]
pub struct DnsPacket {
    pub header: DnsHeader,
    pub question: Vec<DnsQuestion>,
    pub answer: Vec<DnsAnswer>,
    pub authority: Vec<DnsAuthority>,
    pub additional: Vec<DnsAdditional>,
}


