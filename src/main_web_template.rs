use actix_cors::Cors;

use actix_web::{ http::header, web, App, HttpServer, Responder, HttpResponse };

use serde::{ Deserialize, Serialize };

use reqwest::Client as HttpClient;

use async_trait::async_trait;

use actix_files::Files;

use tokio::time::Duration;

use std::sync::Mutex;
use std::collections::HashMap;
use std::fs;
use std::io::Write;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Goal {
    id: u64,
    name: String,
    completed: bool
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    id: u64,
    username: String,
    password: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Database {
    goals: HashMap<u64, Goal>,
    users: HashMap<u64, User>
}

impl Database {
    fn new() -> Self {
        Self {
            goals: HashMap::new(),
            users: HashMap::new()
        }
    }

    // CRUD DATA
    fn insert(&mut self, goal: Goal) {
        self.goals.insert(goal.id, goal);
    }

    fn get(&self, id: &u64) -> Option<&Goal> {
        self.goals.get(id)
    }

    fn get_all(&self) -> Vec<&Goal> {
        self.goals.values().collect()
    }

    fn delete(&mut self, id: &u64) {
        self.goals.remove(id);
    }

    fn update(&mut self, goal: Goal) {
        self.goals.insert(goal.id, goal);
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

async fn create_goal(app_state: web::Data<AppState>, goal: web::Json<Goal>) -> impl Responder {
    let mut db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    db.insert(goal.into_inner());
    let _ = db.save_to_file();
    HttpResponse::Ok().finish()
}

async fn read_goal(app_state: web::Data<AppState>, id: web::Path<u64>) -> impl Responder {
    let db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    match db.get(&id.into_inner()) {
        Some(goal) => HttpResponse::Ok().json(goal),
        None => HttpResponse::NotFound().finish()
    }
}

async fn read_all_goals(app_state: web::Data<AppState>) -> impl Responder {
    let db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    let goals = db.get_all();
    HttpResponse::Ok().json(goals)
}

async fn update_goal(app_state: web::Data<AppState>, goal: web::Json<Goal>) -> impl Responder {
    let mut db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    db.update(goal.into_inner());
    let _ = db.save_to_file();
    HttpResponse::Ok().finish()
}

async fn delete_goal(app_state: web::Data<AppState>, id: web::Path<u64>) -> impl Responder {
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
                Files::new("/", "frontend/").index_file("index.html")
            )
            .service(
                
                web
                    ::scope("/api/v1")
                    .service(
                        web
                            ::resource("/goals")
                            .route(web::post().to(create_goal))
                            .route(web::get().to(read_all_goals))
                            .route(web::put().to(update_goal))
                    )
                    .service(
                        web
                            ::resource("/goals/{id}")
                            .route(web::get().to(read_goal))
                            .route(web::delete().to(delete_goal))
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