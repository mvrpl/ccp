use reqwest::blocking::multipart;

use crate::VAULT;

pub struct Telegram;

impl Telegram {
    pub fn make_request(chat_id: String, file_path: String) {
        let url = format!(
            "https://api.telegram.org/bot{}/sendDocument",
            VAULT.get("telegram_token").unwrap()
        );
        let client = reqwest::blocking::Client::new();

        let form = multipart::Form::new()
            .text("chat_id", chat_id)
            .text("parse_mode", "HTML")
            .text("caption", "test")
            .file("document", file_path)
            .unwrap();

        let response = client
            .post(url)
            .multipart(form)
            .send()
            .unwrap()
            .error_for_status();

        if let Err(err) = response {
            panic!("{}", err)
        }
    }
}
