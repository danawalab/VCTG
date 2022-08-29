pub mod user {
    struct User {
        user_id: i32,
        point: i32,
        wallet_address: String,
    }

    pub fn create_user(user_id: i32) -> User {
        User {
            user_id,
            point: 1000,
            wallet_address: "123",
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn create_user_success() {

        assert_eq!();
    }
}