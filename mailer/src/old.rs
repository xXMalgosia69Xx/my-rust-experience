// send mail with attachment to fixed email address with fixed cridentials

use lettre_email::Email;
use lettre::smtp::authentication::Credentials;
use lettre::{SmtpClient, Transport};

const SENDER: &str = "trafisz22@gmail.com";
const PASSWORD: &str = "lsjtkwyqixxevbxa";
const RECEIVER: &str = "trafisz22_y0mjwk@kindle.com";

fn main() {
    let creds = Credentials::new(SENDER.to_string(), PASSWORD.to_string());
    let mut mailer = SmtpClient::new_simple("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .transport();
    
    let paths = std::fs::read_dir(".").unwrap();
    for path in paths {
        let pathh = path.unwrap().path();
        if pathh.extension().is_some() && pathh.extension().unwrap() == "epub" {
            let email = Email::builder()
                .to(RECEIVER)
                .from(SENDER)
                .subject("Kindle file transfer")
                .text("transfered by rust")
                .attachment_from_file(&pathh, None, &mime::TEXT_HTML_UTF_8)
                .unwrap()
                .build()
                .unwrap();
            let result = mailer.send(email.into());
            if result.is_ok() {
                println!("File at {:?} have been sent", pathh);
            } else {
                println!("File at {:?} have not been sent. Reason: {:?}", pathh, result);
            }
        }
    }
}