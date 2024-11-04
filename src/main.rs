use rand::Rng;
use reqwest::Client;
use serde::Serialize;
use serde_json::json;
use std::error::Error;

struct GoogleLogger {
    project_id: String,
    app_name: String,
}

impl GoogleLogger {
    fn get_log_name(&self) -> String {
        format!("projects/{}/logs/{}", self.project_id, self.app_name)
    }

    pub fn new(project_id: String, app_name: String) -> Self {
        Self {
            project_id,
            app_name,
        }
    }

    pub async fn log<T: Serialize>(&self, data: &T) -> Result<(), Box<dyn Error>> {
        let token = self.get_token().await?;
        // Google Cloud Logging 的 API 端点
        let url = "https://logging.googleapis.com/v2/entries:write";

        // 创建日志条目请求体
        let log_entry = json!({
            "logName": self.get_log_name(),
            "resource": {
                "type": "global",
                "labels": {
                    "from": "rust"
                }
            },
            "entries": [
                {
                    "jsonPayload": data,
                }
            ]
        });

        // 创建 HTTP 客户端
        let client = Client::new();

        // 发送 POST 请求
        let response = client
            .post(url)
            .bearer_auth(token) // 添加 Authorization header
            .json(&log_entry) // 添加 JSON 请求体
            .send()
            .await?;

        // 检查响应状态
        if response.status().is_success() {
            println!("Log entry sent successfully!");
        } else {
            let error_message = response.text().await?;
            eprintln!("Failed to send log entry: {:?}", error_message);
        }
        Ok(())
    }

    async fn get_token(&self) -> Result<String, Box<dyn Error>> {
        let client: Client = Client::new();
        let response = client
            .get("http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/default/token")
            .header("Metadata-Flavor", "Google") // 必须添加此 header
            .send()
            .await?;
        if response.status().is_success() {
            let token_info: serde_json::Value = response.json().await?;
            let token = token_info["access_token"]
                .as_str()
                .ok_or("Failed to retrieve access_token")?
                .to_string();
            Ok(token)
        } else {
            Err("Failed to retrieve token from metadata server".into())
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    // generate a random number between 1 and 100
    let random_value = rand::thread_rng().gen_range(200..300);
    println!("current number : {}", random_value);
    let project_id = std::env::var("project_id").unwrap();
    let app_name = std::env::var("app_name").unwrap();
    let logger = GoogleLogger::new(project_id, app_name);
    logger
        .log(&json!({ "message": "rust logging","value":random_value,"ok":true }))
        .await
        .unwrap();
    Ok(())
}
