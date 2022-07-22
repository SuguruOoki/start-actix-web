use actix_web::{get, web, App, HttpServer};

// #[get("/{id}/{name}/index.html")]
// async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
//     format!("Hello {}! id:{}", name, id)
// }

#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; // <- get app_name
    format!("Hello {app_name}!") // <- response with app_name
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: String::from("Actix Web"),
            }))
            .service(index)
    })
    // HttpServer::new(|| {
    // App::new()
    //     .app_data(web::Data::new(AppState {
    //         app_name: String::from("Actix Web"),
    //     }))
    //     .service(index)
    // .service(
    //     // prefixes all resources and routes attached to it...
    //     web::scope("/app")
    //         // ...so this handles requests for `GET /app/index.html`
    //         .route("/index.html", web::get().to(index)),
    // )
    // .service(hello)
    // })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
    // HttpServer::new(|| App::new().service(index))
    // HttpServer::new(|| {
    //     App::new()
    //         // .service(hello)
    //         // .service(echo)
    //         .service(
    //             // prefixes all resources and routes attached to it...
    //             web::scope("/app")
    //                 // ...so this handles requests for `GET /app/index.html`
    //                 .route("/index.html", web::get().to(index)),
    //         )
    //     // .route("/hey", web::get().to(manual_hello))
    // })
    // .bind("127.0.0.1:8080")?
    // .run()
    // .await
}

// This struct represents state
struct AppState {
    app_name: String,
}

// async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("Hey there!aaa")
// }
