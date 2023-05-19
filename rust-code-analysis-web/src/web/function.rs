use serde::{Deserialize, Serialize};
use serde_json::{self, Value};

use rust_code_analysis::{function, Callback, FunctionSpan, ParserTrait};

/// Payload containing source code with function spans to be retrieved.
#[derive(Debug, Deserialize, Serialize)]
pub struct WebFunctionPayload {
    /// Payload identifier.
    pub id: String,
    /// Source code filename.
    pub file_name: String,
    /// Source code with function spans to be retrieved.
    pub code: String,
}

/// Server response containing function spans for the requested source code.
#[derive(Debug, Serialize)]
pub struct WebFunctionResponse {
    /// Server response identifier.
    pub id: String,
    /// Function spans for the requested source code.
    pub spans: Vec<FunctionSpan>,
}

/// Source code information.
#[derive(Debug, Deserialize)]
pub struct WebFunctionInfo {
    /// Source code filename.
    pub file_name: String,
}

/// Server request configuration.
pub struct WebFunctionCfg {
    /// Request identifier.
    pub id: String,
}

/// Unit structure to implement the `Callback` trait.
pub struct WebFunctionCallback;

impl Callback for WebFunctionCallback {
    type Res = Value;
    type Cfg = WebFunctionCfg;

    fn call<T: ParserTrait>(cfg: Self::Cfg, parser: &T) -> Self::Res {
        let spans = function(parser);
        serde_json::to_value(WebFunctionResponse { id: cfg.id, spans }).unwrap()
    }
}
