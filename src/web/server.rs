extern crate actix_web;

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use std::path::PathBuf;

use super::ast::{AstCallback, AstCfg, AstPayload};
use crate::languages::{action, get_from_ext};

fn ast_parser(item: web::Json<AstPayload>, req: HttpRequest) -> HttpResponse {
    let language = get_from_ext(&item.language);
    let payload = item.into_inner();
    let cfg = AstCfg {
        id: payload.id,
        comment: payload.comment,
        span: payload.span,
    };
    // TODO: the 4th arg should be preproc data
    HttpResponse::Ok().json(action::<AstCallback>(
        &language.unwrap(),
        payload.code.into_bytes(),
        &PathBuf::from(""),
        None,
        cfg,
    ))
}

pub fn run(host: &str, port: u32, n_threads: usize) -> std::io::Result<()> {
    println!("Run server");
    HttpServer::new(|| {
        App::new().service(
            web::resource("/ast")
                .data(web::JsonConfig::default().limit(std::u32::MAX as usize))
                .route(web::post().to(ast_parser)),
        )
    })
    .workers(n_threads)
    .bind(format!("{}:{}", host, port))?
    .run()
}

// curl --header "Content-Type: application/json" --request POST --data '{"id": "1234", "language": "cpp", "code": "int x = 1;", "comment": true, "span": true}' http://127.0.0.1:8080/ast
