extern crate actix_web;

use actix_web::{dev::Body, web, App, HttpRequest, HttpResponse, HttpServer};
use std::path::PathBuf;

use super::ast::{AstCallback, AstCfg, AstPayload};
use super::comment::{WebCommentCallback, WebCommentCfg, WebCommentPayload};
use crate::languages::action;
use crate::tools::get_language_for_file;

fn ast_parser(item: web::Json<AstPayload>, _req: HttpRequest) -> HttpResponse {
    let language = get_language_for_file(&PathBuf::from(&item.file_name));
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

fn comment_removal(item: web::Json<WebCommentPayload>, _req: HttpRequest) -> HttpResponse {
    let language = get_language_for_file(&PathBuf::from(&item.file_name));
    let payload = item.into_inner();
    let cfg = WebCommentCfg { id: payload.id };
    HttpResponse::Ok().json(action::<WebCommentCallback>(
        &language.unwrap(),
        payload.code.into_bytes(),
        &PathBuf::from(""),
        None,
        cfg,
    ))
}

fn ping() -> HttpResponse {
    HttpResponse::Ok().body(Body::Empty)
}

pub fn run(host: &str, port: u32, n_threads: usize) -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::resource("/ast")
                    .data(web::JsonConfig::default().limit(std::u32::MAX as usize))
                    .route(web::post().to(ast_parser)),
            )
            .service(
                web::resource("/comment")
                    .data(web::JsonConfig::default().limit(std::u32::MAX as usize))
                    .route(web::post().to(comment_removal)),
            )
            .service(web::resource("/ping").route(web::get().to(ping)))
    })
    .workers(n_threads)
    .bind(format!("{}:{}", host, port))?
    .run()
}

// curl --header "Content-Type: application/json" --request POST --data '{"id": "1234", "file_name": "prova.cpp", "code": "int x = 1;", "comment": true, "span": true}' http://127.0.0.1:8080/ast

#[cfg(test)]
mod tests {
    use actix_web::{http::StatusCode, test};
    use serde_json::value::Value;

    use super::*;

    #[test]
    fn test_ping() {
        let mut app = test::init_service(
            App::new().service(web::resource("/ping").route(web::get().to(ping))),
        );
        let req = test::TestRequest::with_uri("/ping").to_request();
        let resp = test::call_service(&mut app, req);

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[test]
    fn test_ast() {
        let mut app = test::init_service(
            App::new().service(web::resource("/ast").route(web::post().to(ast_parser))),
        );
        let req = test::TestRequest::post()
            .uri("/ast")
            .set_json(&AstPayload {
                id: "1234".to_string(),
                file_name: "foo.c".to_string(),
                code: "int x = 1;".to_string(),
                comment: false,
                span: true,
            })
            .to_request();

        let res: Value = test::read_response_json(&mut app, req);
        let expected = json!({
            "id": "1234",
            "root": {
                "Type": "translation_unit",
                "TextValue": "",
                "Span": [1, 1, 1, 11],
                "Children": [
                    {
                        "Type": "declaration",
                        "TextValue": "",
                        "Span": [1, 1, 1, 11],
                        "Children": [
                            {
                                "Type": "primitive_type",
                                "TextValue": "int",
                                "Span": [1, 1, 1, 4],
                                "Children": []
                            },
                            {
                                "Type": "init_declarator",
                                "TextValue": "",
                                "Span": [1, 5, 1, 10],
                                "Children": [
                                    {
                                        "Type": "identifier",
                                        "TextValue": "x",
                                        "Span": [1, 5, 1, 6],
                                        "Children": []
                                    },
                                    {
                                        "Type": "=",
                                        "TextValue": "=",
                                        "Span": [1, 7, 1, 8],
                                        "Children": []
                                    },
                                    {
                                        "Type": "number_literal",
                                        "TextValue": "1",
                                        "Span": [1, 9, 1, 10],
                                        "Children": []
                                    }
                                ]
                            },
                            {
                                "Type": ";",
                                "TextValue": ";",
                                "Span": [1, 10, 1, 11],
                                "Children": []
                            }
                        ]
                    }
                ]
            }
        });
        assert_eq!(res, expected);
    }
}
