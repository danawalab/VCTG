pub mod user {
    use crate::wallet::Wallet;

    pub struct User {
        pub user_id: i32,
        pub point: i32,
        pub wallet_address: String,
    }

    impl User {
        pub fn new(user_id: i32) -> User {
            let wallet_address = Wallet::new(user_id);
            User {
                user_id,
                point: 100,
                wallet_address,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::user::User;

    #[test]
    fn create_user_success() {
        let user = User::new(1);
        assert_eq!(user.user_id, 1);
        assert_eq!(user.point, 100);
        assert_eq!(user.wallet_address, "123".to_string());
    }
}