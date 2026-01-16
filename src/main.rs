mod utils;

use std::net::TcpListener;
use std::io::prelude::*;
use std::net::TcpStream;
use std::thread;
use std::sync::Arc;
use std::sync::Mutex;

fn transfer(src: Arc<Mutex<TcpStream>>, dest: Arc<Mutex<TcpStream>>) {

    let mut src_buffer = [0u8; 1024];

    loop {
        {
            println!("[TRANS] Attempting to lock src");
            let rdat_len = src.lock().unwrap().read(&mut src_buffer).unwrap();

            println!("[TRANS] Src data read");

            let mut _tmpsrc = dest.lock().unwrap().write_all(&src_buffer[..rdat_len]).unwrap(); 

            println!("[TRANS] Data sent to dest");
        }
    }

}

fn handle_client(mut client: Arc<Mutex<TcpStream>>) {
    let mut client_buffer = [0u8; 1024];
    

    let mut server = Arc::new(
        Mutex::new(
            TcpStream::connect("127.0.0.1:25565").unwrap()
        )
    );

    println!("[HANCL] Server connection done");

    let mut rdat_len: usize;

    loop {
        {
            rdat_len = client.lock().unwrap().read(&mut client_buffer).unwrap();

            println!("[HANCL] client data read complete");
        }

        let mut thread_client = Arc::clone(&client);
        let mut thread_server = Arc::clone(&server);

        thread::spawn(move || {
            transfer(thread_server, thread_client);
        });
        println!("[HANCL] transfer thread run sucess");
        
        {
            let mut _tmpsv = server.lock().unwrap(); 
            _tmpsv.write_all(&client_buffer[..rdat_len]).unwrap();
            println!("[HANCL] Sending initial client data to server");
        }
    }
}

fn main() {
    let tcp_server = TcpListener::bind("127.0.0.1:3000").unwrap();
    
    println!("[MAIN] running server on 127.0.0.1:3000");
    for client in tcp_server.incoming() {
        let client_clone = Arc::new(
            Mutex::new(client.unwrap())
        );

        println!("[MAIN] new client detected, running handling thread");

        thread::spawn(move || {
            handle_client(client_clone);
        });

        println!("[MAIN] client thread run sucessfully");
    }
}


