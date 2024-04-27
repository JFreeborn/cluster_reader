pub mod api {

    use std::fmt::format;

    use actix_web::web;
    use actix_web::{get, post, Responder, HttpResponse};
    use tokio::time::Duration;
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct Info {
        user_id: u32,
        friend: String,
    }

    async fn about() -> impl Responder {
        HttpResponse::Ok().body("here is the about section")
    }

    #[post("/echo")]
    async fn echo(req_body: String) -> impl Responder {
        HttpResponse::Ok().body(req_body)
    }

    #[get("/hello")]
    async fn hello() -> impl Responder {
        HttpResponse::Ok().body("Hello world!")
    }

    #[get("/home/{user_id}/{friend}")]
    async fn home(info: web::Path<Info>) -> impl Responder {
        tokio::time::sleep(Duration::from_secs(5)).await; // <-- Ok. Worker thread will handle other requests here
        //HttpResponse::Ok().body("heya fucko")
        HttpResponse::Ok()
            .body(format!("Welcome {}, user number {}!", info.friend, info.user_id)) 
    }

    async fn manual_hello() -> impl Responder {
        HttpResponse::Ok().body("Hey there!")
    }

    pub fn scoped_config(cfg: &mut web::ServiceConfig) {
        cfg
        .service(
            web::resource("/test")
                .route(web::get().to(|| async { actix_web::HttpResponse::Ok().body("this is the new lol routes") }))
                .route(web::head().to(actix_web::HttpResponse::MethodNotAllowed)),
        )
        .route("/about", web::get().to(about))
        .service(echo)
        .service(hello)
        .service(home)
        .route("/hey", web::get().to(manual_hello))
        ;
    }
}