pub struct Wallet {
    wallet_address: String,
    user_id: i32,
}

impl Wallet {
    pub fn new(user_id: i32) -> String {
        let wallet = Wallet {
            wallet_address: "123".to_string(), //todo 랜덤 주소 구현
            user_id,
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