use actix_cors::Cors;
use actix_web::{ http::header, web, App, HttpResponse, HttpServer, Responder };
use serde::{ Deserialize, Serialize };
use std::sync::Mutex;
use std::{ collections::HashMap, fs, io::Write };
use actix_files::Files;

// Define the Task structure for representing tasks in the system.
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    id: u64,
    name: String,
    completed: bool,
}

// Define the User structure for representing users in the system.
#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    id: u64,
    username: String,
    // In a production environment, store hashed passwords, not plaintext.
    password: String,
}

// Define the Database structure to hold tasks and users.
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Database {
    tasks: HashMap<u64, Task>,
    users: HashMap<u64, User>,
}

// Implement methods for the Database structure to perform CRUD operations.
impl Database {
    // Create a new instance of the database.
    fn new() -> Self {
        Self {
            tasks: HashMap::new(),
            users: HashMap::new(),
        }
    }

    // Insert a new task into the database.
    fn insert(&mut self, task: Task) {
        self.tasks.insert(task.id, task);
    }

    // Retrieve a task by its ID.
    fn get(&self, id: &u64) -> Option<&Task> {
        self.tasks.get(id)
    }

    // Retrieve all tasks in the database.
    fn get_all(&self) -> Vec<&Task> {
        self.tasks.values().collect()
    }

    // Delete a task by its ID.
    fn delete(&mut self, id: &u64) {
        self.tasks.remove(id);
    }

    // Update an existing task in the database.
    fn update(&mut self, task: Task) {
        self.tasks.insert(task.id, task);
    }

    // Insert a new user into the database.
    fn insert_user(&mut self, user: User) {
        self.users.insert(user.id, user);
    }

    // Retrieve a user by their username.
    fn get_user_by_name(&self, username: &str) -> Option<&User> {
        self.users.values().find(|u| u.username == username)
    }

    // Save the database content to a file.
    fn save_to_file(&self) -> std::io::Result<()> {
        let data: String = serde_json::to_string(&self)?;
        let mut file: fs::File = fs::File::create("database.json")?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }

    // Load the database content from a file.
    fn load_from_file() -> std::io::Result<Self> {
        let file_content: String = fs::read_to_string("database.json")?;
        let db: Database = serde_json::from_str(&file_content)?;
        Ok(db)
    }
}

// Define the application state structure to hold the database.
struct AppState {
    db: Mutex<Database>,
}

// Helper function to construct a JSON response.
fn json_response<T: Serialize>(data: T) -> HttpResponse {
    HttpResponse::Ok().json(data)
}

// Endpoint to create a new task.
// @desc    Create a new task
// @route   POST /api/v1/tasks
// @access  Public
async fn create_task(app_state: web::Data<AppState>, task: web::Json<Task>) -> impl Responder {
    let mut db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    db.insert(task.into_inner());
    let _ = db.save_to_file();
    json_response("Task created successfully")
}

// Endpoint to get a task by ID.
// @desc    Get a task by ID
// @route   GET /api/v1/tasks/{id}
// @access  Public
async fn read_task(app_state: web::Data<AppState>, id: web::Path<u64>) -> impl Responder {
    let db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    match db.get(&id.into_inner()) {
        Some(task) => json_response(task),
        None => HttpResponse::NotFound().json("Task not found"),
    }
}

// Endpoint to get all tasks.
// @desc    Get all tasks
// @route   GET /api/v1/tasks
// @access  Public
async fn read_all_tasks(app_state: web::Data<AppState>) -> impl Responder {
    let db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    let tasks = db.get_all();
    json_response(tasks)
}

// Endpoint to update a task.
// @desc    Update a task
// @route   PUT /api/v1/tasks
// @access  Public
async fn update_task(app_state: web::Data<AppState>, task: web::Json<Task>) -> impl Responder {
    let mut db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    db.update(task.into_inner());
    let _ = db.save_to_file();
    json_response("Task updated successfully")
}

// Endpoint to delete a task by ID.
// @desc    Delete a task by ID
// @route   DELETE /api/v1/tasks/{id}
// @access  Public
async fn delete_task(app_state: web::Data<AppState>, id: web::Path<u64>) -> impl Responder {
    let mut db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    db.delete(&id.into_inner());
    let _ = db.save_to_file();
    json_response("Task deleted successfully")
}

// Endpoint to register a new user.
// @desc    Register user
// @route   POST /api/v1/auth/register
// @access  Public
async fn register(app_state: web::Data<AppState>, user: web::Json<User>) -> impl Responder {
    let mut db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();

    // Check if the username is already taken
    if db.get_user_by_name(&user.username).is_some() {
        return HttpResponse::Conflict().json("Username already exists");
    }

    db.insert_user(user.into_inner());
    let _ = db.save_to_file();
    json_response("User registered successfully")
}

// Endpoint for user login.
// @desc    User login
// @route   POST /api/v1/auth/login
// @access  Public
async fn login(app_state: web::Data<AppState>, user: web::Json<User>) -> impl Responder {
    let db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    match db.get_user_by_name(&user.username) {
        Some(stored_user) if stored_user.password == user.password => {
            json_response("Logged in successfully")
        }
        _ => HttpResponse::Unauthorized().json("Invalid username or password"),
    }
}

// Entry point for the Actix web server.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load the database from a file or create a new one if not found.
    let db: Database = match Database::load_from_file() {
        Ok(db) => db,
        Err(_) => Database::new(),
    };

    // Create the Actix web application state with a mutex-protected database.
    let data: web::Data<AppState> = web::Data::new(AppState {
        db: Mutex::new(db),
    });

    // Start the Actix web server.
    HttpServer::new(move || {
        App::new()
            .wrap(
                // Configure CORS settings.
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
                // Serve static files from the "static" directory.
                Files::new("/", "frontend/").index_file("index.html")
            )
            .service(
                // Define the API routes and corresponding handlers.
                web
                    ::scope("/api/v1")
                    .service(
                        web
                            ::resource("/tasks")
                            .route(web::post().to(create_task))
                            .route(web::get().to(read_all_tasks))
                            .route(web::put().to(update_task))
                    )
                    .service(
                        web
                            ::resource("/tasks/{id}")
                            .route(web::get().to(read_task))
                            .route(web::delete().to(delete_task))
                    )
                    .service(web::resource("/auth/register").route(web::post().to(register)))
                    .service(web::resource("/auth/login").route(web::post().to(login)))
            )
    })
        .bind("127.0.0.1:8080")?
        .run().await
}
