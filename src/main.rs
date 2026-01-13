mod utils;

use std::net::TcpListener;
use std::io::prelude::*;
use std::net::TcpStream;

fn handle_connection( mut stream: TcpStream) {
    let mut buffer = [0u8; 1024];
    let mut index = 0;

    let mut packet_length: u16 = 0x00;
    let mut packet_id: u16 = 0x00;
    let mut protocol_version: u16 = 0x00;
    
    stream.read(&mut buffer).unwrap(); // Getting data from stream
    
    (packet_length, index) = utils::varint::read_varint(&buffer, None).unwrap();
    (packet_id, index) = utils::varint::read_varint(&buffer, Some(index)).unwrap();
    (protocol_version, index) = utils::varint::read_varint(&buffer, Some(index)).unwrap();
    
    println!("packet_length: {} packet_id: {} protocol_version: {}", packet_length, packet_id, protocol_version);
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("[INFO] Multiplexer started at 127.0.0.1:3000");
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("[INFO] Connection established");
        handle_connection(stream);
    }
}
