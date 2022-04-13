use crate::metrics::halstead::HalsteadType;

use crate::spaces::SpaceKind;
use crate::traits::Search;

use crate::*;

macro_rules! get_operator {
    ($language:ident) => {
        #[inline(always)]
        fn get_operator_id_as_str(id: u16) -> &'static str {
            let typ = id.into();
            match typ {
                $language::LPAREN => "()",
                $language::LBRACK => "[]",
                $language::LBRACE => "{}",
                _ => typ.into(),
            }
        }
    };
}

pub trait Getter {
    fn get_func_name<'a>(node: &Node, code: &'a [u8]) -> Option<&'a str> {
        Self::get_func_space_name(node, code)
    }

    fn get_func_space_name<'a>(node: &Node, code: &'a [u8]) -> Option<&'a str> {
        // we're in a function or in a class
        if let Some(name) = node.child_by_field_name("name") {
            let code = &code[name.start_byte()..name.end_byte()];
            std::str::from_utf8(code).ok()
        } else {
            Some("<anonymous>")
        }
    }

    fn get_space_kind(_node: &Node) -> SpaceKind {
        SpaceKind::Unknown
    }

    fn get_op_type(_node: &Node) -> HalsteadType {
        HalsteadType::Unknown
    }

    fn get_operator_id_as_str(_id: u16) -> &'static str {
        ""
    }
}

impl Getter for PythonCode {
    fn get_space_kind(node: &Node) -> SpaceKind {
        match node.kind_id().into() {
            Python::FunctionDefinition => SpaceKind::Function,
            Python::ClassDefinition => SpaceKind::Class,
            Python::Module => SpaceKind::Unit,
            _ => SpaceKind::Unknown,
        }
    }

    fn get_op_type(node: &Node) -> HalsteadType {
        use Python::*;

        match node.kind_id().into() {
            Import | DOT | From | COMMA | As | STAR | GTGT | Assert | COLONEQ | Return | Def
            | Del | Raise | Pass | Break | Continue | If | Elif | Else | Async | For | In
            | While | Try | Except | Finally | With | DASHGT | EQ | Global | Exec | AT | Not
            | And | Or | PLUS | DASH | SLASH | PERCENT | SLASHSLASH | STARSTAR | PIPE | AMP
            | CARET | LTLT | TILDE | LT | LTEQ | EQEQ | BANGEQ | GTEQ | GT | LTGT | Is | PLUSEQ
            | DASHEQ | STAREQ | SLASHEQ | ATEQ | SLASHSLASHEQ | PERCENTEQ | STARSTAREQ | GTGTEQ
            | LTLTEQ | AMPEQ | CARETEQ | PIPEEQ | Yield | Await | Await2 | Print => {
                HalsteadType::Operator
            }
            Identifier | Integer | Float | True | False | None => HalsteadType::Operand,
            String => {
                let mut operator = HalsteadType::Unknown;
                // check if we've a documentation string or a multiline comment
                if let Some(parent) = node.parent() {
                    if parent.kind_id() != ExpressionStatement || parent.child_count() != 1 {
                        operator = HalsteadType::Operand;
                    };
                }
                operator
            }
            _ => HalsteadType::Unknown,
        }
    }

    fn get_operator_id_as_str(id: u16) -> &'static str {
        Into::<Python>::into(id).into()
    }
}

impl Getter for MozjsCode {
    fn get_space_kind(node: &Node) -> SpaceKind {
        use Mozjs::*;

        match node.kind_id().into() {
            Function
            | MethodDefinition
            | GeneratorFunction
            | FunctionDeclaration
            | GeneratorFunctionDeclaration
            | ArrowFunction => SpaceKind::Function,
            Class | ClassDeclaration => SpaceKind::Class,
            Program => SpaceKind::Unit,
            _ => SpaceKind::Unknown,
        }
    }

    fn get_func_space_name<'a>(node: &Node, code: &'a [u8]) -> Option<&'a str> {
        if let Some(name) = node.child_by_field_name("name") {
            let code = &code[name.start_byte()..name.end_byte()];
            std::str::from_utf8(code).ok()
        } else {
            // We can be in a pair: foo: function() {}
            // Or in a variable declaration: var aFun = function() {}
            if let Some(parent) = node.parent() {
                match parent.kind_id().into() {
                    Mozjs::Pair => {
                        if let Some(name) = parent.child_by_field_name("key") {
                            let code = &code[name.start_byte()..name.end_byte()];
                            return std::str::from_utf8(code).ok();
                        }
                    }
                    Mozjs::VariableDeclarator => {
                        if let Some(name) = parent.child_by_field_name("name") {
                            let code = &code[name.start_byte()..name.end_byte()];
                            return std::str::from_utf8(code).ok();
                        }
                    }
                    _ => {}
                }
            }
            Some("<anonymous>")
        }
    }

    fn get_op_type(node: &Node) -> HalsteadType {
        use Mozjs::*;

        match node.kind_id().into() {
            Export | Import | Import2 | Extends | DOT | From | LPAREN | COMMA | As | STAR
            | GTGT | GTGTGT | COLON | Return | Delete | Throw | Break | Continue | If | Else
            | Switch | Case | Default | Async | For | In | Of | While | Try | Catch | Finally
            | With | EQ | AT | AMPAMP | PIPEPIPE | PLUS | DASH | DASHDASH | PLUSPLUS | SLASH
            | PERCENT | STARSTAR | PIPE | AMP | LTLT | TILDE | LT | LTEQ | EQEQ | BANGEQ | GTEQ
            | GT | PLUSEQ | BANG | BANGEQEQ | EQEQEQ | DASHEQ | STAREQ | SLASHEQ | PERCENTEQ
            | STARSTAREQ | GTGTEQ | GTGTGTEQ | LTLTEQ | AMPEQ | CARET | CARETEQ | PIPEEQ
            | Yield | LBRACK | LBRACE | Await | QMARK | QMARKQMARK | New | Let | Var | Const
            | Function | Function2 | SEMI => HalsteadType::Operator,
            Identifier | Identifier2 | MemberExpression | PropertyIdentifier | String | Number
            | True | False | Null | Void | This | Super | Undefined | Set | Get | Typeof
            | Instanceof => HalsteadType::Operand,
            _ => HalsteadType::Unknown,
        }
    }

    get_operator!(Mozjs);
}

impl Getter for JavascriptCode {
    fn get_space_kind(node: &Node) -> SpaceKind {
        use Javascript::*;

        match node.kind_id().into() {
            Function
            | MethodDefinition
            | GeneratorFunction
            | FunctionDeclaration
            | GeneratorFunctionDeclaration
            | ArrowFunction => SpaceKind::Function,
            Class | ClassDeclaration => SpaceKind::Class,
            Program => SpaceKind::Unit,
            _ => SpaceKind::Unknown,
        }
    }

    fn get_func_space_name<'a>(node: &Node, code: &'a [u8]) -> Option<&'a str> {
        if let Some(name) = node.child_by_field_name("name") {
            let code = &code[name.start_byte()..name.end_byte()];
            std::str::from_utf8(code).ok()
        } else {
            // We can be in a pair: foo: function() {}
            // Or in a variable declaration: var aFun = function() {}
            if let Some(parent) = node.parent() {
                match parent.kind_id().into() {
                    Mozjs::Pair => {
                        if let Some(name) = parent.child_by_field_name("key") {
                            let code = &code[name.start_byte()..name.end_byte()];
                            return std::str::from_utf8(code).ok();
                        }
                    }
                    Mozjs::VariableDeclarator => {
                        if let Some(name) = parent.child_by_field_name("name") {
                            let code = &code[name.start_byte()..name.end_byte()];
                            return std::str::from_utf8(code).ok();
                        }
                    }
                    _ => {}
                }
            }
            Some("<anonymous>")
        }
    }

    fn get_op_type(node: &Node) -> HalsteadType {
        use Javascript::*;

        match node.kind_id().into() {
            Export | Import | Import2 | Extends | DOT | From | LPAREN | COMMA | As | STAR
            | GTGT | GTGTGT | COLON | Return | Delete | Throw | Break | Continue | If | Else
            | Switch | Case | Default | Async | For | In | Of | While | Try | Catch | Finally
            | With | EQ | AT | AMPAMP | PIPEPIPE | PLUS | DASH | DASHDASH | PLUSPLUS | SLASH
            | PERCENT | STARSTAR | PIPE | AMP | LTLT | TILDE | LT | LTEQ | EQEQ | BANGEQ | GTEQ
            | GT | PLUSEQ | BANG | BANGEQEQ | EQEQEQ | DASHEQ | STAREQ | SLASHEQ | PERCENTEQ
            | STARSTAREQ | GTGTEQ | GTGTGTEQ | LTLTEQ | AMPEQ | CARET | CARETEQ | PIPEEQ
            | Yield | LBRACK | LBRACE | Await | QMARK | QMARKQMARK | New | Let | Var | Const
            | Function | Function2 | SEMI => HalsteadType::Operator,
            Identifier | Identifier2 | MemberExpression | PropertyIdentifier | String | Number
            | True | False | Null | Void | This | Super | Undefined | Set | Get | Typeof
            | Instanceof => HalsteadType::Operand,
            _ => HalsteadType::Unknown,
        }
    }

    get_operator!(Javascript);
}

impl Getter for TypescriptCode {
    fn get_space_kind(node: &Node) -> SpaceKind {
        use Typescript::*;

        match node.kind_id().into() {
            Function
            | MethodDefinition
            | GeneratorFunction
            | FunctionDeclaration
            | GeneratorFunctionDeclaration
            | ArrowFunction => SpaceKind::Function,
            Class | ClassDeclaration => SpaceKind::Class,
            InterfaceDeclaration => SpaceKind::Interface,
            Program => SpaceKind::Unit,
            _ => SpaceKind::Unknown,
        }
    }

    fn get_func_space_name<'a>(node: &Node, code: &'a [u8]) -> Option<&'a str> {
        if let Some(name) = node.child_by_field_name("name") {
            let code = &code[name.start_byte()..name.end_byte()];
            std::str::from_utf8(code).ok()
        } else {
            // We can be in a pair: foo: function() {}
            // Or in a variable declaration: var aFun = function() {}
            if let Some(parent) = node.parent() {
                match parent.kind_id().into() {
                    Mozjs::Pair => {
                        if let Some(name) = parent.child_by_field_name("key") {
                            let code = &code[name.start_byte()..name.end_byte()];
                            return std::str::from_utf8(code).ok();
                        }
                    }
                    Mozjs::VariableDeclarator => {
                        if let Some(name) = parent.child_by_field_name("name") {
                            let code = &code[name.start_byte()..name.end_byte()];
                            return std::str::from_utf8(code).ok();
                        }
                    }
                    _ => {}
                }
            }
            Some("<anonymous>")
        }
    }

    fn get_op_type(node: &Node) -> HalsteadType {
        use Typescript::*;

        match node.kind_id().into() {
            Export | Import | Import2 | Extends | DOT | From | LPAREN | COMMA | As | STAR
            | GTGT | GTGTGT | COLON | Return | Delete | Throw | Break | Continue | If | Else
            | Switch | Case | Default | Async | For | In | Of | While | Try | Catch | Finally
            | With | EQ | AT | AMPAMP | PIPEPIPE | PLUS | DASH | DASHDASH | PLUSPLUS | SLASH
            | PERCENT | STARSTAR | PIPE | AMP | LTLT | TILDE | LT | LTEQ | EQEQ | BANGEQ | GTEQ
            | GT | PLUSEQ | BANG | BANGEQEQ | EQEQEQ | DASHEQ | STAREQ | SLASHEQ | PERCENTEQ
            | STARSTAREQ | GTGTEQ | GTGTGTEQ | LTLTEQ | AMPEQ | CARET | CARETEQ | PIPEEQ
            | Yield | LBRACK | LBRACE | Await | QMARK | QMARKQMARK | New | Let | Var | Const
            | Function | Function2 | SEMI => HalsteadType::Operator,
            Identifier | NestedIdentifier | MemberExpression | PropertyIdentifier | String
            | Number | True | False | Null | Void | This | Super | Undefined | Set | Get
            | Typeof | Instanceof => HalsteadType::Operand,
            _ => HalsteadType::Unknown,
        }
    }

    get_operator!(Typescript);
}

impl Getter for TsxCode {
    fn get_space_kind(node: &Node) -> SpaceKind {
        use Tsx::*;

        match node.kind_id().into() {
            Function
            | MethodDefinition
            | GeneratorFunction
            | FunctionDeclaration
            | GeneratorFunctionDeclaration
            | ArrowFunction => SpaceKind::Function,
            Class | ClassDeclaration => SpaceKind::Class,
            InterfaceDeclaration => SpaceKind::Interface,
            Program => SpaceKind::Unit,
            _ => SpaceKind::Unknown,
        }
    }

    fn get_func_space_name<'a>(node: &Node, code: &'a [u8]) -> Option<&'a str> {
        if let Some(name) = node.child_by_field_name("name") {
            let code = &code[name.start_byte()..name.end_byte()];
            std::str::from_utf8(code).ok()
        } else {
            // We can be in a pair: foo: function() {}
            // Or in a variable declaration: var aFun = function() {}
            if let Some(parent) = node.parent() {
                match parent.kind_id().into() {
                    Mozjs::Pair => {
                        if let Some(name) = parent.child_by_field_name("key") {
                            let code = &code[name.start_byte()..name.end_byte()];
                            return std::str::from_utf8(code).ok();
                        }
                    }
                    Mozjs::VariableDeclarator => {
                        if let Some(name) = parent.child_by_field_name("name") {
                            let code = &code[name.start_byte()..name.end_byte()];
                            return std::str::from_utf8(code).ok();
                        }
                    }
                    _ => {}
                }
            }
            Some("<anonymous>")
        }
    }

    fn get_op_type(node: &Node) -> HalsteadType {
        use Tsx::*;

        match node.kind_id().into() {
            Export | Import | Import2 | Extends | DOT | From | LPAREN | COMMA | As | STAR
            | GTGT | GTGTGT | COLON | Return | Delete | Throw | Break | Continue | If | Else
            | Switch | Case | Default | Async | For | In | Of | While | Try | Catch | Finally
            | With | EQ | AT | AMPAMP | PIPEPIPE | PLUS | DASH | DASHDASH | PLUSPLUS | SLASH
            | PERCENT | STARSTAR | PIPE | AMP | LTLT | TILDE | LT | LTEQ | EQEQ | BANGEQ | GTEQ
            | GT | PLUSEQ | BANG | BANGEQEQ | EQEQEQ | DASHEQ | STAREQ | SLASHEQ | PERCENTEQ
            | STARSTAREQ | GTGTEQ | GTGTGTEQ | LTLTEQ | AMPEQ | CARET | CARETEQ | PIPEEQ
            | Yield | LBRACK | LBRACE | Await | QMARK | QMARKQMARK | New | Let | Var | Const
            | Function | Function2 | SEMI => HalsteadType::Operator,
            Identifier | NestedIdentifier | MemberExpression | PropertyIdentifier | String
            | Number | True | False | Null | Void | This | Super | Undefined | Set | Get
            | Typeof | Instanceof => HalsteadType::Operand,
            _ => HalsteadType::Unknown,
        }
    }

    get_operator!(Tsx);
}

impl Getter for RustCode {
    fn get_func_space_name<'a>(node: &Node, code: &'a [u8]) -> Option<&'a str> {
        // we're in a function or in a class or an impl
        // for an impl: we've  'impl ... type {...'
        if let Some(name) = node
            .child_by_field_name("name")
            .or_else(|| node.child_by_field_name("type"))
        {
            let code = &code[name.start_byte()..name.end_byte()];
            std::str::from_utf8(code).ok()
        } else {
            Some("<anonymous>")
        }
    }

    fn get_space_kind(node: &Node) -> SpaceKind {
        use Rust::*;

        match node.kind_id().into() {
            FunctionItem | ClosureExpression => SpaceKind::Function,
            TraitItem => SpaceKind::Trait,
            ImplItem => SpaceKind::Impl,
            SourceFile => SpaceKind::Unit,
            _ => SpaceKind::Unknown,
        }
    }

    fn get_op_type(node: &Node) -> HalsteadType {
        use Rust::*;

        match node.kind_id().into() {
            LPAREN | LBRACE | LBRACK | EQGT | PLUS | STAR | Async | Await | Continue | For | If
            | Let | Loop | Match | Return | Unsafe | While | BANG | EQ | COMMA | DASHGT | QMARK
            | LT | GT | AMP | MutableSpecifier | DOTDOT | DOTDOTEQ | DASH | AMPAMP | PIPEPIPE
            | PIPE | CARET | EQEQ | BANGEQ | LTEQ | GTEQ | LTLT | GTGT | SLASH | PERCENT
            | PLUSEQ | DASHEQ | STAREQ | SLASHEQ | PERCENTEQ | AMPEQ | PIPEEQ | CARETEQ
            | LTLTEQ | GTGTEQ | Move | DOT | PrimitiveType | Fn | SEMI => HalsteadType::Operator,
            Identifier | StringLiteral | RawStringLiteral | IntegerLiteral | FloatLiteral
            | BooleanLiteral | Zelf | CharLiteral | UNDERSCORE => HalsteadType::Operand,
            _ => HalsteadType::Unknown,
        }
    }

    get_operator!(Rust);
}

impl Getter for CppCode {
    fn get_func_space_name<'a>(node: &Node, code: &'a [u8]) -> Option<&'a str> {
        match node.kind_id().into() {
            Cpp::FunctionDefinition | Cpp::FunctionDefinition2 | Cpp::FunctionDefinition3 => {
                if let Some(op_cast) = node.first_child(|id| Cpp::OperatorCast == id) {
                    let code = &code[op_cast.start_byte()..op_cast.end_byte()];
                    return std::str::from_utf8(code).ok();
                }
                // we're in a function_definition so need to get the declarator
                if let Some(declarator) = node.child_by_field_name("declarator") {
                    let declarator_node = declarator;
                    if let Some(fd) = declarator_node.first_occurence(|id| {
                        Cpp::FunctionDeclarator == id
                            || Cpp::FunctionDeclarator2 == id
                            || Cpp::FunctionDeclarator3 == id
                    }) {
                        if let Some(first) = fd.child(0) {
                            match first.kind_id().into() {
                                Cpp::TypeIdentifier
                                | Cpp::Identifier
                                | Cpp::FieldIdentifier
                                | Cpp::DestructorName
                                | Cpp::OperatorName
                                | Cpp::TemplateFunction
                                | Cpp::TemplateMethod => {
                                    let code = &code[first.start_byte()..first.end_byte()];
                                    return std::str::from_utf8(code).ok();
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
            _ => {
                if let Some(name) = node.child_by_field_name("name") {
                    let code = &code[name.start_byte()..name.end_byte()];
                    return std::str::from_utf8(code).ok();
                }
            }
        }
        None
    }

    fn get_space_kind(node: &Node) -> SpaceKind {
        use Cpp::*;

        match node.kind_id().into() {
            FunctionDefinition | FunctionDefinition2 | FunctionDefinition3 => SpaceKind::Function,
            StructSpecifier => SpaceKind::Struct,
            ClassSpecifier => SpaceKind::Class,
            NamespaceDefinition => SpaceKind::Namespace,
            TranslationUnit => SpaceKind::Unit,
            _ => SpaceKind::Unknown,
        }
    }

    fn get_op_type(node: &Node) -> HalsteadType {
        use Cpp::*;

        match node.kind_id().into() {
            DOT | LPAREN | LPAREN2 | COMMA | STAR | GTGT | COLON | SEMI | Return | Break
            | Continue | If | Else | Switch | Case | Default | For | While | Goto | Do | Delete
            | New | Try | Catch | Throw | EQ | AMPAMP | PIPEPIPE | DASH | DASHDASH | DASHGT
            | PLUS | PLUSPLUS | SLASH | PERCENT | PIPE | AMP | LTLT | TILDE | LT | LTEQ | EQEQ
            | BANGEQ | GTEQ | GT | GT2 | PLUSEQ | BANG | STAREQ | SLASHEQ | PERCENTEQ | GTGTEQ
            | LTLTEQ | AMPEQ | CARET | CARETEQ | PIPEEQ | LBRACK | LBRACE | QMARK | COLONCOLON
            | PrimitiveType | TypeSpecifier | Sizeof => HalsteadType::Operator,
            Identifier | TypeIdentifier | FieldIdentifier | RawStringLiteral | StringLiteral
            | NumberLiteral | True | False | Null | Nullptr | DOTDOTDOT => HalsteadType::Operand,
            _ => HalsteadType::Unknown,
        }
    }

    get_operator!(Cpp);
}

impl Getter for PreprocCode {}
impl Getter for CcommentCode {}
impl Getter for JavaCode {
    fn get_space_kind(node: &Node) -> SpaceKind {
        use Java::*;

        match node.kind_id().into() {
            ClassDeclaration => SpaceKind::Class,
            MethodDeclaration | ConstructorDeclaration | LambdaExpression => SpaceKind::Function,
            InterfaceDeclaration => SpaceKind::Interface,
            Program => SpaceKind::Unit,
            _ => SpaceKind::Unknown,
        }
    }

    fn get_op_type(node: &Node) -> HalsteadType {
        use Java::*;
        // Some guides that informed grammar choice for Halstead
        // keywords, operators, literals: https://docs.oracle.com/javase/specs/jls/se18/html/jls-3.html#jls-3.12
        // https://www.geeksforgeeks.org/software-engineering-halsteads-software-metrics/?msclkid=5e181114abef11ecbb03527e95a34828
        match node.kind_id().into() {
            // Operator: function calls
            MethodInvocation
            // Operator: control flow
            | If | Else | Switch | Case | Try | Catch | Throw | Throws | Throws2 | For | While | Continue | Break | Do | Finally
            // Operator: keywords
            | New | Return | Default | Abstract | Assert | Instanceof | Extends | Final | Implements | Transient | Synchronized | Super | This | VoidType
            // Operator: brackets and comma and terminators (separators)
            | SEMI | COMMA | COLONCOLON | LBRACE | LBRACK | LPAREN | RBRACE | RBRACK | RPAREN | DOTDOTDOT | DOT
            // Operator: operators
            | EQ | LT | GT | BANG | TILDE | QMARK | COLON // no grammar for lambda operator ->
            | EQEQ | LTEQ | GTEQ | BANGEQ | AMPAMP | PIPEPIPE | PLUSPLUS | DASHDASH
            | PLUS | DASH | STAR | SLASH | AMP | PIPE | CARET | PERCENT| LTLT | GTGT | GTGTGT
            | PLUSEQ | DASHEQ | STAREQ | SLASHEQ | AMPEQ | PIPEEQ | CARETEQ | PERCENTEQ | LTLTEQ | GTGTEQ | GTGTGTEQ
            // type identifier
            | TypeIdentifier | IntegralType | FloatingPointType | BooleanType
            => {
                HalsteadType::Operator
            },
            // Operands: variables, constants, literals
            Identifier | NullLiteral | ClassLiteral | StringLiteral | CharacterLiteral | HexIntegerLiteral | OctalIntegerLiteral | BinaryIntegerLiteral | DecimalIntegerLiteral | HexFloatingPointLiteral | DecimalFloatingPointLiteral  => {
                HalsteadType::Operand
            },
            _ => {
                HalsteadType::Unknown
            },
        }
    }

    get_operator!(Java);
}

impl Getter for KotlinCode {}
