use lettre_email::Email;
use lettre::smtp::authentication::Credentials;
use lettre::{SmtpClient, Transport};
use std::fs;

const SENDER: &str = "trafisz22@gmail.com";
const PASSWORD: &str = "lsjtkwyqixxevbxa";
const RECEIVER: &str = "trafisz22_y0mjwk@kindle.com";

fn main() {
    // Create an SmtpClient with the given credentials
    let creds = Credentials::new(SENDER.to_string(), PASSWORD.to_string());
    let mut mailer = SmtpClient::new_simple("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .transport();

    // Read the current directory
    let paths = match fs::read_dir(".") {
        Ok(paths) => paths,
        Err(err) => {
            // Handle the error by printing a message and returning from the function
            println!("Failed to read the current directory: {}", err);
            return;
        }
    };

    for path in paths {
        let file_path = path.unwrap().path();
        if file_path.extension().is_some() && file_path.extension().unwrap() == "txt" {
            // Construct the email with the file as an attachment
            let email = match Email::builder()
                .to(RECEIVER)
                .from(SENDER)
                .subject("Kindle file transfer")
                .text("transfered by rust")
                .attachment_from_file(&file_path, None, &mime::TEXT_HTML_UTF_8)
            {
                Ok(email) => email,
                Err(err) => {
                    // Handle the error by printing a message and continuing to the next file
                    println!("Failed to construct email for file at {:?}: {}", file_path, err);
                    continue;
                }
            }
            .build()
            .unwrap();

            // Send the email
            let result = mailer.send(email.into());
            if result.is_ok() {
                println!("File at {:?} have been sent", file_path);
            } else {
                // Handle the error by printing a message
                println!("Failed to send file at {:?}: {:?}", file_path, result);
            }
        }
    }
}