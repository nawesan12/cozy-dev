// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use reqwest::Error;
use serde_json::json;
use uuid::Uuid;

use std::{
    error::Error as StdError,
    fmt::{Display, Formatter, Result as FmtResult},
};

#[derive(Debug)]
enum CustomError {
    Reqwest(reqwest::Error),
    // Aquí puedes agregar otros tipos de errores según lo necesites
}

impl Display for CustomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            CustomError::Reqwest(err) => write!(f, "Reqwest Error: {}", err),
            // Agrega las representaciones en string de los otros tipos de errores aquí
        }
    }
}

impl StdError for CustomError {}

impl From<reqwest::Error> for CustomError {
    fn from(err: reqwest::Error) -> CustomError {
        CustomError::Reqwest(err)
    }
}

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

#[tauri::command]
async fn get_weather_data_from_api(city: String) -> Result<serde_json::Value, String> {
    let client = reqwest::get(
        "https://weather.visualcrossing.com/VisualCrossingWebServices/rest/services/timeline/"
            .to_owned()
            + &city
            + "?unitGroup=metric&key=CTHL8CX6AF99E6W3KJG9MF9JH&contentType=json",
    )
    .await
    .map_err(|e| format!("Reqwest Error: {}", e))?
    .json::<serde_json::Value>()
    .await
    .map_err(|e| format!("Reqwest Error: {}", e))?;

    Ok(client)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![get_weather_data_from_api])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
