pub mod user {
    pub struct User {
        pub user_id: i32,
        pub point: i32,
        pub wallet_address: String,
    }

    impl User {
        pub fn new(user_id: i32) -> User {
            User {
                user_id,
                point: 100,
                wallet_address: "123",
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use user;
    #[test]
    fn create_user_success() {
        let user = User::new(1);
        assert_eq!(user::user_id, 2);
    }
}