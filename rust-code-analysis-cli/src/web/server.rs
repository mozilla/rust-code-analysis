use actix_rt::Runtime;
use actix_web::{
    dev::Body,
    guard, http,
    web::{self, BytesMut, Query},
    App, FromRequest, HttpRequest, HttpResponse, HttpServer,
};
use futures::StreamExt;
use std::path::PathBuf;

use super::comment::{WebCommentCallback, WebCommentCfg, WebCommentInfo, WebCommentPayload};
use super::function::{WebFunctionCallback, WebFunctionCfg, WebFunctionInfo, WebFunctionPayload};
use super::metrics::{WebMetricsCallback, WebMetricsCfg, WebMetricsInfo, WebMetricsPayload};

use rust_code_analysis::{action, guess_language, AstCallback, AstCfg, AstPayload, LANG};

const INVALID_LANGUAGE: &str = "The file extension doesn't correspond to a valid language";

#[derive(Debug, Deserialize, Serialize)]
struct Error {
    id: String,
    error: &'static str,
}

async fn get_code(mut body: web::Payload) -> Result<Vec<u8>, actix_web::Error> {
    let mut code = BytesMut::new();
    while let Some(item) = body.next().await {
        code.extend_from_slice(&item?);
    }

    Ok(code.to_vec())
}

fn ast_parser(item: web::Json<AstPayload>) -> HttpResponse {
    let path = PathBuf::from(&item.file_name);
    let payload = item.into_inner();
    let buf = payload.code.into_bytes();
    let (language, _) = guess_language(&buf, &path);
    if let Some(language) = language {
        let cfg = AstCfg {
            id: payload.id,
            comment: payload.comment,
            span: payload.span,
        };

        // TODO: the 4th arg should be preproc data
        HttpResponse::Ok().json(action::<AstCallback>(
            &language,
            buf,
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

fn comment_removal_json(item: web::Json<WebCommentPayload>) -> HttpResponse {
    let path = PathBuf::from(&item.file_name);
    let payload = item.into_inner();
    let buf = payload.code.into_bytes();
    let (language, _) = guess_language(&buf, &path);
    if let Some(language) = language {
        let cfg = WebCommentCfg { id: payload.id };
        let language = if language == LANG::Cpp {
            LANG::Ccomment
        } else {
            language
        };
        HttpResponse::Ok().json(action::<WebCommentCallback>(
            &language,
            buf,
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

async fn comment_removal_plain(
    body: web::Payload,
    info: Query<WebCommentInfo>,
) -> Result<HttpResponse, actix_web::Error> {
    let buf = get_code(body).await?;
    let path = PathBuf::from(&info.file_name);
    let (language, _) = guess_language(&buf, &path);
    if let Some(language) = language {
        let cfg = WebCommentCfg { id: "".to_string() };
        let res = action::<WebCommentCallback>(&language, buf, &PathBuf::from(""), None, cfg);
        if let Some(res_code) = res.code {
            Ok(HttpResponse::Ok()
                .header(http::header::CONTENT_TYPE, "application/octet-stream")
                .body(res_code))
        } else {
            Ok(HttpResponse::NoContent()
                .header(http::header::CONTENT_TYPE, "application/octet-stream")
                .body(Body::Empty))
        }
    } else {
        Ok(HttpResponse::NotFound()
            .header(http::header::CONTENT_TYPE, "text/plain")
            .body(format!("error: {}", INVALID_LANGUAGE)))
    }
}

fn metrics_json(item: web::Json<WebMetricsPayload>) -> HttpResponse {
    let path = PathBuf::from(&item.file_name);
    let payload = item.into_inner();
    let buf = payload.code.into_bytes();
    let (language, name) = guess_language(&buf, &path);
    if let Some(language) = language {
        let cfg = WebMetricsCfg {
            id: payload.id,
            path,
            unit: payload.unit,
            language: name,
        };
        HttpResponse::Ok().json(action::<WebMetricsCallback>(
            &language,
            buf,
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

async fn metrics_plain(
    body: web::Payload,
    info: Query<WebMetricsInfo>,
) -> Result<HttpResponse, actix_web::Error> {
    let buf = get_code(body).await?;
    let path = PathBuf::from(&info.file_name);
    let (language, name) = guess_language(&buf, &path);
    if let Some(language) = language {
        let cfg = WebMetricsCfg {
            id: "".to_string(),
            path,
            unit: info
                .unit
                .as_ref()
                .map_or(false, |s| s == "1" || s == "true"),
            language: name,
        };
        Ok(HttpResponse::Ok().json(action::<WebMetricsCallback>(
            &language,
            buf,
            &PathBuf::from(""),
            None,
            cfg,
        )))
    } else {
        Ok(HttpResponse::NotFound()
            .header(http::header::CONTENT_TYPE, "text/plain")
            .body(format!("error: {}", INVALID_LANGUAGE)))
    }
}

fn function_json(item: web::Json<WebFunctionPayload>, _req: HttpRequest) -> HttpResponse {
    let path = PathBuf::from(&item.file_name);
    let payload = item.into_inner();
    let buf = payload.code.into_bytes();
    let (language, _) = guess_language(&buf, &path);
    if let Some(language) = language {
        let cfg = WebFunctionCfg { id: payload.id };
        HttpResponse::Ok().json(action::<WebFunctionCallback>(
            &language,
            buf,
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

async fn function_plain(
    body: web::Payload,
    info: Query<WebFunctionInfo>,
) -> Result<HttpResponse, actix_web::Error> {
    let buf = get_code(body).await?;
    let path = PathBuf::from(&info.file_name);
    let (language, _) = guess_language(&buf, &path);
    if let Some(language) = language {
        let cfg = WebFunctionCfg { id: "".to_string() };
        Ok(HttpResponse::Ok().json(action::<WebFunctionCallback>(
            &language,
            buf,
            &PathBuf::from(""),
            None,
            cfg,
        )))
    } else {
        Ok(HttpResponse::NotFound()
            .header(http::header::CONTENT_TYPE, "text/plain")
            .body(format!("error: {}", INVALID_LANGUAGE)))
    }
}

fn ping() -> HttpResponse {
    HttpResponse::Ok().body(Body::Empty)
}

pub fn run(host: String, port: u16, n_threads: usize) -> std::io::Result<()> {
    let _ = actix_rt::System::new("server");
    let mut rt = Runtime::new()?;
    let max_size = 1024 * 1024 * 4;

    rt.block_on(async move {
        HttpServer::new(move || {
            App::new()
                .service(
                    web::resource("/ast")
                        .guard(guard::Header("content-type", "application/json"))
                        .app_data(web::Json::<AstPayload>::configure(|cfg| {
                            cfg.limit(max_size)
                        }))
                        .route(web::post().to(ast_parser)),
                )
                .service(
                    web::resource("/comment")
                        .guard(guard::Header("content-type", "application/json"))
                        .app_data(web::Json::<WebCommentPayload>::configure(|cfg| {
                            cfg.limit(max_size)
                        }))
                        .route(web::post().to(comment_removal_json)),
                )
                .service(
                    web::resource("/comment")
                        .guard(guard::Header("content-type", "application/octet-stream"))
                        .data(web::PayloadConfig::default().limit(max_size))
                        .route(web::post().to(comment_removal_plain)),
                )
                .service(
                    web::resource("/metrics")
                        .guard(guard::Header("content-type", "application/json"))
                        .app_data(web::Json::<WebMetricsPayload>::configure(|cfg| {
                            cfg.limit(max_size)
                        }))
                        .route(web::post().to(metrics_json)),
                )
                .service(
                    web::resource("/metrics")
                        .guard(guard::Header("content-type", "application/octet-stream"))
                        .data(web::PayloadConfig::default().limit(max_size))
                        .route(web::post().to(metrics_plain)),
                )
                .service(
                    web::resource("/function")
                        .guard(guard::Header("content-type", "application/json"))
                        .app_data(web::Json::<WebFunctionPayload>::configure(|cfg| {
                            cfg.limit(max_size)
                        }))
                        .route(web::post().to(function_json)),
                )
                .service(
                    web::resource("/function")
                        .guard(guard::Header("content-type", "application/octet-stream"))
                        .data(web::PayloadConfig::default().limit(max_size))
                        .route(web::post().to(function_plain)),
                )
                .service(web::resource("/ping").route(web::get().to(ping)))
        })
        .workers(n_threads)
        .bind((host.as_str(), port))?
        .run()
        .await
    })
}

// curl --header "Content-Type: application/json" --request POST --data '{"id": "1234", "file_name": "prova.cpp", "code": "int x = 1;", "comment": true, "span": true}' http://127.0.0.1:8081/ast

#[cfg(test)]
mod tests {
    use actix_web::web::Bytes;
    use actix_web::{http::header::ContentType, http::StatusCode, test};
    use pretty_assertions::assert_eq;
    use serde_json::value::Value;

    use super::*;

    #[actix_rt::test]
    async fn test_web_ping() {
        let mut app = test::init_service(
            App::new().service(web::resource("/ping").route(web::get().to(ping))),
        )
        .await;
        let req = test::TestRequest::with_uri("/ping").to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_web_ast() {
        let mut app = test::init_service(
            App::new().service(web::resource("/ast").route(web::post().to(ast_parser))),
        )
        .await;
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

        let res: Value = test::read_response_json(&mut app, req).await;
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

    #[actix_rt::test]
    async fn test_web_ast_string() {
        let mut app = test::init_service(
            App::new().service(web::resource("/ast").route(web::post().to(ast_parser))),
        )
        .await;
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

        let res: Value = test::read_response_json(&mut app, req).await;
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

    #[actix_rt::test]
    async fn test_web_comment_json() {
        let mut app = test::init_service(
            App::new()
                .service(web::resource("/comment").route(web::post().to(comment_removal_json))),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/comment")
            .set_json(&WebCommentPayload {
                id: "1234".to_string(),
                file_name: "foo.c".to_string(),
                code: "int x = 1; // hello".to_string(),
            })
            .to_request();

        let res: Value = test::read_response_json(&mut app, req).await;
        let expected = json!({
            "id": "1234",
            "code": b"int x = 1; ",
        });

        assert_eq!(res, expected);
    }

    #[actix_rt::test]
    async fn test_web_comment_json_invalid() {
        let mut app = test::init_service(
            App::new()
                .service(web::resource("/comment").route(web::post().to(comment_removal_json))),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/comment")
            .set_json(&WebCommentPayload {
                id: "1234".to_string(),
                file_name: "foo.unexisting_extension".to_string(),
                code: "int x = 1; // hello".to_string(),
            })
            .to_request();

        let res: Value = test::read_response_json(&mut app, req).await;
        let expected = json!({
            "id": "1234",
            "error": INVALID_LANGUAGE,
        });

        assert_eq!(res, expected);
    }

    #[actix_rt::test]
    async fn test_web_comment_json_no_comment() {
        let mut app = test::init_service(
            App::new()
                .service(web::resource("/comment").route(web::post().to(comment_removal_json))),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/comment")
            .set_json(&WebCommentPayload {
                id: "1234".to_string(),
                file_name: "foo.c".to_string(),
                code: "int x = 1;".to_string(),
            })
            .to_request();

        let res: Value = test::read_response_json(&mut app, req).await;

        // No comment in the code so the code is null
        let expected = json!({
            "id": "1234",
            "code": (),
        });

        assert_eq!(res, expected);
    }

    #[actix_rt::test]
    async fn test_web_comment_plain() {
        let mut app = test::init_service(
            App::new()
                .service(web::resource("/comment").route(web::post().to(comment_removal_plain))),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/comment?file_name=foo.c")
            .set(ContentType::plaintext())
            .set_payload("int x = 1; // hello")
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let res = test::read_body(resp).await;
        let expected = Bytes::from_static(b"int x = 1; ");

        assert_eq!(res, expected);
    }

    #[actix_rt::test]
    async fn test_web_comment_plain_invalid() {
        let mut app = test::init_service(
            App::new()
                .service(web::resource("/comment").route(web::post().to(comment_removal_plain))),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/comment?file_name=foo.unexisting_extension")
            .set(ContentType::plaintext())
            .set_payload("int x = 1; // hello")
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);

        let res = test::read_body(resp).await;
        let expected = Bytes::from(format!("error: {}", INVALID_LANGUAGE));

        assert_eq!(res, expected);
    }

    #[actix_rt::test]
    async fn test_web_comment_plain_no_comment() {
        let mut app = test::init_service(
            App::new()
                .service(web::resource("/comment").route(web::post().to(comment_removal_plain))),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/comment?file_name=foo.c")
            .set(ContentType::plaintext())
            .set_payload("int x = 1;")
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::NO_CONTENT);

        let res = test::read_body(resp).await;

        // No comment in the code so the code is empty
        let expected = Bytes::from_static(b"");

        assert_eq!(res, expected);
    }

    // Inspired from https://hg.mozilla.org/mozilla-central/file/9b2a99adc05e53cd4010de512f50118594756650/extensions/java/xpcom/tests/testparams/TestParams.java#l64.
    #[actix_rt::test]
    async fn test_web_comment_plain_bad_chars() {
        let bad_bytes: &[u8] = &[142, 137, 138, 136, 140, 141, 10];
        let input_vec = [b"/*char*/s: ", bad_bytes].concat();
        let output_vec = [b"s: ", bad_bytes].concat();

        let mut app = test::init_service(
            App::new()
                .service(web::resource("/comment").route(web::post().to(comment_removal_plain))),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/comment?file_name=foo.java")
            .set(ContentType::plaintext())
            .set_payload(input_vec)
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let res = test::read_body(resp).await;

        assert_eq!(res, output_vec);
    }

    #[actix_rt::test]
    async fn test_web_metrics_json() {
        let mut app = test::init_service(
            App::new().service(web::resource("/metrics").route(web::post().to(metrics_json))),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/metrics")
            .set_json(&WebMetricsPayload {
                id: "1234".to_string(),
                file_name: "test.py".to_string(),
                code: "# -*- Mode: Objective-C++; tab-width: 2; indent-tabs-mode: nil; c-basic-offset: 2 -*-\n\ndef foo():\n    pass\n".to_string(),
                unit: false,
            })
            .to_request();

        let res: Value = test::read_response_json(&mut app, req).await;
        let expected = json!({
            "id": "1234",
            "language": "python",
            "spaces": {"kind": "unit",
                       "start_line": 1,
                       "end_line": 4,
                       "metrics": {"cyclomatic": {"sum": 2.0, "average": 1.0},
                                   "cognitive": {"sum": 0.0, "average": 0.0},
                                   "nargs": {"total_functions": 0.0, "average_functions": 0.0, "total_closures": 0.0, "average_closures": 0.0, "total": 0.0, "average": 0.0},
                                   "nexits": {"sum": 0.0, "average": 0.0},
                                   "halstead": {"bugs": 0.000_942_552_557_372_941_4,
                                                "difficulty": 1.0,
                                                "effort": 4.754_887_502_163_468,
                                                "length": 3.0,
                                                "estimated_program_length": 2.0,
                                                "purity_ratio": 0.666_666_666_666_666_6,
                                                "level": 1.0,
                                                "N2": 1.0,
                                                "N1": 2.0,
                                                "vocabulary": 3.0,
                                                "time": 0.264_160_416_786_859_36,
                                                "n2": 1.0,
                                                "n1": 2.0,
                                                "volume": 4.754_887_502_163_468},
                                   "loc": {"cloc": 1.0, "ploc": 2.0, "lloc": 1.0, "sloc": 4.0, "blank": 1.0},
                                   "nom": {"functions": 1.0, "closures": 0.0, "total": 1.0},
                                   "mi": {"mi_original": 139.974_331_558_152_1,
                                          "mi_sei": 161.414_455_240_662_22,
                                          "mi_visual_studio": 81.856_334_244_533_39}},
                       "name": "test.py",
                       "spaces": [{"kind": "function",
                                   "start_line": 3,
                                   "end_line": 4,
                                   "metrics": {"cyclomatic": {"sum": 1.0, "average": 1.0},
                                               "cognitive": {"sum": 0.0, "average": 0.0},
                                               "nargs": {"total_functions": 0.0, "average_functions": 0.0, "total_closures": 0.0, "average_closures": 0.0, "total": 0.0, "average": 0.0},
                                               "nexits": {"sum": 0.0, "average": 0.0},
                                               "halstead": {"bugs": 0.000_942_552_557_372_941_4,
                                                            "difficulty": 1.0,
                                                            "effort": 4.754_887_502_163_468,
                                                            "length": 3.0,
                                                            "estimated_program_length": 2.0,
                                                            "purity_ratio": 0.666_666_666_666_666_6,
                                                            "level": 1.0,
                                                            "N2": 1.0,
                                                            "N1": 2.0,
                                                            "vocabulary": 3.0,
                                                            "time": 0.264_160_416_786_859_36,
                                                            "n2": 1.0,
                                                            "n1": 2.0,
                                                            "volume": 4.754_887_502_163_468},
                                               "loc": {"cloc": 0.0, "ploc": 2.0, "lloc": 1.0, "sloc": 2.0, "blank": 0.0},
                                               "nom": {"functions": 1.0, "closures": 0.0, "total": 1.0},
                                               "mi": {"mi_original": 151.433_315_883_223_23,
                                                      "mi_sei": 142.873_061_717_489_78,
                                                      "mi_visual_studio": 88.557_494_668_551_6}},
                                   "name": "foo",
                                   "spaces": []}]}
        });

        assert_eq!(res, expected);
    }

    #[actix_rt::test]
    async fn test_web_metrics_json_unit() {
        let mut app = test::init_service(
            App::new().service(web::resource("/metrics").route(web::post().to(metrics_json))),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/metrics")
            .set_json(&WebMetricsPayload {
                id: "1234".to_string(),
                file_name: "test.py".to_string(),
                code: "def foo():\n    pass\n".to_string(),
                unit: true,
            })
            .to_request();

        let res: Value = test::read_response_json(&mut app, req).await;
        let expected = json!({
            "id": "1234",
            "language": "python",
            "spaces": {"kind": "unit",
                       "start_line": 1,
                       "end_line": 2,
                       "metrics": {"cyclomatic": {"sum": 2.0, "average": 1.0},
                                   "cognitive": {"sum": 0.0, "average": 0.0},
                                   "nargs": {"total_functions": 0.0, "average_functions": 0.0, "total_closures": 0.0, "average_closures": 0.0, "total": 0.0, "average": 0.0},
                                   "nexits": {"sum": 0.0, "average": 0.0},
                                   "halstead": {"bugs": 0.000_942_552_557_372_941_4,
                                                "difficulty": 1.0,
                                                "effort": 4.754_887_502_163_468,
                                                "length": 3.0,
                                                "estimated_program_length": 2.0,
                                                "purity_ratio": 0.666_666_666_666_666_6,
                                                "level": 1.0,
                                                "N2": 1.0,
                                                "N1": 2.0,
                                                "vocabulary": 3.0,
                                                "time": 0.264_160_416_786_859_36,
                                                "n2": 1.0,
                                                "n1": 2.0,
                                                "volume": 4.754_887_502_163_468},
                                   "loc": {"cloc": 0.0, "ploc": 2.0, "lloc": 1.0, "sloc": 2.0, "blank": 0.0},
                                   "nom": {"functions": 1.0, "closures": 0.0, "total": 1.0},
                                   "mi": {"mi_original": 151.203_315_883_223_2,
                                          "mi_sei": 142.643_061_717_489_76,
                                          "mi_visual_studio": 88.422_991_744_574_97}},
                       "name": "test.py",
                       "spaces": []}
        });

        assert_eq!(res, expected);
    }

    #[actix_rt::test]
    async fn test_web_metrics_plain() {
        let mut app = test::init_service(
            App::new().service(web::resource("/metrics").route(web::post().to(metrics_plain))),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/metrics?file_name=test.py")
            .set(ContentType::plaintext())
            .set_payload("def foo():\n    pass\n")
            .to_request();

        let res: Value = test::read_response_json(&mut app, req).await;
        let expected = json!({
            "id": "",
            "language": "python",
            "spaces": {"kind": "unit",
                       "start_line": 1,
                       "end_line": 2,
                       "metrics": {"cyclomatic": {"sum": 2.0, "average": 1.0},
                                   "cognitive": {"sum": 0.0, "average": 0.0},
                                   "nargs": {"total_functions": 0.0, "average_functions": 0.0, "total_closures": 0.0, "average_closures": 0.0, "total": 0.0, "average": 0.0},
                                   "nexits": {"sum": 0.0, "average": 0.0},
                                   "halstead": {"bugs": 0.000_942_552_557_372_941_4,
                                                "difficulty": 1.0,
                                                "effort": 4.754_887_502_163_468,
                                                "length": 3.0,
                                                "estimated_program_length": 2.0,
                                                "purity_ratio": 0.666_666_666_666_666_6,
                                                "level": 1.0,
                                                "N2": 1.0,
                                                "N1": 2.0,
                                                "vocabulary": 3.0,
                                                "time": 0.264_160_416_786_859_36,
                                                "n2": 1.0,
                                                "n1": 2.0,
                                                "volume": 4.754_887_502_163_468},
                                   "loc": {"cloc": 0.0, "ploc": 2.0, "lloc": 1.0, "sloc": 2.0, "blank": 0.0},
                                   "nom": {"functions": 1.0, "closures": 0.0, "total": 1.0},
                                   "mi": {"mi_original": 151.203_315_883_223_2,
                                          "mi_sei": 142.643_061_717_489_76,
                                          "mi_visual_studio": 88.422_991_744_574_97}},
                       "name": "test.py",
                       "spaces": [{"kind": "function",
                                   "start_line": 1,
                                   "end_line": 2,
                                   "metrics": {"cyclomatic": {"sum": 1.0, "average": 1.0},
                                               "cognitive": {"sum": 0.0, "average": 0.0},
                                               "nargs": {"total_functions": 0.0, "average_functions": 0.0, "total_closures": 0.0, "average_closures": 0.0, "total": 0.0, "average": 0.0},
                                               "nexits": {"sum": 0.0, "average": 0.0},
                                               "halstead": {"bugs": 0.000_942_552_557_372_941_4,
                                                            "difficulty": 1.0,
                                                            "effort": 4.754_887_502_163_468,
                                                            "length": 3.0,
                                                            "estimated_program_length": 2.0,
                                                            "purity_ratio": 0.666_666_666_666_666_6,
                                                            "level": 1.0,
                                                            "N2": 1.0,
                                                            "N1": 2.0,
                                                            "vocabulary": 3.0,
                                                            "time": 0.264_160_416_786_859_36,
                                                            "n2": 1.0,
                                                            "n1": 2.0,
                                                            "volume": 4.754_887_502_163_468},
                                               "loc": {"cloc": 0.0, "ploc": 2.0, "lloc": 1.0, "sloc": 2.0, "blank": 0.0},
                                               "nom": {"functions": 1.0, "closures": 0.0, "total": 1.0},
                                               "mi": {"mi_original": 151.433_315_883_223_23,
                                                      "mi_sei": 142.873_061_717_489_78,
                                                      "mi_visual_studio": 88.557_494_668_551_6}},
                                   "name": "foo",
                                   "spaces": []}]}
        });

        assert_eq!(res, expected);
    }

    #[actix_rt::test]
    async fn test_web_function_json() {
        let mut app = test::init_service(
            App::new().service(web::resource("/function").route(web::post().to(function_json))),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/function")
            .set_json(&WebCommentPayload {
                id: "1234".to_string(),
                file_name: "test.py".to_string(),
                code: "def foo():\n    pass\n\ndef bar():\n    pass".to_string(),
            })
            .to_request();

        let res: Value = test::read_response_json(&mut app, req).await;
        let expected = json!({
            "id": "1234",
            "spans": [
                {
                    "end_line": 2,
                    "error": false,
                    "name": "foo",
                    "start_line": 1
                },
                {
                    "end_line": 5,
                    "error": false,
                    "name": "bar",
                    "start_line": 4
                }
            ]
        });

        assert_eq!(res, expected);
    }

    #[actix_rt::test]
    async fn test_web_function_plain() {
        let mut app = test::init_service(
            App::new().service(web::resource("/function").route(web::post().to(function_plain))),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/function?file_name=test.py")
            .set(ContentType::plaintext())
            .set_payload("def foo():\n    pass\n\ndef bar():\n    pass")
            .to_request();

        let res: Value = test::read_response_json(&mut app, req).await;
        let expected = json!({
            "id": "",
            "spans": [
                {
                    "end_line": 2,
                    "error": false,
                    "name": "foo",
                    "start_line": 1
                },
                {
                    "end_line": 5,
                    "error": false,
                    "name": "bar",
                    "start_line": 4
                }
            ]
        });

        assert_eq!(res, expected);
    }
}
