use std::net::UdpSocket;

use dns_server::{
    dns::{DnsPacket, DnsQuestion, QueryType},
    parser::DnsBytePacketBuffer,
    Result,
};

fn main() -> Result<()> {
    // Perform an A query for google.com
    let query_name: &str = "google.com";
    let query_type: QueryType = QueryType::A;
    // Using googles public DNS server
    let server: (&str, u16) = ("8.8.8.8", 53);

    // Build our query packet. It's important that we remember to set the
    // `recursion_desired` flag. As noted earlier, the packet id is arbitrary.
    let mut packet: DnsPacket = DnsPacket::new();
    packet.header.id = 6666;
    packet.header.qd_count = 1;
    packet.header.rd = true;
    packet
        .questions
        .push(DnsQuestion::new(query_name.to_string(), query_type));

    // Use our new write method to write the packet to a buffer...
    let mut req_buffer = DnsBytePacketBuffer::new();
    packet.write(&mut req_buffer)?;

    // Bind a UDP socket to an arbitrary port
    let socket: UdpSocket = UdpSocket::bind(("0.0.0.0", 43210)).map_err(|err| {
        eprintln!("ERROR: Unable to open UDP socket: {}", err);
    })?;

    // ...and send it off to the server using our socket:
    socket
        .send_to(
            &req_buffer
            .get_buf_range(0..req_buffer.get_pos())
            .expect("Unable to read given range of buffer"),
            server,
            )
        .map_err(|err| {
            eprintln!("ERROR: error while standing data: {}", err);
        })?;
    let mut res_buffer = DnsBytePacketBuffer::new();
    let (bytes_read, socket_addr) = socket
        .recv_from(&mut res_buffer.get_buf())
        .map_err(|err|{
        eprintln!("ERROR: Unable to recieve data from socket:{}", err);
    })?;
    res_buffer.set_bytes_read(bytes_read);
    println!("INFO: Bytes Read: {}, socket address: {}", bytes_read, socket_addr);

    // As per the previous section, `DnsPacket::from_buffer()` is then used to
    // actually parse the packet after which we can print the response.
    let res_packet = DnsPacket::from_buffer(&mut res_buffer)?;
    println!("{:#?}", res_packet.header);

    for q in res_packet.questions {
        println!("{:#?}", q);
    }
    for rec in res_packet.answers {
        println!("{:#?}", rec);
    }
    for rec in res_packet.authorities {
        println!("{:#?}", rec);
    }
    for rec in res_packet.additionals{
        println!("{:#?}", rec);
    }

    Ok(())
}
