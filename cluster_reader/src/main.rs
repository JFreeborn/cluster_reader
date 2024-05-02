use actix_web::{error, web, App, HttpResponse, HttpServer};

mod api;
mod api_service;
mod use_case;
use crate::api::api::scoped_config;
use crate::api_service::api_service::check_config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    const ENVIRONMENT_VARIABLE_KEY: &str = "KUBECONFIG";
    const CONFIG_LOCATION: &str = "/home/joe/.kube/k3s.yaml";

    let _ = check_config(&ENVIRONMENT_VARIABLE_KEY, &CONFIG_LOCATION)?;

    HttpServer::new(move || {
        let json_config = web::JsonConfig::default()
            .limit(4096)
            .error_handler(|err, _req| {
                error::InternalError::from_response(err, HttpResponse::BadRequest().finish())
                .into()
            });

        App::new()
            .service(web::scope("/api/v1")
                .app_data(json_config)
                .configure(scoped_config))
    })
    .workers(2)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}