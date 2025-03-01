use std::sync::Mutex;
use rocket::{delete, get, launch, post, put, routes, State};
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;

struct AppState {
    tasks: Mutex<Vec<Task>>,
    last_id: Mutex<u32>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Task {
    id: u32,
    title: String,
    completed: bool,
}

#[get("/tasks")]
fn get_tasks(state: &State<AppState>) -> Json<Vec<Task>> {
    let tasks = state.tasks.lock().unwrap();
    Json(tasks.clone())
}

#[get("/tasks/<id>")]
fn get_task(state: &State<AppState>, id: u32) -> Option<Json<Task>> {
    let tasks = state.tasks.lock().unwrap();
    tasks.iter().find(|t| t.id == id).map(|t| Json(t.clone()))
}

#[post("/tasks", data = "<task>")]
fn create_task(state: &State<AppState>, task: Json<Task>) -> Json<Task> {
    let mut tasks = state.tasks.lock().unwrap();
    let mut task = task.into_inner();
    //task.id = tasks.len() as u32 + 1; // Asignar un ID único
    let mut last_id = state.last_id.lock().unwrap();
    task.id = *last_id + 1;
    *last_id = task.id;
    tasks.push(task.clone());
    Json(task)
}

#[put("/tasks/<id>", data = "<task>")]
fn update_task(state: &State<AppState>, id: u32, task: Json<Task>) -> Option<Json<Task>> {
    let mut tasks = state.tasks.lock().unwrap();
    if let Some(existing_task) = tasks.iter_mut().find(|t| t.id == id) {
        existing_task.title = task.title.clone();
        existing_task.completed = task.completed;
        Some(Json(existing_task.clone()))
    } else {
        None
    }
}

#[delete("/tasks/<id>")]
fn delete_task(state: &State<AppState>, id: u32) -> Option<Json<Task>> {
    let mut tasks = state.tasks.lock().unwrap();
    if let Some(index) = tasks.iter().position(|t| t.id == id) {
        Some(Json(tasks.remove(index)))
    } else {
        None
    }
}

#[launch]
fn rocket() -> _ {
    let initial_tasks = vec![
        Task {
            id: 1,
            title: "Aprender Rust".to_string(),
            completed: false,
        },
        Task {
            id: 2,
            title: "Crear una API REST".to_string(),
            completed: false,
        },
    ];

    rocket::build()
        .manage(AppState {
            tasks: Mutex::new(initial_tasks),
            last_id: Mutex::new(2),
        })
        .mount("/", routes![get_tasks, get_task, create_task, update_task, delete_task])
}


