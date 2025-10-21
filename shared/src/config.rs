use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub access_secret: String,
    pub refresh_secret: String,
    pub access_token_duration: String,
    pub refresh_token_duration: String,
    pub smtp_username: String,
    pub smtp_password: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            database_url: env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set")
                .to_owned(),
            access_secret: env::var("ACCESS_SECRET")
                .expect("ACCESS_SECRET must be set")
                .to_owned(),
            refresh_secret: env::var("REFRESH_SECRET")
                .expect("REFRESH_SECRET must be set")
                .to_owned(),
            access_token_duration: env::var("ACCESS_TOKEN_DURATION_MINUTES")
                .expect("ACCESS_TOKEN_DURATION must be set")
                .into(),
            refresh_token_duration: env::var("REFRESH_TOKEN_DURATION_DAYS")
                .expect("ACCESS_TOKEN_DURATION must be set")
                .into(),
            smtp_password: env::var("SMTP_PASSWORD")
                .expect("SMTP_PASSWORD must be set")
                .to_owned(),
            smtp_username: env::var("SMTP_USERNAME")
                .expect("SMTP_USERNAME must be set")
                .to_owned(),
        }
    }
}
