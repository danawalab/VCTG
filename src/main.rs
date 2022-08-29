use std::borrow::Cow;
use std::io::Read;
use std::net::{TcpListener, TcpStream};

pub fn listening_for_loop() {
    let listener = TcpListener::bind("192.168.0.71:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

pub fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    // let mut buffer = String::new();

    stream.read(&mut buffer).unwrap();
    let request = std::str::from_utf8(&buffer[..]).unwrap();
    let mut splited = request.split("|");

    // let request: Vec<char> = String::from_utf8_lossy(&buffer[..]).to_mut().chars().collect();
    // println!("{} {}", splited.next().unwrap(), splited.next().unwrap());
    handle_request(splited.next().unwrap(), splited.next().unwrap());
}

pub fn handle_request(route: &str, user_id: &str){
    println!("{} {}", route, user_id);
    match route {
        "register" => println!("register {}", user_id),
        "mining" => println!("mining {}", user_id),
        "wallet" => println!("wallet {}", user_id),
        _ => {}
    }
}

fn main() {
    listening_for_loop()
}
