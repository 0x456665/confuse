pub fn activate_email_template(
    name: &str,
    otp: &str,
    expiry_minutes: u64,
    activation_link: &str,
) -> String {
    format!(
        r#"<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Activate your account</title>
    <style>
      body {{ margin:0; padding:0; font-family: Arial, sans-serif; background:#f4f6f8; color:#333; }}
      .container {{ max-width:600px; margin:24px auto; background:#fff; border-radius:8px; padding:24px; }}
      h1 {{ font-size:24px; margin:0 0 16px; }}
      p {{ margin:0 0 16px; line-height:1.5; }}
      .otp {{ font-size:32px; letter-spacing:8px; color:#2563eb; font-weight:700; }}
      .btn {{ display:inline-block; padding:12px 24px; background:#2563eb; color:#fff; text-decoration:none; border-radius:6px; }}
      .footer {{ margin-top:32px; padding-top:16px; border-top:1px solid #e5e7eb; font-size:13px; color:#666; }}
    </style>
  </head>
  <body>
    <div class="container">
      <h1>Activate your account</h1>
      <p>Hi <strong>{}</strong>,</p>
      <p>Your activation code is:</p>
      <p style="text-align:center; margin:24px 0;">
        <span class="otp">{}</span>
      </p>
      <p style="text-align:center;">
        <a href="{}" class="btn">Activate Account</a>
      </p>
      <div class="footer">
        <p>This code expires in {} minutes.</p>
        <p>If you didn't request this, ignore this email.</p>
      </div>
    </div>
  </body>
</html>"#,
        name,
        otp,
        activation_link,
        expiry_minutes
    )
}

pub fn forgot_password_email_template(
    name: &str,
    reset_link: &str,
    expiry_minutes: u64,
) -> String {
    format!(
        r#"<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Reset your password</title>
    <style>
      body {{ margin:0; padding:0; font-family: Arial, sans-serif; background:#f4f6f8; color:#333; }}
      .container {{ max-width:600px; margin:24px auto; background:#fff; border-radius:8px; padding:24px; }}
      h1 {{ font-size:24px; margin:0 0 16px; }}
      p {{ margin:0 0 16px; line-height:1.5; }}
      .btn {{ display:inline-block; padding:12px 24px; background:#dc2626; color:#fff; text-decoration:none; border-radius:6px; }}
      .warning {{ background:#fef3c7; padding:12px; border-radius:6px; margin:16px 0; }}
      .footer {{ margin-top:32px; padding-top:16px; border-top:1px solid #e5e7eb; font-size:13px; color:#666; }}
    </style>
  </head>
  <body>
    <div class="container">
      <h1>Reset Your Password</h1>
      <p>Hi <strong>{}</strong>,</p>
      <p>Click the button below to reset your password:</p>
      <p style="text-align:center; margin:24px 0;">
        <a href="{}" class="btn">Reset Password</a>
      </p>
      <div class="warning">
        <p style="margin:0; font-size:14px;"><strong>⏰ This link expires in {} minutes</strong></p>
      </div>
      <div class="footer">
        <p>If you didn't request this, you can safely ignore this email.</p>
      </div>
    </div>
  </body>
</html>"#,
        name,
        reset_link,
        expiry_minutes
    )
}