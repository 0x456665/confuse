use dotenvy::dotenv;
use std::env;

pub struct Config {
    pub DATABASE_URL: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set").to_owned(),
    pub ACCESS_SECRET: String = env::var("ACCESS_SECRET").expect("JWT_SECRET must be set").to_owned(),
    pub REFRESH_SECRET: String = env::var("REFRESH_SECRET").expect("JWT_SECRET must be set").to_owned(),
}
