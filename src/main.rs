use std::fs::File;
use std::io::Read;
use dns_server::{BytePacketBuffer, DnsPacket};
use std::path::Path;

fn main() {
    let path = Path::new("response_packet.txt");
    let mut f = File::open(&path).unwrap();
    let mut buffer = BytePacketBuffer::new();
    f.read(&mut buffer.buf).unwrap();

    let packet = DnsPacket::from_buffer(&mut buffer).unwrap();
    println!("{:?}", packet.header);

    for q in packet.questions {
        println!("{:?}", q);
    }
    for rec in packet.answers {
        println!("{:?}", rec);
    }
    for rec in packet.authorities {
        println!("{:?}", rec);
    }
    for rec in packet.resources {
        println!("{:?}", rec);
    }
}