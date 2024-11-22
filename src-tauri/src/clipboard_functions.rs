use chrono::{DateTime, Utc};
use serde::Serialize;
use arboard::Clipboard;
use once_cell::sync::Lazy;
use std::os::linux::net::SocketAddrExt;
use std::sync::Mutex;
use std::error::Error;

#[derive(Serialize)]
pub struct ClipboardStruct{
    id: i32,
    content: String,
    created: i64
}

static ORIG_RESULT: Lazy<Mutex<Option<String>>> = Lazy::new(|| Mutex::new(None));

fn read_clipboard() -> Result<String, Box<dyn std::error::Error>> {
    let mut clipboard = Clipboard::new()?;
    let content = clipboard.get_text()?;
    Ok(content)
}

pub fn write_clipboard(content: &str) -> Result<Option<i32>, Box<dyn std::error::Error>> {
    let mut clipboard = Clipboard::new()?;
    let mut orig_result = ORIG_RESULT.lock().unwrap();
    *orig_result = Some(content.to_string());
    clipboard.set_text(content)?;
    Ok(Some(200))
}



#[tauri::command]
pub fn monitor_clipboard_changes() -> Result<Option<ClipboardStruct>, String> {
    // Read the initial clipboard content
    let clipboard_result: Result<String, Box<dyn Error>> = read_clipboard();

    match clipboard_result {
        Ok(data) => {
            let mut orig_result = ORIG_RESULT.lock().unwrap();

            // Check if there's a previous value and if it's different
            if orig_result.as_ref() != Some(&data) {
                println!("Clipboard Changed: {:?}", data);

                // Update the stored result
                *orig_result = Some(data.clone());
                let timestamp = Utc::now();
                let clipboard_content = ClipboardStruct {
                    id: 1,
                    content: Some(data.clone().to_string()).unwrap(),
                    created: timestamp.timestamp_millis()
                };

                return Ok(Some(clipboard_content))
            }
            Ok(None)

        },
        Err(err) => {
            println!("{}", err);
            return Err(err.to_string())
        },
    }
}
