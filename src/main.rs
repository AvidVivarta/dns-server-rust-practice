mod dns;
mod parser;
use anyhow::Result;
use dns::DnsPacket;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_name: &str = "res/response_packet.txt";
    let dns = DnsPacket::new(file_name);
    println!("{:#?}", dns);
    Ok(())
}
