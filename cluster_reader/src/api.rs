pub mod api {

    use actix_web::web;
    use actix_web::{get, post, Responder, HttpResponse};
    use tokio::time::Duration;
    use serde::Deserialize;

    use crate::use_case::use_case::{handle, get_namespaces, get_namespace_details_handler};

    #[derive(Deserialize)]
    struct Info {
        user_id: u32,
        friend: String,
    }

    #[derive(Deserialize)]
    struct Name {
        username: String,
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

    #[get("/query")]
    async fn query(name: web::Query<Name>) -> impl Responder {
        HttpResponse::Ok()
            .body(format!("Welcome {}!", name.username))
    }

    #[post("/submit/body")]
    async fn submit(name: web::Json<Name>) -> impl Responder {
        HttpResponse::Ok()
            .body(format!("Welcome {}!", name.username))
    }

    #[post("/submit/formdata")]
    async fn formdata(form: web::Form<Name>) -> impl Responder {
        HttpResponse::Ok()
            .body(format!("Welcome {}!", form.username))
    }

    async fn manual_hello() -> impl Responder {
        HttpResponse::Ok().body("Hey there!")
    }

    #[get("/testroute")]
    async fn test_route() -> impl Responder {
        
        match handle().await {
            Ok(cluster_values) => HttpResponse::Ok().json(cluster_values),
            Err(err) => HttpResponse::InternalServerError().body(format!("Error: {}", err)),
        }
    }

    #[get("namespaces")]
    async fn get_namespaces_for_cluster() -> impl Responder {
        
        match get_namespaces().await {
            Ok(namespaces) => HttpResponse::Ok().json(namespaces),
            Err(err) => HttpResponse::InternalServerError().body(format!("Error: {}", err)),
        }
    }

    #[get("namespaces/deployment-details")]
    async fn get_deployment_details() -> impl Responder {
        match get_namespace_details_handler().await {
            Ok(details) => HttpResponse::Ok().json(details),
            Err(err) => HttpResponse::InternalServerError().body(format!("Error: {}", err)),
        }
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
        .service(query)
        .service(submit)
        .service(formdata)
        .service(test_route)
        .service(get_namespaces_for_cluster)
        .service(get_deployment_details)
        ;
    }
}