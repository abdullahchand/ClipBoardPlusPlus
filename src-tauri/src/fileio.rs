use dirs_next::data_local_dir;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::fs;


fn get_app_data_path() -> String {
    let base_dir = data_local_dir().expect("Failed to get data directory");
    let app_dir = base_dir.join("your-app-name");
    if !app_dir.exists() {
        create_dir_all(&app_dir).expect("Failed to create app data directory");
    }
    app_dir.join("user.json").to_string_lossy().to_string()
}

pub fn write_user_data(google_user: crate::controller::GoogleUser){
    let user_path = get_app_data_path();
    println!("Path for hidden user data: {}", user_path);

    let user_data= serde_json::to_string(&google_user).expect("Failed to serialize struct");
    let mut file = File::create(&user_path).expect("Failed to create file");
    file.write_all(user_data.as_bytes()).expect("Failed to write data");
}

pub fn read_user_data() -> Option<crate::controller::GoogleUser>{
    let file_path = get_app_data_path();
    let json_data = fs::read_to_string(file_path).map_err(|e| e.to_string());
    match json_data {
        Ok(user_data) => {
            let user = serde_json::from_str(&user_data);
            match user {
                Ok(exists) =>  {
                    let logged_user: crate::controller::GoogleUser = exists;
                    Some(logged_user)
                },
                Err(_)=> None
            }
        }
        ,
        Err(_) => None
    }
}

#[tauri::command]
pub fn logout_user(){
    let file_path = get_app_data_path();
    match fs::remove_file(file_path) {
        Ok(_) => println!("File deleted successfully!"),
        Err(e) => eprintln!("Failed to delete the file: {}", e),
    }
}

