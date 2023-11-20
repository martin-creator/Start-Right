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
struct Farm {
    id: u64,
    name: String,
    cow_count: u64,
    goat_count: u64,
    production: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Database {
    farms: HashMap<u64, Farm>,
}

impl Database {
    fn new() -> Self {
        Self {
            farms: HashMap::new(),
        }
    }

    fn insert(&mut self, farm: Farm) {
        self.farms.insert(farm.id, farm);
    }

    fn get(&self, id: &u64) -> Option<&Farm> {
        self.farms.get(id)
    }

    fn get_all(&self) -> Vec<&Farm> {
        self.farms.values().collect()
    }

    fn delete(&mut self, id: &u64) {
        self.farms.remove(id);
    }

    fn update(&mut self, farm: Farm) {
        self.farms.insert(farm.id, farm);
    }

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

async fn create_farm(app_state: web::Data<AppState>, farm: web::Json<Farm>) -> impl Responder {
    let mut db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    db.insert(farm.into_inner());
    let _ = db.save_to_file();
    HttpResponse::Ok().finish()
}

async fn read_farm(app_state: web::Data<AppState>, id: web::Path<u64>) -> impl Responder {
    let db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    match db.get(&id.into_inner()) {
        Some(farm) => HttpResponse::Ok().json(farm),
        None => HttpResponse::NotFound().finish()
    }
}

async fn read_all_farms(app_state: web::Data<AppState>) -> impl Responder {
    let db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    let farms = db.get_all();
    HttpResponse::Ok().json(farms)
}

async fn update_farm(app_state: web::Data<AppState>, farm: web::Json<Farm>) -> impl Responder {
    let mut db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    db.update(farm.into_inner());
    let _ = db.save_to_file();
    HttpResponse::Ok().finish()
}

async fn delete_farm(app_state: web::Data<AppState>, id: web::Path<u64>) -> impl Responder {
    let mut db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    db.delete(&id.into_inner());
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
                            ::resource("/farms")
                            .route(web::post().to(create_farm))
                            .route(web::get().to(read_all_farms))
                            .route(web::put().to(update_farm))
                    )
                    .service(
                        web
                            ::resource("/farms/{id}")
                            .route(web::get().to(read_farm))
                            .route(web::delete().to(delete_farm))
                    )
            )
    })
    .bind("localhost:8080")?
    .run();
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(60)).await;
        println!("Server has run for 60 seconds. Now sleeping.");
    });
    server.await
}