//! 服务端拦截请求

use std::error::Error;
use actix_cors::Cors;
use actix_web::http::{header, Method};
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer, HttpRequest, HttpResponse, middleware};
use colored::*;
use std::io::Result;
use std::process::exit;
use penet_utils::config::Config;
use interceptor::Interceptor;

mod interceptor;

const LOGGER_PREFIX: &str = "[Rust Penetrate Server]: ";

async fn request(req: HttpRequest, method: Method, body: String) -> Result<HttpResponse> {
    println!("request ....");
    println!("method {:#?}", method);
    println!("body {:#?}", body);
    let path = req
        .uri()
        .path()
        .strip_prefix("/v1")
        .unwrap_or(req.uri().path());

    println!("uri: {}", path);
    Ok(HttpResponse::MethodNotAllowed().finish())
}

#[actix_web::main]
async fn main() -> Result<()> {
    let config = Config::new();
    let instance = config.instance.unwrap();
    let server = instance.server.clone();
    let interceptor = instance.interceptor.clone();
    let host = server.host.as_str();
    let port = server.port;

    println!("{} server port: {}", LOGGER_PREFIX.cyan().bold(), port.to_string().magenta().bold());

    let address = format!("http://{}:{}", host, port);
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let mut web_interceptor = Interceptor::new();
    web_interceptor.get_path(&interceptor.paths);
    let list = web_interceptor.get_list();

    HttpServer::new(move || {
        let mut app =  App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST", "OPTIONS"])
                    // .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .block_on_origin_mismatch(false)
                    .max_age(36000),
            )
            .wrap(middleware::Compress::default())
            .wrap(Logger::default());

        if !list.is_empty() {
            for path in list.iter() {
                app = app.service(web::scope(path).default_service(web::to(request)));
            }
        }

        app
    })
        .bind((host, port))
        .and_then(|server| {
            println!("{} server listening on: {}", LOGGER_PREFIX.cyan().bold(), address.to_string().magenta().bold());
            Ok(server)
        })
        .unwrap_or_else(|_err| {
            println!("{} could not bind server to address {}", LOGGER_PREFIX.cyan().bold(), address.to_string().magenta().bold());
            println!("error : {}", _err.to_string());
            exit(-1)
        })
        .run()
        .await
}
