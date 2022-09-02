pub struct Coin {
    pub coin_id: i32,
    pub wallet_address: String,
    pub coin_address: String,
}

static mut COIN_ID:i32 = 0;

impl Coin {
    pub fn new(wallet_address: String) -> Coin {
        unsafe { COIN_ID = COIN_ID + 1; }
        unsafe {
            Coin {
                coin_id: COIN_ID,
                wallet_address,
                coin_address: "123".to_string(), //todo 랜덤 주소 구현
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::structs::coin::Coin;

    #[test]
    fn create_coin_success() {
        let coin1 = Coin::new("wlt_123".to_string());
        assert_eq!(coin1.coin_id, 1);
        assert_eq!(coin1.wallet_address, "wlt_123".to_string());
        assert_eq!(coin1.coin_address, "123".to_string());
        let coin2 = Coin::new("wlt_123".to_string());
        assert_eq!(coin2.coin_id, 2);
    }
}