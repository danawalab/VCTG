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
        // db서 seller_id  조회 후 해당 유저는 point 증가, 코인 삭제
        // db서 buyer_id 조회 후 해당 유저는 point 감소, 코인 추가

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
