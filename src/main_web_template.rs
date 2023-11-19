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
struct Train {
    id: u64,
    name: String,
    status: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Station {
    id: u64,
    name: String,
    trains: HashMap<u64, Train>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Database {
    stations: HashMap<u64, Station>
}

impl Database {
    fn new() -> Self {
        Self {
            stations: HashMap::new()
        }
    }

    fn insert(&mut self, station: Station) {
        self.stations.insert(station.id, station);
    }

    fn get(&self, id: &u64) -> Option<&Station> {
        self.stations.get(id)
    }

    fn get_all(&self) -> Vec<&Station> {
        self.stations.values().collect()
    }

    fn delete(&mut self, id: &u64) {
        self.stations.remove(id);
    }

    fn update(&mut self, station: Station) {
        self.stations.insert(station.id, station);
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

async fn create_station(app_state: web::Data<AppState>, station: web::Json<Station>) -> impl Responder {
    let mut db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    db.insert(station.into_inner());
    let _ = db.save_to_file();
    HttpResponse::Ok().finish()
}

async fn read_station(app_state: web::Data<AppState>, id: web::Path<u64>) -> impl Responder {
    let db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    match db.get(&id.into_inner()) {
        Some(station) => HttpResponse::Ok().json(station),
        None => HttpResponse::NotFound().finish()
    }
}

async fn read_all_stations(app_state: web::Data<AppState>) -> impl Responder {
    let db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    let stations = db.get_all();
    HttpResponse::Ok().json(stations)
}

async fn update_station(app_state: web::Data<AppState>, station: web::Json<Station>) -> impl Responder {
    let mut db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    db.update(station.into_inner());
    let _ = db.save_to_file();
    HttpResponse::Ok().finish()
}

async fn delete_station(app_state: web::Data<AppState>, id: web::Path<u64>) -> impl Responder {
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
                            ::resource("/stations")
                            .route(web::post().to(create_station))
                            .route(web::get().to(read_all_stations))
                            .route(web::put().to(update_station))
                    )
                    .service(
                        web
                            ::resource("/stations/{id}")
                            .route(web::get().to(read_station))
                            .route(web::delete().to(delete_station))
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