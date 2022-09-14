use std::net::{TcpListener, TcpStream};
use std::io::{Write};
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let server = "127.0.0.1:7878";
    let listener = TcpListener::bind(server)?;

    // accept connections and process them serially
    println!("Server is running on {}...", server);
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}

fn handle_client(mut stream: TcpStream) {
    //get request and dump
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer[..]);
    // println!("Received HTTP Request...\n{}", request);

    let mut first_line = request.split("\r\n").next().unwrap().replace("GET /", "");
    println!("first_line1 : {}", first_line);
    let first_line = first_line.replace(" HTTP/1.1", "");
    println!("first_line2 : {}", first_line);
    let data = first_line.replace("/", "|");
    println!("first_line3 : {}", first_line);
    // data : action|필요데이터1|필요데이터2|필요데이터3|...

    // let content = first_line.split("/");
    //
    // let body = content.join("|");
    //
    // println!("body : {}", body);
    // println!("{}", content[1]);
    // println!("{}", content[2]);
    // println!("{}", content[3]);

    // 1. request -> 서버에 넘겨줌

    // 2. 서버에서 제대로 되었는지 확인 -> 이부분이 어떻게 할 것인지를 모르겠네여

    // 3. 서버에서 확인된 데이터를 토대로 response 제작하여 화면에 띄움.
    let action = client_action(buffer);
    println!("{}", action);

    // let request = make_send_request_data(action, request);

    show_page(buffer, stream);
}

pub fn client_action(buffer: [u8; 1024]) -> &'static str {
    let action;
    if buffer.starts_with(b"GET / HTTP/1.1\r\n") {
        action = "main";
    } else if buffer.starts_with(b"GET /register") {
        action = "register";
    } else if buffer.starts_with(b"GET /mining") {
        action = "mining";
    } else if buffer.starts_with(b"GET /wallet") {
        action = "wallet";
    } else {
        action = "None";
    }
    action
}

pub fn make_send_request_data(action: &str, request: String) -> &'static str {
    let send_request_data = "1";
    if action == "register" {
        let first_line = request.split("\r\n").next().unwrap();

    }
    send_request_data
}

pub fn show_page (buffer: [u8; 1024], mut stream: TcpStream) {
    let response;
    if buffer.starts_with(b"GET / HTTP/1.1\r\n") {
        //send hello world response
        response = "HTTP/1.1 200 OK\
        \r\nContent-Type: text/html; charset=iso-8859-1\
        \r\nConnection: close\
        \r\nContent-Length: 14\
        \r\n\r\nHello world!\r\n";
    } else if buffer.starts_with(b"GET /user/ HTTP/1.1\r\n") {
        response = "HTTP/1.1 200 OK\
        \r\nContent-Type: text/html; charset=iso-8859-1\
        \r\nConnection: close\
        \r\nContent-Length: 14\
        \r\n\r\n user\r\n";
    } else {
        response = "HTTP/1.1 200 OK\
        \r\nContent-Type: text/html; charset=iso-8859-1\
        \r\nConnection: close\
        \r\nContent-Length: 30\
        \r\n\r\n 404 Page Not Found\r\n";
    }
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}