pub struct Contract {
    pub contract_id: i32,
    pub sell_coin_id: i32,
    pub seller_id: i32,
    pub buyer_id: i32,
    pub coin_address: String,
    pub point: i32,
}

impl Contract {
    pub fn new(
        sell_coin_id: i32,
        seller_id: i32,
        buyer_id: i32,
        coin_address: String,
        point: i32,
    ) -> Contract {
        Contract {
            contract_id: 0,
            sell_coin_id,
            seller_id,
            buyer_id,
            coin_address,
            point,
        }
    }
}
