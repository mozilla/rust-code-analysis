use serde::{Deserialize, Serialize};
use serde_json::{self, Value};
use std::path::PathBuf;

use rust_code_analysis::{metrics, Callback, FuncSpace, ParserTrait};

/// Payload containing source code used to compute metrics.
#[derive(Debug, Deserialize, Serialize)]
pub struct WebMetricsPayload {
    /// Payload identifier.
    pub id: String,
    /// Source code filename.
    pub file_name: String,
    /// Source code used to compute metrics.
    pub code: String,
    /// Flag to consider only unit space metrics.
    pub unit: bool,
}

/// Server response containing metrics for every space present in
/// the requested source code.
#[derive(Debug, Serialize)]
pub struct WebMetricsResponse {
    /// Server response identifier.
    pub id: String,
    /// Source code programming language.
    pub language: String,
    /// Metrics for every space contained in the requested source code.
    ///
    /// If `None`, an error occurred processing the request.
    pub spaces: Option<FuncSpace>,
}

/// Source code information.
#[derive(Debug, Deserialize)]
pub struct WebMetricsInfo {
    /// Source code filename.
    pub file_name: String,
    /// Unit space code.
    ///
    ///
    /// If `None`, the entire code is considered.
    pub unit: Option<String>,
}

/// Server request configuration.
pub struct WebMetricsCfg {
    /// Request identifier.
    pub id: String,
    /// Path to the source file.
    pub path: PathBuf,
    /// Flag to consider only unit space metrics.
    pub unit: bool,
    /// Source code programming language.
    pub language: String,
}

/// Unit structure to implement the `Callback` trait.
pub struct WebMetricsCallback;

impl Callback for WebMetricsCallback {
    type Res = Value;
    type Cfg = WebMetricsCfg;

    fn call<T: ParserTrait>(cfg: Self::Cfg, parser: &T) -> Self::Res {
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
