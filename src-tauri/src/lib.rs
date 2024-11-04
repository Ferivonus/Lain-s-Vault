use serde_json::{self, json, Value};
use std::fs::{create_dir_all, File};
use std::io::{Read, Write};
use std::path::PathBuf;
use tauri::Manager;

const CONFIG_FILE: &str = "markdown_files/markdown_config.json";
const DEFAULT_MARKDOWN_DIR: &str = "markdown_files";

fn read_config() -> Result<Value, String> {
    // Klasörün varlığını kontrol et ve oluştur
    match create_dir_all(DEFAULT_MARKDOWN_DIR) {
        Ok(_) => println!(
            "Successfully created the directory: {}",
            DEFAULT_MARKDOWN_DIR
        ),
        Err(e) => return Err(format!("Failed to create directory: {}", e)),
    }

    // Read the configuration file or create a default one if it doesn't exist
    let config_data = if let Ok(mut file) = File::open(CONFIG_FILE) {
        let mut content = String::new();
        file.read_to_string(&mut content)
            .map_err(|e| format!("Failed to read config file: {}", e))?; // Hata mesajı
        serde_json::from_str(&content).map_err(|e| format!("Failed to parse config JSON: {}", e))?
    } else {
        // Eğer config dosyası yoksa, başlangıç verileri ile yeni dosya oluştur
        let initial_data = json!({ "users": {} });
        match File::create(CONFIG_FILE) {
            Ok(mut file) => {
                file.write_all(initial_data.to_string().as_bytes())
                    .map_err(|e| format!("Failed to create config file: {}", e))?; // Hata mesajı
                println!("Config file created successfully: {}", CONFIG_FILE); // Başarılı mesajı
            }
            Err(e) => return Err(format!("Failed to create config file: {}", e)),
        }

        initial_data // İlk verileri döndür
    };

    Ok(config_data)
}

fn write_config(config_data: &Value) -> Result<(), String> {
    // Write updated data to `config.json`
    let mut file = File::create(CONFIG_FILE).map_err(|e| e.to_string())?;
    file.write_all(config_data.to_string().as_bytes())
        .map_err(|e| e.to_string())
}

fn get_or_create_user_folder(user_name: String) -> Result<(), String> {
    let mut config_data = read_config()?;

    // Get user list or initialize it if missing
    let users = config_data["users"]
        .as_object_mut()
        .ok_or("Failed to parse users list")?;

    // Add the new user if it doesn’t already exist
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

    println!("Attempting to save user name: {}", user_name); // Hata ayıklama mesajı

    // Kullanıcı klasörünü al veya oluştur
    let result = get_or_create_user_folder(user_name.clone());

    match result {
        Ok(_) => {
            eprintln!("User folder created successfully."); // Başarılı mesajı
            Ok(())
        }
        Err(e) => {
            eprintln!("Error creating user folder: {}", e); // Hata mesajı
            Err(e)
        }
    }
}

// Command to load the user names from the config file
#[tauri::command]
fn load_user_names() -> Result<Vec<String>, String> {
    let config_data = read_config()?;
    let users = config_data["users"]
        .as_object()
        .ok_or("Failed to parse users list")?;

    Ok(users.keys().cloned().collect())
}

// Command to save markdown content to the default markdown directory under a specific user
#[tauri::command]
fn save_markdown(user_name: String, filename: String, content: String) -> Result<(), String> {
    // Create the default markdown directory if it doesn't exist
    create_dir_all(DEFAULT_MARKDOWN_DIR).map_err(|e| e.to_string())?;

    // Create a user-specific directory
    let user_directory = PathBuf::from(DEFAULT_MARKDOWN_DIR).join(&user_name);
    create_dir_all(&user_directory).map_err(|e| e.to_string())?;

    // Construct the full file path in the user directory
    let path = user_directory.join(format!("{}.md", filename));
    let mut file = File::create(&path).map_err(|e| e.to_string())?;
    file.write_all(content.as_bytes())
        .map_err(|e| e.to_string())?;

    // Update the user's folder in the config
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

// Command to load markdown content from a specified file in the default markdown directory

#[tauri::command]
fn load_markdown(user_name: String, filename: String) -> Result<String, String> {
    // Construct the user-specific directory
    let user_directory = PathBuf::from(DEFAULT_MARKDOWN_DIR).join(&user_name);
    // Construct the full file path in the user's directory
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

    // Check if the user directory exists
    if !user_directory.exists() {
        return Err("User directory not found.".to_string());
    }

    // List markdown files in the user-specific directory
    let mut files = Vec::new();
    for entry in std::fs::read_dir(&user_directory).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.extension().map(|s| s == "md").unwrap_or(false) {
            // Extract the filename without the extension
            if let Some(filename) = path.file_stem().and_then(|s| s.to_str()) {
                files.push(filename.to_string());
            }
        }
    }

    Ok(files)
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
            list_user_markdown_files
        ])
        .setup(|app| {
            // Additional setup can be done here
            let main_window = app.get_webview_window("main").unwrap();
            main_window.set_title("Lain’s Vault").unwrap();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}
