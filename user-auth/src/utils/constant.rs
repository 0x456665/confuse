use std::collections::HashMap;



pub fn redis_key_map() -> HashMap<&'static str, &'static str>{
    let mut redis_map: HashMap<&'static str, &'static str> = HashMap::new();
    
    redis_map.insert("email_activation", "email_otp");
    redis_map.insert("forgot_password", "password_reset_token");
    redis_map.insert("refresh_token", "user_refresh_token");
    
    redis_map
}
