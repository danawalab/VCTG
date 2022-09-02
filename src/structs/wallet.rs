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
            wallet_address: "123".to_string(),
        };
        wallet.wallet_address
    }
}


#[cfg(test)]
mod tests {
    use crate::structs::wallet::Wallet;

    #[test]
    fn create_wallet_success() {
        let wallet = Wallet::new(1);
        assert_eq!(wallet, "123".to_string());
    }
}