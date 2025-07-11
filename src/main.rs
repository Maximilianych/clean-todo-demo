mod application;
mod domain;
mod infrastructure;
mod presentation;

use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() {
    let mut task_service = application::services::TaskService::new(Box::new(infrastructure::in_memory::InMemoryTaskRepository::new()));
    let task_service = web::Data::new(task_service);

    HttpServer::new(move || {
        App::new()
            .service(presentation::web_controller::get_all_tasks)
            .app_data(task_service.clone())
    })
    .bind(("127.0.0.1", 8080))
    .unwrap()
    .run()
    .await;
}
