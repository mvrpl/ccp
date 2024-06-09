use reqwest::blocking::multipart;
use serde_json::json;

use crate::VAULT;

const GRAPH_API: &str = "https://graph.facebook.com/v19.0";

pub struct WhatsApp;

impl WhatsApp {
    pub fn make_request(to_recipient: String, file_path: std::path::PathBuf, number_id: String, caption: String) {
        let url_media = format!("{}/{}/media", GRAPH_API, number_id);
        let url_mgs = format!("{}/{}/messages", GRAPH_API, number_id);

        let client = reqwest::blocking::Client::new();

        let form = multipart::Form::new()
            .text("messaging_product", "whatsapp")
            .file("file", file_path)
            .unwrap();

        let response = client
            .post(url_media)
            .header("Authorization", VAULT.get("graph_bearer_token").unwrap())
            .multipart(form);

        match response.send() {
            Ok(resp) => {
                let media_json: serde_json::Value = resp.json().unwrap();
                let body = json!({
                    "messaging_product": "whatsapp",
                    "recipient_type": "individual",
                    "to": to_recipient,
                    "type": "document",
                    "document": {
                        "id": media_json["id"],
                        "caption": caption
                    }
                });
                client
                    .post(url_mgs)
                    .header("Authorization", VAULT.get("graph_bearer_token").unwrap())
                    .body(body.to_string())
                    .send().ok();
            }
            Err(err) => {
                panic!("{}", err)
            }
        }
    }
}
