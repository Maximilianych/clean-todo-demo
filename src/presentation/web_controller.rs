use std::sync::Mutex;

use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};

use crate::application::services::{TaskService, TaskServiceError};
use crate::presentation::dto::{TaskResponse, CreateTaskRequest};

#[get("/tasks")]
pub async fn get_all_tasks(task_service: web::Data<Mutex<TaskService>>) -> impl Responder {
    println!("get_all_tasks");
    let tasks = task_service.lock().unwrap().get_all();
    let response: Vec<TaskResponse> = tasks.into_iter().map(TaskResponse::from).collect();
    HttpResponse::Ok().body(serde_json::to_string_pretty(&response).unwrap())
}

#[get("/task/{id}")]
pub async fn get_task_by_id(task_service: web::Data<Mutex<TaskService>>, id: web::Path<u32>) -> impl Responder {
    println!("get_task_by_id/{id}");
    match task_service.lock().unwrap().get_by_id(*id) {
        Ok(task) => {
            println!("Task found");
            let response = TaskResponse::from(task);
            HttpResponse::Ok().json(response)
        }
        Err(_) => {
            println!("Task not found");
            HttpResponse::NotFound().body("Task not found")
        },
    }
}

#[post("/tasks")]
pub async fn create_task(task_service: web::Data<Mutex<TaskService>>, request: web::Json<CreateTaskRequest>) -> impl Responder {
    println!("create_task title: {}, description: {}", request.title, request.description);
    match task_service.lock().unwrap().create(request.title.to_string(), request.description.to_string()) {
        Ok(_) => {
            println!("Task created");
            HttpResponse::Ok().body("Task created")},
        Err(e) => {
            match e {
                TaskServiceError::MissingTitle => {
                    println!("Missing title");
                    HttpResponse::BadRequest().body("Missing title")
                },
                TaskServiceError::MissingDescription => {
                    println!("Missing description");
                    HttpResponse::BadRequest().body("Missing description")
                },
                TaskServiceError::TaskAlreadyExists => {
                    println!("Task already exists");
                    HttpResponse::BadRequest().body("Task already exists")
                },
                _ => {
                    println!("Unknown error");
                    HttpResponse::InternalServerError().body("Unknown error")
                }
            }
        },
    }
}

#[patch("/tasks/{id}")]
pub async fn toggle_task(task_service: web::Data<Mutex<TaskService>>, id: web::Path<u32>) -> impl Responder {
    println!("toggle_task/{id}");
    match task_service.lock().unwrap().toggle(*id) {
        Ok(_) => {
            println!("Task toggled");
            HttpResponse::Ok().body("Task toggled")
        },
        Err(_) => {
            println!("Task not found");
            HttpResponse::NotFound().body("Task not found")
        },
    }
}

#[delete("/tasks/{id}")]
pub async fn delete_task(task_service: web::Data<Mutex<TaskService>>, id: web::Path<u32>) -> impl Responder {
    println!("delete_task/{id}");
    match task_service.lock().unwrap().delete(*id) {
        Ok(_) => {
            println!("Task deleted");
            HttpResponse::Ok().body("Task deleted")
        },
        Err(_) => {
            println!("Task not found");
            HttpResponse::NotFound().body("Task not found")
        },
    }
}
