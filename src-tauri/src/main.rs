#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    print!("Message!");
    return format!("Hello, {}! You've been greeted from Rust!", name);
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}