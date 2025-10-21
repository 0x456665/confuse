use lettre::{
    Message, SmtpTransport, Transport, message::header::ContentType,
    transport::smtp::authentication::Credentials,
};
use shared::{config::Config, errors::AppError};

pub fn send_email(
    from: String,
    to: String,
    subject: String,
    body: String,
    reply_to_option: Option<String>,
    config: &Config,
) -> Result<(), AppError> {
    let reply_to = reply_to_option.unwrap_or("no-reply@gmail.com".to_string());
    let email = Message::builder()
        .from(from.parse().unwrap())
        .reply_to(reply_to.parse().unwrap())
        .to(to.parse().unwrap())
        .subject(subject)
        .header(ContentType::TEXT_HTML)
        .body(body)
        .unwrap();

    let creds = Credentials::new(config.smtp_username.clone(), config.smtp_password.clone());

    // Open a remote connection to gmail using STARTTLS
    let mailer = SmtpTransport::starttls_relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    if let Err(e) = mailer.send(&email) {
        return Err(AppError::InternalServerError(format!(
            "Email failed to send: {:?}",
            e
        )));
    };

    Ok(())
}
