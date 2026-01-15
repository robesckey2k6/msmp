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

    let mut server_buffer = [0u8; 1024];
    loop {
        {   
            let data_read = server.lock().unwrap().read(&mut server_buffer).unwrap();
            println!("DATA RECV");
            {
            let mut tmp_client = client.lock().unwrap();
            tmp_client.write_all(&server_buffer[..data_read]).unwrap(); // code stuck here? could client be
                                                              // locked from somewhere else?
            }
            println!("DATA SENT TO CLIENT");
        }
    }
}
fn handle_connection( mut stream: Arc<Mutex<TcpStream>>) {

    let mut current_client_state: ClientState = ClientState::HANDSHAKE;

    
    // Server bound buffer
    let mut buffer = [0u8; 1024];
    let mut original_server_stream = Arc::new(Mutex::new(TcpStream::connect("127.0.0.1:25565").unwrap()));
    loop {
        
            // Getting data from stream
           let data_read =  stream.lock().unwrap().read(&mut buffer).unwrap(); // Getting data from stream
        

        

        if current_client_state == ClientState::HANDSHAKE {
            let mut index = 0;

            let mut packet_length: u64 = 0x00;
            let mut packet_id: u64 = 0x00;
            let mut protocol_version: u64 = 0x00;


            (packet_length, index) = utils::varint::read_varint(&buffer, None).unwrap();
            (packet_id, index) = utils::varint::read_varint(&buffer, Some(index)).unwrap();
            (protocol_version, index) = utils::varint::read_varint(&buffer, Some(index)).unwrap();

            println!("packet_length: {} packet_id: {} protocol_version: {}", packet_length, packet_id, protocol_version);

            let mut server_stream_c = Arc::clone(&original_server_stream);
            let mut client_stream_c = Arc::clone(&stream);
            
            { 
                let mut tmp_stream = server_stream_c.lock().unwrap();
                tmp_stream.write_all(&buffer[..data_read]).unwrap();
            }

            println!("BACK THREAD LOADing");
            let backward_thread_handle = thread::spawn(move || {
                backward(server_stream_c, client_stream_c);
            });
            
            println!("BACK THREAD LOADED");
            current_client_state =  ClientState::STATUS;
        }
        else {
            println!("SENDING DATA TO SERVER");
            let mut server_stream_c = Arc::clone(&original_server_stream);

            {
                let mut tmp_ss = server_stream_c.lock().unwrap();
                tmp_ss.write_all(&buffer[..data_read]).unwrap();
            }
            println!("DATA SENT TO SERVER");
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("[INFO] Multiplexer started at 127.0.0.1:3000");

    for stream in listener.incoming() {
        
        // Cloning stream variable
        let orginal_stream = Arc::new(Mutex::new(stream.unwrap()));
        let clone_stream = Arc::clone(&orginal_stream);

        println!("[INFO] Connection established");
        handle_connection(clone_stream);
    }
}
