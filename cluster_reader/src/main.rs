use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[get("/home")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("heya fucko")
}

async fn about() -> impl Responder {
    HttpResponse::Ok().body("here is the about section")
}

// This struct represents state
struct AppState {
    app_name: String,
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; // <- get app_name
    format!("Hello {app_name}!") // <- response with app_name
}

struct AppStateWithCounter {
    counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}

async fn indextwo(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter += 1; // <- access counter inside MutexGuard

    format!("Request number: {counter}") // <- response with count
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            // .app_data(web::Data::new(AppState {
            //     app_name: String::from("Actix Web")
            // }))
            .app_data(counter.clone()) // <- register the created data
            .route("/", web::get().to(indextwo))
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .service(home)
            .service(
                web::scope("/api")
                .route("/about", web::get().to(about)))
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}