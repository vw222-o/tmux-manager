use std::{process::Command, sync::Mutex};

use regex::Regex;
use tauri::Manager;

#[derive(Default)]
struct Session {
    name: String,
    created: String,
}

impl Session {
    pub fn from_tmux_line(line: &str) -> Session {
        // mc: 1 windows (created Wed Dec 31 14:23:42 2025)

        let regex = Regex::new(r"(\: \d windows \(created )|\)").expect("invalid regex");
        let split = regex.split(line);
        

        todo!("hi")
    }
}

#[derive(Default)]
struct AppState {
    sessions: Vec<Session>,
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn refresh_sessions() -> String {
    let result = Command::new("tmux")
        .arg("list-sessions")
        .output()
        .expect("tmux didnt work lol");

    let output = str::from_utf8(&result.stdout)
        .expect("some stdout error?")
        .to_string();

    let split = output.split("\n");

    let mut sessions: Vec<Session> = Vec::default();
    for line in split {
        sessions.push(Session::from_tmux_line(line));
    }

    String::from("mogus")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(Mutex::new(AppState::default()));
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![refresh_sessions])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
