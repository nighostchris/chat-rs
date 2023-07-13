use dotenvy::dotenv;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

// Read environment variables from '.env' file
pub fn load_env_vars() {
    match dotenv() {
        Ok(_) => println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({
              "function": "load_env_vars",
              "level": "info",
              "message": "environment variables successfully loaded",
              "target": "chat_rs::config",
              "timestamp": OffsetDateTime::now_utc().format(&Rfc3339).unwrap()
            }))
            .unwrap()
        ),
        Err(_) => panic!("Failed to load environment variables from '.env' file"),
    }
}
