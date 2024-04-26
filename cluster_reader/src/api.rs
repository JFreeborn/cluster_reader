pub mod api {

    use actix_web::web;
    use actix_web::{get, post, Responder, HttpResponse};

    #[post("/echo")]
    async fn echo(req_body: String) -> impl Responder {
        HttpResponse::Ok().body(req_body)
    }

    #[get("/hello")]
    async fn hello() -> impl Responder {
        HttpResponse::Ok().body("Hello world!")
    }

    #[get("/home")]
    async fn home() -> impl Responder {
        HttpResponse::Ok().body("heya fucko")   
    }

    pub fn scoped_config(cfg: &mut web::ServiceConfig) {
        cfg
        .service(
            web::resource("/test")
                .route(web::get().to(|| async { actix_web::HttpResponse::Ok().body("this is the new lol routes") }))
                .route(web::head().to(actix_web::HttpResponse::MethodNotAllowed)),
        )
        .service(echo)
        .service(hello)
        .service(home)
        ;
    }
}