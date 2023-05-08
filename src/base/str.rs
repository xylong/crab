pub fn check_password_length(password:&str) ->bool {
    return match password.len() {
        6..=18 => true,
        _ => false,
    }
}