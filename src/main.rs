use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

// #[get("/{id}/{name}/index.html")]
// async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
//     format!("Hello {}! id:{}", name, id)
// }

async fn index() -> impl Responder {
    "Hello world!aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // HttpServer::new(|| App::new().service(index))
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(
                // prefixes all resources and routes attached to it...
                web::scope("/app")
                    // ...so this handles requests for `GET /app/index.html`
                    .route("/index.html", web::get().to(index)),
            )
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!2222")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!aaa")
}
