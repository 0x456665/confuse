use std::collections::HashMap;

pub fn get_redis_keys() -> HashMap<&'static str, &'static str> {
    let mut redis_keys: HashMap<&'static str, &'static str> = HashMap::new();
    redis_keys.insert("email_activation", "email_activation");
    redis_keys.insert("forgot_password", "forgot_password");
    redis_keys
}
