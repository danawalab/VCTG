use mysql::{from_row, Pool, PooledConn, QueryResult, Text};
use mysql::prelude::Queryable;

pub struct DataAccessStruct {
    pub(crate) id: String,
    pub(crate) password: String,
    pub(crate) host: String,
    pub(crate) port: String,
    pub(crate) database: String
}

impl DataAccessStruct {

    pub fn do_connect(&mut self) -> PooledConn {
        // return 결과로 Pool Struct를 준다

        let mut url = String::from("");
        url.push_str("mysql://");
        url.push_str(self.id.as_str());
        url.push_str(":");
        url.push_str(self.password.as_str());
        url.push_str("@");
        url.push_str(self.host.as_str());
        url.push_str(":");
        url.push_str(self.port.as_str());
        url.push_str("/");
        url.push_str(self.database.as_str());
        let pool = Pool::new(url.as_str()).unwrap();
        let mut conn = pool.get_conn().unwrap();
        return conn;
    }

    pub fn query(&self, conn: &mut PooledConn, query: &str) -> Vec<(i32, String, i32, String)> {
        let mut v = Vec::new();
        conn.query_iter(query).unwrap().for_each(|row| {
                let r:(i32, String, i32, String) = from_row(row.unwrap());
                v.push(r.clone());
                println!("{}, {}, {}, {}", r.0, r.1, r.2, r.3);
            });
        return v;
    }
}