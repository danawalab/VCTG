use crate::common::generate_address::generate_address;

const COIN_ADDRESS_PREFIX: &str = "cin_";

pub struct Coin {
    pub coin_address: String,
    pub wallet_address: String,
}

impl Coin {
    pub fn new(wallet_address: String) -> Coin {
        let mut coin_address: String = COIN_ADDRESS_PREFIX.to_string();
        coin_address.push_str(&*generate_address());

        Coin {
            coin_address,
            wallet_address,
        }
    }
}

todo!(
    코인 db 등록,
    채굴량 100개시 코인 채굴 X,
    코인 채굴량 반환,
);

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