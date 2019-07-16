use serde::ser::{SerializeStruct, Serializer};
use serde::{Deserialize, Serialize};

use crate::checker::Checker;
use crate::comment_rm::rm_comments;
use crate::traits::{Callback, TSParserTrait};

#[derive(Debug, Deserialize)]
pub struct WebCommentPayload {
    pub id: String,
    pub language: String,
    pub code: String,
}

#[derive(Debug, Serialize)]
pub struct WebCommentResponse {
    id: String,
    code: Option<String>,
}

pub struct WebCommentCallback {}

pub struct WebCommentCfg {
    pub id: String,
}

impl Callback for WebCommentCallback {
    type Res = WebCommentResponse;
    type Cfg = WebCommentCfg;

    fn call<T: TSParserTrait>(cfg: Self::Cfg, parser: &T) -> Self::Res {
        let code = if let Some(code) = rm_comments(parser) {
            Some(String::from_utf8(code).unwrap())
        } else {
            None
        };
        WebCommentResponse { id: cfg.id, code }
    }
}
