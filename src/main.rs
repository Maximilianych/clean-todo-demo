mod application;
mod domain;
mod infrastructure;
mod presentation;

use std::sync::Mutex;

use actix_web::{App, HttpServer, web};
use clap::Parser;
use sqlx::SqlitePool;

use crate::{
    application::services::TaskService,
    infrastructure::{in_memory::InMemoryTaskRepository, sqlite::SqliteTaskRepository},
    presentation::web_controller,
};

#[derive(clap::Parser)]
struct Cli {
    #[command(subcommand)]
    repository: Repository,
}

#[derive(clap::Subcommand)]
enum Repository {
    InMemory,
    Sqlite,
}

#[actix_web::main]
async fn main() {
    dotenv::dotenv().ok();
    let cli = Cli::parse();

    let task_service = match cli.repository {
            Repository::InMemory => {
                println!("Using in-memory repository");
                TaskService::new(Box::new(InMemoryTaskRepository::new()))
            }
            Repository::Sqlite => {
                println!("Using sqlite repository");
                TaskService::new(Box::new(
                    SqliteTaskRepository::new(
                        SqlitePool::connect(&std::env::var("DATABASE_URL").unwrap())
                            .await
                            .unwrap(),
                    )
                    .await,
                ))
            }
    };
    let task_service = web::Data::new(Mutex::new(task_service));

    HttpServer::new(move || {
        App::new()
            .service(web_controller::get_all_tasks)
            .service(web_controller::get_task_by_id)
            .service(web_controller::create_task)
            .service(web_controller::toggle_task)
            .service(web_controller::delete_task)
            .app_data(task_service.clone())
    })
    .bind(("127.0.0.1", 8080))
    .unwrap()
    .run()
    .await
    .unwrap();
}
