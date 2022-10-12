#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
extern crate objc;


use tauri::{Manager, WindowEvent};

mod window_ext;

use window_ext::WindowExt;
use window_ext::ToolbarThickness;

use std::env;
use std::process::Command;
#[tauri::command]

fn open_in_explorer(path: &str) {
    // FOR OTHER OS REFER - https://doc.rust-lang.org/std/env/consts/constant.OS.html
    // REF - https://github.com/tauri-apps/tauri/issues/4062
    // TARGET - WINDOWS
    if env::consts::OS == "windows" {
        Command::new("explorer")
            .args(["/select,", path])
            .spawn()
            .unwrap();
    } else if env::consts::OS == "linux" {
        Command::new("explorer")
            .args(["/select,", path])
            .spawn()
            .unwrap();
    }
}

#[tauri::command]
fn open_terminal(path: &str) {
    if env::consts::OS == "windows" {
        // programs for windows: [cmd, powershell, wt]
        // programs for ubuntu: [gnome-terminal]
        // .args(["/C", "start", "wt"])
        Command::new("cmd")
        .args(["/C", "wt", "-d", path])
        .spawn()
        .unwrap();
    }
    else if env::consts::OS =="macos" {
        Command::new("zsh")
        .arg(format!("--working-directory={}", path).as_str())
        .spawn()
        .unwrap();
    } else {
        Command::new("gnome-terminal")
        .arg(format!("--working-directory={}", path).as_str())
        .spawn()
        .unwrap();
    }
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let win = app.get_window("main").unwrap();
            win.set_transparent_titlebar(ToolbarThickness::Medium);
            Ok(())
        })
        .on_window_event(|e| {
            if let WindowEvent::Resized(..) = e.event() {
                let win = e.window();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
