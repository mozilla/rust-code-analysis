use serde::{Deserialize, Serialize};

use crate::comment_rm::rm_comments;
use crate::traits::{Callback, TSParserTrait};

#[derive(Debug, Deserialize, Serialize)]
pub struct WebCommentPayload {
    pub id: String,
    pub file_name: String,
    pub code: String,
}

#[derive(Debug, Serialize)]
pub struct WebCommentResponse {
    pub id: String,
    pub code: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct WebCommentInfo {
    pub file_name: String,
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
