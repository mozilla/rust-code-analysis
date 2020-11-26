use serde::{Deserialize, Serialize};

use rust_code_analysis::{rm_comments, Callback, ParserTrait};

#[derive(Debug, Deserialize, Serialize)]
pub struct WebCommentPayload {
    pub id: String,
    pub file_name: String,
    pub code: String,
}

#[derive(Debug, Serialize)]
pub struct WebCommentResponse {
    pub id: String,
    pub code: Option<Vec<u8>>,
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

    fn call<T: ParserTrait>(cfg: Self::Cfg, parser: &T) -> Self::Res {
        WebCommentResponse {
            id: cfg.id,
            code: rm_comments(parser),
        }
    }
}
