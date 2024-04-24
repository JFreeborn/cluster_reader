use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::Serialize; 
use std::io;

#[derive(Debug, serde::Serialize)]
struct MyResponse {
    message: String,
    status: i32,
}

async fn index() -> impl Responder {
    let response = MyResponse {
        message: "Hello, world!".to_string(),
        status: 200,
    };

    // Return the struct as JSON using HttpResponse
    HttpResponse::Ok().json(response)
}

#[tokio::main]
async fn main() -> io::Result<()> {
    
    HttpServer::new(|| {
        App::new()
            .route("/api/v1/cluster-info", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

