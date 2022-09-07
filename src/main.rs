use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

use mysql::*;
use mysql::prelude::*;
use crate::database::database::DataAccessStruct;

mod database;

// pub fn listening_for_loop() {
//     let listener = TcpListener::bind("192.168.0.71:7878").unwrap();
//
//     for stream in listener.incoming() {
//         let stream = stream.unwrap();
//
//         thread::spawn(|| {
//             handle_connection(stream);
//         });
//     }
// }
//
// pub fn handle_connection(mut stream: TcpStream) {
//     let mut buffer = [0; 1024];
//     // let mut buffer = String::new();
//
//     stream.read(&mut buffer).unwrap();
//     let request = std::str::from_utf8(&buffer[..]).unwrap();
//     let mut splited = request.split("|");
//
//     // let request: Vec<char> = String::from_utf8_lossy(&buffer[..]).to_mut().chars().collect();
//     // println!("{} {}", splited.next().unwrap(), splited.next().unwrap());
//     handle_request(stream, splited.next().unwrap(), splited.next().unwrap());
// }
//
// pub fn handle_request(mut stream: TcpStream, route: &str, user_name: &str){
//     // return 값은 OK 혹은 FAIL로만 준다
//     // 각 값은 |로 구분 한다
//     match route {
//         "register" => {
//             let response = "OK|register done|\r\n";
//
//             stream.write(response.as_bytes()).unwrap();
//             stream.flush().unwrap();
//         },
//         "mining" => {
//             let response = "OK|mining done|\r\n";
//
//             stream.write(response.as_bytes()).unwrap();
//             stream.flush().unwrap();
//         },
//         "wallet" => {
//             let response = "OK|wallet info returned|\r\n";
//
//             stream.write(response.as_bytes()).unwrap();
//             stream.flush().unwrap();
//         },
//         _ => {
//             let response = "FAIL|ROUTE_PATH is not found|\r\n";
//
//             stream.write(response.as_bytes()).unwrap();
//             stream.flush().unwrap();
//         }
//     }
// }
//
// pub fn connect_to_db(){
//     let url = "mysql://root:root@localhost:3306/test_db";
//     let pool = Pool::new(url).expect("연결 실패");
//
//     let mut conn = pool.get_conn().expect("커넥션 가져오기 실패");
//
//     let mut result = conn.query_iter("SELECT * from USERS LIMIT 10")
//         .expect("select 조회 실패");
//
//     while let Some(result_set) = result.iter() {
//         println!("Result set columns: {:?}", result_set.columns());
//     }
//
//     println!("연결 되었습니다.");
// }
//
// pub fn register_user_db(user_name: &str) {
//     let mut query = String::from("");
//     query.push_str("INSERT INTO USERS (user_name, point, wallet_address) VALUES ('");
//     query.push_str(user_name);
//     query.push_str("', 100, '67e55044-10b1-426f-9247-bb680e5fe0c8');");
//
//     // let query = "select user_name, point, wallet_address from USERS";
//     // let query = "INSERT INTO USERS (user_name, point, wallet_address) VALUES ('admin', 100, '67e55044-10b1-426f-9247-bb680e5fe0c8');";
//
//     let url = "mysql://root:root@localhost:3306/test_db";
//     let pool = Pool::new(url).unwrap();
//
//     let mut conn = pool.get_conn().unwrap();
//
//     let query_iter = conn.query_iter(query).unwrap();
//
//     conn.query_iter(query.as_str()).unwrap().for_each(|row| {
//         let r:(String, i32, String) = from_row(row.unwrap());
//         println!("{}, {}, {}", r.0, r.1, r.2);
//     });
//     // while let Some(result_set) = result.iter() {
//     //     println!("Result set columns: {:?}", result_set.columns());
//     // }
//
// }


fn main() {
    let mut dao = DataAccessStruct {
        id: String::from("root"),
        password: String::from("root"),
        host: String::from("localhost"),
        port: String::from("3306"),
        database: String::from("test_db")
    };

    let conn = dao.do_connect();


    // register_user_db("test_user");
    // connect_to_db();
    // listening_for_loop();
}
