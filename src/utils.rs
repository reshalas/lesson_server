use lettre::{
    message::{Mailbox, MultiPart},
    transport::smtp::authentication::Credentials,
    SmtpTransport, Transport,
};

pub async fn send_email(email: String, body: String) -> Result<(), ()> {
    let message = lettre::Message::builder()
        .from(Mailbox::new(None, get_company_email().parse().unwrap()))
        .to(Mailbox::new(None, email.parse().unwrap()))
        .subject("Регистрация")
        .multipart(MultiPart::alternative_plain_html(String::new(), body))
        .unwrap();
    let creds = Credentials::new(get_company_email(), get_smtp_key());
    let client = SmtpTransport::relay("smtp-relay.sendinblue.com")
        .unwrap()
        .credentials(creds)
        .build();
    let res = client.send(&message).unwrap();
    if res.is_positive() {
        println!("{:?}", res);
        return Ok(());
    }
    Err(())
}

pub fn get_company_email() -> String {
    dotenvy::var("EMAIL_ADDRES").unwrap()
}

fn get_smtp_key() -> String {
    dotenvy::var("SMTP_KEY").unwrap()
}

pub fn build_register_mesage(name: &str, verefication_key: &str, username: &str) -> String {
    let verfy_ref = format!("http://127.0.0.1:8090/verefication/email/verfy/{}/{}", username, verefication_key);
    let cancel_ref = format!("http://127.0.0.1:8090/verefication/email/cancel/{}/{}", username, verefication_key);
    format!("
    <div>
        <p>Здравствуй, {}! C твоего аддреса электронной почты кто-то хочет зарегестрироваться в нашу маленькую и ламповою систему. Если это ты, то добро пожаловать. Смело нажимай на кнопку \"Законьчить регистрацию\", в противном случае прошу нажать на кнопку \"Отклонить регистрацию\"</p>

            <a href=\"{}\"
      style=\"background-color:#28b422;border-radius:4px;color:#ffffff;display:inline-block;font-family:sans-serif;font-size:13px;font-weight:bold;line-height:35px;text-align:center;text-decoration:none;-webkit-text-size-adjust:none;\">Законьчить регистрацию</a>
      <a href=\"{}\"
      style=\"background-color:#28b422;border-radius:4px;color:#ffffff;display:inline-block;font-family:sans-serif;font-size:13px;font-weight:bold;line-height:35px;text-align:center;text-decoration:none;width:120px;-webkit-text-size-adjust:none;\">Отклонить регистрацию</a>

    </div>"
 , name, verfy_ref, cancel_ref)
}
