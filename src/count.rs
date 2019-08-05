extern crate num_format;

use num_format::{Locale, ToFormattedString};
use std::fmt;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use crate::traits::*;

pub fn count<'a, T: TSParserTrait>(parser: &'a T, filters: &[String]) -> (usize, usize) {
    let filters = parser.get_filters(filters);
    let node = parser.get_root();
    let mut cursor = node.walk();
    let mut stack = Vec::new();
    let mut good = 0;
    let mut total = 0;

    stack.push(node);

    while let Some(node) = stack.pop() {
        total += 1;
        if filters.any(&node) {
            good += 1;
        }
        cursor.reset(node);
        if cursor.goto_first_child() {
            loop {
                stack.push(cursor.node());
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
        }
    }
    (good, total)
}

pub struct CountCfg {
    pub path: Option<PathBuf>,
    pub filters: Vec<String>,
    pub stats: Arc<Mutex<Count>>,
}

#[derive(Debug, Default)]
pub struct Count {
    pub good: usize,
    pub total: usize,
}

impl Callback for Count {
    type Res = std::io::Result<()>;
    type Cfg = CountCfg;

    fn call<T: TSParserTrait>(cfg: Self::Cfg, parser: &T) -> Self::Res {
        let (good, total) = count(parser, &cfg.filters);
        let mut results = cfg.stats.lock().unwrap();
        results.good += good;
        results.total += total;
        Ok(())
    }
}

impl fmt::Display for Count {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "Total nodes: {}",
            self.total.to_formatted_string(&Locale::en)
        )?;
        writeln!(
            f,
            "Found nodes: {}",
            self.good.to_formatted_string(&Locale::en)
        )?;
        write!(
            f,
            "Percentage: {:.2}%",
            (self.good as f64) / (self.total as f64) * 100.
        )
    }
}
