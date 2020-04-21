use regex::Regex;
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
use crate::exit::{self, Exit};
use crate::fn_args::{self, NArgs};
use crate::getter::Getter;
use crate::halstead::{self, Halstead};
use crate::loc::{self, Loc};
use crate::mi::{self, Mi};
use crate::nom::{self, Nom};
use crate::tools::write_file;
use crate::traits::*;

#[derive(Debug)]
pub struct CodeMetrics<'a> {
    pub cyclomatic: cyclomatic::Stats,
    pub halstead: halstead::Stats<'a>,
    pub loc: loc::Stats,
    pub nom: nom::Stats,
    pub mi: mi::Stats,
    pub nargs: fn_args::Stats,
    pub nexits: exit::Stats,
}

impl<'a> Serialize for CodeMetrics<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut st = serializer.serialize_struct("metrics", 3)?;
        st.serialize_field("cyclomatic", &self.cyclomatic)?;
        st.serialize_field("halstead", &self.halstead)?;
        st.serialize_field("loc", &self.loc)?;
        st.serialize_field("nom", &self.nom)?;
        st.serialize_field("mi", &self.mi)?;
        st.serialize_field("nargs", &self.nargs)?;
        st.serialize_field("nexits", &self.nexits)?;
        st.end()
    }
}

impl<'a> Default for CodeMetrics<'a> {
    fn default() -> Self {
        Self {
            cyclomatic: cyclomatic::Stats::default(),
            halstead: halstead::Stats::default(),
            loc: loc::Stats::default(),
            nom: nom::Stats::default(),
            mi: mi::Stats::default(),
            nargs: fn_args::Stats::default(),
            nexits: exit::Stats::default(),
        }
    }
}

impl<'a> fmt::Display for CodeMetrics<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.cyclomatic)?;
        writeln!(f, "{}", self.halstead)?;
        writeln!(f, "{}", self.loc)?;
        writeln!(f, "{}", self.nom)?;
        writeln!(f, "{}", self.mi)?;
        writeln!(f, "{}", self.nargs)?;
        write!(f, "{}", self.nexits)
    }
}

impl<'a> CodeMetrics<'a> {
    pub fn merge(&mut self, other: &CodeMetrics<'a>) {
        self.cyclomatic.merge(&other.cyclomatic);
        self.halstead.merge(&other.halstead);
        self.loc.merge(&other.loc);
        self.nom.merge(&other.nom);
        self.mi.merge(&other.mi);
        self.nargs.merge(&other.nargs);
        self.nexits.merge(&other.nexits);
    }
}

#[derive(Debug)]
pub struct FuncSpace<'a> {
    pub name: Option<&'a str>,
    pub spaces: Vec<FuncSpace<'a>>,
    pub metrics: CodeMetrics<'a>,
    pub kind: NodeKind,
    pub start_line: usize,
    pub end_line: usize,
}

impl<'a> Serialize for FuncSpace<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut st = serializer.serialize_struct("metrics", 5)?;
        st.serialize_field("name", self.name.map_or("", |name| name))?;
        st.serialize_field("start_line", &self.start_line)?;
        st.serialize_field("end_line", &self.end_line)?;
        st.serialize_field("metrics", &self.metrics)?;
        st.serialize_field("kind", &format!("{}", self.kind))?;
        st.serialize_field("spaces", &self.spaces)?;
        st.end()
    }
}

impl<'a> FuncSpace<'a> {
    fn new<T: Getter>(node: &Node<'a>, code: &'a [u8], kind: NodeKind) -> Self {
        let (start_position, end_position) = match kind {
            NodeKind::Unit => {
                if node.child_count() == 0 {
                    (0, 0)
                } else {
                    (node.start_position().row + 1, node.end_position().row)
                }
            }
            _ => (node.start_position().row + 1, node.end_position().row + 1),
        };
        Self {
            name: T::get_func_space_name(&node, code),
            spaces: Vec::new(),
            metrics: CodeMetrics::default(),
            kind,
            start_line: start_position,
            end_line: end_position,
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
        writeln!(stdout, " (@{})", space.start_line)?;

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
        Self::dump_nargs(&metrics.nargs, &prefix, false, stdout)?;
        Self::dump_nexits(&metrics.nexits, &prefix, false, stdout)?;
        Self::dump_halstead(&metrics.halstead, &prefix, false, stdout)?;
        Self::dump_loc(&metrics.loc, &prefix, false, stdout)?;
        Self::dump_nom(&metrics.nom, &prefix, false, stdout)?;
        Self::dump_mi(&metrics.mi, &prefix, true, stdout)
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

        Self::dump_value("n1", stats.u_operators(), &prefix, false, stdout)?;
        Self::dump_value("N1", stats.operators(), &prefix, false, stdout)?;
        Self::dump_value("n2", stats.u_operands(), &prefix, false, stdout)?;
        Self::dump_value("N2", stats.operands(), &prefix, false, stdout)?;

        Self::dump_value("length", stats.length(), &prefix, false, stdout)?;
        Self::dump_value(
            "estimated program length",
            stats.estimated_program_length(),
            &prefix,
            false,
            stdout,
        )?;
        Self::dump_value("purity ratio", stats.purity_ratio(), &prefix, false, stdout)?;
        Self::dump_value("vocabulary", stats.vocabulary(), &prefix, false, stdout)?;
        Self::dump_value("volume", stats.volume(), &prefix, false, stdout)?;
        Self::dump_value("difficulty", stats.difficulty(), &prefix, false, stdout)?;
        Self::dump_value("level", stats.level(), &prefix, false, stdout)?;
        Self::dump_value("effort", stats.effort(), &prefix, false, stdout)?;
        Self::dump_value("time", stats.time(), &prefix, false, stdout)?;
        Self::dump_value("bugs", stats.bugs(), &prefix, true, stdout)
    }

    fn dump_loc(
        stats: &loc::Stats,
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
        Self::dump_value("ploc", stats.ploc(), &prefix, false, stdout)?;
        Self::dump_value("lloc", stats.lloc(), &prefix, false, stdout)?;
        Self::dump_value("cloc", stats.cloc(), &prefix, false, stdout)?;
        Self::dump_value("blank", stats.blank(), &prefix, true, stdout)
    }

    fn dump_nom(
        stats: &nom::Stats,
        prefix: &str,
        last: bool,
        stdout: &mut StandardStreamLock,
    ) -> std::io::Result<()> {
        let (pref_child, pref) = if last { ("   ", "`- ") } else { ("|  ", "|- ") };

        color!(stdout, Blue);
        write!(stdout, "{}{}", prefix, pref)?;

        color!(stdout, Green, true);
        writeln!(stdout, "nom")?;

        let prefix = format!("{}{}", prefix, pref_child);
        Self::dump_value("functions", stats.functions(), &prefix, false, stdout)?;
        Self::dump_value("closures", stats.closures(), &prefix, false, stdout)?;
        Self::dump_value("total", stats.total(), &prefix, true, stdout)
    }

    fn dump_mi(
        stats: &mi::Stats,
        prefix: &str,
        last: bool,
        stdout: &mut StandardStreamLock,
    ) -> std::io::Result<()> {
        let (pref_child, pref) = if last { ("   ", "`- ") } else { ("|  ", "|- ") };

        color!(stdout, Blue);
        write!(stdout, "{}{}", prefix, pref)?;

        color!(stdout, Green, true);
        writeln!(stdout, "mi")?;

        let prefix = format!("{}{}", prefix, pref_child);
        Self::dump_value("mi_original", stats.mi_original(), &prefix, false, stdout)?;
        Self::dump_value("mi_sei", stats.mi_sei(), &prefix, false, stdout)?;
        Self::dump_value(
            "mi_visual_studio",
            stats.mi_visual_studio(),
            &prefix,
            true,
            stdout,
        )
    }

    fn dump_nargs(
        stats: &fn_args::Stats,
        prefix: &str,
        last: bool,
        stdout: &mut StandardStreamLock,
    ) -> std::io::Result<()> {
        let pref = if last { "`- " } else { "|- " };

        color!(stdout, Blue);
        write!(stdout, "{}{}", prefix, pref)?;

        color!(stdout, Green, true);
        write!(stdout, "n_args: ")?;

        color!(stdout, White);
        writeln!(stdout, "{}", stats.n_args())
    }

    fn dump_nexits(
        stats: &exit::Stats,
        prefix: &str,
        last: bool,
        stdout: &mut StandardStreamLock,
    ) -> std::io::Result<()> {
        let pref = if last { "`- " } else { "|- " };

        color!(stdout, Blue);
        write!(stdout, "{}{}", prefix, pref)?;

        color!(stdout, Green, true);
        write!(stdout, "n_exits: ")?;

        color!(stdout, White);
        writeln!(stdout, "{}", stats.exit())
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

    fn dump_json(
        &self,
        path: &PathBuf,
        output_path: &PathBuf,
        pretty: bool,
    ) -> std::io::Result<()> {
        let json_data = if pretty {
            serde_json::to_string_pretty(&self).unwrap()
        } else {
            serde_json::to_string(&self).unwrap()
        };

        let mut file = path.as_path().file_name().unwrap().to_os_string();
        file.push(".json");

        let mut json_path = output_path.clone();
        json_path.push(file);

        if json_path.as_path().exists() {
            let mut new_filename = path.to_str().unwrap().to_string();
            let re = Regex::new(r"[\\:/]").unwrap();
            new_filename = re.replace_all(&new_filename, "_").to_string();
            new_filename.push_str(".json");
            json_path.pop();
            json_path.push(new_filename);
        }
        write_file(&json_path, json_data.as_bytes())
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

        let kind = T::Getter::get_kind(&node);

        let func_space = T::Checker::is_func(&node) || T::Checker::is_func_space(&node);
        let unit = kind == NodeKind::Unit;

        let new_level = if func_space {
            space_stack.push(FuncSpace::new::<T::Getter>(&node, code, kind));
            last_level = level + 1;
            last_level
        } else {
            level
        };

        if let Some(last) = space_stack.last_mut() {
            T::Cyclomatic::compute(&node, &mut last.metrics.cyclomatic);
            T::Halstead::compute(&node, code, &mut last.metrics.halstead);
            T::Loc::compute(&node, &mut last.metrics.loc, func_space, unit);
            T::Nom::compute(&node, &mut last.metrics.nom);
            T::Mi::compute(
                &node,
                &last.metrics.loc,
                &last.metrics.cyclomatic,
                &last.metrics.halstead,
                &mut last.metrics.mi,
            );
            T::NArgs::compute(&node, &mut last.metrics.nargs);
            T::Exit::compute(&node, &mut last.metrics.nexits);
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
    pub pretty: bool,
    pub output_path: Option<PathBuf>,
}

pub struct Metrics {}

impl Callback for Metrics {
    type Res = std::io::Result<()>;
    type Cfg = MetricsCfg;

    fn call<T: TSParserTrait>(cfg: Self::Cfg, parser: &T) -> Self::Res {
        if let Some(space) = metrics(parser, &cfg.path) {
            if let Some(output_path) = cfg.output_path {
                space.dump_json(&cfg.path, &output_path, cfg.pretty)
            } else {
                space.dump_root()
            }
        } else {
            Ok(())
        }
    }
}
