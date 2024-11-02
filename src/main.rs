use reqwest::Client;
use serde_json::json;
use std::error::Error;

async fn get_token_from_metadata_server() -> Result<String, Box<dyn Error>> {
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

async fn send_log(token: &str) -> anyhow::Result<()> {
    // Google Cloud Logging 的 API 端点
    let url = "https://logging.googleapis.com/v2/entries:write";

    // 创建日志条目请求体
    let log_entry = json!({
        "logName": "projects/level-poetry-395302/logs/my-log",  // 将 YOUR_PROJECT_ID 替换为你的 Google Cloud 项目 ID
        "resource": {
            "type": "global"
        },
        "entries": [
            {
                "textPayload": "Hello, Google Cloud Logging!"
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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let result = match get_token_from_metadata_server().await {
        Ok(token) => send_log(&token).await,
        Err(e) => {
            eprintln!("Error: {:?}", e);
            Err(anyhow::anyhow!("Failed to get token: {:?}", e))
        }
    }?;

    Ok(())
}
