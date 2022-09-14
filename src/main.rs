mod structs;
mod common;
mod database;

fn main() {
    database::database::connect_db();
}
