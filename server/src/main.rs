use std::net;
use std::io::{Read, Write};
use std::fs;
use std::process;

const BUILD: &str = "cd ./client; cargo build --release; cd ..; mv ./client/target/wasm32-unknown-unknown/release/dungeons.wasm ./content/client.wasm";

fn main() {
    let socket = net::TcpListener::bind("127.0.0.1:8080").unwrap();
    for con in socket.incoming() {
        let mut con = con.unwrap();
        let mut buf = [0u8; 2048];
        con.read(&mut buf).unwrap();
        let msg = String::from_utf8_lossy(&buf);
        
        if !msg.is_empty() {
            let mut path = msg.split(' ').collect::<Vec<&str>>()[1];
            if path == "/" { path = "/main.html" }
            if path == "/client.wasm" {
                process::Command::new("/bin/bash")
                    .arg("-c")
                    .arg(format!("{BUILD}"))
                    .output()
                    .unwrap();
            }
            
            if let Ok(file) = fs::read(format!("./content{path}")) {
                let content_type = match path.split('.').collect::<Vec<&str>>()[1] {
                    "html" => "text/html",
                    "css"  => "text/css",
                    "js"   => "text/javascript",
                    "wasm" => "application/wasm",
                    _ => panic!()
                };
                
                con.write_all(format!("HTTP/1.1 200 OK\nContent-Type: {content_type}\r\n\r\n").as_bytes()).unwrap();
                con.write(file.as_slice()).unwrap();
                con.flush().unwrap();
            } else {
                eprintln!("Invalid file: {path}")
            }
        }
    }
}
