use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;
use std::fmt;
use std::io::Write;
use std::path::PathBuf;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, StandardStreamLock, WriteColor};
use tree_sitter::Node;

use crate::checker::Checker;
use crate::cyclomatic::{self, Cyclomatic};
use crate::enums::NodeKind;
use crate::getter::Getter;
use crate::halstead::{self, Halstead};
use crate::sloc::{self, SourceLoc};
use crate::traits::*;

#[derive(Debug)]
pub struct CodeMetrics<'a> {
    pub cyclomatic: cyclomatic::Stats,
    pub halstead: halstead::Stats<'a>,
    pub sloc: sloc::Stats,
}

impl<'a> Serialize for CodeMetrics<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut st = serializer.serialize_struct("metrics", 3)?;
        st.serialize_field("cyclomatic", &self.cyclomatic)?;
        st.serialize_field("halstead", &self.halstead)?;
        st.serialize_field("loc", &self.sloc)?;
        st.end()
    }
}

impl<'a> Default for CodeMetrics<'a> {
    fn default() -> Self {
        Self {
            cyclomatic: cyclomatic::Stats::default(),
            halstead: halstead::Stats::default(),
            sloc: sloc::Stats::default(),
        }
    }
}

impl<'a> fmt::Display for CodeMetrics<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.cyclomatic)?;
        writeln!(f, "{}", self.halstead)?;
        write!(f, "{}", self.sloc)
    }
}

impl<'a> CodeMetrics<'a> {
    pub fn merge(&mut self, other: &CodeMetrics<'a>) {
        self.cyclomatic.merge(&other.cyclomatic);
        self.halstead.merge(&other.halstead);
        self.sloc.merge(&other.sloc);
    }
}

#[derive(Debug)]
pub struct FuncSpace<'a> {
    pub name: Option<&'a str>,
    pub spaces: Vec<FuncSpace<'a>>,
    pub metrics: CodeMetrics<'a>,
    pub kind: NodeKind,
    pub line: usize,
}

impl<'a> Serialize for FuncSpace<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut st = serializer.serialize_struct("metrics", 5)?;
        st.serialize_field("name", self.name.map_or("", |name| name))?;
        st.serialize_field("line", &self.line)?;
        st.serialize_field("metrics", &self.metrics)?;
        st.serialize_field("kind", &format!("{}", self.kind))?;
        st.serialize_field("spaces", &self.spaces)?;
        st.end()
    }
}

impl<'a> FuncSpace<'a> {
    fn new<T: Getter>(node: &Node<'a>, code: &'a [u8], kind: NodeKind) -> Self {
        Self {
            name: T::get_func_space_name(&node, code),
            spaces: Vec::new(),
            metrics: CodeMetrics::default(),
            kind,
            line: node.start_position().row + 1,
        }
    }

    fn dump_root(&self) -> std::io::Result<()> {
        let stdout = StandardStream::stdout(ColorChoice::Always);
        let mut stdout = stdout.lock();
        Self::dump_space(&self, "", true, &mut stdout)?;
        color!(stdout, White);

        Ok(())
    }

    fn dump_space(
        space: &FuncSpace,
        prefix: &str,
        last: bool,
        stdout: &mut StandardStreamLock,
    ) -> std::io::Result<()> {
        let (pref_child, pref) = if last { ("   ", "`- ") } else { ("|  ", "|- ") };

        color!(stdout, Blue);
        write!(stdout, "{}{}", prefix, pref)?;

        color!(stdout, Yellow, true);
        write!(stdout, "{}: ", space.kind)?;

        color!(stdout, Cyan, true);
        write!(stdout, "{}", space.name.map_or("", |name| name))?;

        color!(stdout, Red, true);
        writeln!(stdout, " (@{})", space.line)?;

        let prefix = format!("{}{}", prefix, pref_child);
        Self::dump_metrics(&space.metrics, &prefix, space.spaces.is_empty(), stdout)?;

        if let Some((last, spaces)) = space.spaces.split_last() {
            for space in spaces {
                Self::dump_space(space, &prefix, false, stdout)?;
            }
            Self::dump_space(last, &prefix, true, stdout)?;
        }

        Ok(())
    }

    fn dump_metrics(
        metrics: &CodeMetrics,
        prefix: &str,
        last: bool,
        stdout: &mut StandardStreamLock,
    ) -> std::io::Result<()> {
        let (pref_child, pref) = if last { ("   ", "`- ") } else { ("|  ", "|- ") };

        color!(stdout, Blue);
        write!(stdout, "{}{}", prefix, pref)?;

        color!(stdout, Yellow, true);
        writeln!(stdout, "metrics")?;

        let prefix = format!("{}{}", prefix, pref_child);
        Self::dump_cyclomatic(&metrics.cyclomatic, &prefix, false, stdout)?;
        Self::dump_halstead(&metrics.halstead, &prefix, false, stdout)?;
        Self::dump_sloc(&metrics.sloc, &prefix, true, stdout)
    }

    fn dump_cyclomatic(
        stats: &cyclomatic::Stats,
        prefix: &str,
        last: bool,
        stdout: &mut StandardStreamLock,
    ) -> std::io::Result<()> {
        let pref = if last { "`- " } else { "|- " };

        color!(stdout, Blue);
        write!(stdout, "{}{}", prefix, pref)?;

        color!(stdout, Green, true);
        write!(stdout, "cyclomatic: ")?;

        color!(stdout, White);
        writeln!(stdout, "{}", stats.cyclomatic())
    }

    fn dump_halstead(
        stats: &halstead::Stats,
        prefix: &str,
        last: bool,
        stdout: &mut StandardStreamLock,
    ) -> std::io::Result<()> {
        let (pref_child, pref) = if last { ("   ", "`- ") } else { ("|  ", "|- ") };

        color!(stdout, Blue);
        write!(stdout, "{}{}", prefix, pref)?;

        color!(stdout, Green, true);
        writeln!(stdout, "halstead")?;

        let prefix = format!("{}{}", prefix, pref_child);
        Self::dump_value(
            "unique operands",
            stats.u_operands(),
            &prefix,
            false,
            stdout,
        )?;
        Self::dump_value("operands", stats.operands(), &prefix, false, stdout)?;
        Self::dump_value(
            "unique operators",
            stats.u_operators(),
            &prefix,
            false,
            stdout,
        )?;
        Self::dump_value("operators", stats.operators(), &prefix, false, stdout)?;
        Self::dump_value("length", stats.length(), &prefix, false, stdout)?;
        Self::dump_value("size", stats.size(), &prefix, false, stdout)?;
        Self::dump_value("volume", stats.volume(), &prefix, false, stdout)?;
        Self::dump_value("difficulty", stats.difficulty(), &prefix, false, stdout)?;
        Self::dump_value("level", stats.level(), &prefix, false, stdout)?;
        Self::dump_value("effort", stats.effort(), &prefix, false, stdout)?;
        Self::dump_value("time", stats.time(), &prefix, false, stdout)?;
        Self::dump_value("bugs", stats.bugs(), &prefix, true, stdout)
    }

    fn dump_sloc(
        stats: &sloc::Stats,
        prefix: &str,
        last: bool,
        stdout: &mut StandardStreamLock,
    ) -> std::io::Result<()> {
        let (pref_child, pref) = if last { ("   ", "`- ") } else { ("|  ", "|- ") };

        color!(stdout, Blue);
        write!(stdout, "{}{}", prefix, pref)?;

        color!(stdout, Green, true);
        writeln!(stdout, "loc")?;

        let prefix = format!("{}{}", prefix, pref_child);
        Self::dump_value("sloc", stats.sloc(), &prefix, false, stdout)?;
        Self::dump_value("lloc", stats.lloc(), &prefix, true, stdout)
    }

    fn dump_value(
        name: &str,
        val: f64,
        prefix: &str,
        last: bool,
        stdout: &mut StandardStreamLock,
    ) -> std::io::Result<()> {
        let pref = if last { "`- " } else { "|- " };

        color!(stdout, Blue);
        write!(stdout, "{}{}", prefix, pref)?;

        color!(stdout, Magenta, true);
        write!(stdout, "{}: ", name)?;

        color!(stdout, White);
        writeln!(stdout, "{}", val)
    }
}

fn finalize<'a>(space_stack: &mut Vec<FuncSpace<'a>>, diff_level: usize) {
    for _ in 0..diff_level {
        if space_stack.len() <= 1 {
            break;
        }
        let space = space_stack.pop().unwrap();
        let last_space = space_stack.last_mut().unwrap();
        last_space.metrics.merge(&space.metrics);
        last_space.spaces.push(space);
    }
}

pub fn metrics<'a, T: TSParserTrait>(parser: &'a T, path: &'a PathBuf) -> Option<FuncSpace<'a>> {
    let code = parser.get_code();
    let node = parser.get_root();
    let mut cursor = node.walk();
    let mut stack = Vec::new();
    let mut children = Vec::new();
    let mut space_stack: Vec<FuncSpace> = Vec::new();
    let mut last_level = 0;

    stack.push((node, 0));

    while let Some((node, level)) = stack.pop() {
        if level < last_level {
            finalize(&mut space_stack, last_level - level);
            last_level = level;
        }

        let func_space = T::Checker::is_func(&node) || T::Checker::is_func_space(&node);

        let new_level = if func_space {
            space_stack.push(FuncSpace::new::<T::Getter>(
                &node,
                code,
                T::Getter::get_kind(&node),
            ));
            last_level = level + 1;
            last_level
        } else {
            level
        };

        if let Some(last) = space_stack.last_mut() {
            T::Cyclomatic::compute(&node, &mut last.metrics.cyclomatic);
            T::Halstead::compute(&node, code, &mut last.metrics.halstead);
            T::SourceLoc::compute(&node, code, &mut last.metrics.sloc, func_space);
        }

        cursor.reset(node);
        if cursor.goto_first_child() {
            loop {
                children.push((cursor.node(), new_level));
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
            for child in children.drain(..).rev() {
                stack.push(child);
            }
        }
    }

    finalize(&mut space_stack, std::usize::MAX);

    space_stack.pop().map(|mut space| {
        space.name = path.to_str();
        space
    })
}

pub struct MetricsCfg {
    pub path: PathBuf,
}

pub struct Metrics {}

impl Callback for Metrics {
    type Res = std::io::Result<()>;
    type Cfg = MetricsCfg;

    fn call<T: TSParserTrait>(cfg: Self::Cfg, parser: &T) -> Self::Res {
        if let Some(space) = metrics(parser, &cfg.path) {
            space.dump_root()
        } else {
            Ok(())
        }
    }
}
