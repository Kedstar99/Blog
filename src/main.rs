#![allow(unused_variables)]
#![cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]

extern crate actix;
extern crate actix_web;
extern crate bytes;
extern crate env_logger;
extern crate futures;

use actix_web::http::{header, Method, StatusCode};
use actix_web::middleware::session::{self};
use actix_web::{
    fs, middleware, pred, server, App, HttpRequest, HttpResponse,
    Result,
};
use std::{env};

/// favicon handler
fn favicon(req: &HttpRequest) -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/favicon.ico")?)
}

fn blog(req: &HttpRequest) -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/blog.html")?.set_status_code(StatusCode::OK))
}

fn about(req: &HttpRequest) -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/about.html")?.set_status_code(StatusCode::OK))
}

fn experience(req: &HttpRequest) -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/experience.html")?.set_status_code(StatusCode::OK))
}

fn projects(req: &HttpRequest) -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/projects.html")?.set_status_code(StatusCode::OK))
}

fn interests(req: &HttpRequest) -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/interests.html")?.set_status_code(StatusCode::OK))
}

/// 404 handler
fn p404(req: &HttpRequest) -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/404.html")?.set_status_code(StatusCode::NOT_FOUND))
}

fn main() {
    env::set_var("RUST_LOG", "actix_web=debug");
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    let sys = actix::System::new("basic-example");

    let addr = server::new(
        || App::new()
            // enable logger
            .middleware(middleware::Logger::default())
            // cookie session middleware
            .middleware(session::SessionStorage::new(
                session::CookieSessionBackend::signed(&[0; 32]).secure(false)
            ))
            // register favicon
            .resource("/favicon", |r| r.f(favicon))
            // register simple route, handle all methods
            .resource("/blog", |r| r.f(blog))
            .resource("/about", |r| r.f(about))
            .resource("/experience", |r| r.f(experience))
            .resource("/projects", |r| r.f(projects))
            .resource("/interests", |r| r.f(interests))
            // static files
            .handler("/", fs::StaticFiles::new("static").unwrap())
            // redirect
            .resource("/", |r| r.f(blog))
            // default
            .default_resource(|r| {
                // 404 for GET request
                r.method(Method::GET).f(p404);

                // all requests that are not `GET`
                r.route().filter(pred::Not(pred::Get())).f(
                    |req| HttpResponse::MethodNotAllowed());
            }))

        .bind("127.0.0.1:8080").expect("Can not bind to 127.0.0.1:8080")
        .shutdown_timeout(0)    // <- Set shutdown timeout to 0 seconds (default 60s)
        .start();

    println!("Starting http server: 127.0.0.1:8080");
    let _ = sys.run();
}
