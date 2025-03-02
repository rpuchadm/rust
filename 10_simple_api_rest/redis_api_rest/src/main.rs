use core::net;
use std::sync::Mutex;
use redis::Commands;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use rocket::{delete, get, launch, post, put, routes, State};

struct AppState {
    tasks: Mutex<Vec<Task>>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Task {
    id: u32,
    title: String,
    completed: bool,
}

#[derive(Serialize, Deserialize, Clone)]
struct TaskFields {
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

#[post("/tasks", data = "<task_fields>")]
fn create_task(state: &State<AppState>, task_fields: Json<TaskFields>) -> Json<Task> {
    let mut tasks = state.tasks.lock().unwrap();
    let task_fields = task_fields.into_inner();
    // Asignar un ID único
    let next_id_ptr = redis_get_integer(NEXT_ID_KEY).unwrap();
    let next_id = next_id_ptr as u32;
    redis_set_integer(NEXT_ID_KEY, next_id + 1).unwrap();
    let task = Task {
        id: next_id,
        title: task_fields.title.clone(),
        completed: task_fields.completed,
    };
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

    let next_id = 1;

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

    let next_id = 3;
    redis_set_integer(NEXT_ID_KEY, next_id).unwrap();

    rocket::build()
        .manage(AppState {
            tasks: Mutex::new(initial_tasks),
        })
        .mount("/", routes![get_tasks, get_task, create_task, update_task, delete_task])
}


const REDIS_SERVER: &str = "redis://127.0.0.1/";
const NEXT_ID_KEY: &str = "next_id";
fn redis_get_integer( key_name: &str) -> redis::RedisResult<isize> {
    // connect to redis
    let client = redis::Client::open(REDIS_SERVER)?;
    let mut con = client.get_connection()?;
    // throw away the result, just make sure it does not fail
    con.get(key_name)
}

fn redis_set_integer( key_name: &str, value: u32) -> redis::RedisResult<()> {
    // connect to redis
    let client = redis::Client::open(REDIS_SERVER)?;
    let mut con = client.get_connection()?;
    // throw away the result, just make sure it does not fail
    let _: () = con.set(key_name, value)?;
    Ok(())
}
