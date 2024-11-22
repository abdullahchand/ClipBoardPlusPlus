#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod clipboard_functions;
mod tray;
mod fileio;
mod controller;
use tokio;

#[tokio::main]
async fn main() {
    //controller::start_server();
    //clipboardpro_lib::run();
    tokio::join!(
        async {
            controller::start_server().await.unwrap();
        },
        async {
            clipboardpro_lib::run();
        }
    );
}
