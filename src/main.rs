use crate::structs::user::User;

mod structs;

fn main() {
    let user = User::new(1);

    println!("{}", user.user_id);
}