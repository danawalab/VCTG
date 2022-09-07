use std::string::ToString;
use crate::common::generate_address::generate_address;

const WALLET_ADDRESS_PREFIX: &str = "wlt_";

pub struct Wallet {
    wallet_address: String,
    user_id: i32,
}

impl Wallet {
    pub fn new(user_id: i32) -> String {
        let mut wallet_address: String = WALLET_ADDRESS_PREFIX.to_string();
        wallet_address.push_str(&*generate_address());

        let wallet = Wallet {
            wallet_address,
            user_id,
        };
        wallet.wallet_address
    }

    pub fn get_coins(wallet_address: String) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::common::generate_address::generate_address;
    use crate::structs::wallet::{Wallet, WALLET_ADDRESS_PREFIX};

    #[test]
    fn create_wallet_success() {
        let wallet = Wallet::new(1);
        let address_prefix = &wallet[0..4];
        assert_eq!(address_prefix, "wlt_");
    }

    #[test]
    fn rand_wallet_address() {
        let mut address: String = WALLET_ADDRESS_PREFIX.to_string();
        address.push_str(&*generate_address());

        println!("{}", address);
    }
}