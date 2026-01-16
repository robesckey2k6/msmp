mod utils;

use std::net::TcpListener;
use std::io::prelude::*;
use std::net::TcpStream;
use std::thread;
use std::sync::Arc;
use std::sync::Mutex;

fn transfer(mut src: TcpStream,mut dest: TcpStream) {

    let mut src_buffer = [0u8; 1024];

    loop {
        {
            println!("[TRANS] Attempting to lock src");
            let rdat_len = src.read(&mut src_buffer).unwrap();

            println!("[TRANS] Src data read");

            let mut _tmpsrc = dest.write_all(&src_buffer[..rdat_len]).unwrap(); 

            println!("[TRANS] Data sent to dest");
        }
    }

}
fn handle_client(mut client: TcpStream) {
    let mut client_buffer = [0u8; 1024];
    

    let mut server = TcpStream::connect("127.0.0.1:25565").unwrap();

    println!("[HANCL] Server connection done");
    let mut server_cloned = server.try_clone().unwrap();
    let mut client_cloned= client.try_clone().unwrap();
    thread::spawn( move || {
        transfer(server_cloned, client_cloned);
    });

    let mut rdat_len: usize;

    loop {
        {
            rdat_len = client.read(&mut client_buffer).unwrap();
            println!("[HANCL] 1 client data read complete");
        }
        
        println!("[HANCL] Locking server.. ");
        
        {
            server.write_all(&client_buffer[..rdat_len]).unwrap();
            println!("[HANCL] 2 Sent initial client data to server");
        }

        println!("[HANCL] Unlocked server");

    }
}

fn main() {
    let tcp_server = TcpListener::bind("127.0.0.1:3000").unwrap();
    
    println!("[MAIN] running server on 127.0.0.1:3000");
    for client in tcp_server.incoming() {

        println!("[MAIN] new client detected, running handling thread");

        thread::spawn(move || {
            handle_client(client.unwrap().try_clone().unwrap());
        });

        println!("[MAIN] client thread run sucessfully");
    }
}


