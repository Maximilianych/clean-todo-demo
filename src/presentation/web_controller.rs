use actix_web::{get, rt::task, web, Responder};

use crate::application::services::TaskService;

#[get("/tasks")]
pub async fn get_all_tasks(task_service: web::Data<TaskService>) -> impl Responder {
    let tasks = task_service.get_all();
    let mut response = "".to_string();
    for task in tasks {
        response += &format!("id: {}, title: {}, description: {}, status: {}\n", task.id, task.title, task.description, task.status);
    }
    println!("response: {}", response);
    response
}
