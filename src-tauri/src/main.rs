// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn login(username: &str, password: &str) -> String {
    println!("Message from rust: Username {}", username);
    println!("Message from rust: Password {}", password);
    if username == "admin" {
        if password == "admin" {
            println!("Message from rust: logged");
            return format!("Hello User {}", username)
        }
        println!("Message from Rust: Wrong password!");
        return format!("Wrong password!")
    }
    println!("Message from Rust: Wrong password and username!");
    format!("Wrong password and username")
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![login])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
