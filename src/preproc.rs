use petgraph::{
    algo::kosaraju_scc, graph::NodeIndex, stable_graph::StableGraph, visit::Dfs, Direction,
};
use std::collections::{hash_map, HashMap, HashSet};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use crate::langs::*;
use crate::languages::language_preproc::*;
use crate::tools::*;
use crate::traits::*;

include!(concat!(env!("OUT_DIR"), "/gen_c_specials.rs"));

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct PreprocFile {
    pub includes: HashSet<String>,
    pub extra_includes: HashSet<String>,
    pub macros: HashSet<String>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct PreprocResults {
    pub files: HashMap<PathBuf, PreprocFile>,
}

impl PreprocFile {
    pub fn new_macros(macros: &[&str]) -> Self {
        let mut pf = Self::default();
        for m in macros {
            pf.macros.insert((*m).to_string());
        }
        pf
    }
}

pub fn get_macros(file: &PathBuf, files: &HashMap<PathBuf, PreprocFile>) -> HashSet<String> {
    let mut macros = HashSet::new();
    if let Some(pf) = files.get(file) {
        for m in pf.macros.iter() {
            macros.insert(m.to_string());
        }
        for f in pf.extra_includes.iter() {
            if let Some(pf) = files.get(&PathBuf::from(f)) {
                for m in pf.macros.iter() {
                    macros.insert(m.to_string());
                }
            }
        }
    }
    macros
}

pub fn fix_includes(
    files: &mut HashMap<PathBuf, PreprocFile>,
    all_files: &HashMap<String, Vec<PathBuf>>,
) {
    let mut nodes: HashMap<PathBuf, NodeIndex> = HashMap::new();
    // Since we'll remove strong connected components we need to have a stable graph
    // in order to use the nodes we've in the nodes HashMap.
    let mut g = StableGraph::new();

    // First we build a graph of include dependencies
    for (file, pf) in files.iter() {
        let node = match nodes.entry(file.clone()) {
            hash_map::Entry::Occupied(l) => *l.get(),
            hash_map::Entry::Vacant(p) => *p.insert(g.add_node(file.clone())),
        };
        let includes = &pf.includes;
        for i in includes {
            let possibilities = guess_file(&file, i, all_files);
            for i in possibilities {
                if &i != file {
                    let i = match nodes.entry(i.clone()) {
                        hash_map::Entry::Occupied(l) => *l.get(),
                        hash_map::Entry::Vacant(p) => *p.insert(g.add_node(i)),
                    };
                    g.add_edge(node, i, 0);
                } else {
                    // TODO: add an option to display warning
                    eprintln!("Warning: possible self inclusion {:?}", file);
                }
            }
        }
    }

    // In order to walk in the graph without issues due to cycles
    // we replace strong connected components by a unique node
    // All the paths in a scc finally represents a kind of unique file containing
    // all the files in the scc.
    let mut scc = kosaraju_scc(&g);
    let mut scc_map: HashMap<NodeIndex, HashSet<String>> = HashMap::new();
    for component in scc.iter_mut() {
        if component.len() > 1 {
            // For Firefox, there are only few scc and all of them are pretty small
            // So no need to take a hammer here (for 'contains' stuff).
            // TODO: in some case a hammer can be useful: check perf Vec vs HashSet
            let mut incoming = Vec::new();
            let mut outgoing = Vec::new();
            let mut paths = HashSet::new();

            for c in component.iter() {
                for i in g.neighbors_directed(*c, Direction::Incoming) {
                    if !component.contains(&i) && !incoming.contains(&i) {
                        incoming.push(i);
                    }
                }
                for o in g.neighbors_directed(*c, Direction::Outgoing) {
                    if !component.contains(&o) && !outgoing.contains(&o) {
                        outgoing.push(o);
                    }
                }
            }

            let replacement = g.add_node(PathBuf::from(""));
            for i in incoming.drain(..) {
                g.add_edge(i, replacement, 0);
            }
            for o in outgoing.drain(..) {
                g.add_edge(replacement, o, 0);
            }
            for c in component.drain(..) {
                let path = g.remove_node(c).unwrap();
                paths.insert(path.to_str().unwrap().to_string());
                *nodes.get_mut(&path).unwrap() = replacement;
            }

            eprintln!("Warning: possible include cycle:");
            for p in paths.iter() {
                eprintln!("  - {:?}", p);
            }
            eprintln!("");

            scc_map.insert(replacement, paths);
        }
    }

    for (path, node) in nodes {
        let mut dfs = Dfs::new(&g, node);
        if let Some(pf) = files.get_mut(&path) {
            let x_inc = &mut pf.extra_includes;
            while let Some(node) = dfs.next(&g) {
                let w = g.node_weight(node).unwrap();
                if w == &PathBuf::from("") {
                    let paths = scc_map.get(&node);
                    if let Some(paths) = paths {
                        for p in paths {
                            x_inc.insert(p.to_string());
                        }
                    } else {
                        eprintln!("DEBUG: {:?} {:?}", path, node);
                    }
                } else {
                    x_inc.insert(w.to_str().unwrap().to_string());
                }
            }
        } else {
            //eprintln!("Warning: included file which has not been preprocessed: {:?}", path);
        }
    }
}

pub fn preprocess(parser: &PreprocParser, path: &PathBuf, results: Arc<Mutex<PreprocResults>>) {
    let node = parser.get_root();
    let mut cursor = node.walk();
    let mut stack = Vec::new();
    let code = parser.get_code();
    let mut file_result = PreprocFile::default();

    //dump_node(code, &node, -1, None, None);
    //eprintln!("DEBUG {:?}", path);
    stack.push(node);

    while let Some(node) = stack.pop() {
        cursor.reset(node);
        if cursor.goto_first_child() {
            loop {
                stack.push(cursor.node());
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
        }

        let id = Preproc::from(node.kind_id());
        match id {
            Preproc::Define | Preproc::Undef => {
                cursor.reset(node);
                cursor.goto_first_child();
                let identifier = cursor.node();

                if identifier.kind_id() == Preproc::Identifier {
                    let r#macro = identifier.utf8_text(&code).unwrap();
                    if !SPECIALS.contains(r#macro) {
                        file_result.macros.insert(r#macro.to_string());
                    }
                }
            }
            Preproc::PreprocInclude => {
                cursor.reset(node);
                cursor.goto_first_child();
                let file = cursor.node();

                if file.kind_id() == Preproc::StringLiteral {
                    // remove the starting/ending double quote
                    let file = &code[file.start_byte() + 1..file.end_byte() - 1];
                    let start = file.iter().position(|&c| c != b' ' && c != b'\t').unwrap();
                    let end = file.iter().rposition(|&c| c != b' ' && c != b'\t').unwrap();
                    let file = &file[start..=end];
                    let file = String::from_utf8(file.to_vec()).unwrap();
                    file_result.includes.insert(file);
                }
            }
            _ => {}
        }
    }

    let mut results = results.lock().unwrap();
    results.files.insert(path.to_path_buf(), file_result);
}
