use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

use crate::database::database::DataAccessStruct;
use crate::structs::user::User;

mod common;
mod database;
mod structs;

const IP_ADDRESS: &str = "localhost:7878";

pub fn listening_for_loop() {
    let listener = TcpListener::bind(IP_ADDRESS).unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

pub fn handle_connection(mut stream: TcpStream) {
    let mut readBuffer = [0; 512];

    stream.read(&mut readBuffer).unwrap();
    // println!("Request: {}", String::from_utf8_lossy(&readBuffer[..]));

    let readBufferStr = std::str::from_utf8(&readBuffer[..]).unwrap();
    let mut splited = readBufferStr.split("|");

    handle_request(stream, splited.next().unwrap(), splited.next().unwrap());
}

pub fn handle_request(mut stream: TcpStream, route: &str, user_name: &str) {
    // return 값은 OK 혹은 FAIL로만 준다
    // 각 값은 |로 구분 한다
    println!("route: {} / arg: {}", route, user_name);

    match route {
        "register" => {
            register_user_db(user_name);
            let response = b"OK|register done|\r\n";
            stream.write(response);
        }
        "signin" => {
            let writeBuffer = b"OK|signin done|\r\n";
            stream.write(writeBuffer);
        }

        "wallet" => {
            let writeBuffer = b"OK|wallet info returned|\r\n";
            stream.write(writeBuffer);
        }
        "mining" => {
            let writeBuffer = b"OK|mining done|\r\n";
            stream.write(writeBuffer);
        }
        "sell" => {
            let writeBuffer = b"OK|selling done|\r\n";
            stream.write(writeBuffer);
        }
        "buy" => {
            let writeBuffer = b"OK|buying done|\r\n";
            stream.write(writeBuffer);
        }
        _ => {
            let writeBuffer = b"FAIL|ROUTE_PATH is not found|\r\n";
            stream.write(writeBuffer);
        }
    }
}

pub fn register_user_db(user_name: &str) {
    let mut dao = DataAccessStruct {
        id: String::from("user"),
        password: String::from("password"),
        host: String::from("localhost"),
        port: String::from("3306"),
        database: String::from("VCTG"),
    };
    let mut conn = dao.do_connect();

    let mut exists_user_name_query: String = String::from("SELECT EXISTS(SELECT user_name FROM USERS) FROM USERS WHERE user_name = '");
    exists_user_name_query.push_str(user_name.trim());
    exists_user_name_query.push_str("';");
    println!("{}", exists_user_name_query);

    let is_exists = dao.query(&mut conn, exists_user_name_query.as_str());

    println!("{:?}", is_exists);

    let user = User::new(user_name.to_string());
    let mut user_query = String::from("INSERT INTO USERS(user_name, point, wallet_address) VALUES ('");

    user_query.push_str(user_name.trim());
    user_query.push_str("', 100, '");
    user_query.push_str(user.wallet_address.as_str());
    user_query.push_str("');");

    println!("{}", user_query);

    let mut wallet_query = String::from("INSERT INTO WALLETS(wallet_address, user_id) VALUES('");

    wallet_query.push_str(user.wallet_address.as_str());
    wallet_query.push_str("', ");
    wallet_query.push_str(user.user_id.to_string().as_str());
    wallet_query.push_str(");");

    println!("{}", wallet_query);
    let vector = dao.query(&mut conn, user_query.as_str());
    dao.query(&mut conn, wallet_query.as_str());

    for row in vector {
        println!("{} {} {} {}", row.0, row.1, row.2, row.3);
    }
}


fn main() {
    // register_user_db("test_user");
    // connect_to_db();
    listening_for_loop();
}