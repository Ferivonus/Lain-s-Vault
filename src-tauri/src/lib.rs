use serde_json::{self, json, Value};
use std::fs::{create_dir_all, File};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::Command;
use tauri::Manager;

const CONFIG_FILE: &str = "markdown_files/markdown_config.json";
const DEFAULT_MARKDOWN_DIR: &str = "markdown_files";
// C# API's base URL production api:
// const CSHARP_API_URL: &str = "http://localhost:5000/Lain";
// C# API's base URL debug use api:
const CSHARP_API_URL: &str = "https://localhost:7102/Lain";

fn read_config() -> Result<Value, String> {
    match create_dir_all(DEFAULT_MARKDOWN_DIR) {
        Ok(_) => println!(
            "Successfully created the directory: {}",
            DEFAULT_MARKDOWN_DIR
        ),
        Err(e) => return Err(format!("Failed to create directory: {}", e)),
    }

    let config_data = if let Ok(mut file) = File::open(CONFIG_FILE) {
        let mut content = String::new();
        file.read_to_string(&mut content)
            .map_err(|e| format!("Failed to read config file: {}", e))?;
        serde_json::from_str(&content).map_err(|e| format!("Failed to parse config JSON: {}", e))?
    } else {
        let initial_data = json!({ "users": {} });
        match File::create(CONFIG_FILE) {
            Ok(mut file) => {
                file.write_all(initial_data.to_string().as_bytes())
                    .map_err(|e| format!("Failed to create config file: {}", e))?;
                println!("Config file created successfully: {}", CONFIG_FILE);
            }
            Err(e) => return Err(format!("Failed to create config file: {}", e)),
        }

        initial_data
    };

    Ok(config_data)
}

fn write_config(config_data: &Value) -> Result<(), String> {
    let mut file = File::create(CONFIG_FILE).map_err(|e| e.to_string())?;
    file.write_all(config_data.to_string().as_bytes())
        .map_err(|e| e.to_string())
}

fn get_or_create_user_folder(user_name: String) -> Result<(), String> {
    let mut config_data = read_config()?;

    let users = config_data["users"]
        .as_object_mut()
        .ok_or("Failed to parse users list")?;

    if !users.contains_key(&user_name) {
        users.insert(user_name.clone(), json!([]));
        write_config(&config_data)?;
    }

    Ok(())
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn save_user_name(user_name: String) -> Result<(), String> {
    if user_name.is_empty() {
        return Err("User name cannot be empty.".to_string());
    }

    println!("Attempting to save user name: {}", user_name);

    let result = get_or_create_user_folder(user_name.clone());

    match result {
        Ok(_) => {
            eprintln!("User folder created successfully.");
            Ok(())
        }
        Err(e) => {
            eprintln!("Error creating user folder: {}", e);
            Err(e)
        }
    }
}

#[tauri::command]
fn load_user_names() -> Result<Vec<String>, String> {
    let config_data = read_config()?;
    let users = config_data["users"]
        .as_object()
        .ok_or("Failed to parse users list")?;

    Ok(users.keys().cloned().collect())
}

#[tauri::command]
fn save_markdown(user_name: String, filename: String, content: String) -> Result<(), String> {
    create_dir_all(DEFAULT_MARKDOWN_DIR).map_err(|e| e.to_string())?;

    let user_directory = PathBuf::from(DEFAULT_MARKDOWN_DIR).join(&user_name);
    create_dir_all(&user_directory).map_err(|e| e.to_string())?;

    let path = user_directory.join(format!("{}.md", filename));
    let mut file = File::create(&path).map_err(|e| e.to_string())?;
    file.write_all(content.as_bytes())
        .map_err(|e| e.to_string())?;

    let mut config_data = read_config()?;
    if let Some(files) = config_data["users"]
        .get_mut(&user_name)
        .and_then(|v| v.as_array_mut())
    {
        files.push(Value::String(filename));
    }
    write_config(&config_data)?;

    Ok(())
}

#[tauri::command]
fn load_markdown(user_name: String, filename: String) -> Result<String, String> {
    let user_directory = PathBuf::from(DEFAULT_MARKDOWN_DIR).join(&user_name);
    let path = user_directory.join(format!("{}.md", filename));
    let mut file = File::open(&path).map_err(|e| e.to_string())?;
    let mut content = String::new();
    file.read_to_string(&mut content)
        .map_err(|e| e.to_string())?;
    Ok(content)
}

#[tauri::command]
fn list_user_markdown_files(user_name: String) -> Result<Vec<String>, String> {
    let user_directory = PathBuf::from(DEFAULT_MARKDOWN_DIR).join(&user_name);

    if !user_directory.exists() {
        return Err("User directory not found.".to_string());
    }

    let mut files = Vec::new();
    for entry in std::fs::read_dir(&user_directory).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.extension().map(|s| s == "md").unwrap_or(false) {
            if let Some(filename) = path.file_stem().and_then(|s| s.to_str()) {
                files.push(filename.to_string());
            }
        }
    }

    Ok(files)
}

#[derive(serde::Serialize, serde::Deserialize)]
struct EncryptionRequest {
    key: String,
    iv: String,
    filePath: String,
    excludedFiles: Vec<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct DecryptionRequest {
    key: String,
    iv: String,
    filePath: String,
    excludedFiles: Vec<String>,
}

#[tauri::command]
async fn encrypt_files(
    filePath: String,
    excludedFiles: Vec<String>,
    key: String,
    iv: String,
) -> Result<String, String> {
    let exe_dir = std::env::current_dir().map_err(|e| e.to_string())?;
    let user_directory = exe_dir.join(DEFAULT_MARKDOWN_DIR).join(&filePath);

    if !user_directory.exists() {
        return Err("User directory not found.".to_string());
    }

    println!("Sending folder path: {}", user_directory.to_string_lossy());
    println!("Sending excludedFiles {:?}", excludedFiles);

    let request = EncryptionRequest {
        key,
        iv,
        filePath: user_directory.to_string_lossy().to_string(),
        excludedFiles: excludedFiles.clone(),
    };

    let client = reqwest::Client::new();
    let response = client
        .post(&format!("{}/encrypt", CSHARP_API_URL))
        .json(&request)
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?;

    if !response.status().is_success() {
        return Err("Error occurred during encryption.".to_string());
    }

    Ok("Encryption request sent with full folder path.".to_string())
}

#[tauri::command]
async fn decrypt_files(
    filePath: String,
    key: String,
    iv: String,
    excludedFiles: Vec<String>,
) -> Result<String, String> {
    let exe_dir = std::env::current_dir().map_err(|e| e.to_string())?;
    let user_directory = exe_dir.join(DEFAULT_MARKDOWN_DIR).join(&filePath);

    if !user_directory.exists() {
        return Err("User directory not found.".to_string());
    }

    println!("Sending folder path: {}", user_directory.to_string_lossy());
    println!("Sending excludedFiles {:?}", excludedFiles);

    let request = DecryptionRequest {
        key,
        iv,
        filePath: user_directory.to_string_lossy().to_string(),
        excludedFiles,
    };

    let client = reqwest::Client::new();
    let response = client
        .post(&format!("{}/decrypt", CSHARP_API_URL))
        .json(&request)
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?;

    if !response.status().is_success() {
        return Err("Error occurred during decryption.".to_string());
    }

    Ok("Decryption request sent with full folder path.".to_string())
}

#[tauri::command]
async fn check_connection() -> Result<String, String> {
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/ConnectionCheck", CSHARP_API_URL))
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?;

    if response.status().is_success() {
        let body = response
            .text()
            .await
            .map_err(|e| format!("Failed to read response: {}", e))?;
        Ok(body)
    } else {
        Err("Error occurred while checking connection.".to_string())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            save_user_name,
            load_user_names,
            save_markdown,
            load_markdown,
            list_user_markdown_files,
            encrypt_files,
            decrypt_files,
            check_connection
        ])
        .setup(|app| {
            // Start the microservice by running the command
            /*
            if let Err(e) = Command::new("lain vault.exe").spawn() {
                 eprintln!("Failed to start the microservice: {:?}", e);
             }

             */
            let main_window = app.get_webview_window("main").unwrap();
            main_window.set_title("Lain’s Vault").unwrap();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}
