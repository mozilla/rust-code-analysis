use serde::{Deserialize, Serialize};

use rust_code_analysis::{rm_comments, Callback, ParserTrait};

/// Payload containing source code with comments to be removed.
#[derive(Debug, Deserialize, Serialize)]
pub struct WebCommentPayload {
    /// Payload identifier.
    pub id: String,
    /// Source code filename.
    pub file_name: String,
    /// Source code with comments to be removed.
    pub code: String,
}

/// Server response containing the source code without comments.
#[derive(Debug, Serialize)]
pub struct WebCommentResponse {
    /// Server response identifier.
    pub id: String,
    /// Source code without comments.
    ///
    /// If `None`, an error occurred processing the request.
    pub code: Option<Vec<u8>>,
}

/// Source code information.
#[derive(Debug, Deserialize)]
pub struct WebCommentInfo {
    /// Source code filename.
    pub file_name: String,
}

/// Server request configuration.
#[derive(Debug)]
pub struct WebCommentCfg {
    /// Request identifier.
    pub id: String,
}

/// Unit structure to implement the `Callback` trait.
#[derive(Debug)]
pub struct WebCommentCallback;

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
