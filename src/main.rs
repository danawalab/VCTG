use crate::server::db_connection::listening_for_loop;

mod structs;
mod common;
mod server;

fn main() {
  listening_for_loop();
}
