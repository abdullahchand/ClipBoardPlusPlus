use keyring::{Entry, Error};
use sys_info;

use base64::{Engine as _, engine::{self, general_purpose}};
use rand::RngCore;

const APP_NAME: &str = "clipboardplusplus";

fn generate_key() -> [u8; 32] {
    let mut key = [0u8; 32]; // 256-bit key for AES-256
    rand::thread_rng().fill_bytes(&mut key);
    key
}

fn retrieve_key() {
    
}

fn get_hardware_uuid() -> Result<String, sys_info::Error> {
    // Fetch the hardware UUID (unique for each device)
    sys_info::hostname().map(|uuid| uuid.trim().to_string())
}

fn store_key_with_uuid(purpose: String) -> Result<(), Error> {
    // Retrieve the hardware UUID
    let uuid = get_hardware_uuid().expect("Failed to retrieve hardware UUID");
    let final_path = uuid + &purpose;
    let key = generate_key();

    // Use UUID as the username for storing the key
    let entry = Entry::new(APP_NAME, &final_path).unwrap();
    entry.set_password(&general_purpose::STANDARD.encode(key))?;
    Ok(())
}

fn retrieve_key_with_uuid(purpose: String) -> Result<Vec<u8>, String> {
    // Retrieve the hardware UUID
    let uuid = get_hardware_uuid().expect("Failed to retrieve hardware UUID");
    let final_path = uuid + &purpose;
    // Use UUID as the username for retrieving the key
    let entry = Entry::new(APP_NAME, &final_path);
    match entry {
            Ok(val) => {
                let encoded_key = val.get_password().unwrap();
                Ok(general_purpose::STANDARD
                    .decode(&encoded_key).unwrap())
        },
        Err(_) => {
            Err("Failed to retrieve value".to_string())
        }
    }

}

