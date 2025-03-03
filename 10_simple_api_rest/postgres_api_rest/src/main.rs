use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use rocket::{delete, get, launch, post, put, routes, State};

struct AppState {
    pool: sqlx::Pool<sqlx::Postgres>,
}

#[derive(Serialize, Deserialize, Clone, sqlx::FromRow, sqlx::Decode)]
struct Task {
    id: i32,
    title: String,
    completed: bool,
}

#[derive(Serialize, Deserialize, Clone, sqlx::FromRow, sqlx::Decode)]
struct TaskFields {
    title: String,
    completed: bool,
}

#[get("/tasks")]
async fn get_tasks( state: &State<AppState>) -> Json<Vec<Task>> {
    let pool = state.pool.clone();
    let tasks = postgres_get_all_tasks(&pool)
        .await
        .unwrap();
    Json(tasks.clone())
}

#[get("/tasks/<id>")]
async fn get_task( state: &State<AppState>, id: i32) -> Option<Json<Task>> {
    let pool = state.pool.clone();
    let task = postgres_get_task(&pool,id)
        .await
        .unwrap();
    Some(Json(task.clone()))
}

#[post("/tasks", data = "<task_fields>")]
async fn create_task(state: &State<AppState>, task_fields: Json<TaskFields>) -> Json<Task> {
    let task_fields: TaskFields = task_fields.into_inner();
    let pool = state.pool.clone();
    let task = postgres_create_task(&pool, task_fields)
        .await
        .unwrap();
    Json(task)
}

#[put("/tasks/<id>", data = "<task_fields>")]
async fn update_task( state: &State<AppState>, id: i32, task_fields: Json<TaskFields>) -> Option<Json<Task>> {
    let task_fields = task_fields.into_inner();
    let pool = state.pool.clone();
    let task = postgres_update_task(&pool,id, task_fields).await.unwrap();
    Some(Json(task))
}

#[delete("/tasks/<id>")]
async fn delete_task( state: &State<AppState>, id: i32) -> Option<Json<Task>> {
    let pool = state.pool.clone();
    let task = postgres_delete_task( pool,id).await.unwrap();
    Some(Json(task.clone()))
}

#[launch]
async fn rocket() -> _ {

    let pool: sqlx::Pool<sqlx::Postgres> = sqlx::postgres::PgPool::connect(POSTGRES_SERVER)
        .await.or_else(|err| {
            eprintln!("Error connecting to the database: {:?}", err);
            Err(err)
        })
        .unwrap();

    let task1 = TaskFields {
        title: "Aprender Rust".to_string(),
        completed: false,
    };
    postgres_create_task(&pool,task1).await.unwrap();

    let task2 = TaskFields {
        title: "Crear una API REST".to_string(),
        completed: false,
    };
    postgres_create_task(&pool,task2).await.unwrap();

    rocket::build()
        .manage(AppState {
            pool,
        })
        .mount("/", routes![get_tasks, get_task, create_task, update_task, delete_task])
}

// constante con el servidor de postgres
const POSTGRES_SERVER: &str = "postgresql://myuser:mypassword@localhost:5432/mydatabase";
// funcion para obtener todos los Tasks de postgres
async fn postgres_get_all_tasks( pool: &sqlx::Pool<sqlx::Postgres>) -> Result<Vec<Task>, sqlx::Error> {
    // throw away the result, just make sure it does not fail
    let tasks = sqlx::query_as::<_, Task>("SELECT * FROM tasks").fetch_all(pool).await?;
    Ok(tasks)
}
// funcion para obtener un Task de postgres
async fn postgres_get_task( pool: &sqlx::Pool<sqlx::Postgres>, id: i32) -> Result<Task, sqlx::Error> {
    // throw away the result, just make sure it does not fail
    let task = sqlx::query_as::<_, Task>("SELECT * FROM tasks WHERE id = $1").bind(id).fetch_one(pool).await?;
    Ok(task)
}
// funcion para crear un Task en postgres
async fn postgres_create_task( pool: &sqlx::Pool<sqlx::Postgres>, task_fields: TaskFields) -> Result<Task, sqlx::Error> {
    // throw away the result, just make sure it does not fail
    let task = sqlx::query_as::<_, Task>("INSERT INTO tasks (title, completed) VALUES ($1, $2) RETURNING *")
        .bind(task_fields.title)
        .bind(task_fields.completed)
        .fetch_one(pool).await?;
    Ok(task)
}
// funcion para actualizar un Task en postgres
async fn postgres_update_task( pool: &sqlx::Pool<sqlx::Postgres>, id: i32, task_fields: TaskFields) -> Result<Task, sqlx::Error> {
    // throw away the result, just make sure it does not fail
    let task = sqlx::query_as::<_, Task>("UPDATE tasks SET title = $1, completed = $2 WHERE id = $3 RETURNING *")
        .bind(task_fields.title)
        .bind(task_fields.completed)
        .bind(id)
        .fetch_one(pool).await?;
    Ok(task)
}
// funcion para borrar un Task en postgres
async fn postgres_delete_task( pool: sqlx::Pool<sqlx::Postgres>, id: i32) -> Result<Task, sqlx::Error> {
    // throw away the result, just make sure it does not fail
    let task = sqlx::query_as::<_, Task>("DELETE FROM tasks WHERE id = $1 RETURNING *")
        .bind(id)
        .fetch_one(&pool).await?;
    Ok(task)
}



