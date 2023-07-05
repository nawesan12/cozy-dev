// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use reqwest::Error;
use serde_json::json;
use uuid::Uuid;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn generar_uuid() -> String {
    let my_uuid = Uuid::new_v4();
    my_uuid.to_string()
}

#[tokio::main]
async fn call_data_from_api() -> Result<(), Error> {
    let res = reqwest::get("https://api.github.com/events")
        .await?
        .text()
        .await?;

    println!("{}", res);

    Ok(())
}

#[tokio::main]
async fn send_data_to_api() -> Result<(), Error> {
    let client = reqwest::Client::new();
    let res = client
        .post("http://httpbin.org/post")
        .json(&json!({
            "key": "value"
        }))
        .send()
        .await?;

    println!("{}", res.text().await?);

    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
