use axum::http::header;
use dotenv;
use once_cell::sync::Lazy;
use serde_json::json;
use reqwest::Client;

static MAIL_API_KEY: Lazy<Option<String>> = Lazy::new(|| {
    match dotenv::var("SENDGRID_API_KEY") {
        Ok(val) => Some(val),
        Err(_) => {
            match std::env::var("SENDGRID_API_KEY") {
                Ok(val) => Some(val),
                Err(_) => None,
            }
        }
    }
});
static HTTP_CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder().build().expect("Couldnt build reqwest::Client in mail module")
});

pub async fn send_registration_confirmation_mail(to: &str, confirmation_token: &str) -> Result<(), String> {
    let content = format!("<h1>Willkommen bei Preiskampf</h1><p>Schön, dass du dabei bist. Du musst bloß noch <a href=\"https://preiskampf.julian-lindner.com/activate?token={}\">hier</a> klicken, um deinen Account zu aktivieren.", confirmation_token);
    send_mail(
        to,
        "Ihre Registrierung",
        content.as_str(),
    ).await
}

pub async fn send_mail(to: &str, subject: &str, content: &str) -> Result<(), String> {
    if MAIL_API_KEY.is_none() {
        return Result::Err("SENDGRID_API_KEY not present in env".into());
    }

    let body = json!({
        "personalizations": [{"to": [{"email": to}]}],
        "from": {"email": "preiskampf@julian-lindner.com"},
        "subject": subject,
        "content": [
            {"type": "text/html", "value": content},
        ]
    });
    match HTTP_CLIENT
        .post("https://api.sendgrid.com/v3/mail/send")
        .bearer_auth(MAIL_API_KEY.as_ref().unwrap())
        .header(header::CONTENT_TYPE, "application/json")
        .body(body.to_string())
        .send()
        .await {
            Ok(response) => {
                println!("send_mail response {:?}", response.status());
                ()
            },
            Err(e) => {
                eprintln!("send_mail error {:?}", e);
                ()
            }
        };

    Result::Ok(())
}