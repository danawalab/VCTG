use crate::structs::wallet::Wallet;

pub struct User {
    pub user_id: i32,
    pub user_name: String,
    pub point: i32,
    pub wallet_address: String,
}

static mut USER_ID:i32 = 0;

impl User {
    pub fn new(user_name: String) -> User {
        unsafe { USER_ID = USER_ID + 1; }
        let wallet_address = Wallet::new(unsafe { USER_ID });
        unsafe {
            User {
                user_id: USER_ID,
                user_name,
                point: 100,
                wallet_address,
            }
        }
    }
}

fn exists_user_name(user_name: String) -> bool {
    false
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
        assert_eq!(user.wallet_address, "123".to_string());
    }
}