pub mod register;

pub struct UserDB<'r> {
    user_id: u32,
    first_name: &'r str,
    last_name: &'r str,
    email: &'r str,
    password: &'r str,
    gender: bool,
}
