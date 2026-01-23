
mod utils;
mod db;
mod models;

use utils::packet::parse_handshake_data;

use std::net::TcpListener;
use std::io::prelude::*;
use std::net::TcpStream;
use std::thread;

use dotenv::dotenv;

use sea_orm::{DatabaseConnection,EntityTrait};
use models::server;

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

async fn handle_client(mut client: TcpStream, db: DatabaseConnection) {
    
    println!("DONE");
    let mut client_buffer = [0u8; 1024];
    let mut rdat_len: usize;

    // Reading handshake data
    rdat_len = client.read(&mut client_buffer).unwrap();
    
    let (_, _, _, svadr) = parse_handshake_data(&client_buffer);
    
    // TODO setup database configuration for address -> port determination
    let parts: Vec<&str> = svadr.split('.').collect();
    
    println!("{}",parts[0]);

    let sv = server::Entity::find_by_id(parts[0]).one(&db).await.unwrap().unwrap();

    let port: i32 = sv.sport.unwrap();
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

    let db_conn = db::init_db().await;
     let tcp_server = TcpListener::bind("127.0.0.1:2001").unwrap();
    
    println!("[MAIN] Multiplexer on 127.0.0.1:2001");


    for client in tcp_server.incoming() {
        let db_clone= db_conn.clone();
        println!("connected");

        tokio::spawn(async move {
            handle_client(client.unwrap(), db_clone).await;
        });
    }
}


