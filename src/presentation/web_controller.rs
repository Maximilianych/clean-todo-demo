use std::sync::Mutex;

use actix_web::{HttpResponse, Responder, delete, get, patch, post, web};

use crate::application::services::TaskService;
use crate::presentation::dto::{CreateTaskRequest, TaskResponse};

#[get("/tasks")]
pub async fn get_all_tasks(task_service: web::Data<Mutex<TaskService>>) -> impl Responder {
    println!("get_all_tasks");
    let tasks = task_service.lock().unwrap().get_all();
    let response: Vec<TaskResponse> = tasks.into_iter().map(TaskResponse::from).collect();
    HttpResponse::Ok().body(serde_json::to_string_pretty(&response).unwrap())
}

#[get("/task/{id}")]
pub async fn get_task_by_id(
    task_service: web::Data<Mutex<TaskService>>,
    id: web::Path<u32>
) -> impl Responder {
    println!("get_task_by_id/{id}");
    match task_service
        .lock()
        .unwrap()
        .get_by_id(*id) {
            Ok(task) => {
                println!("response: {:?}", task);
                let response = TaskResponse::from(task);
                HttpResponse::Ok().body(serde_json::to_string_pretty(&response).unwrap())
            }
            Err(e) => {
                println!("Task not found");
                HttpResponse::from(e)
            },
        }
}

#[post("/tasks")]
pub async fn create_task(
    task_service: web::Data<Mutex<TaskService>>,
    request: web::Json<CreateTaskRequest>,
) -> impl Responder {
    println!(
        "create_task title: {}, description: {}",
        request.title, request.description
    );
    match task_service
        .lock()
        .unwrap()
        .create(request.title.to_string(), request.description.to_string()) {
            Ok(_) => {
                println!("Task created");
                HttpResponse::Ok().json("Task created")
            }
            Err(e) => {
                println!("Error creating task");
                HttpResponse::from(e)
            }
        }
}

#[patch("/tasks/{id}")]
pub async fn toggle_task(
    task_service: web::Data<Mutex<TaskService>>,
    id: web::Path<u32>,
) -> impl Responder {
    println!("toggle_task/{id}");
    match task_service.lock().unwrap().toggle(*id) {
        Ok(_) => {
            println!("Task toggled");
            HttpResponse::Ok().json("Task toggled")
        }
        Err(e) => {
            println!("Task not found");
            HttpResponse::from(e)
        }
    }
}

#[delete("/tasks/{id}")]
pub async fn delete_task(
    task_service: web::Data<Mutex<TaskService>>,
    id: web::Path<u32>,
) -> impl Responder {
    println!("delete_task/{id}");
    match task_service.lock().unwrap().delete(*id) {
        Ok(_) => {
            println!("Task deleted");
            HttpResponse::Ok().json("Task deleted")
        }
        Err(e) => {
            println!("Task not found");
            HttpResponse::from(e)
        }
    }
}
