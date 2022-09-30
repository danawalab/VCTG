pub struct SellCoin {
    pub sell_coin_id: i32,
    pub seller_id: i32,
    pub point: i32,
    pub coin_address: String,
}

impl SellCoin {
    pub fn new(seller_id: i32, point: i32, coin_address: String) -> SellCoin {
        SellCoin {
            sell_coin_id: 0,
            seller_id,
            point,
            coin_address,
        }
    }
}