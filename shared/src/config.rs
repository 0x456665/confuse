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
    pub otp_expiry_minutes: u64,
    pub from_email: String,
    pub support_email: Option<String>,
    pub frontend_activation_url: Option<String>,
    pub frontend_url: String,
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
            otp_expiry_minutes: env::var("OTP_EXPIRY_MINUTES")
                .expect("OTP_EXPIRY_MINTES must be set")
                .parse()
                .expect("OTP expiry not a number"),
            from_email: env::var("FROM_EMAIL").expect("FROM_EMAIL must be set"),
            support_email: env::var("SUPPORT_EMAIL").ok(),
            frontend_activation_url: env::var("FRONTEND_ACTIVATION_URL").ok(),
            frontend_url: env::var("FRONTEND_URL")
                .expect("FRONTEND_URL must be set")
                .to_owned(),
        }
    }
}
