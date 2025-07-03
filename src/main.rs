use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, get, web};

struct AppState {
    app_name: String,
}

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/{id}/{name}/index.html")]
async fn index(path: web::Path<(u32, String)>) -> impl Responder {
    let (id, name) = path.into_inner();
    format!("hello {}! id:{}", name, id)
}

#[get("/app/name")]
async fn app_name(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    format!("hello {app_name}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: String::from("My first app"),
            }))
            .service(index)
            .service(app_name)
            .service(health_check)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
