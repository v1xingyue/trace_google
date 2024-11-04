# trace_google

Use google cloud logging to trace the rust application.


## Example Code 

```rust
let project_id = std::env::var("project_id").unwrap();
let app_name = std::env::var("app_name").unwrap();
let logger = logger::GoogleLogger::new(project_id, app_name);
logger
    .log(&json!({ "message": "rust logging","value":random_value,"ok":true }))
    .await
.unwrap();
```


## How to query the logs 

1. Go to the [Google Cloud Logging](https://console.cloud.google.com/logs/viewer)
2. Select the project and the log name
3. Write the query, for example:

```shell
resource.type="global"
logName="projects/YOUR_PROJECT_ID/logs/YOUR_APP_NAME"
```