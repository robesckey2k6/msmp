
mod utils;
mod models;

use models::server::ServerConfig;
use utils::packet::parse_handshake_data;

use std::net::TcpListener;
use std::io::prelude::*;
use std::net::TcpStream;
use std::thread;

use dotenv::dotenv;
use std::fs;


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

async fn handle_client(mut client: TcpStream, servers: Vec<ServerConfig>) {

    let mut client_buffer = [0u8; 1024];
    let mut rdat_len: usize;

    // Reading handshake data
    rdat_len = client.read(&mut client_buffer).unwrap();
    
    let (_, _, _, svadr, _, intent) = parse_handshake_data(&client_buffer);

    
    // Splitting sv adr for subdomain 
    let parts: Vec<&str> = svadr.split('.').collect();
     
    let mut out_port: Option<i32> =  None;

    for server in servers {
        if server.name == parts[0] {
            out_port = Some(server.port);
        }
    }
    
    let port: i32 = out_port.unwrap();
     
        // Create server connection
    let mut server = TcpStream::connect(format!("127.0.0.1:{}", port)).unwrap();

    println!("Connection established {} -> 127.0.0.1:{}", svadr, port);

    server.write_all(&client_buffer[..rdat_len]).unwrap();

    let server_cloned = server.try_clone().unwrap();
    let client_cloned= client.try_clone().unwrap();
    

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


#[tokio::main]
async fn main() {
    dotenv().ok();
    
    let contents = fs::read_to_string("servers")
        .expect("failed to read 'servers' file");

    let servers: Vec<ServerConfig> = contents
        .lines()
        .filter(
            |line| !line.trim().is_empty()
        )
        .map(
            |line| {
                let mut parts = line.splitn(2, ',');
                let name = parts.next().expect("Missing server name").trim().to_string();
                let port = parts.next().expect("Missing port").trim().parse::<i32>().expect("Invalid port");
                ServerConfig {
                    name,
                    port
                }
        })
        .collect();

    let tcp_server = TcpListener::bind("127.0.0.1:2001").unwrap();
    
    println!("[MAIN] MSMP running on 127.0.0.1:2001");
    
    for client in tcp_server.incoming() {
        let servers_clone  = servers.clone();

        tokio::spawn(async move {
            handle_client(client.unwrap(), servers_clone).await;
        });
    }
}


