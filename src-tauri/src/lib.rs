use std::{process::Command, sync::Mutex};

use regex::Regex;
use serde::{Deserialize, Serialize, ser::SerializeStruct};
use tauri::{Manager, State};

#[derive(Default)]
struct Session {
    name: String,
    created: String,
}

impl Session {
    pub fn from_tmux_line(line: &str) -> Session {
        // mc: 1 windows (created Wed Dec 31 14:23:42 2025)

        let regex = Regex::new(r"(\: \d windows \(created )|\)").expect("invalid regex");
        let mut split = regex.split(line);

        let (name, created) = (split.next(), split.next());
        Session {
            name: name.unwrap_or_default().to_string(),
            created: created.unwrap_or_default().to_string(),
        }
    }
}

impl Serialize for Session {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("Session", 2)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("created", &self.created)?;
        s.end()
    }
}

#[derive(Default)]
struct AppState {
    sessions: Vec<Session>,
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn refresh_sessions(state: State<'_, Mutex<AppState>>) -> String {
    let result = Command::new("tmux")
        .arg("list-sessions")
        .output()
        .expect("tmux didnt work lol");

    let output = str::from_utf8(&result.stdout)
        .expect("some stdout error")
        .to_string();

    let parts: Vec<&str> = output.split('\n').collect();
    let parts = &parts[..parts.len().saturating_sub(1)];

    let mut sessions: Vec<Session> = Vec::default();
    for line in parts {
        sessions.push(Session::from_tmux_line(line));
    }

    let mut state = state.lock().unwrap();
    state.sessions = sessions;

    String::from("Success!")
}

#[tauri::command]
fn get_sessions(state: State<'_, Mutex<AppState>>) -> String {
    let state = state.lock().unwrap();
    
    serde_json::to_string(&state.sessions).unwrap()
}

#[derive(Deserialize)]
struct NativeCommand {
    name: String,
    args: Vec<String>
}

#[tauri::command]
fn run_command(command: NativeCommand) -> String {
    let result = Command::new(command.name)
        .args(command.args)
        .output()
        .expect("command failed!");

    let output = str::from_utf8(&result.stdout)
        .expect("some stdout error")
        .to_string();
    output
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(Mutex::new(AppState::default()));
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![refresh_sessions, get_sessions, run_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
