pub mod wallet {
    struct Wallet {
        wallet_id: i32,
        user_id: i32,
        wallet_address: String,
    }

    pub fn create_wallet(user_id: i32) -> String {
        let wallet = Wallet {
            wallet_id: user_id,
            user_id,
            wallet_address: "123"
        };
        wallet.wallet_address
    }
}