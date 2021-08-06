use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};


/*
A type implements the Responder trait if it can be converted into a HttpResponse

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
} 
*/


async fn health_check() -> impl Responder {
    HttpResponse::Ok()
} 

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // HttpServer handles all transport layer concerns
    // App is the application builder - all application logic
    HttpServer::new(|| {
        /*App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet)) */
        
        App::new()
        // call with curl http://127.0.0.1:8000/health_check
            .route("/health_check", web::get().to(health_check))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
