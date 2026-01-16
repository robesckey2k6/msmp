mod utils;

use std::net::TcpListener;
use std::io::prelude::*;
use std::net::TcpStream;
use std::thread;
use std::sync::Arc;
use std::sync::Mutex;


#[derive(PartialEq)]
enum ClientState{
    HANDSHAKE,
    STATUS,
    LOGIN,
    OTHER
}



fn transfer(mut src: TcpStream,mut dest: TcpStream) {
    let mut src_buffer = [0u8; 1024];
    loop {
        {
            let rdat_len = src.read(&mut src_buffer).unwrap();
            let mut _tmpsrc = dest.write_all(&src_buffer[..rdat_len]).unwrap(); 

        }
    }

}
fn handle_client(mut client: TcpStream) {

    let mut client_buffer = [0u8; 1024];
    let mut rdat_len: usize;

    let mut cclient_state: ClientState = ClientState::HANDSHAKE;
    

    rdat_len = client.read(&mut client_buffer).unwrap();
    
    let mut index = 0;
    
    let mut packet_length: u64;
    let mut packet_id: u64;
    let mut protocol_version: u64;
    let mut address_length: u64;
    
    (packet_length, index) = utils::varint::read_varint(&client_buffer, None).unwrap();
    (packet_id, index) = utils::varint::read_varint(&client_buffer, Some(index)).unwrap();
    (protocol_version, index) = utils::varint::read_varint(&client_buffer, Some(index)).unwrap();
    (address_length, index) = utils::varint::read_varint(&client_buffer, Some(index)).unwrap();

    let end_index = (index + (address_length as i32)) as usize;
    let us_index = index as usize;

    let raw_svaddr = client_buffer[us_index .. end_index].to_vec();
    let server_address = String::from_utf8(raw_svaddr).unwrap();
    


    let mut server = TcpStream::connect("127.0.0.1:25565").unwrap();

    server.write_all(&client_buffer[..rdat_len]).unwrap();
    println!("{}", server_address);

    let mut server_cloned = server.try_clone().unwrap();
    let mut client_cloned= client.try_clone().unwrap();

    thread::spawn( move || {
        transfer(server_cloned, client_cloned);
    });


    loop {
        rdat_len = client.read(&mut client_buffer).unwrap();
        server.write_all(&client_buffer[..rdat_len]).unwrap();
    }
}

fn main() {
    let tcp_server = TcpListener::bind("127.0.0.1:3000").unwrap();
    
    println!("[MAIN] running server on 127.0.0.1:3000");
    for client in tcp_server.incoming() {

        println!("[MAIN] new client detected, running handling thread");
        thread::spawn(move || {
            handle_client(client.unwrap());
        });
    }
}


