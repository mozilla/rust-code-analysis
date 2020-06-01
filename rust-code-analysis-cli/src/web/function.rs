use serde::{Deserialize, Serialize};
use serde_json::{self, Value};

use rust_code_analysis::{function, Callback, FunctionSpan, TSParserTrait};

#[derive(Debug, Deserialize, Serialize)]
pub struct WebFunctionPayload {
    pub id: String,
    pub file_name: String,
    pub code: String,
}

#[derive(Debug, Serialize)]
pub struct WebFunctionResponse {
    pub id: String,
    pub spans: Vec<FunctionSpan>,
}

#[derive(Debug, Deserialize)]
pub struct WebFunctionInfo {
    pub file_name: String,
}

pub struct WebFunctionCallback {}

pub struct WebFunctionCfg {
    pub id: String,
}

impl Callback for WebFunctionCallback {
    type Res = Value;
    type Cfg = WebFunctionCfg;

    fn call<T: TSParserTrait>(cfg: Self::Cfg, parser: &T) -> Self::Res {
        let spans = function(parser);
        serde_json::to_value(WebFunctionResponse { id: cfg.id, spans }).unwrap()
    }
}
