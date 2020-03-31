mod state;

mod user {
    pub struct UserID (pub i32);
}

pub fn add_ids
    ( user::UserID(x_num) : user::UserID,
      user::UserID(y_num) : user::UserID
    ) -> i32
{
    // let self::user::UserID(y_num) = y;
    x_num + y_num
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let x = UserId();
        assert_eq!(2+2, 4);
    }
}
