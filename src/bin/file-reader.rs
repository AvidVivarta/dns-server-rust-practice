use dns_server::dns::DnsPacket;
use dns_server::Result;

fn main() -> Result<()> {
    let file_name: &str = "res/response_packet.txt";
    let dns = DnsPacket::read(file_name);
    println!("{:#?}", dns);
    Ok(())
}
