use url::Url;

use super::telegram::Telegram;
use super::whatsapp::WhatsApp;
use crate::CpArgs;

use std::collections::HashMap;

pub struct Sender;

impl Sender {
    pub fn copy_file_to_chat(params: CpArgs) {
        let uri = Url::parse(params.messenger_target.as_str()).expect("Send valid URI!");
        match uri.scheme() {
            "telegram" | "tgm" => {
                let hash_query: HashMap<String, String> = uri.query_pairs().into_owned().collect();
                Telegram::make_request(
                    uri.host_str().unwrap().to_string(),
                    params.input_file,
                    match hash_query.get("caption") {
                        Some(c) => c
                            .strip_prefix("'")
                            .unwrap()
                            .strip_suffix("'")
                            .unwrap()
                            .to_string(),
                        None => String::from(""),
                    },
                )
            }
            "whatsapp" | "wa" => {
                let hash_query: HashMap<String, String> = uri.query_pairs().into_owned().collect();
                WhatsApp::make_request(
                    uri.host_str().unwrap().to_string(),
                    params.input_file,
                    match hash_query.get("number_id") {
                        Some(c) => c.to_string(),
                        None => panic!("Failed get whatsapp number_id"),
                    },
                    match hash_query.get("caption") {
                        Some(c) => c
                            .strip_prefix("'")
                            .unwrap()
                            .strip_suffix("'")
                            .unwrap()
                            .to_string(),
                        None => String::from(""),
                    },
                )
            }
            _ => panic!("Invalid messenger!"),
        }
    }
}
