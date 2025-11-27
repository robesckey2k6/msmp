mod utils;

use std::net::TcpListener;
use std::io::prelude::*;
use std::net::TcpStream;

fn handle_connection( mut stream: TcpStream) {
    let mut buffer = [0u8; 1024];
    let mut index = 0;
    let mut data = 0x00;

    stream.read(&mut buffer).unwrap();

    (data, index) = utils::varint::read_varint(&buffer, None).unwrap();

    (data, index) = utils::varint::read_varint(&buffer, Some(index)).unwrap();

    println!("data: {:02x}", data);
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
