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
struct Question {
    id: u64,
    question: String,
    options: Vec<String>,
    answer: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    id: u64,
    username: String,
    password: String,
    score: u64
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Database {
    questions: HashMap<u64, Question>,
    users: HashMap<u64, User>
}

impl Database {
    fn new() -> Self {
        Self {
            questions: HashMap::new(),
            users: HashMap::new()
        }
    }

    // CRUD DATA
    fn insert_question(&mut self, question: Question) {
        self.questions.insert(question.id, question);
    }

    fn get_question(&self, id: &u64) -> Option<&Question> {
        self.questions.get(id)
    }

    fn get_all_questions(&self) -> Vec<&Question> {
        self.questions.values().collect()
    }

    fn delete_question(&mut self, id: &u64) {
        self.questions.remove(id);
    }

    fn update_question(&mut self, question: Question) {
        self.questions.insert(question.id, question);
    }

    // USER DATA RELATED FUNCTIONS
    fn insert_user(&mut self, user: User) {
        self.users.insert(user.id, user);
    }

    fn get_user_by_name(&self, username: &str) -> Option<&User> {
        self.users.values().find(|u| u.username == username)
    }

    fn update_user_score(&mut self, username: &str, score: u64) {
        if let Some(user) = self.users.values_mut().find(|u| u.username == username) {
            user.score = score;
        }
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

async fn create_question(app_state: web::Data<AppState>, question: web::Json<Question>) -> impl Responder {
    let mut db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    db.insert_question(question.into_inner());
    let _ = db.save_to_file();
    HttpResponse::Ok().finish()
}

async fn read_question(app_state: web::Data<AppState>, id: web::Path<u64>) -> impl Responder {
    let db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    match db.get_question(&id.into_inner()) {
        Some(question) => HttpResponse::Ok().json(question),
        None => HttpResponse::NotFound().finish()
    }
}

async fn read_all_questions(app_state: web::Data<AppState>) -> impl Responder {
    let db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    let questions = db.get_all_questions();
    HttpResponse::Ok().json(questions)
}

async fn update_question(app_state: web::Data<AppState>, question: web::Json<Question>) -> impl Responder {
    let mut db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    db.update_question(question.into_inner());
    let _ = db.save_to_file();
    HttpResponse::Ok().finish()
}

async fn delete_question(app_state: web::Data<AppState>, id: web::Path<u64>) -> impl Responder {
    let mut db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    db.delete_question(&id.into_inner());
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

async fn update_score(app_state: web::Data<AppState>, username: web::Path<String>, score: web::Path<u64>) -> impl Responder {
    let mut db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    db.update_user_score(&username.into_inner(), score.into_inner());
    let _ = db.save_to_file();
    HttpResponse::Ok().finish()
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
                            ::resource("/questions")
                            .route(web::post().to(create_question))
                            .route(web::get().to(read_all_questions))
                            .route(web::put().to(update_question))
                    )
                    .service(
                        web
                            ::resource("/questions/{id}")
                            .route(web::get().to(read_question))
                            .route(web::delete().to(delete_question))
                    )
                    .service(web::resource("/auth/register").route(web::post().to(register)))
                    .service(web::resource("/auth/login").route(web::post().to(login)))
                    .service(web::resource("/users/{username}/score/{score}").route(web::put().to(update_score)))
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