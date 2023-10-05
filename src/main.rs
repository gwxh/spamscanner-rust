use std::{fs::{File, self}, io::Read};
use mail_parser::{*, core::message};
mod spam;
use spam::SpamScanner;

// fn main() {
//     let email_path = "fixtures/spam.eml";
//     let email_content = fs::read_to_string(email_path).expect("Unable to read file");
    
//     let message = MessageParser::default().parse(email_content.as_str()).unwrap();
//     let subject = message.subject().unwrap();
//     println!("{}",subject);

//     let contentLanguage = message.content_language().as_text().unwrap_or("default");
//     println!("{}",contentLanguage);

//     let description = message.body_text(0).unwrap();
//     println!("{}",description);
// }

#[tokio::main]
async fn main() {
    let email_path = "fixtures/spam.eml".to_string();
    let mut scanner = SpamScanner::new();
    let result = scanner.scan(email_path).await;
    println!("{}",result.is_spam)
}
