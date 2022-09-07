use std::net::{TcpListener, TcpStream};
use std::io::{Write};
use std::io::prelude::*;

fn handle_client(mut stream: TcpStream) {
    //get request and dump
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Received HTTP Request...\n{}", request);

    let first_line = request.split("\r\n").next().unwrap();
    let splited_first_line = first_line.split(" ");
    let all_content = splited_first_line.collect::<Vec<&str>>()[1];
    let content = all_content.split("/").collect::<Vec<&str>>();
    let body = content.join("|");

    println!("{}", body);
    println!("{}", content[1]);
    println!("{}", content[2]);
    println!("{}", content[3]);

    // 1. request -> 서버에 넘겨줌

    // 2. 서버에서 제대로 되었는지 확인 -> 이부분이 어떻게 할 것인지를 모르겠네여

    // 3. 서버에서 확인된 데이터를 토대로 response 제작하여 화면에 띄움.


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
