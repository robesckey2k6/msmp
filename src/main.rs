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
fn backward(server: Arc<Mutex<TcpStream>>, client: Arc<Mutex<TcpStream>>) {
    loop {
        let mut server_buffer = [0u8; 1024];
        {
            server.lock().unwrap().read(&mut server_buffer).unwrap();
            println!("DATA RECV");
            client.lock().unwrap().write_all(&server_buffer);
            println!("DATA SENT TO CLIENT");
        }
    }
}
fn handle_connection( mut stream: Arc<Mutex<TcpStream>>) {

    let mut current_client_state: ClientState= ClientState::HANDSHAKE;

    
    loop {
        let mut buffer = [0u8; 1024];
        stream.lock().unwrap().read(&mut buffer).unwrap(); // Getting data from stream
        let mut server_stream = Arc::new(Mutex::new(TcpStream::connect("127.0.0.1:25565").unwrap()));

        if current_client_state == ClientState::HANDSHAKE {
            let mut index = 0;

            let mut packet_length: u64 = 0x00;
            let mut packet_id: u64 = 0x00;
            let mut protocol_version: u64 = 0x00;


            (packet_length, index) = utils::varint::read_varint(&buffer, None).unwrap();
            (packet_id, index) = utils::varint::read_varint(&buffer, Some(index)).unwrap();
            (protocol_version, index) = utils::varint::read_varint(&buffer, Some(index)).unwrap();

            println!("packet_length: {} packet_id: {} protocol_version: {}", packet_length, packet_id, protocol_version);

            let mut server_stream_c = Arc::clone(&server_stream);
            let mut client_stream_c = Arc::clone(&stream);
            
            
            server_stream_c.lock().unwrap().write_all(&buffer).unwrap();

            println!("BACK THREAD LOADing");
            let backward_thread_handle = thread::spawn(move || {
                backward(server_stream_c, client_stream_c);
            });
            
            println!("BACK THREAD LOADED");
            current_client_state =  ClientState::STATUS;
        }
        else {
            println!("SENDING DATA TO SERVER");
            let mut server_stream_c = Arc::clone(&server_stream);
            server_stream_c.lock().unwrap().write_all(&buffer).unwrap();
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("[INFO] Multiplexer started at 127.0.0.1:3000");
    for stream in listener.incoming() {
        let client_stream = Arc::new(Mutex::new(stream.unwrap()));
        let clone_stream = Arc::clone(&client_stream);
        println!("[INFO] Connection established");
        handle_connection(clone_stream);
    }
}
