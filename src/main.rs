mod application;
mod domain;
mod infrastructure;
mod presentation;

use std::sync::Mutex;

use actix_web::{App, HttpServer, web};

#[actix_web::main]
async fn main() {
    let task_service = application::services::TaskService::new(Box::new(
        infrastructure::in_memory::InMemoryTaskRepository::new(),
    ));
    let task_service = web::Data::new(Mutex::new(task_service));

    HttpServer::new(move || {
        App::new()
            .service(presentation::web_controller::get_all_tasks)
            .service(presentation::web_controller::get_task_by_id)
            .service(presentation::web_controller::create_task)
            .service(presentation::web_controller::toggle_task)
            .service(presentation::web_controller::delete_task)
            .app_data(task_service.clone())
    })
    .bind(("127.0.0.1", 8080))
    .unwrap()
    .run()
    .await
    .unwrap();
}
