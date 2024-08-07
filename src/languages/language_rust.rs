// Code generated; DO NOT EDIT.

use num_derive::FromPrimitive;

#[derive(Clone, Debug, PartialEq, Eq, FromPrimitive)]
pub enum Rust {
    End = 0,
    Identifier = 1,
    SEMI = 2,
    MacroRulesBANG = 3,
    LPAREN = 4,
    RPAREN = 5,
    LBRACK = 6,
    RBRACK = 7,
    LBRACE = 8,
    RBRACE = 9,
    EQGT = 10,
    COLON = 11,
    DOLLAR = 12,
    TokenRepetitionPatternToken1 = 13,
    PLUS = 14,
    STAR = 15,
    QMARK = 16,
    Block2 = 17,
    Expr = 18,
    Ident = 19,
    Item = 20,
    Lifetime2 = 21,
    Literal = 22,
    Meta = 23,
    Pat = 24,
    Path = 25,
    Stmt = 26,
    Tt = 27,
    Ty = 28,
    Vis = 29,
    PrimitiveType = 30,
    PrimitiveType2 = 31,
    PrimitiveType3 = 32,
    PrimitiveType4 = 33,
    PrimitiveType5 = 34,
    PrimitiveType6 = 35,
    PrimitiveType7 = 36,
    PrimitiveType8 = 37,
    PrimitiveType9 = 38,
    PrimitiveType10 = 39,
    PrimitiveType11 = 40,
    PrimitiveType12 = 41,
    PrimitiveType13 = 42,
    PrimitiveType14 = 43,
    PrimitiveType15 = 44,
    PrimitiveType16 = 45,
    PrimitiveType17 = 46,
    DASH = 47,
    SLASH = 48,
    PERCENT = 49,
    CARET = 50,
    BANG = 51,
    AMP = 52,
    PIPE = 53,
    AMPAMP = 54,
    PIPEPIPE = 55,
    LTLT = 56,
    GTGT = 57,
    PLUSEQ = 58,
    DASHEQ = 59,
    STAREQ = 60,
    SLASHEQ = 61,
    PERCENTEQ = 62,
    CARETEQ = 63,
    AMPEQ = 64,
    PIPEEQ = 65,
    LTLTEQ = 66,
    GTGTEQ = 67,
    EQ = 68,
    EQEQ = 69,
    BANGEQ = 70,
    GT = 71,
    LT = 72,
    GTEQ = 73,
    LTEQ = 74,
    AT = 75,
    UNDERSCORE = 76,
    DOT = 77,
    DOTDOT = 78,
    DOTDOTDOT = 79,
    DOTDOTEQ = 80,
    COMMA = 81,
    COLONCOLON = 82,
    DASHGT = 83,
    HASH = 84,
    SQUOTE = 85,
    As = 86,
    Async = 87,
    Await = 88,
    Break = 89,
    Const = 90,
    Continue = 91,
    Default = 92,
    Enum = 93,
    Fn = 94,
    For = 95,
    If = 96,
    Impl = 97,
    Let = 98,
    Loop = 99,
    Match = 100,
    Mod = 101,
    Pub = 102,
    Return = 103,
    Static = 104,
    Struct = 105,
    Trait = 106,
    Type = 107,
    Union = 108,
    Unsafe = 109,
    Use = 110,
    Where = 111,
    While = 112,
    Extern = 113,
    Ref = 114,
    Else = 115,
    In = 116,
    LT2 = 117,
    Dyn = 118,
    MutableSpecifier = 119,
    Yield = 120,
    Move = 121,
    Try = 122,
    IntegerLiteral = 123,
    DQUOTE = 124,
    DQUOTE2 = 125,
    CharLiteral = 126,
    EscapeSequence = 127,
    True = 128,
    False = 129,
    SLASHSLASH = 130,
    LineCommentToken1 = 131,
    LineCommentToken2 = 132,
    LineCommentToken3 = 133,
    InnerDocCommentMarker = 134,
    OuterDocCommentMarker = 135,
    SLASHSTAR = 136,
    STARSLASH = 137,
    Shebang = 138,
    Zelf = 139,
    Super = 140,
    Crate = 141,
    Metavariable = 142,
    StringContent = 143,
    RawStringLiteralStart = 144,
    StringContent2 = 145,
    RawStringLiteralEnd = 146,
    FloatLiteral = 147,
    OuterDocCommentMarker2 = 148,
    InnerDocCommentMarker2 = 149,
    BlockCommentContent = 150,
    DocComment = 151,
    ErrorSentinel = 152,
    SourceFile = 153,
    Statement = 154,
    EmptyStatement = 155,
    ExpressionStatement = 156,
    MacroDefinition = 157,
    MacroRule = 158,
    TokenPattern = 159,
    TokenTreePattern = 160,
    TokenBindingPattern = 161,
    TokenRepetitionPattern = 162,
    FragmentSpecifier = 163,
    TokenTree = 164,
    TokenRepetition = 165,
    AttributeItem = 166,
    InnerAttributeItem = 167,
    Attribute = 168,
    ModItem = 169,
    ForeignModItem = 170,
    DeclarationList = 171,
    StructItem = 172,
    UnionItem = 173,
    EnumItem = 174,
    EnumVariantList = 175,
    EnumVariant = 176,
    FieldDeclarationList = 177,
    FieldDeclaration = 178,
    OrderedFieldDeclarationList = 179,
    ExternCrateDeclaration = 180,
    ConstItem = 181,
    StaticItem = 182,
    TypeItem = 183,
    FunctionItem = 184,
    FunctionSignatureItem = 185,
    FunctionModifiers = 186,
    WhereClause = 187,
    WherePredicate = 188,
    ImplItem = 189,
    TraitItem = 190,
    AssociatedType = 191,
    TraitBounds = 192,
    HigherRankedTraitBound = 193,
    RemovedTraitBound = 194,
    TypeParameters = 195,
    ConstParameter = 196,
    ConstrainedTypeParameter = 197,
    OptionalTypeParameter = 198,
    LetDeclaration = 199,
    UseDeclaration = 200,
    UseClause = 201,
    ScopedUseList = 202,
    UseList = 203,
    UseAsClause = 204,
    UseWildcard = 205,
    Parameters = 206,
    SelfParameter = 207,
    VariadicParameter = 208,
    Parameter = 209,
    ExternModifier = 210,
    VisibilityModifier = 211,
    Type2 = 212,
    BracketedType = 213,
    QualifiedType = 214,
    Lifetime = 215,
    ArrayType = 216,
    ForLifetimes = 217,
    FunctionType = 218,
    TupleType = 219,
    UnitType = 220,
    GenericFunction = 221,
    GenericType = 222,
    GenericTypeWithTurbofish = 223,
    BoundedType = 224,
    TypeArguments = 225,
    TypeBinding = 226,
    ReferenceType = 227,
    PointerType = 228,
    NeverType = 229,
    AbstractType = 230,
    DynamicType = 231,
    ExpressionExceptRange = 232,
    Expression = 233,
    MacroInvocation = 234,
    TokenTree2 = 235,
    DelimTokens = 236,
    NonDelimToken = 237,
    ScopedIdentifier = 238,
    ScopedTypeIdentifier = 239,
    ScopedTypeIdentifier2 = 240,
    RangeExpression = 241,
    UnaryExpression = 242,
    TryExpression = 243,
    ReferenceExpression = 244,
    BinaryExpression = 245,
    AssignmentExpression = 246,
    CompoundAssignmentExpr = 247,
    TypeCastExpression = 248,
    ReturnExpression = 249,
    YieldExpression = 250,
    CallExpression = 251,
    Arguments = 252,
    ArrayExpression = 253,
    ParenthesizedExpression = 254,
    TupleExpression = 255,
    UnitExpression = 256,
    StructExpression = 257,
    FieldInitializerList = 258,
    ShorthandFieldInitializer = 259,
    FieldInitializer = 260,
    BaseFieldInitializer = 261,
    IfExpression = 262,
    LetCondition = 263,
    LetChain2 = 264,
    Condition = 265,
    ElseClause = 266,
    MatchExpression = 267,
    MatchBlock = 268,
    MatchArm = 269,
    MatchArm2 = 270,
    MatchPattern = 271,
    WhileExpression = 272,
    LoopExpression = 273,
    ForExpression = 274,
    ConstBlock = 275,
    ClosureExpression = 276,
    ClosureParameters = 277,
    Label = 278,
    BreakExpression = 279,
    ContinueExpression = 280,
    IndexExpression = 281,
    AwaitExpression = 282,
    FieldExpression = 283,
    UnsafeBlock = 284,
    AsyncBlock = 285,
    TryBlock = 286,
    Block = 287,
    Pattern = 288,
    TuplePattern = 289,
    SlicePattern = 290,
    TupleStructPattern = 291,
    StructPattern = 292,
    FieldPattern = 293,
    RemainingFieldPattern = 294,
    MutPattern = 295,
    RangePattern = 296,
    RefPattern = 297,
    CapturedPattern = 298,
    ReferencePattern = 299,
    OrPattern = 300,
    Literal2 = 301,
    LiteralPattern = 302,
    NegativeLiteral = 303,
    StringLiteral = 304,
    RawStringLiteral = 305,
    BooleanLiteral = 306,
    LineComment = 307,
    LineDocCommentMarker = 308,
    BlockComment = 309,
    BlockDocCommentMarker = 310,
    SourceFileRepeat1 = 311,
    MacroDefinitionRepeat1 = 312,
    TokenTreePatternRepeat1 = 313,
    TokenTreeRepeat1 = 314,
    NonSpecialTokenRepeat1 = 315,
    DeclarationListRepeat1 = 316,
    EnumVariantListRepeat1 = 317,
    EnumVariantListRepeat2 = 318,
    FieldDeclarationListRepeat1 = 319,
    OrderedFieldDeclarationListRepeat1 = 320,
    FunctionModifiersRepeat1 = 321,
    WhereClauseRepeat1 = 322,
    TraitBoundsRepeat1 = 323,
    TypeParametersRepeat1 = 324,
    UseListRepeat1 = 325,
    ParametersRepeat1 = 326,
    ForLifetimesRepeat1 = 327,
    TupleTypeRepeat1 = 328,
    TypeArgumentsRepeat1 = 329,
    DelimTokenTreeRepeat1 = 330,
    ArgumentsRepeat1 = 331,
    TupleExpressionRepeat1 = 332,
    FieldInitializerListRepeat1 = 333,
    MatchBlockRepeat1 = 334,
    MatchArmRepeat1 = 335,
    ClosureParametersRepeat1 = 336,
    TuplePatternRepeat1 = 337,
    SlicePatternRepeat1 = 338,
    StructPatternRepeat1 = 339,
    StringLiteralRepeat1 = 340,
    FieldIdentifier = 341,
    LetChain = 342,
    ShorthandFieldIdentifier = 343,
    TypeIdentifier = 344,
    Error = 345,
}

impl From<Rust> for &'static str {
    #[inline(always)]
    fn from(tok: Rust) -> Self {
        match tok {
            Rust::End => "end",
            Rust::Identifier => "identifier",
            Rust::SEMI => ";",
            Rust::MacroRulesBANG => "macro_rules!",
            Rust::LPAREN => "(",
            Rust::RPAREN => ")",
            Rust::LBRACK => "[",
            Rust::RBRACK => "]",
            Rust::LBRACE => "{",
            Rust::RBRACE => "}",
            Rust::EQGT => "=>",
            Rust::COLON => ":",
            Rust::DOLLAR => "$",
            Rust::TokenRepetitionPatternToken1 => "token_repetition_pattern_token1",
            Rust::PLUS => "+",
            Rust::STAR => "*",
            Rust::QMARK => "?",
            Rust::Block2 => "block",
            Rust::Expr => "expr",
            Rust::Ident => "ident",
            Rust::Item => "item",
            Rust::Lifetime2 => "lifetime",
            Rust::Literal => "literal",
            Rust::Meta => "meta",
            Rust::Pat => "pat",
            Rust::Path => "path",
            Rust::Stmt => "stmt",
            Rust::Tt => "tt",
            Rust::Ty => "ty",
            Rust::Vis => "vis",
            Rust::PrimitiveType => "primitive_type",
            Rust::PrimitiveType2 => "primitive_type",
            Rust::PrimitiveType3 => "primitive_type",
            Rust::PrimitiveType4 => "primitive_type",
            Rust::PrimitiveType5 => "primitive_type",
            Rust::PrimitiveType6 => "primitive_type",
            Rust::PrimitiveType7 => "primitive_type",
            Rust::PrimitiveType8 => "primitive_type",
            Rust::PrimitiveType9 => "primitive_type",
            Rust::PrimitiveType10 => "primitive_type",
            Rust::PrimitiveType11 => "primitive_type",
            Rust::PrimitiveType12 => "primitive_type",
            Rust::PrimitiveType13 => "primitive_type",
            Rust::PrimitiveType14 => "primitive_type",
            Rust::PrimitiveType15 => "primitive_type",
            Rust::PrimitiveType16 => "primitive_type",
            Rust::PrimitiveType17 => "primitive_type",
            Rust::DASH => "-",
            Rust::SLASH => "/",
            Rust::PERCENT => "%",
            Rust::CARET => "^",
            Rust::BANG => "!",
            Rust::AMP => "&",
            Rust::PIPE => "|",
            Rust::AMPAMP => "&&",
            Rust::PIPEPIPE => "||",
            Rust::LTLT => "<<",
            Rust::GTGT => ">>",
            Rust::PLUSEQ => "+=",
            Rust::DASHEQ => "-=",
            Rust::STAREQ => "*=",
            Rust::SLASHEQ => "/=",
            Rust::PERCENTEQ => "%=",
            Rust::CARETEQ => "^=",
            Rust::AMPEQ => "&=",
            Rust::PIPEEQ => "|=",
            Rust::LTLTEQ => "<<=",
            Rust::GTGTEQ => ">>=",
            Rust::EQ => "=",
            Rust::EQEQ => "==",
            Rust::BANGEQ => "!=",
            Rust::GT => ">",
            Rust::LT => "<",
            Rust::GTEQ => ">=",
            Rust::LTEQ => "<=",
            Rust::AT => "@",
            Rust::UNDERSCORE => "_",
            Rust::DOT => ".",
            Rust::DOTDOT => "..",
            Rust::DOTDOTDOT => "...",
            Rust::DOTDOTEQ => "..=",
            Rust::COMMA => ",",
            Rust::COLONCOLON => "::",
            Rust::DASHGT => "->",
            Rust::HASH => "#",
            Rust::SQUOTE => "'",
            Rust::As => "as",
            Rust::Async => "async",
            Rust::Await => "await",
            Rust::Break => "break",
            Rust::Const => "const",
            Rust::Continue => "continue",
            Rust::Default => "default",
            Rust::Enum => "enum",
            Rust::Fn => "fn",
            Rust::For => "for",
            Rust::If => "if",
            Rust::Impl => "impl",
            Rust::Let => "let",
            Rust::Loop => "loop",
            Rust::Match => "match",
            Rust::Mod => "mod",
            Rust::Pub => "pub",
            Rust::Return => "return",
            Rust::Static => "static",
            Rust::Struct => "struct",
            Rust::Trait => "trait",
            Rust::Type => "type",
            Rust::Union => "union",
            Rust::Unsafe => "unsafe",
            Rust::Use => "use",
            Rust::Where => "where",
            Rust::While => "while",
            Rust::Extern => "extern",
            Rust::Ref => "ref",
            Rust::Else => "else",
            Rust::In => "in",
            Rust::LT2 => "<",
            Rust::Dyn => "dyn",
            Rust::MutableSpecifier => "mutable_specifier",
            Rust::Yield => "yield",
            Rust::Move => "move",
            Rust::Try => "try",
            Rust::IntegerLiteral => "integer_literal",
            Rust::DQUOTE => "\"",
            Rust::DQUOTE2 => "\"",
            Rust::CharLiteral => "char_literal",
            Rust::EscapeSequence => "escape_sequence",
            Rust::True => "true",
            Rust::False => "false",
            Rust::SLASHSLASH => "//",
            Rust::LineCommentToken1 => "line_comment_token1",
            Rust::LineCommentToken2 => "line_comment_token2",
            Rust::LineCommentToken3 => "line_comment_token3",
            Rust::InnerDocCommentMarker => "inner_doc_comment_marker",
            Rust::OuterDocCommentMarker => "outer_doc_comment_marker",
            Rust::SLASHSTAR => "/*",
            Rust::STARSLASH => "*/",
            Rust::Shebang => "shebang",
            Rust::Zelf => "self",
            Rust::Super => "super",
            Rust::Crate => "crate",
            Rust::Metavariable => "metavariable",
            Rust::StringContent => "string_content",
            Rust::RawStringLiteralStart => "_raw_string_literal_start",
            Rust::StringContent2 => "string_content",
            Rust::RawStringLiteralEnd => "_raw_string_literal_end",
            Rust::FloatLiteral => "float_literal",
            Rust::OuterDocCommentMarker2 => "outer_doc_comment_marker",
            Rust::InnerDocCommentMarker2 => "inner_doc_comment_marker",
            Rust::BlockCommentContent => "_block_comment_content",
            Rust::DocComment => "doc_comment",
            Rust::ErrorSentinel => "_error_sentinel",
            Rust::SourceFile => "source_file",
            Rust::Statement => "_statement",
            Rust::EmptyStatement => "empty_statement",
            Rust::ExpressionStatement => "expression_statement",
            Rust::MacroDefinition => "macro_definition",
            Rust::MacroRule => "macro_rule",
            Rust::TokenPattern => "_token_pattern",
            Rust::TokenTreePattern => "token_tree_pattern",
            Rust::TokenBindingPattern => "token_binding_pattern",
            Rust::TokenRepetitionPattern => "token_repetition_pattern",
            Rust::FragmentSpecifier => "fragment_specifier",
            Rust::TokenTree => "token_tree",
            Rust::TokenRepetition => "token_repetition",
            Rust::AttributeItem => "attribute_item",
            Rust::InnerAttributeItem => "inner_attribute_item",
            Rust::Attribute => "attribute",
            Rust::ModItem => "mod_item",
            Rust::ForeignModItem => "foreign_mod_item",
            Rust::DeclarationList => "declaration_list",
            Rust::StructItem => "struct_item",
            Rust::UnionItem => "union_item",
            Rust::EnumItem => "enum_item",
            Rust::EnumVariantList => "enum_variant_list",
            Rust::EnumVariant => "enum_variant",
            Rust::FieldDeclarationList => "field_declaration_list",
            Rust::FieldDeclaration => "field_declaration",
            Rust::OrderedFieldDeclarationList => "ordered_field_declaration_list",
            Rust::ExternCrateDeclaration => "extern_crate_declaration",
            Rust::ConstItem => "const_item",
            Rust::StaticItem => "static_item",
            Rust::TypeItem => "type_item",
            Rust::FunctionItem => "function_item",
            Rust::FunctionSignatureItem => "function_signature_item",
            Rust::FunctionModifiers => "function_modifiers",
            Rust::WhereClause => "where_clause",
            Rust::WherePredicate => "where_predicate",
            Rust::ImplItem => "impl_item",
            Rust::TraitItem => "trait_item",
            Rust::AssociatedType => "associated_type",
            Rust::TraitBounds => "trait_bounds",
            Rust::HigherRankedTraitBound => "higher_ranked_trait_bound",
            Rust::RemovedTraitBound => "removed_trait_bound",
            Rust::TypeParameters => "type_parameters",
            Rust::ConstParameter => "const_parameter",
            Rust::ConstrainedTypeParameter => "constrained_type_parameter",
            Rust::OptionalTypeParameter => "optional_type_parameter",
            Rust::LetDeclaration => "let_declaration",
            Rust::UseDeclaration => "use_declaration",
            Rust::UseClause => "_use_clause",
            Rust::ScopedUseList => "scoped_use_list",
            Rust::UseList => "use_list",
            Rust::UseAsClause => "use_as_clause",
            Rust::UseWildcard => "use_wildcard",
            Rust::Parameters => "parameters",
            Rust::SelfParameter => "self_parameter",
            Rust::VariadicParameter => "variadic_parameter",
            Rust::Parameter => "parameter",
            Rust::ExternModifier => "extern_modifier",
            Rust::VisibilityModifier => "visibility_modifier",
            Rust::Type2 => "_type",
            Rust::BracketedType => "bracketed_type",
            Rust::QualifiedType => "qualified_type",
            Rust::Lifetime => "lifetime",
            Rust::ArrayType => "array_type",
            Rust::ForLifetimes => "for_lifetimes",
            Rust::FunctionType => "function_type",
            Rust::TupleType => "tuple_type",
            Rust::UnitType => "unit_type",
            Rust::GenericFunction => "generic_function",
            Rust::GenericType => "generic_type",
            Rust::GenericTypeWithTurbofish => "generic_type_with_turbofish",
            Rust::BoundedType => "bounded_type",
            Rust::TypeArguments => "type_arguments",
            Rust::TypeBinding => "type_binding",
            Rust::ReferenceType => "reference_type",
            Rust::PointerType => "pointer_type",
            Rust::NeverType => "never_type",
            Rust::AbstractType => "abstract_type",
            Rust::DynamicType => "dynamic_type",
            Rust::ExpressionExceptRange => "_expression_except_range",
            Rust::Expression => "_expression",
            Rust::MacroInvocation => "macro_invocation",
            Rust::TokenTree2 => "token_tree",
            Rust::DelimTokens => "_delim_tokens",
            Rust::NonDelimToken => "_non_delim_token",
            Rust::ScopedIdentifier => "scoped_identifier",
            Rust::ScopedTypeIdentifier => "scoped_type_identifier",
            Rust::ScopedTypeIdentifier2 => "scoped_type_identifier",
            Rust::RangeExpression => "range_expression",
            Rust::UnaryExpression => "unary_expression",
            Rust::TryExpression => "try_expression",
            Rust::ReferenceExpression => "reference_expression",
            Rust::BinaryExpression => "binary_expression",
            Rust::AssignmentExpression => "assignment_expression",
            Rust::CompoundAssignmentExpr => "compound_assignment_expr",
            Rust::TypeCastExpression => "type_cast_expression",
            Rust::ReturnExpression => "return_expression",
            Rust::YieldExpression => "yield_expression",
            Rust::CallExpression => "call_expression",
            Rust::Arguments => "arguments",
            Rust::ArrayExpression => "array_expression",
            Rust::ParenthesizedExpression => "parenthesized_expression",
            Rust::TupleExpression => "tuple_expression",
            Rust::UnitExpression => "unit_expression",
            Rust::StructExpression => "struct_expression",
            Rust::FieldInitializerList => "field_initializer_list",
            Rust::ShorthandFieldInitializer => "shorthand_field_initializer",
            Rust::FieldInitializer => "field_initializer",
            Rust::BaseFieldInitializer => "base_field_initializer",
            Rust::IfExpression => "if_expression",
            Rust::LetCondition => "let_condition",
            Rust::LetChain2 => "_let_chain",
            Rust::Condition => "_condition",
            Rust::ElseClause => "else_clause",
            Rust::MatchExpression => "match_expression",
            Rust::MatchBlock => "match_block",
            Rust::MatchArm => "match_arm",
            Rust::MatchArm2 => "match_arm",
            Rust::MatchPattern => "match_pattern",
            Rust::WhileExpression => "while_expression",
            Rust::LoopExpression => "loop_expression",
            Rust::ForExpression => "for_expression",
            Rust::ConstBlock => "const_block",
            Rust::ClosureExpression => "closure_expression",
            Rust::ClosureParameters => "closure_parameters",
            Rust::Label => "label",
            Rust::BreakExpression => "break_expression",
            Rust::ContinueExpression => "continue_expression",
            Rust::IndexExpression => "index_expression",
            Rust::AwaitExpression => "await_expression",
            Rust::FieldExpression => "field_expression",
            Rust::UnsafeBlock => "unsafe_block",
            Rust::AsyncBlock => "async_block",
            Rust::TryBlock => "try_block",
            Rust::Block => "block",
            Rust::Pattern => "_pattern",
            Rust::TuplePattern => "tuple_pattern",
            Rust::SlicePattern => "slice_pattern",
            Rust::TupleStructPattern => "tuple_struct_pattern",
            Rust::StructPattern => "struct_pattern",
            Rust::FieldPattern => "field_pattern",
            Rust::RemainingFieldPattern => "remaining_field_pattern",
            Rust::MutPattern => "mut_pattern",
            Rust::RangePattern => "range_pattern",
            Rust::RefPattern => "ref_pattern",
            Rust::CapturedPattern => "captured_pattern",
            Rust::ReferencePattern => "reference_pattern",
            Rust::OrPattern => "or_pattern",
            Rust::Literal2 => "_literal",
            Rust::LiteralPattern => "_literal_pattern",
            Rust::NegativeLiteral => "negative_literal",
            Rust::StringLiteral => "string_literal",
            Rust::RawStringLiteral => "raw_string_literal",
            Rust::BooleanLiteral => "boolean_literal",
            Rust::LineComment => "line_comment",
            Rust::LineDocCommentMarker => "_line_doc_comment_marker",
            Rust::BlockComment => "block_comment",
            Rust::BlockDocCommentMarker => "_block_doc_comment_marker",
            Rust::SourceFileRepeat1 => "source_file_repeat1",
            Rust::MacroDefinitionRepeat1 => "macro_definition_repeat1",
            Rust::TokenTreePatternRepeat1 => "token_tree_pattern_repeat1",
            Rust::TokenTreeRepeat1 => "token_tree_repeat1",
            Rust::NonSpecialTokenRepeat1 => "_non_special_token_repeat1",
            Rust::DeclarationListRepeat1 => "declaration_list_repeat1",
            Rust::EnumVariantListRepeat1 => "enum_variant_list_repeat1",
            Rust::EnumVariantListRepeat2 => "enum_variant_list_repeat2",
            Rust::FieldDeclarationListRepeat1 => "field_declaration_list_repeat1",
            Rust::OrderedFieldDeclarationListRepeat1 => "ordered_field_declaration_list_repeat1",
            Rust::FunctionModifiersRepeat1 => "function_modifiers_repeat1",
            Rust::WhereClauseRepeat1 => "where_clause_repeat1",
            Rust::TraitBoundsRepeat1 => "trait_bounds_repeat1",
            Rust::TypeParametersRepeat1 => "type_parameters_repeat1",
            Rust::UseListRepeat1 => "use_list_repeat1",
            Rust::ParametersRepeat1 => "parameters_repeat1",
            Rust::ForLifetimesRepeat1 => "for_lifetimes_repeat1",
            Rust::TupleTypeRepeat1 => "tuple_type_repeat1",
            Rust::TypeArgumentsRepeat1 => "type_arguments_repeat1",
            Rust::DelimTokenTreeRepeat1 => "delim_token_tree_repeat1",
            Rust::ArgumentsRepeat1 => "arguments_repeat1",
            Rust::TupleExpressionRepeat1 => "tuple_expression_repeat1",
            Rust::FieldInitializerListRepeat1 => "field_initializer_list_repeat1",
            Rust::MatchBlockRepeat1 => "match_block_repeat1",
            Rust::MatchArmRepeat1 => "match_arm_repeat1",
            Rust::ClosureParametersRepeat1 => "closure_parameters_repeat1",
            Rust::TuplePatternRepeat1 => "tuple_pattern_repeat1",
            Rust::SlicePatternRepeat1 => "slice_pattern_repeat1",
            Rust::StructPatternRepeat1 => "struct_pattern_repeat1",
            Rust::StringLiteralRepeat1 => "string_literal_repeat1",
            Rust::FieldIdentifier => "field_identifier",
            Rust::LetChain => "let_chain",
            Rust::ShorthandFieldIdentifier => "shorthand_field_identifier",
            Rust::TypeIdentifier => "type_identifier",
            Rust::Error => "ERROR",
        }
    }
}

impl From<u16> for Rust {
    #[inline(always)]
    fn from(x: u16) -> Self {
        num::FromPrimitive::from_u16(x).unwrap_or(Self::Error)
    }
}

// Rust == u16
impl PartialEq<u16> for Rust {
    #[inline(always)]
    fn eq(&self, x: &u16) -> bool {
        *self == Into::<Self>::into(*x)
    }
}

// u16 == Rust
impl PartialEq<Rust> for u16 {
    #[inline(always)]
    fn eq(&self, x: &Rust) -> bool {
        *x == *self
    }
}
