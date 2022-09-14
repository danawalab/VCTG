use mysql::prelude::Queryable;

const DB_USER_NAME: &'static str = "vctg-svc";
const DB_PASSWORD: &'static str = "vctg-pass";
const DB_IP: &'static str = "localhost";
const DB_PORT: u16 = 3306_u16;
const DB_NAME: &'static str = "vctg";

pub fn connect_db() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let db_url = "mysql://vctg-svc:vctg-pass@localhost:3306/vctg";
    let pool = mysql::Pool::new(db_url)?;
    let mut conn = pool.get_conn()?;

    let inset = "select * from users";
    let results = conn.query(inset);

    Ok(())
}
