use serde::{Deserialize, Serialize};
use serde_json::{self, Value};
use std::path::PathBuf;

use crate::metrics::{metrics, FuncSpace};
use crate::traits::{Callback, TSParserTrait};

#[derive(Debug, Deserialize, Serialize)]
pub struct WebMetricsPayload {
    pub id: String,
    pub file_name: String,
    pub code: String,
}

#[derive(Debug, Serialize)]
pub struct WebMetricsResponse<'a> {
    pub id: String,
    pub spaces: Option<FuncSpace<'a>>,
}

#[derive(Debug, Deserialize)]
pub struct WebMetricsInfo {
    pub file_name: String,
}

pub struct WebMetricsCallback {}

pub struct WebMetricsCfg {
    pub id: String,
    pub path: PathBuf,
}

impl Callback for WebMetricsCallback {
    type Res = Value;
    type Cfg = WebMetricsCfg;

    fn call<T: TSParserTrait>(cfg: Self::Cfg, parser: &T) -> Self::Res {
        let spaces = metrics(parser, &cfg.path);
        serde_json::to_value(WebMetricsResponse { id: cfg.id, spaces }).unwrap()
    }
}
