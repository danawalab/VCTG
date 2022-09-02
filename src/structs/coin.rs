pub struct Coin {
    pub coin_address: String,
    pub wallet_address: String,
}

impl Coin {
    pub fn new(wallet_address: String) -> Coin {
        Coin {
            coin_address: "123".to_string(), //todo 랜덤 주소 구현
            wallet_address,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::structs::coin::Coin;

    #[test]
    fn create_coin_success() {
        let coin1 = Coin::new("wlt_123".to_string());
        assert_eq!(coin1.wallet_address, "wlt_123".to_string());
        assert_eq!(coin1.coin_address, "123".to_string());
    }
}