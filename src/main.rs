mod dns;
use dns::DnsPacket;

fn main() {
    println!("Hello, world!");
    let dns = DnsPacket::default();
    println!("{:#?}", dns);
}
