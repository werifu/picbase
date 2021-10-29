// use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
// use std::sync::Mutex;
// #[get("/")]
// async fn hello() -> impl Responder {
//     HttpResponse::Ok().body("Hello world!")
// }

// #[post("/echo")]
// async fn echo(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }

// struct AppStateWithCounter {
//     counter: Mutex<i32>,
// }

// #[get("/counter")]
// async fn manual_hello(data: web::Data<AppStateWithCounter>) -> impl Responder {
//     let mut counter = data.counter.lock().unwrap();
//     *counter += 1;
    
//     HttpResponse::Ok().body(format!("Request number: {}", counter))
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     let counter = web::Data::new(AppStateWithCounter {
//         counter: Mutex::new(0),
//     });
//     HttpServer::new(move || {
//         App::new()
//             .app_data(counter.clone())
//             .service(hello)
//             .service(echo)
//             .service(manual_hello)
//     })
//     .bind("127.0.0.1:8080")?
//     .run()
//     .await
// }
use actix_web::{get, App, HttpRequest, HttpServer, Responder};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

#[get("/")]
async fn index(_req: HttpRequest) -> impl Responder {
    "Welcome!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // load ssl keys
    // to create a self-signed temporary cert for testing:
    // `openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'`
    let mut builder =
        SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    HttpServer::new(|| App::new().service(index))
        .bind_openssl("127.0.0.1:8080", builder)?
        .run()
        .await
}