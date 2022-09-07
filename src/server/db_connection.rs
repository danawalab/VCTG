use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use mysql::*;
use mysql::prelude::*;


pub fn listening_for_loop() {
    let listener = TcpListener::bind("localhost:7878").unwrap();

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
    handle_request(stream, splited.next().unwrap(), splited.next().unwrap());
}

pub fn handle_request(mut stream: TcpStream, route: &str, user_id: &str){
    // return 값은 OK 혹은 FAIL로만 준다
    // 각 값은 |로 구분 한다
    match route {
        "register" => {
            let response = "OK|register done\r\n\r\n";

            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        },
        "mining" => {
            let response = "OK|mining done\r\n\r\n";

            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        },
        "wallet" => {
            let response = "OK|wallet info returned\r\n\r\n";

            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        },
        _ => {
            let response = "FAIL|ROUTE_PATH is not found\r\n\r\n";

            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
    }
}

pub fn connect_to_db(){
    let url = "mysql://root:root@localhost:3306/vctg";

    let pool = Pool::new(url).expect("연결 실패");

    let mut conn = pool.get_conn().expect("커넥션 가져오기 실패");

    let mut result = conn.query_iter("SELECT * from USERS LIMIT 10")
        .expect("select 조회 실패");

    while let Some(result_set) = result.iter() {
        println!("Result set columns: {:?}", result_set.columns());
    }

    println!("connected!")
}