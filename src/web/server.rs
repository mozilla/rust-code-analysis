use actix_web::{
    dev::Body, guard, http, web, web::Query, App, FromRequest, HttpRequest, HttpResponse,
    HttpServer,
};
use bytes::Bytes;
use std::path::PathBuf;

use super::ast::{AstCallback, AstCfg, AstPayload};
use super::comment::{WebCommentCallback, WebCommentCfg, WebCommentInfo, WebCommentPayload};
use super::metrics::{WebMetricsCallback, WebMetricsCfg, WebMetricsInfo, WebMetricsPayload};
use crate::languages::action;
use crate::tools::get_language_for_file;

const INVALID_LANGUAGE: &str = "The file extension doesn't correspond to a valid language";

#[derive(Debug, Deserialize, Serialize)]
struct Error {
    id: String,
    error: &'static str,
}

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

fn comment_removal_json(item: web::Json<WebCommentPayload>, _req: HttpRequest) -> HttpResponse {
    let language = get_language_for_file(&PathBuf::from(&item.file_name));
    let payload = item.into_inner();
    if let Some(language) = language {
        let cfg = WebCommentCfg { id: payload.id };
        HttpResponse::Ok().json(action::<WebCommentCallback>(
            &language,
            payload.code.into_bytes(),
            &PathBuf::from(""),
            None,
            cfg,
        ))
    } else {
        HttpResponse::NotFound().json(Error {
            id: payload.id,
            error: INVALID_LANGUAGE,
        })
    }
}

fn comment_removal_plain(code: String, info: Query<WebCommentInfo>) -> HttpResponse {
    let language = get_language_for_file(&PathBuf::from(&info.file_name));
    if let Some(language) = language {
        let cfg = WebCommentCfg { id: "".to_string() };
        let res = action::<WebCommentCallback>(
            &language,
            code.into_bytes(),
            &PathBuf::from(""),
            None,
            cfg,
        );
        if let Some(res_code) = res.code {
            HttpResponse::Ok()
                .header(http::header::CONTENT_TYPE, "text/plain")
                .body(res_code)
        } else {
            HttpResponse::NoContent()
                .header(http::header::CONTENT_TYPE, "text/plain")
                .body(Body::Empty)
        }
    } else {
        HttpResponse::NotFound()
            .header(http::header::CONTENT_TYPE, "text/plain")
            .body(format!("error: {}", INVALID_LANGUAGE))
    }
}

fn metrics_json(item: web::Json<WebMetricsPayload>, _req: HttpRequest) -> HttpResponse {
    let path = PathBuf::from(&item.file_name);
    let language = get_language_for_file(&path);
    let payload = item.into_inner();
    if let Some(language) = language {
        let cfg = WebMetricsCfg {
            id: payload.id,
            path,
        };
        HttpResponse::Ok().json(action::<WebMetricsCallback>(
            &language,
            payload.code.into_bytes(),
            &PathBuf::from(""),
            None,
            cfg,
        ))
    } else {
        HttpResponse::NotFound().json(Error {
            id: payload.id,
            error: INVALID_LANGUAGE,
        })
    }
}

fn metrics_plain(code: Bytes, info: Query<WebMetricsInfo>) -> HttpResponse {
    let path = PathBuf::from(&info.file_name);
    let language = get_language_for_file(&path);
    if let Some(language) = language {
        let cfg = WebMetricsCfg {
            id: "".to_string(),
            path,
        };
        HttpResponse::Ok().json(action::<WebMetricsCallback>(
            &language,
            code.to_vec(),
            &PathBuf::from(""),
            None,
            cfg,
        ))
    } else {
        HttpResponse::NotFound()
            .header(http::header::CONTENT_TYPE, "text/plain")
            .body(format!("error: {}", INVALID_LANGUAGE))
    }
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
                    .guard(guard::Header("content-type", "application/json"))
                    .data(web::JsonConfig::default().limit(std::u32::MAX as usize))
                    .route(web::post().to(comment_removal_json)),
            )
            .service(
                web::resource("/comment")
                    .guard(guard::Header("content-type", "text/plain"))
                    .data(String::configure(|cfg| cfg.limit(std::u32::MAX as usize)))
                    .route(web::post().to(comment_removal_plain)),
            )
            .service(
                web::resource("/metrics")
                    .guard(guard::Header("content-type", "application/json"))
                    .data(web::JsonConfig::default().limit(std::u32::MAX as usize))
                    .route(web::post().to(metrics_json)),
            )
            .service(
                web::resource("/metrics")
                    .guard(guard::Header("content-type", "application/octet-stream"))
                    .data(String::configure(|cfg| cfg.limit(std::u32::MAX as usize)))
                    .route(web::post().to(metrics_plain)),
            )
            .service(web::resource("/ping").route(web::get().to(ping)))
    })
    .workers(n_threads)
    .bind(format!("{}:{}", host, port))?
    .run()
}

// curl --header "Content-Type: application/json" --request POST --data '{"id": "1234", "file_name": "prova.cpp", "code": "int x = 1;", "comment": true, "span": true}' http://127.0.0.1:8081/ast

#[cfg(test)]
mod tests {
    use actix_web::{http::header::ContentType, http::StatusCode, test};
    use bytes::Bytes;
    use serde_json::value::Value;

    use super::*;

    #[test]
    fn test_web_ping() {
        let mut app = test::init_service(
            App::new().service(web::resource("/ping").route(web::get().to(ping))),
        );
        let req = test::TestRequest::with_uri("/ping").to_request();
        let resp = test::call_service(&mut app, req);

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[test]
    fn test_web_ast() {
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

    #[test]
    fn test_web_ast_string() {
        let mut app = test::init_service(
            App::new().service(web::resource("/ast").route(web::post().to(ast_parser))),
        );
        let req = test::TestRequest::post()
            .uri("/ast")
            .set_json(&AstPayload {
                id: "1234".to_string(),
                file_name: "foo.js".to_string(),
                code: "var x = \"hello world\";".to_string(),
                comment: false,
                span: true,
            })
            .to_request();

        let res: Value = test::read_response_json(&mut app, req);
        let expected = json!({
            "id": "1234",
            "root": {"Children": [{"Children": [{"Children": [],
                                                 "Span": [1, 1, 1, 4],
                                                 "TextValue": "var",
                                                 "Type": "var"},
                                                {"Children": [{"Children": [],
                                                               "Span": [1, 5, 1, 6],
                                                               "TextValue": "x",
                                                               "Type": "identifier"},
                                                              {"Children": [],
                                                               "Span": [1, 7, 1, 8],
                                                               "TextValue": "=",
                                                               "Type": "="},
                                                              {"Children": [],
                                                               "Span": [1, 9, 1, 22],
                                                               "TextValue": "\"hello world\"",
                                                               "Type": "string"}],
                                                 "Span": [1, 5, 1, 22],
                                                 "TextValue": "",
                                                 "Type": "variable_declarator"},
                                                {"Children": [],
                                                 "Span": [1, 22, 1, 23],
                                                 "TextValue": ";",
                                                 "Type": ";"}],
                                   "Span": [1, 1, 1, 23],
                                   "TextValue": "",
                                   "Type": "variable_declaration"}],
                     "Span": [1, 1, 1, 23],
                     "TextValue": "",
                     "Type": "program"}
        });
        assert_eq!(res, expected);
    }

    #[test]
    fn test_web_comment_json() {
        let mut app = test::init_service(
            App::new()
                .service(web::resource("/comment").route(web::post().to(comment_removal_json))),
        );
        let req = test::TestRequest::post()
            .uri("/comment")
            .set_json(&WebCommentPayload {
                id: "1234".to_string(),
                file_name: "foo.c".to_string(),
                code: "int x = 1; // hello".to_string(),
            })
            .to_request();

        let res: Value = test::read_response_json(&mut app, req);
        let expected = json!({
            "id": "1234",
            "code": "int x = 1; ",
        });

        assert_eq!(res, expected);
    }

    #[test]
    fn test_web_comment_json_invalid() {
        let mut app = test::init_service(
            App::new()
                .service(web::resource("/comment").route(web::post().to(comment_removal_json))),
        );
        let req = test::TestRequest::post()
            .uri("/comment")
            .set_json(&WebCommentPayload {
                id: "1234".to_string(),
                file_name: "foo.unexisting_extension".to_string(),
                code: "int x = 1; // hello".to_string(),
            })
            .to_request();

        let res: Value = test::read_response_json(&mut app, req);
        let expected = json!({
            "id": "1234",
            "error": INVALID_LANGUAGE,
        });

        assert_eq!(res, expected);
    }

    #[test]
    fn test_web_comment_json_no_comment() {
        let mut app = test::init_service(
            App::new()
                .service(web::resource("/comment").route(web::post().to(comment_removal_json))),
        );
        let req = test::TestRequest::post()
            .uri("/comment")
            .set_json(&WebCommentPayload {
                id: "1234".to_string(),
                file_name: "foo.c".to_string(),
                code: "int x = 1;".to_string(),
            })
            .to_request();

        let res: Value = test::read_response_json(&mut app, req);

        // No comment in the code so the code is null
        let expected = json!({
            "id": "1234",
            "code": (),
        });

        assert_eq!(res, expected);
    }

    #[test]
    fn test_web_comment_plain() {
        let mut app = test::init_service(
            App::new()
                .service(web::resource("/comment").route(web::post().to(comment_removal_plain))),
        );
        let req = test::TestRequest::post()
            .uri("/comment?file_name=foo.c")
            .set(ContentType::plaintext())
            .set_payload("int x = 1; // hello")
            .to_request();

        let resp = test::call_service(&mut app, req);
        assert_eq!(resp.status(), StatusCode::OK);

        let res = test::read_body(resp);
        let expected = Bytes::from_static(b"int x = 1; ");

        assert_eq!(res, expected);
    }

    #[test]
    fn test_web_comment_plain_invalid() {
        let mut app = test::init_service(
            App::new()
                .service(web::resource("/comment").route(web::post().to(comment_removal_plain))),
        );
        let req = test::TestRequest::post()
            .uri("/comment?file_name=foo.unexisting_extension")
            .set(ContentType::plaintext())
            .set_payload("int x = 1; // hello")
            .to_request();

        let resp = test::call_service(&mut app, req);
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);

        let res = test::read_body(resp);
        let expected = Bytes::from(format!("error: {}", INVALID_LANGUAGE).as_bytes());

        assert_eq!(res, expected);
    }

    #[test]
    fn test_web_comment_plain_no_comment() {
        let mut app = test::init_service(
            App::new()
                .service(web::resource("/comment").route(web::post().to(comment_removal_plain))),
        );
        let req = test::TestRequest::post()
            .uri("/comment?file_name=foo.c")
            .set(ContentType::plaintext())
            .set_payload("int x = 1;")
            .to_request();

        let resp = test::call_service(&mut app, req);
        assert_eq!(resp.status(), StatusCode::NO_CONTENT);

        let res = test::read_body(resp);

        // No comment in the code so the code is empty
        let expected = Bytes::from_static(b"");

        assert_eq!(res, expected);
    }

    #[test]
    fn test_web_metrics_json() {
        let mut app = test::init_service(
            App::new().service(web::resource("/metrics").route(web::post().to(metrics_json))),
        );
        let req = test::TestRequest::post()
            .uri("/metrics")
            .set_json(&WebCommentPayload {
                id: "1234".to_string(),
                file_name: "test.py".to_string(),
                code: "def foo():\n    pass\n".to_string(),
            })
            .to_request();

        let res: Value = test::read_response_json(&mut app, req);
        let expected = json!({
            "id": "1234",
            "spaces": {"kind": "unit",
                       "metrics": {"cyclomatic": 1.0,
                                   "halstead": {"bugs": 0.000_942_552_557_372_941_4,
                                                "difficulty": 1.0,
                                                "effort": 4.754_887_502_163_468,
                                                "length": 3.0,
                                                "level": 1.0,
                                                "operands": 1.0,
                                                "operators": 2.0,
                                                "size": 3.0,
                                                "time": 0.264_160_416_786_859_36,
                                                "unique_operands": 1.0,
                                                "unique_operators": 2.0,
                                                "volume": 4.754_887_502_163_468},
                                   "loc": {"lloc": 2.0, "sloc": 3.0}},
                       "name": "test.py",
                       "spaces": [{"kind": "function",
                                   "metrics": {"cyclomatic": 1.0,
                                               "halstead": {"bugs": 0.000_942_552_557_372_941_4,
                                                            "difficulty": 1.0,
                                                            "effort": 4.754_887_502_163_468,
                                                            "length": 3.0,
                                                            "level": 1.0,
                                                            "operands": 1.0,
                                                            "operators": 2.0,
                                                            "size": 3.0,
                                                            "time": 0.264_160_416_786_859_36,
                                                            "unique_operands": 1.0,
                                                            "unique_operators": 2.0,
                                                            "volume": 4.754_887_502_163_468},
                                               "loc": {"lloc": 2.0, "sloc": 2.0}},
                                   "name": "foo",
                                   "spaces": []}]}
        });

        assert_eq!(res, expected);
    }

    #[test]
    fn test_web_metrics_plain() {
        let mut app = test::init_service(
            App::new().service(web::resource("/metrics").route(web::post().to(metrics_plain))),
        );
        let req = test::TestRequest::post()
            .uri("/metrics?file_name=test.py")
            .set(ContentType::plaintext())
            .set_payload("def foo():\n    pass\n")
            .to_request();

        let res: Value = test::read_response_json(&mut app, req);
        let expected = json!({
            "id": "",
            "spaces": {"kind": "unit",
                       "metrics": {"cyclomatic": 1.0,
                                   "halstead": {"bugs": 0.000_942_552_557_372_941_4,
                                                "difficulty": 1.0,
                                                "effort": 4.754_887_502_163_468,
                                                "length": 3.0,
                                                "level": 1.0,
                                                "operands": 1.0,
                                                "operators": 2.0,
                                                "size": 3.0,
                                                "time": 0.264_160_416_786_859_36,
                                                "unique_operands": 1.0,
                                                "unique_operators": 2.0,
                                                "volume": 4.754_887_502_163_468},
                                   "loc": {"lloc": 2.0, "sloc": 3.0}},
                       "name": "test.py",
                       "spaces": [{"kind": "function",
                                   "metrics": {"cyclomatic": 1.0,
                                               "halstead": {"bugs": 0.000_942_552_557_372_941_4,
                                                            "difficulty": 1.0,
                                                            "effort": 4.754_887_502_163_468,
                                                            "length": 3.0,
                                                            "level": 1.0,
                                                            "operands": 1.0,
                                                            "operators": 2.0,
                                                            "size": 3.0,
                                                            "time": 0.264_160_416_786_859_36,
                                                            "unique_operands": 1.0,
                                                            "unique_operators": 2.0,
                                                            "volume": 4.754_887_502_163_468},
                                               "loc": {"lloc": 2.0, "sloc": 2.0}},
                                   "name": "foo",
                                   "spaces": []}]}
        });

        assert_eq!(res, expected);
    }
}
