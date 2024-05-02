pub mod api {

    use actix_web::web;
    use actix_web::{get, Responder, HttpResponse};

    use crate::use_case::use_case::{handle, get_namespaces, get_namespace_details_handler};


    #[get("/cluster-info")]
    async fn test_route() -> impl Responder {
        
        match handle().await {
            Ok(cluster_values) => HttpResponse::Ok().json(cluster_values),
            Err(err) => HttpResponse::InternalServerError().body(format!("Error: {}", err)),
        }
    }

    #[get("/namespaces")]
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
        .service(test_route)
        .service(get_namespaces_for_cluster)
        .service(get_deployment_details)
        ;
    }
}