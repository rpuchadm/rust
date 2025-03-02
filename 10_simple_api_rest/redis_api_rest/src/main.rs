use redis::Commands;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use rocket::{delete, get, launch, post, put, routes};

struct AppState {
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
fn get_tasks() -> Json<Vec<Task>> {
    // saca todos los Task de redis
    let tasks = redis_get_all_tasks().unwrap();
    Json(tasks.clone())
}

#[get("/tasks/<id>")]
fn get_task( id: u32) -> Option<Json<Task>> {
    // saca un Task de redis
    let task = redis_get_task(id).unwrap();
    Some(Json(task.clone()))
}

#[post("/tasks", data = "<task_fields>")]
fn create_task(task_fields: Json<TaskFields>) -> Json<Task> {
    // crea un Task en redis
    let task_fields = task_fields.into_inner();
    let next_id_ptr = redis_get_integer(NEXT_ID_KEY).unwrap();
    let mut next_id = next_id_ptr as u32;
    let id = next_id;
    next_id += 1;
    redis_set_integer(NEXT_ID_KEY, next_id).unwrap();
    let task = Task {
        id: id,
        title: task_fields.title.clone(),
        completed: task_fields.completed,
    };
    
    redis_set_task(next_id, task.clone()).unwrap();

    Json(task)
}

#[put("/tasks/<id>", data = "<task_fields>")]
fn update_task( id: u32, task_fields: Json<TaskFields>) -> Option<Json<Task>> {
    // actualiza un Task de redis
    let task_fields = task_fields.into_inner();
    let task = Task {
        id: id,
        title: task_fields.title.clone(),
        completed: task_fields.completed,
    };
    redis_set_task(id, task.clone()).unwrap();
    Some(Json(task))
}

#[delete("/tasks/<id>")]
fn delete_task( id: u32) -> Option<Json<Task>> {
    // borra un Task de redis
    let task = redis_get_task(id).unwrap();
    redis_delete_task(id).unwrap();
    Some(Json(task.clone()))
}

#[launch]
fn rocket() -> _ {

    let next_id = redis_get_integer(NEXT_ID_KEY).unwrap_or(0);

    let mut id = next_id as u32;

    let task1 = Task {
        id: id,
        title: "Aprender Rust".to_string(),
        completed: false,
    };
    redis_set_task(id, task1).unwrap();

    id += 1;
    let task2 = Task {
        id: id,
        title: "Crear una API REST".to_string(),
        completed: false,
    };
    redis_set_task(id, task2).unwrap();

    id += 1;
    redis_set_integer(NEXT_ID_KEY, id).unwrap();

    rocket::build()
        .manage(AppState {
            
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

const TASK_PREFIX: &str = "task_";
// funcion para meter un struct TaskFields como json en redis y como key usaremos TASK_PREFIX + id
fn redis_set_task( id: u32, task: Task) -> redis::RedisResult<()> {
    // connect to redis
    let client = redis::Client::open(REDIS_SERVER)?;
    let mut con = client.get_connection()?;
    // preparar el json del struct Task
    let json = serde_json::to_string(&task).unwrap();
    // throw away the result, just make sure it does not fail
    let _: () = con.set(TASK_PREFIX.to_string() + &id.to_string(), json)?;
    Ok(())
}
// funcion para obtener un struct TaskFields de redis y como key usaremos TASK_PREFIX + id
fn redis_get_task( id: u32) -> redis::RedisResult<Task> {
    // connect to redis
    let client = redis::Client::open(REDIS_SERVER)?;
    let mut con = client.get_connection()?;
    // throw away the result, just make sure it does not fail
    let json: String = con.get(TASK_PREFIX.to_string() + &id.to_string())?;
    let task: Task = serde_json::from_str(&json).unwrap();
    Ok(task)
}
// funcion para borrar un struct TaskFields de redis y como key usaremos TASK_PREFIX + id
fn redis_delete_task( id: u32) -> redis::RedisResult<()> {
    // connect to redis
    let client = redis::Client::open(REDIS_SERVER)?;
    let mut con = client.get_connection()?;
    // throw away the result, just make sure it does not fail
    let _: () = con.del(TASK_PREFIX.to_string() + &id.to_string())?;
    Ok(())
}
// funcion para obtener todos los TaskFields de redis y como key usaremos TASK_PREFIX + id
fn redis_get_all_tasks() -> redis::RedisResult<Vec<Task>> {
    // connect to redis
    let client = redis::Client::open(REDIS_SERVER)?;
    let mut con = client.get_connection()?;
    // throw away the result, just make sure it does not fail
    let keys: Vec<String> = con.keys(TASK_PREFIX.to_string() + "*")?;
    let mut tasks: Vec<Task> = Vec::new();
    for key in keys {
        let json: String = con.get(key)?;
        let task: Task = serde_json::from_str(&json).unwrap();
        tasks.push(task);
    }
    Ok(tasks)
}
