use actix_web::{App, HttpResponse, HttpServer, web};

#[actix_web::main]
pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| {        
        App::new()
            .route("/health_check", web::get().to(health_check))
    })
        // call with curl http://127.0.0.1:8000/health_check
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

