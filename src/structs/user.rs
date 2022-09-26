use mysql::{from_row, from_value, Value};

use crate::database::database::DataAccessStruct;
use crate::structs::wallet::Wallet;

/// User 구조체
/// user 생성시 wallet도 같이 생성 wallet에 user_id를 넣어주기 위해 user_id 사용
/// user_id는 ORM이 아니라 직접 User가 생성시 +1 해준다
/// user_id를 순차 증가를 위해서 unsafe 사용
pub struct User {
    pub user_id: i32,
    pub user_name: String,
    pub point: i32,
    pub wallet_address: String,
}

static mut USER_ID: i32 = 0;

impl User {
    pub fn new(user_name: String) -> User {
        // exists_user_name(&user_name); // panic 발생함
        unsafe { USER_ID = USER_ID + 1; }
        let wallet_address = Wallet::new(unsafe { USER_ID });
        unsafe {
            User {
                user_id: USER_ID,
                user_name: user_name,
                point: 100,
                wallet_address,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::structs::user::User;

    #[test]
    fn create_user_success() {
        let user = User::new("홍길동".to_string());
        assert_eq!(user.user_id, 1);
        assert_eq!(user.user_name, "홍길동".to_string());
        assert_eq!(user.point, 100);
        let wlt_prefix = &user.wallet_address[0..4];
        assert_eq!(wlt_prefix, "wlt_");
    }
}