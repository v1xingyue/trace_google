mod logger;

use rand::Rng;
use serde_json::json;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    // generate a random number between 1 and 100
    let random_value = rand::thread_rng().gen_range(200..300);
    println!("current number : {}", random_value);

    let project_id = std::env::var("project_id").unwrap();
    let app_name = std::env::var("app_name").unwrap();
    let logger = logger::GoogleLogger::new(project_id, app_name);
    logger
        .log(&json!({ "message": "rust logging","value":random_value,"ok":true }))
        .await
        .unwrap();

    Ok(())
}
