use url::Url;

use super::telegram::Telegram;
use crate::CmdCpArgs;

pub struct Sender;

impl Sender {
    pub fn copy_file_to_chat(params: &CmdCpArgs) {
        let uri = Url::parse(params.messenger_target.as_str()).expect("Send valid URI!");
        match uri.scheme() {
            "telegram" | "tgm" => Telegram::make_request(uri.host_str().unwrap().to_string(), params.input_file.to_string()),
            _ => panic!("Invalid messenger!")
        }
    }
}