
mod utils;
use utils::packet::parse_handshake_data;

use std::net::TcpListener;
use std::io::prelude::*;
use std::net::TcpStream;
use std::thread;

use redis::{Client, Commands};



fn transfer(mut src: TcpStream,mut dest: TcpStream) {
    let mut src_buffer = [0u8; 1024];
    
    loop {
           
            let rdat_len = src.read(&mut src_buffer).unwrap();
            
            if rdat_len == 0 {
                return;
            }


            let mut _tmpsrc = dest.write_all(&src_buffer[..rdat_len]).unwrap(); 
    }
}

fn handle_client(mut client: TcpStream, mut redis_instance: redis::Connection) {

    let mut client_buffer = [0u8; 1024];
    let mut rdat_len: usize;

    // Reading handshake data
    rdat_len = client.read(&mut client_buffer).unwrap();
    
    let (_, _, _, svadr) = utils::packet::parse_handshake_data(&client_buffer);
    
    // TODO setup database configuration for address -> port determination
    let parts: Vec<&str> = svadr.split('.').collect();

    
    let key_exist: bool = redis_instance.exists(parts[0]).unwrap();
    
    let port: String;

    if key_exist {
        port = redis_instance.get(parts[0]).unwrap();
    }
    else {
        client.shutdown(std::net::Shutdown::Both).unwrap();
        return;
    }

    // Create server connection
    let mut server = TcpStream::connect(format!("127.0.0.1:{}", port)).unwrap();

    println!("Connection established {} -> 127.0.0.1:{}", svadr, port);
    drop(redis_instance);

    server.write_all(&client_buffer[..rdat_len]).unwrap();

    let mut server_cloned = server.try_clone().unwrap();
    let mut client_cloned= client.try_clone().unwrap();
    

    // Spawning server -> client thread
    thread::spawn( move || {
        transfer(server_cloned, client_cloned);
    });


    // Sending data from client -> server
    loop {

        rdat_len = client.read(&mut client_buffer).unwrap();

        if rdat_len == 0 {
            return;
        }


        server.write_all(&client_buffer[..rdat_len]).unwrap();
    }
}

fn main() {
    let redis_client = redis::Client::open("redis://127.0.0.1:6379").unwrap();


    let tcp_server = TcpListener::bind("127.0.0.1:3000").unwrap();
    
    println!("[MAIN] Multiplexer on 127.0.0.1:3000");


    for client in tcp_server.incoming() {

        let mut connection = redis_client.get_connection().unwrap();
        thread::spawn(move || {
            handle_client(client.unwrap(), connection);
        });
    }
}


