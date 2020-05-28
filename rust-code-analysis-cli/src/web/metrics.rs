use serde::{Deserialize, Serialize};
use serde_json::{self, Value};
use std::path::PathBuf;

use rust_code_analysis::{metrics, Callback, FuncSpace, TSParserTrait};

#[derive(Debug, Deserialize, Serialize)]
pub struct WebMetricsPayload {
    pub id: String,
    pub file_name: String,
    pub code: String,
    pub unit: bool,
}

#[derive(Debug, Serialize)]
pub struct WebMetricsResponse<'a> {
    pub id: String,
    pub language: String,
    pub spaces: Option<FuncSpace<'a>>,
}

#[derive(Debug, Deserialize)]
pub struct WebMetricsInfo {
    pub file_name: String,
    pub unit: Option<String>,
}

pub struct WebMetricsCallback {}

pub struct WebMetricsCfg {
    pub id: String,
    pub path: PathBuf,
    pub unit: bool,
    pub language: String,
}

impl Callback for WebMetricsCallback {
    type Res = Value;
    type Cfg = WebMetricsCfg;

    fn call<T: TSParserTrait>(cfg: Self::Cfg, parser: &T) -> Self::Res {
        let spaces = metrics(parser, &cfg.path);
        let spaces = if cfg.unit {
            if let Some(mut spaces) = spaces {
                spaces.spaces.clear();
                Some(spaces)
            } else {
                None
            }
        } else {
            spaces
        };

        serde_json::to_value(WebMetricsResponse {
            id: cfg.id,
            language: cfg.language,
            spaces,
        })
        .unwrap()
    }
}
