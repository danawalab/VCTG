fn main() {

}

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

pub mod wallet {
    pub struct Wallet {
        wallet_id: i32,
        user_id: i32,
        wallet_address: String,
    }

    impl Wallet {
        pub fn new(user_id: i32) -> String {
            let wallet = Wallet {
                wallet_id: user_id,
                user_id,
                wallet_address: "123".to_string()
            };
            wallet.wallet_address
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::user::User;
    use crate::wallet::Wallet;

    #[test]
    fn create_wallet_success() {
        let wallet = Wallet::new(1);
        assert_eq!(wallet, "123".to_string());
    }

    #[test]
    fn create_user_success() {
        let user = User::new(1);
        assert_eq!(user.user_id, 1);
        assert_eq!(user.point, 100);
        assert_eq!(user.wallet_address, "123".to_string());
    }
}