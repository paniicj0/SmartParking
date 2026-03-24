use lettre::message::{
    header::{ContentDisposition, ContentId, ContentType},
    MultiPart, SinglePart,
};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};

pub async fn send_invoice_email(
    to_email: &str,
    invoice_number: &str,
    amount: f64,
    qr_code_data: &str,
    qr_png: Vec<u8>,
) -> Result<(), String> {
    let email_user =
        std::env::var("EMAIL_USER").map_err(|_| "EMAIL_USER nije setovan.".to_string())?;
    let email_pass =
        std::env::var("EMAIL_PASS").map_err(|_| "EMAIL_PASS nije setovan.".to_string())?;

    let html_body = format!(
        r#"
        <html>
          <body style="font-family: Arial, sans-serif; color: #222;">
            <h2>Vaš račun za parking</h2>

            <p><b>Broj računa:</b> {}</p>
            <p><b>Iznos za uplatu:</b> {:.2} RSD</p>
            <p><b>QR identifikator:</b> {}</p>
            <p><b>Status računa:</b> Unpaid</p>

            <p>U nastavku se nalazi QR kod koji možete skenirati:</p>

            <div style="margin: 20px 0;">
              <img src="cid:invoice_qr_code" alt="QR kod računa" style="width: 220px; height: 220px;" />
            </div>
          </body>
        </html>
        "#,
        invoice_number, amount, qr_code_data
    );

    let html_part = SinglePart::builder()
        .header(ContentType::TEXT_HTML)
        .body(html_body);

    let image_part = SinglePart::builder()
        .header(
            ContentType::parse("image/png")
                .map_err(|e| format!("Greška pri ContentType image/png: {}", e))?,
        )
        .header(ContentId::from(String::from("invoice_qr_code")))
        .header(ContentDisposition::inline())
        .body(qr_png);

    let email = Message::builder()
        .from(
            email_user
                .parse()
                .map_err(|_| "Neispravan EMAIL_USER.".to_string())?,
        )
        .to(
            to_email
                .parse()
                .map_err(|_| "Neispravna email adresa primaoca.".to_string())?,
        )
        .subject("Račun za parking")
        .multipart(
            MultiPart::related()
                .singlepart(html_part)
                .singlepart(image_part),
        )
        .map_err(|e| format!("Greška pri kreiranju email poruke: {}", e))?;

    let creds = Credentials::new(email_user.clone(), email_pass);

    let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay("smtp.gmail.com")
        .map_err(|e| format!("Greška pri kreiranju SMTP transporta: {}", e))?
        .credentials(creds)
        .build();

    mailer
        .send(email)
        .await
        .map_err(|e| format!("Greška pri slanju email-a: {}", e))?;

    Ok(())
}