// // Prevents additional console window on Windows in release, DO NOT REMOVE!!
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// // Learn more about Tauri commands at https://v1.tauri.app/v1/guides/features/command
// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// }

// fn main() {
//     tauri::Builder::default()
//         .invoke_handler(tauri::generate_handler![greet])
//         .run(tauri::generate_context!())
//         .expect("error while running tauri application");
// }





// use tauri::api::dialog::FileDialog;
// use tauri::api::dialog::blocking::FileDialog;

use tauri::api::dialog::blocking::FileDialog;

// use tauri::Manager;

// fn open_folder() -> Option<String>{
//     let(tx,rx) = std::sync::mpsc::channel();

//     FileDialog::new()
//         .set_directory(".")
//         .pick_folder(move |folder|{
//             if let Some(path) = folder{
//                 tx.send(path.to_string_lossy().to_string()).unwrap();
//             }
//         });

//         rx.recv().ok()
// }

// fn main(){
//     tauri::Builder::default()
//         .invoke_handler(tauri::generate_handler![open_folder])
//         .run(tauri::generate_context!())
//         .expect("error while running tauri application");
// }



// #[tauri::command]
// fn open_folder() -> Option<String> {
//     FileDialog::new()
//         .set_directory(".") // Start from current directory
//         .pick_folder()
//         .map(|path| path.to_string_lossy().to_string())
// }

// fn main() {
//     tauri::Builder::default()
//         .invoke_handler(tauri::generate_handler![open_folder]) // Ensure function is registered
//         .run(tauri::generate_context!())
//         .expect("error while running Tauri application");
// }



#[tauri::command]
async fn open_folder(window: tauri::Window) -> Option<String> {
    let path = window
        .dialog()
        .blocking()
        .pick_folder(); // Open folder picker

    path.map(|p| p.to_string_lossy().to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![open_folder]) // Ensure function is registered
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}
