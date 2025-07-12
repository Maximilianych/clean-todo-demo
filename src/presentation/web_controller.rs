use std::sync::Mutex;

use actix_web::{get, web, HttpResponse, Responder};

use crate::application::services::{TaskService, TaskServiceError};

#[get("/tasks")]
pub async fn get_all_tasks(task_service: web::Data<Mutex<TaskService>>) -> impl Responder {
    println!("get_all_tasks");
    let tasks = task_service.lock().unwrap().get_all();
    let mut response = "".to_string();
    for task in tasks {
        response += &format!("id: {}, title: {}, description: {}, status: {}\n", task.id, task.title, task.description, task.status);
    }
    println!("response: {}", response.trim_end());
    HttpResponse::Ok().body(response)
}

#[get("/task/{id}")]
pub async fn get_task_by_id(task_service: web::Data<Mutex<TaskService>>, id: web::Path<u32>) -> impl Responder {
    println!("get_task_by_id/{id}");
    match task_service.lock().unwrap().get_by_id(*id) {
        Ok(task) => {
            let response = format!("id: {}, title: {}, description: {}, status: {}\n", task.id, task.title, task.description, task.status);
            println!("response: {}", response.trim_end());
            HttpResponse::Ok().body(response)
        }
        Err(_) => {
            println!("Task not found");
            HttpResponse::NotFound().body("Task not found")
        },
    }
}

#[derive(serde::Deserialize, Debug)]
struct TaskDetails {
    title: String,
    description: String,
}

#[get("/create_task")]
pub async fn create_task(task_service: web::Data<Mutex<TaskService>>, task_details: web::Query<TaskDetails>) -> impl Responder {
    println!("create_task title: {}, description: {}", task_details.title, task_details.description);
    match task_service.lock().unwrap().create(task_details.title.to_string(), task_details.description.to_string()) {
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

#[get("/toggle_task/{id}")]
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

#[get("/delete_task/{id}")]
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
