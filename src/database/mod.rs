pub mod database;

// {
//     use mysql::{Pool, QueryResult, Text};
//
//     pub struct DataAccessStruct {
//         id: String,
//         password: String,
//         host: String,
//         port: String,
//         database: String
//     }
//
//     impl DataAccess for DataAccessStruct {
//
//         fn do_connect(&mut self) -> &Pool {
//             // return 결과로 Pool Struct를 준다
//
//             let mut url = String::from("");
//             url.push_str("mysql://");
//             url.push_str(self.id.as_str());
//             url.push_str(":");
//             url.push_str(self.password.as_str());
//             url.push_str("@");
//             url.push_str(self.host.as_str());
//             url.push_str(":");
//             url.push_str(self.port.as_str());
//             url.push_str("/");
//             url.push_str(self.database.as_str());
//             return &Pool::new(url.as_str()).unwrap();
//         }
//
//         fn query(&self, conn: &Pool, query: &str) -> QueryResult<Text>{
//             // 사용법
//             // conn.query_iter(query).unwrap().for_each(|row| {
//             //         let r:(String, i32, String) = from_row(row.unwrap());
//             //         println!("{}, {}, {}", r.0, r.1, r.2);
//             //     });
//             return conn.query_iter(query).unwrap();
//         }
//     }
// }
//