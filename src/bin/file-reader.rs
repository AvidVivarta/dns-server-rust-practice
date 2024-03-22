use lib::dns::DnsPacket;
use lib::Result;

fn main() -> Result<()> {
    let file_name: &str = "res/response_packet.txt";
    let dns = DnsPacket::read(file_name);
    println!("{:#?}", dns);
    Ok(())
}
