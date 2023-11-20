use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use reqwest::Client as HttpClient;
use async_trait::async_trait;
use actix_files::Files;
use tokio::time::Duration;
use std::sync::Mutex;
use std::collections::HashMap;
use std::fs;
use std::io::Write;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Nutrition {
    id: u64,
    name: String,
    requirements: String,
    allergies: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    id: u64,
    username: String,
    password: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Database {
    nutrition: HashMap<u64, Nutrition>,
    users: HashMap<u64, User>
}

impl Database {
    fn new() -> Self {
        Self {
            nutrition: HashMap::new(),
            users: HashMap::new()
        }
    }

    // CRUD DATA
    fn insert(&mut self, nutrition: Nutrition) {
        self.nutrition.insert(nutrition.id, nutrition);
    }

    fn get(&self, id: &u64) -> Option<&Nutrition> {
        self.nutrition.get(id)
    }

    fn get_all(&self) -> Vec<&Nutrition> {
        self.nutrition.values().collect()
    }

    fn delete(&mut self, id: &u64) {
        self.nutrition.remove(id);
    }

    fn update(&mut self, nutrition: Nutrition) {
        self.nutrition.insert(nutrition.id, nutrition);
    }

    // USER DATA RELATED FUNCTIONS
    fn insert_user(&mut self, user: User) {
        self.users.insert(user.id, user);
    }

    fn get_user_by_name(&self, username: &str) -> Option<&User> {
        self.users.values().find(|u| u.username == username)
    }

    // DATABASE SAVING
    fn save_to_file(&self) -> std::io::Result<()> {
        let data: String = serde_json::to_string(&self)?;
        let mut file: fs::File = fs::File::create("database.json")?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }

    fn load_from_file() -> std::io::Result<Self> {
        let file_content: String = fs::read_to_string("database.json")?;
        let db: Database = serde_json::from_str(&file_content)?;
        Ok(db)
    }
}

struct AppState {
    db: Mutex<Database>
}

async fn create_nutrition(app_state: web::Data<AppState>, nutrition: web::Json<Nutrition>) -> impl Responder {
    let mut db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    db.insert(nutrition.into_inner());
    let _ = db.save_to_file();
    HttpResponse::Ok().finish()
}

async fn read_nutrition(app_state: web::Data<AppState>, id: web::Path<u64>) -> impl Responder {
    let db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    match db.get(&id.into_inner()) {
        Some(nutrition) => HttpResponse::Ok().json(nutrition),
        None => HttpResponse::NotFound().finish()
    }
}

async fn read_all_nutrition(app_state: web::Data<AppState>) -> impl Responder {
    let db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    let nutrition = db.get_all();
    HttpResponse::Ok().json(nutrition)
}

async fn update_nutrition(app_state: web::Data<AppState>, nutrition: web::Json<Nutrition>) -> impl Responder {
    let mut db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    db.update(nutrition.into_inner());
    let _ = db.save_to_file();
    HttpResponse::Ok().finish()
}

async fn delete_nutrition(app_state: web::Data<AppState>, id: web::Path<u64>) -> impl Responder {
    let mut db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    db.delete(&id.into_inner());
    let _ = db.save_to_file();
    HttpResponse::Ok().finish()
}

async fn register(app_state: web::Data<AppState>, user: web::Json<User>) -> impl Responder {
    let mut db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    db.insert_user(user.into_inner());
    let _ = db.save_to_file();
    HttpResponse::Ok().finish()
}

async fn login(app_state: web::Data<AppState>, user: web::Json<User>) -> impl Responder {
    let db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    match db.get_user_by_name(&user.username) {
        Some(stored_user) if stored_user.password == user.password => {
            HttpResponse::Ok().body("Logged in!")
        },
        _ => HttpResponse::BadRequest().body("Invalid username or password")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let db: Database = match Database::load_from_file() {
        Ok(db) => db,
        Err(_) => Database::new()
    };

    let data: web::Data<AppState> = web::Data::new(AppState {
        db: Mutex::new(db)
    });

    let server = HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::permissive()
                    .allowed_origin_fn(|origin, _req_head| {
                        origin.as_bytes().starts_with(b"http://localhost") || origin == "null"
                    })
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600)
            )
            .app_data(data.clone())
            .service(
                Files::new("/", "frontend_index/").index_file("index.html")
            )
            .service(
                
                web
                    ::scope("/api/v1")
                    .service(
                        web
                            ::resource("/nutrition")
                            .route(web::post().to(create_nutrition))
                            .route(web::get().to(read_all_nutrition))
                            .route(web::put().to(update_nutrition))
                    )
                    .service(
                        web
                            ::resource("/nutrition/{id}")
                            .route(web::get().to(read_nutrition))
                            .route(web::delete().to(delete_nutrition))
                    )
                    .service(web::resource("/auth/register").route(web::post().to(register)))
                    .service(web::resource("/auth/login").route(web::post().to(login)))
            )
    })
    .bind("localhost:8080")?
    .run();
    // Spawn a background task to sleep after the server has run for 60 seconds.
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(60)).await;
        println!("Server has run for 60 seconds. Now sleeping.");
    });
    server.await
}