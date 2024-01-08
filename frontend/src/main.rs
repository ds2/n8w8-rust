// Copyright (C) 2024 Dirk Strauss
//
// This file is part of Nachtwacht.
//
// Nachtwacht is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Nachtwacht is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::io;

use actix_files::{Files, NamedFile};
use actix_session::{storage::CookieSessionStore, Session, SessionMiddleware};
use actix_web::cookie::Key;
use actix_web::{
    error, get,
    http::{header::ContentType, Method, StatusCode},
    middleware, web, App, Either, HttpRequest, HttpResponse, HttpServer, Responder, Result,
};
//use async_stream::stream;

/// simple index handler
#[get("/welcome")]
async fn welcome(req: HttpRequest, session: Session) -> Result<HttpResponse> {
    println!("{:?}", req);

    // session
    let mut counter = 1;
    if let Some(count) = session.get::<i32>("counter")? {
        println!("SESSION value: {}", count);
        counter = count + 1;
    }

    // set counter to session
    session.insert("counter", counter)?;

    // response
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::plaintext())
        .body(include_str!("../static/welcome.html")))
}

#[get("/{id}/{name}/index.html")]
async fn index(params: web::Path<(u32, String)>) -> impl Responder {
    let (id, name) = params.into_inner();
    format!("Hello {}! id:{}", name, id)
}

/// favicon handler
#[get("/favicon")]
async fn favicon() -> Result<impl Responder> {
    Ok(NamedFile::open("frontend/static/favicon.ico")?)
}

async fn default_handler(req_method: Method) -> Result<impl Responder> {
    match req_method {
        Method::GET => {
            let file = NamedFile::open("frontend/static/404.html")?;
            let resp = Responder::customize(file).with_status(StatusCode::NOT_FOUND);
            Ok(Either::Left(resp))
        }
        _ => Ok(Either::Right(HttpResponse::MethodNotAllowed().finish())),
    }
}

// The secret key would usually be read from a configuration file/environment variables.
fn get_secret_key() -> Key {
    Key::generate()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Init..");
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("starting HTTP server at http://localhost:8080");
    println!("Server should start soon");
    let secret_key = get_secret_key();
    HttpServer::new(move || {
        App::new()
            // enable automatic response compression - usually register this first
            .wrap(middleware::Compress::default())
            // enable logger - always register Actix Web Logger middleware last
            .wrap(middleware::Logger::default())
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                secret_key.clone(),
            ))
            // register favicon
            .service(favicon)
            // register simple route, handle all methods
            .service(welcome)
            .service(web::resource("/index.html").to(|| async { "Hello world!" }))
            .service(web::resource("/error").to(|| async {
                error::InternalError::new(
                    io::Error::new(io::ErrorKind::Other, "test"),
                    StatusCode::INTERNAL_SERVER_ERROR,
                )
            }))
            // static files
            .service(Files::new("/static", "frontend/static").show_files_listing())
            .service(index)
            //.service(web::resource("/").to(index))
            .default_service(web::to(default_handler))
    })
    .bind(("0.0.0.0", 8080))?
    .workers(2)
    .run()
    .await
}
