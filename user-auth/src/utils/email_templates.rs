pub fn activate_email_template(
    name: &str,
    otp: &str,
    expiry_minutes: u64,
    activation_link: &str,
    support_email: &str,
) -> String {
    format!(
        r#"<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <meta http-equiv="X-UA-Compatible" content="IE=edge" />
    <title>Activate your account</title>
    <style>
      body {{ margin:0; padding:0; font-family: Arial, Helvetica, sans-serif; background:#f4f6f8; color:#333; }}
      .container {{ max-width:600px; margin:24px auto; background:#ffffff; border-radius:8px; padding:24px; box-shadow:0 2px 6px rgba(0,0,0,0.08); }}
      h1 {{ font-size:20px; margin:0 0 12px; color:#1a1a1a; }}
      p {{ margin:0 0 16px; line-height:1.6; }}
      .otp {{ display:inline-block; font-size:28px; letter-spacing:4px; background:#f0f4ff; padding:12px 18px; border-radius:6px; font-weight:700; color:#1e40af; }}
      .btn {{ display:inline-block; text-decoration:none; padding:12px 24px; background:#2563eb; color:#fff !important; border-radius:6px; margin-top:10px; font-weight:600; }}
      .btn:hover {{ background:#1d4ed8; }}
      .small {{ font-size:13px; color:#666; }}
      .footer {{ font-size:12px; color:#999; text-align:center; margin-top:24px; padding-top:16px; border-top:1px solid #e5e7eb; }}
      a {{ color:#2563eb; }}
      @media (max-width:420px){{
        .container {{ padding:16px; margin:12px; }}
        .otp {{ font-size:24px; padding:10px 14px; letter-spacing:3px; }}
      }}
    </style>
  </head>
  <body>
    <center>
      <table role="presentation" width="100%" cellpadding="0" cellspacing="0" style="max-width:600px;margin:24px auto;background:#f4f6f8;">
        <tr>
          <td>
            <div class="container">
              <h1>Activate your account</h1>
              <p>Hi <strong>{}</strong>,</p>
              <p>Use the OTP below to activate your account. It expires in <strong>{} minutes</strong>.</p>

              <p style="text-align:center; margin:24px 0;">
                <span class="otp">{}</span>
              </p>

              <p style="text-align:center;">
                <a href="{}" class="btn" target="_blank" rel="noopener noreferrer">Activate Account</a>
              </p>

              <p class="small">If the button doesn't work, copy and paste this URL into your browser:</p>
              <p class="small" style="word-break:break-all;"><a href="{}" target="_blank" rel="noopener noreferrer">{}</a></p>

              <div class="footer">
                <p class="small">If you didn't request this, please ignore this email. For support, contact <a href="mailto:{}">{}</a>.</p>
              </div>
            </div>
          </td>
        </tr>
      </table>
    </center>
  </body>
</html>"#,
        name,
        expiry_minutes,
        otp,
        activation_link,
        activation_link,
        activation_link,
        support_email,
        support_email
    )
}
