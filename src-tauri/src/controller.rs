use actix_web::{web, App, HttpServer, HttpResponse, Responder, HttpRequest};
use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, Scope, TokenResponse, CsrfToken, AuthorizationCode};
use oauth2::reqwest::async_http_client;
use serde::{Deserialize, Serialize};

use once_cell::sync::Lazy;
use std::ops::Deref;
use std::sync::{Arc, Mutex, MutexGuard};
use std::env;
use dotenv::dotenv;


#[derive(Deserialize)]
struct AuthRequest {
    code: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GoogleUser {
    id: String,
    name: String,
    email: String,
    picture: String,
}

static LOGGED_USER: Lazy<Mutex<Option<GoogleUser>>> = Lazy::new(|| Mutex::new(None));


fn is_logged_user_none() -> bool {
    let logged_user_lock = LOGGED_USER.lock().unwrap();
    logged_user_lock.is_none()
}

pub async fn start_server() -> std::io::Result<()> {
    let _ = HttpServer::new(|| {
        App::new()
            .route("/auth", web::get().to(auth))
            .route("/callback", web::get().to(callback))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await;
    Ok(())
}

#[tauri::command]
pub async fn login_with_google() -> Result<(), String> {
    // Open system browser for OAuth login (use `webbrowser` crate)
    if let Err(err) = webbrowser::open("http://127.0.0.1:8080/auth") {
        return Err(format!("Failed to open browser: {:?}", err));
    }
    Ok(())
}

#[tauri::command]
pub fn get_user() -> Result<Option<GoogleUser>, String>{
    if is_logged_user_none(){
        let logged_user = crate::fileio::read_user_data();
        {
            let mut global_ref = LOGGED_USER.lock().unwrap();
            *global_ref = logged_user.clone();
        }
        Ok(logged_user)
    }else {
        println!("Sending from secure data");
        let global_ref = LOGGED_USER.lock().unwrap();
        Ok(global_ref.clone())
    }
}

fn set_user(google_user: GoogleUser){
    crate::fileio::write_user_data(google_user);
}

async fn auth() -> impl Responder {
    let client = get_oauth_client();
    let csrf_token = CsrfToken::new(uuid::Uuid::new_v4().to_string());
    let (auth_url, _) = client.authorize_url(|| csrf_token.clone() ).url();
    HttpResponse::Found().header("Location", auth_url.to_string()).finish()
}

async fn callback(req: HttpRequest) -> impl Responder {
    let query = web::Query::<AuthRequest>::from_query(req.query_string()).unwrap();
    let client = get_oauth_client();

    let auth_code = AuthorizationCode::new(query.code.clone());

    let token = client.exchange_code(auth_code)
        .request_async(async_http_client).await.unwrap();
    // Extract the access token
    let access_token = token.access_token().secret();
    println!("Access token : {:?}", access_token);

    // Use the access token to fetch user info
    let client = reqwest::Client::new();
    let response = client
        .get("https://www.googleapis.com/oauth2/v1/userinfo?alt=json")
        .bearer_auth(access_token) // Pass the token in the Authorization header
        .send()
        .await
        .unwrap();


    if !response.status().is_success() {
        return HttpResponse::InternalServerError().body("Failed to fetch user information.");
    }
    let body = response.text().await.unwrap();

    // If you still need to parse it:
    let user_data: GoogleUser = serde_json::from_str(&body).unwrap();
    set_user(user_data.clone());
    //let response = reqwest::get("https://www.googleapis.com/oauth2/v1/userinfo?alt=json")
    //    .await.unwrap();
    //println!("{:?}", response);
    //let user_data = response.json::<GoogleUser>().await.unwrap();
    //
    // Dynamically inject user info into HTML
    HttpResponse::Ok().content_type("text/html").body(format!(r#"
        <html>
            <body>
                <h1>Welcome, {name}!</h1>
                <p>You are now Logged IN! you may close this page.</p>
                <a href="/">Go Back</a>
            </body>
        </html>
    "#, name=user_data.name))
}

fn generate_auth_url() -> String {
    let client_id = std::env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID not set");
    let redirect_uri = "https://accounts.google.com/o/oauth2/auth".to_string();

    let scopes = vec![
        "https://www.googleapis.com/auth/userinfo.profile",
        "https://www.googleapis.com/auth/userinfo.email",
    ];

    let scope_string = scopes.join(" ");

    format!(
        "https://accounts.google.com/o/oauth2/auth?prompt=consent&scope={}&access_type=offline",
        scope_string
    )
}

fn get_oauth_client() -> BasicClient {

    dotenv().ok();
    match env::var("GOOGLE_CLIENT_ID") {
        Ok(value) => println!("APP_NAME:{} ", value),
        Err(e) => println!("Couldn't read APP_NAME: {}", e),
    }

    let client_id = ClientId::new(env::var("GOOGLE_CLIENT_ID").unwrap());
    let client_secret = ClientSecret::new(env::var("GOOGLE_CLIENT_SECRET").unwrap());
    let auth_url =generate_auth_url();
    println!("{}", auth_url);
    let auth_url = AuthUrl::new(auth_url).unwrap();
    let token_url = oauth2::TokenUrl::new("https://oauth2.googleapis.com/token".to_string()).unwrap();
    BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
        .set_redirect_uri(RedirectUrl::new("http://127.0.0.1:8080/callback".to_string()).unwrap())
}

