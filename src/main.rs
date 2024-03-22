mod dns;
mod parser;

use dns::DnsPacket;
use dns_server::Result;

fn main() -> Result<()> {
    let file_name: &str = "res/response_packet.txt";
    let dns = DnsPacket::new(file_name);
    println!("{:#?}", dns);
    Ok(())
}
