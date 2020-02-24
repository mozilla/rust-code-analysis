use fxhash::FxHashMap;
use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;
use std::fmt;
use tree_sitter::Node;

use crate::checker::Checker;

use crate::*;

#[derive(Default, Debug)]
pub struct Stats<'a> {
    pub operators: FxHashMap<u16, u64>,
    pub operands: FxHashMap<&'a [u8], u64>,
}

impl Serialize for Stats<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut st = serializer.serialize_struct("halstead", 12)?;
        st.serialize_field("unique_operands", &self.u_operands())?;
        st.serialize_field("operands", &self.operands())?;
        st.serialize_field("unique_operators", &self.u_operators())?;
        st.serialize_field("operators", &self.operators())?;
        st.serialize_field("length", &self.length())?;
        st.serialize_field("size", &self.size())?;
        st.serialize_field("volume", &self.volume())?;
        st.serialize_field("difficulty", &self.difficulty())?;
        st.serialize_field("level", &self.level())?;
        st.serialize_field("effort", &self.effort())?;
        st.serialize_field("time", &self.time())?;
        st.serialize_field("bugs", &self.bugs())?;
        st.end()
    }
}

impl<'a> fmt::Display for Stats<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unique operands: {}, operands: {}, unique operators: {}, operators: {}, length: {}, size: {}, volume: {}, difficulty: {}, level: {}, effort: {}, time: {}, bugs: {}", self.u_operands(), self.operands(), self.u_operators(), self.operators(), self.length(), self.size(), self.volume(), self.difficulty(), self.level(), self.effort(), self.time(), self.bugs())
    }
}

impl<'a> Stats<'a> {
    pub fn merge(&mut self, other: &Stats<'a>) {
        for (k, v) in other.operators.iter() {
            *self.operators.entry(*k).or_insert(0) += v;
        }
        for (k, v) in other.operands.iter() {
            *self.operands.entry(*k).or_insert(0) += v;
        }
    }

    #[inline(always)]
    pub fn u_operands(&self) -> f64 {
        self.operands.len() as f64
    }

    #[inline(always)]
    pub fn operands(&self) -> f64 {
        self.operands.values().sum::<u64>() as f64
    }

    #[inline(always)]
    pub fn u_operators(&self) -> f64 {
        self.operators.len() as f64
    }

    #[inline(always)]
    pub fn operators(&self) -> f64 {
        self.operators.values().sum::<u64>() as f64
    }

    #[inline(always)]
    pub fn length(&self) -> f64 {
        self.operands() + self.operators()
    }

    #[inline(always)]
    pub fn size(&self) -> f64 {
        self.u_operands() + self.u_operators()
    }

    #[inline(always)]
    pub fn volume(&self) -> f64 {
        self.length() * self.size().log2()
    }

    #[inline(always)]
    pub fn difficulty(&self) -> f64 {
        self.u_operators() / 2. * self.operands() / self.u_operands()
    }

    #[inline(always)]
    pub fn level(&self) -> f64 {
        1. / self.difficulty()
    }

    #[inline(always)]
    pub fn effort(&self) -> f64 {
        self.difficulty() * self.volume()
    }

    #[inline(always)]
    pub fn time(&self) -> f64 {
        self.effort() / 18.
    }

    #[inline(always)]
    pub fn bugs(&self) -> f64 {
        self.effort().powf(2. / 3.) / 3000.
    }
}

pub trait Halstead
where
    Self: Checker,
{
    fn compute<'a>(_node: &Node<'a>, _code: &'a [u8], _stats: &mut Stats<'a>) {}
}

#[inline(always)]
fn get_id<'a>(node: &Node<'a>, code: &'a [u8]) -> &'a [u8] {
    &code[node.start_byte()..node.end_byte()]
}

impl Halstead for PythonCode {
    fn compute<'a>(node: &Node<'a>, code: &'a [u8], stats: &mut Stats<'a>) {
        use Python::*;

        let id = node.kind_id();
        match id.into() {
            Import | DOT | From | LPAREN | COMMA | As | STAR | GTGT | Assert | COLONEQ | Return
            | Del | Raise | Pass | Break | Continue | If | Elif | Else | Async | For | In
            | While | Try | Except | Finally | With | DASHGT | EQ | Global | Exec | AT | Not
            | And | Or | PLUS | DASH | SLASH | PERCENT | SLASHSLASH | STARSTAR | PIPE | AMP
            | CARET | LTLT | TILDE | LT | LTEQ | EQEQ | BANGEQ | GTEQ | GT | LTGT | Is | PLUSEQ
            | DASHEQ | STAREQ | SLASHEQ | ATEQ | SLASHSLASHEQ | PERCENTEQ | STARSTAREQ | GTGTEQ
            | LTLTEQ | AMPEQ | CARETEQ | PIPEEQ | Yield | LBRACK | LBRACE | Await | Await2
            | Print => {
                *stats.operators.entry(id).or_insert(0) += 1;
            }
            Identifier | Integer | Float | True | False | None => {
                *stats.operands.entry(get_id(node, code)).or_insert(0) += 1;
            }
            String => {
                // check if we've a documentation string or a multiline comment
                if let Some(parent) = node.parent() {
                    if parent.kind_id() != ExpressionStatement || parent.child_count() != 1 {
                        *stats.operands.entry(get_id(node, code)).or_insert(0) += 1;
                    }
                }
            }
            _ => {}
        }
    }
}

impl Halstead for MozjsCode {
    fn compute<'a>(node: &Node<'a>, code: &'a [u8], stats: &mut Stats<'a>) {
        use Mozjs::*;

        let id = node.kind_id();
        match id.into() {
            Export | Import | Import2 | Extends | DOT | From | LPAREN | COMMA | As | STAR
            | GTGT | GTGTGT | COLON | Return | Delete | Throw | Break | Continue | If | Else
            | Switch | Case | Default | Async | For | In | Of | While | Try | Catch | Finally
            | With | EQ | AT | AMPAMP | PIPEPIPE | PLUS | DASH | DASHDASH | PLUSPLUS | SLASH
            | PERCENT | STARSTAR | PIPE | AMP | LTLT | TILDE | LT | LTEQ | EQEQ | BANGEQ | GTEQ
            | GT | PLUSEQ | BANG | BANGEQEQ | EQEQEQ | DASHEQ | STAREQ | SLASHEQ | PERCENTEQ
            | STARSTAREQ | GTGTEQ | GTGTGTEQ | LTLTEQ | AMPEQ | CARET | CARETEQ | PIPEEQ
            | Yield | LBRACK | LBRACE | Await | QMARK | QMARKQMARK | New | Let | Var | Const => {
                *stats.operators.entry(id).or_insert(0) += 1;
            }
            Identifier | Identifier2 | String | Number | True | False | Null | Void | This
            | Super | Undefined | Set | Get | Typeof | Instanceof => {
                *stats.operands.entry(get_id(node, code)).or_insert(0) += 1;
            }
            _ => {}
        }
    }
}

impl Halstead for JavascriptCode {
    fn compute<'a>(node: &Node<'a>, code: &'a [u8], stats: &mut Stats<'a>) {
        use Javascript::*;

        let id = node.kind_id();
        match id.into() {
            Export | Import | Import2 | Extends | DOT | From | LPAREN | COMMA | As | STAR
            | GTGT | GTGTGT | COLON | Return | Delete | Throw | Break | Continue | If | Else
            | Switch | Case | Default | Async | For | In | Of | While | Try | Catch | Finally
            | With | EQ | AT | AMPAMP | PIPEPIPE | PLUS | DASH | DASHDASH | PLUSPLUS | SLASH
            | PERCENT | STARSTAR | PIPE | AMP | LTLT | TILDE | LT | LTEQ | EQEQ | BANGEQ | GTEQ
            | GT | PLUSEQ | BANG | BANGEQEQ | EQEQEQ | DASHEQ | STAREQ | SLASHEQ | PERCENTEQ
            | STARSTAREQ | GTGTEQ | GTGTGTEQ | LTLTEQ | AMPEQ | CARET | CARETEQ | PIPEEQ
            | Yield | LBRACK | LBRACE | Await | QMARK | QMARKQMARK | New | Let | Var | Const => {
                *stats.operators.entry(id).or_insert(0) += 1;
            }
            Identifier | Identifier2 | String | Number | True | False | Null | Void | This
            | Super | Undefined | Set | Get | Typeof | Instanceof => {
                *stats.operands.entry(get_id(node, code)).or_insert(0) += 1;
            }
            _ => {}
        }
    }
}

impl Halstead for TypescriptCode {
    fn compute<'a>(node: &Node<'a>, code: &'a [u8], stats: &mut Stats<'a>) {
        use Typescript::*;

        let id = node.kind_id();
        match id.into() {
            Export | Import | Import2 | Extends | DOT | From | LPAREN | COMMA | As | STAR
            | GTGT | GTGTGT | COLON | Return | Delete | Throw | Break | Continue | If | Else
            | Switch | Case | Default | Async | For | In | Of | While | Try | Catch | Finally
            | With | EQ | AT | AMPAMP | PIPEPIPE | PLUS | DASH | DASHDASH | PLUSPLUS | SLASH
            | PERCENT | STARSTAR | PIPE | AMP | LTLT | TILDE | LT | LTEQ | EQEQ | BANGEQ | GTEQ
            | GT | PLUSEQ | BANG | BANGEQEQ | EQEQEQ | DASHEQ | STAREQ | SLASHEQ | PERCENTEQ
            | STARSTAREQ | GTGTEQ | GTGTGTEQ | LTLTEQ | AMPEQ | CARET | CARETEQ | PIPEEQ
            | Yield | LBRACK | LBRACE | Await | QMARK | QMARKQMARK | New | Let | Var | Const => {
                *stats.operators.entry(id).or_insert(0) += 1;
            }
            Identifier | NestedIdentifier | String | Number | True | False | Null | Void | This
            | Super | Undefined | Set | Get | Typeof | Instanceof => {
                *stats.operands.entry(get_id(node, code)).or_insert(0) += 1;
            }
            _ => {}
        }
    }
}

impl Halstead for TsxCode {
    fn compute<'a>(node: &Node<'a>, code: &'a [u8], stats: &mut Stats<'a>) {
        use Tsx::*;

        let id = node.kind_id();
        match id.into() {
            Export | Import | Import2 | Extends | DOT | From | LPAREN | COMMA | As | STAR
            | GTGT | GTGTGT | COLON | Return | Delete | Throw | Break | Continue | If | Else
            | Switch | Case | Default | Async | For | In | Of | While | Try | Catch | Finally
            | With | EQ | AT | AMPAMP | PIPEPIPE | PLUS | DASH | DASHDASH | PLUSPLUS | SLASH
            | PERCENT | STARSTAR | PIPE | AMP | LTLT | TILDE | LT | LTEQ | EQEQ | BANGEQ | GTEQ
            | GT | PLUSEQ | BANG | BANGEQEQ | EQEQEQ | DASHEQ | STAREQ | SLASHEQ | PERCENTEQ
            | STARSTAREQ | GTGTEQ | GTGTGTEQ | LTLTEQ | AMPEQ | CARET | CARETEQ | PIPEEQ
            | Yield | LBRACK | LBRACE | Await | QMARK | QMARKQMARK | New | Let | Var | Const => {
                *stats.operators.entry(id).or_insert(0) += 1;
            }
            Identifier | NestedIdentifier | String | Number | True | False | Null | Void | This
            | Super | Undefined | Set | Get | Typeof | Instanceof => {
                *stats.operands.entry(get_id(node, code)).or_insert(0) += 1;
            }
            _ => {}
        }
    }
}

impl Halstead for RustCode {
    fn compute<'a>(node: &Node<'a>, code: &'a [u8], stats: &mut Stats<'a>) {
        use Rust::*;

        let id = node.kind_id();
        match id.into() {
            LPAREN | LBRACE | LBRACK | EQGT | PLUS | STAR | Async | Await | Continue | For | If
            | Let | Loop | Match | Return | Unsafe | While | BANG | EQ | COMMA | DASHGT | QMARK
            | LT | GT | AMP | MutableSpecifier | DOTDOT | DOTDOTEQ | DASH | AMPAMP | PIPEPIPE
            | PIPE | CARET | EQEQ | BANGEQ | LTEQ | GTEQ | LTLT | GTGT | SLASH | PERCENT
            | PLUSEQ | DASHEQ | STAREQ | SLASHEQ | PERCENTEQ | AMPEQ | PIPEEQ | CARETEQ
            | LTLTEQ | GTGTEQ | Move | DOT => {
                *stats.operators.entry(id).or_insert(0) += 1;
            }
            Identifier | StringLiteral | RawStringLiteral | IntegerLiteral | FloatLiteral
            | BooleanLiteral | Zelf | CharLiteral | UNDERSCORE => {
                *stats.operands.entry(get_id(node, code)).or_insert(0) += 1;
            }
            _ => {}
        }
    }
}

impl Halstead for CppCode {
    fn compute<'a>(node: &Node<'a>, code: &'a [u8], stats: &mut Stats<'a>) {
        use Cpp::*;

        let id = node.kind_id();

        match id.into() {
            DOT | LPAREN | COMMA | STAR | GTGT | COLON | Return | Break | Continue | If | Else
            | Switch | Case | Default | For | While | Goto | Do | Delete | New | Try | Catch
            | Throw | EQ | AMPAMP | PIPEPIPE | PLUS | PLUSPLUS | SLASH | PERCENT | PIPE | AMP
            | LTLT | TILDE | LT | LTEQ | EQEQ | BANGEQ | GTEQ | GT | PLUSEQ | BANG | STAREQ
            | SLASHEQ | PERCENTEQ | GTGTEQ | LTLTEQ | AMPEQ | CARET | CARETEQ | PIPEEQ | LBRACK
            | LBRACE | QMARK | COLONCOLON | TypeSpecifier | Sizeof => {
                *stats.operators.entry(id).or_insert(0) += 1;
            }
            Identifier | TypeIdentifier | FieldIdentifier | PrimitiveType | RawStringLiteral
            | StringLiteral | NumberLiteral | True | False | Null | Nullptr | DOTDOTDOT => {
                *stats.operands.entry(get_id(node, code)).or_insert(0) += 1;
            }
            _ => {}
        }
    }
}

impl Halstead for PreprocCode {}
impl Halstead for CcommentCode {}
impl Halstead for CSharpCode {}
impl Halstead for JavaCode {}
impl Halstead for GoCode {}
impl Halstead for CssCode {}
impl Halstead for HtmlCode {}
