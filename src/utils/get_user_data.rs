use crate::db::orm::user;

pub fn encode() -> Option<(String, String)> {
    let user = user::get_by_id(1).expect("ERROR | Error fetching user from database");

    user.map(|u| (u.encode_key, u.encode_iv))
}
