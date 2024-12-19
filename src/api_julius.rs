use reqwest::header::{HeaderMap, HeaderValue};
//use std::collections::HashMap;
use serde_json::{Value, json};
use dotenv::dotenv;
use sqlx::PgPool;
use std::env;

const URL_AI_DEMO: &str= "https://playground.julius.ai";


fn header_generate_temp() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("Host", HeaderValue::from_static("playground.julius.ai"));
    headers.insert("sec-ch-ua", HeaderValue::from_static("\"Google Chrome\";v=\"119\", \"Chromium\";v=\"119\", \"Not?A_Brand\";v=\"24\""));
    headers.insert("use-dict", HeaderValue::from_static("false"));
    headers.insert("sec-ch-ua-mobile", HeaderValue::from_static("?1"));
    headers.insert("USER_AGENT", HeaderValue::from_static("Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Mobile Safari/537.3623"));
    headers.insert("cypress-test-id", HeaderValue::from_static("None"));
    headers.insert("gcs", HeaderValue::from_static("true"));
    headers.insert("interactive-charts", HeaderValue::from_static("true"));
    headers.insert("sec-ch-ua-platform", HeaderValue::from_static("\"Android\""));
    headers.insert("ACCEPT", HeaderValue::from_static("*/*"));
    headers.insert("ORIGIN", HeaderValue::from_static("https://julius.ai"));
    headers.insert("sec-fetch-site", HeaderValue::from_static("same-site"));
    headers.insert("sec-fetch-mode", HeaderValue::from_static("cors"));
    headers.insert("sec-fetch-dest", HeaderValue::from_static("empty"));
    headers.insert("REFERER", HeaderValue::from_static("https://julius.ai/"));
    headers.insert("ACCEPT_ENCODING", HeaderValue::from_static("gzip, deflate, br, zstd"));
    headers.insert("ACCEPT_LANGUAGE", HeaderValue::from_static("en-US,en;q=0.9"));

    headers
}


fn header_generate_conversation_id(demo_id: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("Host", HeaderValue::from_static("playground.julius.ai"));
    headers.insert("sec-ch-ua", HeaderValue::from_static("\"Google Chrome\";v=\"119\", \"Chromium\";v=\"119\", \"Not?A_Brand\";v=\"24\""));
    headers.insert("use-dict", HeaderValue::from_static("false"));
    headers.insert("sec-ch-ua-mobile", HeaderValue::from_static("?1"));
    headers.insert("USER_AGENT", HeaderValue::from_static("Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Mobile Safari/537.3623"));
    headers.insert("cypress-test-id", HeaderValue::from_static("None"));
    headers.insert("gcs", HeaderValue::from_static("true"));
    headers.insert("interactive-charts", HeaderValue::from_static("true"));
    headers.insert("sec-ch-ua-platform", HeaderValue::from_static("\"Android\""));
    headers.insert("ACCEPT", HeaderValue::from_static("*/*"));
    headers.insert("ORIGIN", HeaderValue::from_static("https://julius.ai"));
    headers.insert("sec-fetch-site", HeaderValue::from_static("same-site"));
    headers.insert("sec-fetch-mode", HeaderValue::from_static("cors"));
    headers.insert("sec-fetch-dest", HeaderValue::from_static("empty"));
    headers.insert("REFERER", HeaderValue::from_static("https://julius.ai/"));
    headers.insert("ACCEPT_ENCODING", HeaderValue::from_static("gzip, deflate, br, zstd"));
    headers.insert("ACCEPT_LANGUAGE", HeaderValue::from_static("en-US,en;q=0.9"));
    headers.insert("pathname", HeaderValue::from_static("/ai-chatbot"));
    headers.insert("authorization", HeaderValue::from_static("Bearer"));
    headers.insert("is-native", HeaderValue::from_static("false"));
    headers.insert("visitor-id", HeaderValue::from_static("undefined"));
    headers.insert("is-demo", HeaderValue::from_str(&demo_id).unwrap());
    
    headers
}


fn header_asking_ai(demo_id: &str, conversation_id: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("Host", HeaderValue::from_static("playground.julius.ai"));
    headers.insert("sec-ch-ua", HeaderValue::from_static("\"Google Chrome\";v=\"119\", \"Chromium\";v=\"119\", \"Not?A_Brand\";v=\"24\""));
    headers.insert("use-dict", HeaderValue::from_static("false"));
    headers.insert("sec-ch-ua-mobile", HeaderValue::from_static("?1"));
    headers.insert("USER_AGENT", HeaderValue::from_static("Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Mobile Safari/537.3623"));
    headers.insert("cypress-test-id", HeaderValue::from_static("None"));
    headers.insert("gcs", HeaderValue::from_static("true"));
    headers.insert("interactive-charts", HeaderValue::from_static("true"));
    headers.insert("sec-ch-ua-platform", HeaderValue::from_static("\"Android\""));
    headers.insert("ACCEPT", HeaderValue::from_static("*/*"));
    headers.insert("ORIGIN", HeaderValue::from_static("https://julius.ai"));
    headers.insert("sec-fetch-site", HeaderValue::from_static("same-site"));
    headers.insert("sec-fetch-mode", HeaderValue::from_static("cors"));
    headers.insert("sec-fetch-dest", HeaderValue::from_static("empty"));
    headers.insert("REFERER", HeaderValue::from_static("https://julius.ai/"));
    headers.insert("ACCEPT_ENCODING", HeaderValue::from_static("gzip, deflate, br, zstd"));
    headers.insert("ACCEPT_LANGUAGE", HeaderValue::from_static("id-ID,id;q=0.9"));
    headers.insert("pathname", HeaderValue::from_static("/ai-chatbot"));
    headers.insert("authorization", HeaderValue::from_static("Bearer"));
    headers.insert("is-native", HeaderValue::from_static("false"));
    headers.insert("visitor-id", HeaderValue::from_static("undefined"));
    headers.insert("is-demo", HeaderValue::from_str(&demo_id).unwrap());
    headers.insert("conversation-id", HeaderValue::from_str(&conversation_id).unwrap());
    
    headers
}



pub async fn generate_demo_id() -> (bool, String) {
    let url_api = URL_AI_DEMO.to_string();
    let url = format!("{}/api/temp_user_id", url_api);
    let headers = header_generate_temp();

    let client = reqwest::Client::new();
    let response = client.get(url).headers(headers).send().await.unwrap();

    if response.status().is_success() {
        let body = response.text().await.unwrap();
        let json: Value = serde_json::from_str(&body).unwrap();
        let demo_id = json["temp_user_id"].as_str().unwrap().to_string();
        (true,demo_id)
    } else {
        (false,"".to_string())
    }
}


pub async fn generate_conversation_id(demo_id: &str) -> (bool, String, String) {
    let url_api = URL_AI_DEMO.to_string();
    let url = format!("{}/api/chat/start", url_api);
    let headers = header_generate_conversation_id(demo_id);
    
    let data = json!({
        "provider": "default",
        "server_type": "CPU",
        "template_id": null,
        "chat_type": null,
        "tool_preferences": null,
        "conversation_plan": null
    });
    let client = reqwest::Client::new();
    let response = client.post(url).headers(headers).json(&data).send().await.unwrap();

    if response.status().is_success() {
        let body = response.text().await.unwrap();
//        println!("{}", &body);
        let json: Value = serde_json::from_str(&body).unwrap();
        let demo_id = json["id"].as_str().unwrap().to_string();
        let conversation_id= json["user_id"].as_str().unwrap().to_string();
        (true, demo_id, conversation_id)
    } else {
//        let body = response.text().await.unwrap();
//        println!("{}", &body);
        (false,"".to_string(), "".to_string())
    }
}



pub async fn asking_ai(prompt: String) -> String {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in the .env file");
    let pool = PgPool::connect(&database_url).await
        .expect("Failed to create pool.");
    let latest_record = sqlx::query!("SELECT demo_id, conversation_id FROM user_data ORDER BY created_at DESC LIMIT 1")
    .fetch_one(&pool).await.expect("Failed to fetch latest record");
    let demo_id = latest_record.demo_id;
    let conversation_id = latest_record.conversation_id;



    let url_api = URL_AI_DEMO.to_string();
    let url = format!("{}/api/chat/message", url_api);
    let headers = header_asking_ai(&demo_id, &conversation_id);
    let prompt = format!("#gunakan bahasa indonesia \n #kamu irfan ai {}", prompt);
    let data = json!({
        "message": {
            "content": prompt
        },
        "provider": "default",
        "chat_mode": "auto",
        "client_version": "20240130",
        "theme": "light",
        "new_images": null,
        "new_attachments": null,
        "dataframe_format": "json",
        "selectedModels": "Llama 3.2"
    });
    
    let client = reqwest::Client::new();
    let response = client.post(url).headers(headers).json(&data).send().await.unwrap();
    let mut content = String::new();
    if response.status().is_success() {
        let body = response.text().await.unwrap();
        for line in body.lines() {
            let v: Value = serde_json::from_str(line).unwrap();
            if let Some(line_content) = v.get("content") {
                if let Some(content_str) = line_content.as_str() {
                    content.push_str(content_str);
                }
            }
        }
        content.to_string()
    } else {
        let body = response.text().await.unwrap();
        body.to_string()
    }
}


