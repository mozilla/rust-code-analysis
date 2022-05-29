use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;
use std::fmt;

use crate::checker::Checker;
use crate::node::Node;
use crate::*;

/// The `ABC` metric.
///
/// The `ABC` metric measures the size of a source code by counting
/// the number of Assignments (`A`), Branches (`B`) and Conditions (`C`).
/// The metric defines an ABC score as a vector of three elements (`<A,B,C>`).
/// The ABC score can be represented by its individual components (`A`, `B` and `C`)
/// or by the magnitude of the vector (`|<A,B,C>| = sqrt(A^2 + B^2 + C^2)`).
///
/// Official paper and definition:
///
/// Fitzpatrick, Jerry (1997). "Applying the ABC metric to C, C++ and Java". C++ Report.
/// https://www.softwarerenovation.com/Articles.aspx
#[derive(Debug, Clone)]
pub struct Stats {
    assignments: f64,
    assignments_sum: f64,
    assignments_min: f64,
    assignments_max: f64,
    branches: f64,
    branches_sum: f64,
    branches_min: f64,
    branches_max: f64,
    conditions: f64,
    conditions_sum: f64,
    conditions_min: f64,
    conditions_max: f64,
    space_count: usize,
    declaration: Vec<DeclKind>,
}

#[derive(Debug, Clone)]
pub enum DeclKind {
    Var,
    Const,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            assignments: 0.,
            assignments_sum: 0.,
            assignments_min: f64::MAX,
            assignments_max: 0.,
            branches: 0.,
            branches_sum: 0.,
            branches_min: f64::MAX,
            branches_max: 0.,
            conditions: 0.,
            conditions_sum: 0.,
            conditions_min: f64::MAX,
            conditions_max: 0.,
            space_count: 1,
            declaration: Vec::new(),
        }
    }
}

impl Serialize for Stats {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut st = serializer.serialize_struct("abc", 13)?;
        st.serialize_field("assignments", &self.assignments_sum())?;
        st.serialize_field("branches", &self.branches_sum())?;
        st.serialize_field("conditions", &self.conditions_sum())?;
        st.serialize_field("magnitude", &self.magnitude_sum())?;
        st.serialize_field("assignments_average", &self.assignments_average())?;
        st.serialize_field("branches_average", &self.branches_average())?;
        st.serialize_field("conditions_average", &self.conditions_average())?;
        st.serialize_field("assignments_min", &self.assignments_min())?;
        st.serialize_field("assignments_max", &self.assignments_max())?;
        st.serialize_field("branches_min", &self.branches_min())?;
        st.serialize_field("branches_max", &self.branches_max())?;
        st.serialize_field("conditions_min", &self.conditions_min())?;
        st.serialize_field("conditions_max", &self.conditions_max())?;
        st.end()
    }
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "assignments: {}, branches: {}, conditions: {}, magnitude: {}, \
            assignments_average: {}, branches_average: {}, conditions_average: {}, \
            assignments_min: {}, assignments_max: {}, \
            branches_min: {}, branches_max: {}, \
            conditions_min: {}, conditions_max: {}",
            self.assignments_sum(),
            self.branches_sum(),
            self.conditions_sum(),
            self.magnitude_sum(),
            self.assignments_average(),
            self.branches_average(),
            self.conditions_average(),
            self.assignments_min(),
            self.assignments_max(),
            self.branches_min(),
            self.branches_max(),
            self.conditions_min(),
            self.conditions_max()
        )
    }
}

impl Stats {
    /// Merges a second `Abc` metric into the first one.
    pub fn merge(&mut self, other: &Stats) {
        // Calculates minimum and maximum values
        self.assignments_min = self.assignments_min.min(other.assignments_min);
        self.assignments_max = self.assignments_max.max(other.assignments_max);
        self.branches_min = self.branches_min.min(other.branches_min);
        self.branches_max = self.branches_max.max(other.branches_max);
        self.conditions_min = self.conditions_min.min(other.conditions_min);
        self.conditions_max = self.conditions_max.max(other.conditions_max);

        self.assignments_sum += other.assignments_sum;
        self.branches_sum += other.branches_sum;
        self.conditions_sum += other.conditions_sum;

        self.space_count += other.space_count;
    }

    /// Returns the `Abc` assignments metric value.
    pub fn assignments(&self) -> f64 {
        self.assignments
    }

    /// Returns the `Abc` assignments sum metric value.
    pub fn assignments_sum(&self) -> f64 {
        self.assignments_sum
    }

    /// Returns the `Abc` assignments average value.
    ///
    /// This value is computed dividing the `Abc`
    /// assignments value for the number of spaces.
    pub fn assignments_average(&self) -> f64 {
        self.assignments_sum() / self.space_count as f64
    }

    /// Returns the `Abc` assignments minimum value.
    pub fn assignments_min(&self) -> f64 {
        self.assignments_min
    }

    /// Returns the `Abc` assignments maximum value.
    pub fn assignments_max(&self) -> f64 {
        self.assignments_max
    }

    /// Returns the `Abc` branches metric value.
    pub fn branches(&self) -> f64 {
        self.branches
    }

    /// Returns the `Abc` branches sum metric value.
    pub fn branches_sum(&self) -> f64 {
        self.branches_sum
    }

    /// Returns the `Abc` branches average value.
    ///
    /// This value is computed dividing the `Abc`
    /// branches value for the number of spaces.
    pub fn branches_average(&self) -> f64 {
        self.branches_sum() / self.space_count as f64
    }

    /// Returns the `Abc` branches minimum value.
    pub fn branches_min(&self) -> f64 {
        self.branches_min
    }

    /// Returns the `Abc` branches maximum value.
    pub fn branches_max(&self) -> f64 {
        self.branches_max
    }

    /// Returns the `Abc` conditions metric value.
    pub fn conditions(&self) -> f64 {
        self.conditions
    }

    /// Returns the `Abc` conditions sum metric value.
    pub fn conditions_sum(&self) -> f64 {
        self.conditions_sum
    }

    /// Returns the `Abc` conditions average value.
    ///
    /// This value is computed dividing the `Abc`
    /// conditions value for the number of spaces.
    pub fn conditions_average(&self) -> f64 {
        self.conditions_sum() / self.space_count as f64
    }

    /// Returns the `Abc` conditions minimum value.
    pub fn conditions_min(&self) -> f64 {
        self.conditions_min
    }

    /// Returns the `Abc` conditions maximum value.
    pub fn conditions_max(&self) -> f64 {
        self.conditions_max
    }

    /// Returns the `Abc` magnitude metric value.
    pub fn magnitude(&self) -> f64 {
        (self.assignments.powi(2) + self.branches.powi(2) + self.conditions.powi(2)).sqrt()
    }

    /// Returns the `Abc` magnitude sum metric value.
    pub fn magnitude_sum(&self) -> f64 {
        (self.assignments_sum.powi(2) + self.branches_sum.powi(2) + self.conditions_sum.powi(2))
            .sqrt()
    }

    #[inline(always)]
    pub(crate) fn compute_sum(&mut self) {
        self.assignments_sum += self.assignments;
        self.branches_sum += self.branches;
        self.conditions_sum += self.conditions;
    }

    #[inline(always)]
    pub(crate) fn compute_minmax(&mut self) {
        self.assignments_min = self.assignments_min.min(self.assignments);
        self.assignments_max = self.assignments_max.max(self.assignments);
        self.branches_min = self.branches_min.min(self.branches);
        self.branches_max = self.branches_max.max(self.branches);
        self.conditions_min = self.conditions_min.min(self.conditions);
        self.conditions_max = self.conditions_max.max(self.conditions);
        self.compute_sum();
    }
}

#[doc(hidden)]
pub trait Abc
where
    Self: Checker,
{
    fn compute(_node: &Node, _stats: &mut Stats) {}
}

// Inspects the content of Java parenthesized expressions
// and `Not` operators to find unary conditional expressions
fn java_inspect_container(container_node: &Node, conditions: &mut f64) {
    use Java::*;

    let mut node = container_node.object();
    let mut node_kind = node.kind_id().into();

    // Initializes the flag to true if the container is known to contain a boolean value
    let mut has_boolean_content = match node.parent().unwrap().kind_id().into() {
        BinaryExpression | IfStatement | WhileStatement | DoStatement | ForStatement => true,
        TernaryExpression => node.prev_sibling().map_or(true, |prev_node| {
            !matches!(prev_node.kind_id().into(), QMARK | COLON)
        }),
        _ => false,
    };

    // Looks inside parenthesized expressions and `Not` operators to find what they contain
    loop {
        // Checks if the node is a parenthesized expression or a `Not` operator
        // The child node of index 0 contains the unary expression operator (we look for the `!` operator)
        let is_parenthesised_exp = matches!(node_kind, ParenthesizedExpression);
        let is_not_operator = matches!(node_kind, UnaryExpression)
            && matches!(node.child(0).unwrap().kind_id().into(), BANG);

        // Stops the exploration if the node is neither
        // a parenthesized expression nor a `Not` operator
        if !is_parenthesised_exp && !is_not_operator {
            break;
        }

        // Sets the flag to true if a `Not` operator is found
        // This is used to prove if a variable or a value returned by a method is actually boolean
        // e.g. `return (!x);`
        if !has_boolean_content && is_not_operator {
            has_boolean_content = true;
        }

        // Parenthesized expressions and `Not` operators nodes
        // always store their expressions in the children nodes of index one
        // https://github.com/tree-sitter/tree-sitter-java/blob/master/src/grammar.json#L2472
        // https://github.com/tree-sitter/tree-sitter-java/blob/master/src/grammar.json#L2150
        node = node.child(1).unwrap();
        node_kind = node.kind_id().into();

        // Stops the exploration when the content is found
        if matches!(node_kind, MethodInvocation | Identifier | True | False) {
            if has_boolean_content {
                *conditions += 1.;
            }
            break;
        }
    }
}

// Inspects a list of elements and counts any unary conditional expression found
fn java_count_unary_conditions(list_node: &Node, conditions: &mut f64) {
    use Java::*;

    let list_kind = list_node.object().kind_id().into();
    let mut cursor = list_node.object().walk();

    // Scans the immediate children nodes of the argument node
    if cursor.goto_first_child() {
        loop {
            // Gets the current child node and its kind
            let node = cursor.node();
            let node_kind = node.kind_id().into();

            // Checks if the node is a unary condition
            if matches!(node_kind, MethodInvocation | Identifier | True | False)
                && matches!(list_kind, BinaryExpression)
                && !matches!(list_kind, ArgumentList)
            {
                *conditions += 1.;
            } else {
                // Checks if the node is a unary condition container
                java_inspect_container(&Node::new(node), conditions);
            }

            // Moves the cursor to the next sibling node of the current node
            // Exits the scan if there is no next sibling node
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
}

impl Abc for PythonCode {}
impl Abc for MozjsCode {}
impl Abc for JavascriptCode {}
impl Abc for TypescriptCode {}
impl Abc for TsxCode {}
impl Abc for RustCode {}
impl Abc for CppCode {}
impl Abc for PreprocCode {}
impl Abc for CcommentCode {}

// Fitzpatrick, Jerry (1997). "Applying the ABC metric to C, C++ and Java". C++ Report.
// Source: https://www.softwarerenovation.com/Articles.aspx
// ABC Java rules: (page 8, figure 4)
// ABC Java example: (page 15, listing 4)
impl Abc for JavaCode {
    fn compute(node: &Node, stats: &mut Stats) {
        use Java::*;

        match node.object().kind_id().into() {
            STAREQ | SLASHEQ | PERCENTEQ | DASHEQ | PLUSEQ | LTLTEQ | GTGTEQ | AMPEQ | PIPEEQ
            | CARETEQ | GTGTGTEQ | PLUSPLUS | DASHDASH => {
                stats.assignments += 1.;
            }
            FieldDeclaration | LocalVariableDeclaration => {
                stats.declaration.push(DeclKind::Var);
            }
            Final => {
                if let Some(DeclKind::Var) = stats.declaration.last() {
                    stats.declaration.push(DeclKind::Const);
                }
            }
            SEMI => {
                if let Some(DeclKind::Const | DeclKind::Var) = stats.declaration.last() {
                    stats.declaration.clear();
                }
            }
            EQ => {
                // Excludes constant declarations
                stats
                    .declaration
                    .last()
                    .map(|decl| {
                        if matches!(decl, DeclKind::Var) {
                            stats.assignments += 1.;
                        }
                    })
                    .unwrap_or_else(|| {
                        stats.assignments += 1.;
                    });
            }
            MethodInvocation | New => {
                stats.branches += 1.;
            }
            GTEQ | LTEQ | EQEQ | BANGEQ | Else | Case | Default | QMARK | Try | Catch => {
                stats.conditions += 1.;
            }
            GT | LT => {
                // Excludes `<` and `>` used for generic types
                if let Some(parent) = node.object().parent() {
                    if !matches!(parent.kind_id().into(), TypeArguments) {
                        stats.conditions += 1.;
                    }
                }
            }
            // Counts unary conditions in elements separated by `&&` or `||` boolean operators
            AMPAMP | PIPEPIPE => {
                if let Some(parent) = node.object().parent() {
                    java_count_unary_conditions(&Node::new(parent), &mut stats.conditions);
                }
            }
            // Counts unary conditions among method arguments
            ArgumentList => {
                java_count_unary_conditions(node, &mut stats.conditions);
            }
            // Counts unary conditions inside assignments
            VariableDeclarator | AssignmentExpression => {
                // The child node of index 2 contains the right operand of an assignment operation
                if let Some(right_operand) = node.object().child(2) {
                    if matches!(
                        right_operand.kind_id().into(),
                        ParenthesizedExpression | UnaryExpression
                    ) {
                        java_inspect_container(&Node::new(right_operand), &mut stats.conditions);
                    }
                }
            }
            // Counts unary conditions inside if and while statements
            IfStatement | WhileStatement => {
                // The child node of index 1 contains the condition
                if let Some(condition) = node.object().child(1) {
                    if matches!(condition.kind_id().into(), ParenthesizedExpression) {
                        java_inspect_container(&Node::new(condition), &mut stats.conditions);
                    }
                }
            }
            // Counts unary conditions do-while statements
            DoStatement => {
                // The child node of index 3 contains the condition
                if let Some(condition) = node.object().child(3) {
                    if matches!(condition.kind_id().into(), ParenthesizedExpression) {
                        java_inspect_container(&Node::new(condition), &mut stats.conditions);
                    }
                }
            }
            // Counts unary conditions inside for statements
            ForStatement => {
                // The child node of index 3 contains the `condition` when
                // the initialization expression is a variable declaration
                // e.g. `for ( int i=0; `condition`; ... ) {}`
                if let Some(condition) = node.object().child(3) {
                    match condition.kind_id().into() {
                        SEMI => {
                            // The child node of index 4 contains the `condition` when
                            // the initialization expression is not a variable declaration
                            // e.g. `for ( i=0; `condition`; ... ) {}`
                            if let Some(cond) = node.object().child(4) {
                                match cond.kind_id().into() {
                                    MethodInvocation | Identifier | True | False | SEMI
                                    | RPAREN => {
                                        stats.conditions += 1.;
                                    }
                                    ParenthesizedExpression | UnaryExpression => {
                                        java_inspect_container(
                                            &Node::new(cond),
                                            &mut stats.conditions,
                                        );
                                    }
                                    _ => {}
                                }
                            }
                        }
                        MethodInvocation | Identifier | True | False => {
                            stats.conditions += 1.;
                        }
                        ParenthesizedExpression | UnaryExpression => {
                            java_inspect_container(&Node::new(condition), &mut stats.conditions);
                        }
                        _ => {}
                    }
                }
            }
            // Counts unary conditions inside return statements
            ReturnStatement => {
                // The child node of index 1 contains the return value
                if let Some(value) = node.object().child(1) {
                    if matches!(
                        value.kind_id().into(),
                        ParenthesizedExpression | UnaryExpression
                    ) {
                        java_inspect_container(&Node::new(value), &mut stats.conditions)
                    }
                }
            }
            // Counts unary conditions inside implicit return statements in lambda expressions
            LambdaExpression => {
                // The child node of index 2 contains the return value
                if let Some(value) = node.object().child(2) {
                    if matches!(
                        value.kind_id().into(),
                        ParenthesizedExpression | UnaryExpression
                    ) {
                        java_inspect_container(&Node::new(value), &mut stats.conditions)
                    }
                }
            }
            // Counts unary conditions inside ternary expressions
            TernaryExpression => {
                // The child node of index 0 contains the condition
                if let Some(condition) = node.object().child(0) {
                    match condition.kind_id().into() {
                        MethodInvocation | Identifier | True | False => {
                            stats.conditions += 1.;
                        }
                        ParenthesizedExpression | UnaryExpression => {
                            java_inspect_container(&Node::new(condition), &mut stats.conditions);
                        }
                        _ => {}
                    }
                }
                // The child node of index 2 contains the first expression
                if let Some(expression) = node.object().child(2) {
                    if matches!(
                        expression.kind_id().into(),
                        ParenthesizedExpression | UnaryExpression
                    ) {
                        java_inspect_container(&Node::new(expression), &mut stats.conditions);
                    }
                }
                // The child node of index 4 contains the second expression
                if let Some(expression) = node.object().child(4) {
                    if matches!(
                        expression.kind_id().into(),
                        ParenthesizedExpression | UnaryExpression
                    ) {
                        java_inspect_container(&Node::new(expression), &mut stats.conditions);
                    }
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    // Constant declarations are not counted as assignments
    #[test]
    fn java_constant_declarations() {
        check_metrics!(
            "class A {
                private final int X1 = 0, Y1 = 0;
                public final float PI = 3.14f;
                final static String HELLO = \"Hello,\";
                protected String world = \" world!\";   // +1a
                public float e = 2.718f;                // +1a
                private int x2 = 1, y2 = 2;             // +2a
            
                void m() {
                    final int Z1 = 0, Z2 = 0, Z3 = 0;
                    final float T = 0.0f;
                    int z1 = 1, z2 = 2, z3 = 3;         // +3a
                    float t = 60.0f;                    // +1a
                }
            }",
            "foo.java",
            JavaParser,
            abc,
            [
                (assignments_sum, 8.0),
                (branches_sum, 0.0),
                (conditions_sum, 0.0),
                (magnitude_sum, 8.0), // sqrt(64 + 0 + 0) = sqrt(64)
                (assignments_average, 2.666_666_666_666_666_6), // space count = 3 (1 unit, 1 class and 1 method)
                (branches_average, 0.0),
                (conditions_average, 0.0),
                (assignments_min, 0.0),
                (branches_min, 0.0),
                (conditions_min, 0.0),
                (assignments_max, 4.0),
                (branches_max, 0.0),
                (conditions_max, 0.0)
            ]
        );
    }

    // "In computer science, conditionals (that is, conditional statements, conditional expressions
    // and conditional constructs,) are programming language commands for handling decisions."
    // Source: https://en.wikipedia.org/wiki/Conditional_(computer_programming)
    // According to this definition, boolean expressions that are evaluated to make a decision are considered as conditions
    // Variables, method invocations and true or false values used inside
    // variable declarations and assignment expressions are not counted as conditions
    #[test]
    fn java_declarations_with_conditions() {
        check_metrics!(
            "
            boolean a = (1 > 2);            // +1a +1c
            boolean b = 3 > 4;              // +1a +1c
            boolean c = (1 > 2) && 3 > 4;   // +1a +2c
            boolean d = b && (x > 5) || c;  // +1a +3c
            boolean e = !d;                 // +1a +1c
            boolean f = ((!false));         // +1a +1c
            boolean g = !(!(true));         // +1a +1c
            boolean h = true;               // +1a
            boolean i = (false);            // +1a
            boolean j = (((((true)))));     // +1a
            boolean k = (((((m())))));      // +1a +1b
            boolean l = (((((!m())))));     // +1a +1b +1c
            boolean m = (!(!((m()))));      // +1a +1b +1c
            List<String> n = null;          // +1a (< and > used for generic types are not counted as conditions)
            ",
            "foo.java",
            JavaParser,
            abc,
            [
                (assignments_sum, 14.0),
                (branches_sum, 3.0),
                (conditions_sum, 12.0),
                (magnitude_sum, 18.681_541_692_269_406), // sqrt(196 + 9 + 144) = sqrt(349)
                (assignments_average, 14.0),             // space count = 1 (1 unit)
                (branches_average, 3.0),
                (conditions_average, 12.0),
                (assignments_min, 14.0),
                (branches_min, 3.0),
                (conditions_min, 12.0),
                (assignments_max, 14.0),
                (branches_max, 3.0),
                (conditions_max, 12.0)
            ]
        );
    }

    // Conditions can be found in assignment expressions
    #[test]
    fn java_assignments_with_conditions() {
        check_metrics!(
            "
            a = 2 < 1;                  // +1a +1c
            b = (4 >= 3) && 2 <= 1;     // +1a +2c
            c = a || (x != 10) && b;    // +1a +3c
            d = !false;                 // +1a +1c
            e = (!false);               // +1a +1c
            f = !(false);               // +1a +1c
            g = (!(((true))));          // +1a +1c
            h = ((true));               // +1a
            i = !m();                   // +1a +1b +1c
            j = !((m()));               // +1a +1b +1c
            k = (!(m()));               // +1a +1b +1c
            l = ((!(m())));             // +1a +1b +1c
            m = !B.<Integer>m(2);       // +1a +1b +1c
            n = !((B.<Integer>m(4)));   // +1a +1b +1c
            ",
            "foo.java",
            JavaParser,
            abc,
            [
                (assignments_sum, 14.0),
                (branches_sum, 6.0),
                (conditions_sum, 16.0),
                (magnitude_sum, 22.090_722_034_374_522), // sqrt(196 + 36 + 256) = sqrt(488)
                (assignments_average, 14.0),             // space count = 1 (1 unit)
                (branches_average, 6.0),
                (conditions_average, 16.0),
                (assignments_min, 14.0),
                (branches_min, 6.0),
                (conditions_min, 16.0),
                (assignments_max, 14.0),
                (branches_max, 6.0),
                (conditions_max, 16.0)
            ]
        );
    }

    // Conditions can be found in method arguments
    #[test]
    fn java_methods_arguments_with_conditions() {
        check_metrics!(
            "
            m1(a);                                  // +1b
            m2(a, b);                               // +1b
            m3(true, (false), (((true))));          // +1b
            m3(m1(false), m1(true), m1(false));     // +4b
            m1(!a);                                 // +1b +1c
            m2((((a))), (!b));                      // +1b +1c
            m3(!(a), b, !!!c);                      // +1b +2c
            m3(a, !b, m2(!a, !m2(!b, !m1(!c))));    // +4b +6c
            ",
            "foo.java",
            JavaParser,
            abc,
            [
                (assignments_sum, 0.0),
                (branches_sum, 14.0),
                (conditions_sum, 10.0),
                (magnitude_sum, 17.204_650_534_085_253), // sqrt(0 + 196 + 100) = sqrt(296)
                (assignments_average, 0.0),              // space count = 1 (1 unit)
                (branches_average, 14.0),
                (conditions_average, 10.0),
                (assignments_min, 0.0),
                (branches_min, 14.0),
                (conditions_min, 10.0),
                (assignments_max, 0.0),
                (branches_max, 14.0),
                (conditions_max, 10.0)
            ]
        );
    }

    // "A unary conditional expression is an implicit condition that uses no relational operators."
    // Source: Fitzpatrick, Jerry (1997). "Applying the ABC metric to C, C++ and Java". C++ Report.
    // https://www.softwarerenovation.com/Articles.aspx (page 5)
    #[test]
    fn java_if_single_conditions() {
        check_metrics!(
            "          
            if ( a < 0 ) {}             // +1c
            if ( ((a != 0)) ) {}        // +1c
            if ( !(a > 0) ) {}          // +1c
            if ( !(((a == 0))) ) {}     // +1c
            if ( b.m1() ) {}            // +1b +1c
            if ( !b.m1() ) {}           // +1b +1c
            if ( !!b.m2() ) {}          // +1b +1c
            if ( (!(b.m1())) ) {}       // +1b +1c
            if ( (!(!b.m1())) ) {}      // +1b +1c
            if ( ((b.m2())) ) {}        // +1b +1c
            if ( ((b.m().m1())) ) {}    // +2b +1c
            if ( c ) {}                 // +1c
            if ( !c ) {}                // +1c
            if ( !!!!!!!!!!c ) {}       // +1c
            if ( (((c))) ) {}           // +1c
            if ( (((!c))) ) {}          // +1c
            if ( ((!(c))) ) {}          // +1c
            if ( true ) {}              // +1c
            if ( !true ) {}             // +1c
            if ( ((false)) ) {}         // +1c
            if ( !(!(false)) ) {}       // +1c
            if ( !!!false ) {}          // +1c
            ",
            "foo.java",
            JavaParser,
            abc,
            [
                (assignments_sum, 0.0),
                (branches_sum, 8.0),
                (conditions_sum, 22.0),
                (magnitude_sum, 23.409_399_821_439_25), // sqrt(0 + 64 + 484) = sqrt(548)
                (assignments_average, 0.0),             // space count = 1 (1 unit)
                (branches_average, 8.0),
                (conditions_average, 22.0),
                (assignments_min, 0.0),
                (branches_min, 8.0),
                (conditions_min, 22.0),
                (assignments_max, 0.0),
                (branches_max, 8.0),
                (conditions_max, 22.0)
            ]
        );
    }

    #[test]
    fn java_if_multiple_conditions() {
        check_metrics!(
            "
            if ( a || b || c || d ) {}              // +4c
            if ( a || b && c && d ) {}              // +4c
            if ( x < y && a == b ) {}               // +2c
            if ( ((z < (x + y))) ) {}               // +1c
            if ( a || ((((b))) && c) ) {}           // +3c
            if ( a && ((((a == b))) && c) ) {}      // +3c
            if ( a || ((((a == b))) || ((c))) ) {}  // +3c
            if ( x < y && B.m() ) {}                // +1b +2c
            if ( x < y && !(((B.m()))) ) {}         // +1b +2c
            if ( !(x < y) && !B.m() ) {}            // +1b +2c
            if ( !!!(!!!(a)) && B.m() ||            // +1b +2c
                 !B.m() && (((x > 4))) ) {}         // +1b +2c
            ",
            "foo.java",
            JavaParser,
            abc,
            [
                (assignments_sum, 0.0),
                (branches_sum, 5.0),
                (conditions_sum, 30.0),
                (magnitude_sum, 30.413_812_651_491_1), // sqrt(0 + 25 + 900) = sqrt(925)
                (assignments_average, 0.0),            // space count = 1 (1 unit)
                (branches_average, 5.0),
                (conditions_average, 30.0),
                (assignments_min, 0.0),
                (branches_min, 5.0),
                (conditions_min, 30.0),
                (assignments_max, 0.0),
                (branches_max, 5.0),
                (conditions_max, 30.0)
            ]
        );
    }

    #[test]
    fn java_while_and_do_while_conditions() {
        check_metrics!(
            "
            while ( (!(!(!(a)))) ) {}                   // +1c
            while ( b || 1 > 2 ) {}                     // +2c
            while ( x.m() && (((c))) ) {}               // +1b +2c
            do {} while ( !!!(((!!!a))) );              // +1c
            do {} while ( a || (b && c) );              // +3c
            do {} while ( !x.m() && 1 > 2 || !true );   // +1b +3c
            ",
            "foo.java",
            JavaParser,
            abc,
            [
                (assignments_sum, 0.0),
                (branches_sum, 2.0),
                (conditions_sum, 12.0),
                (magnitude_sum, 12.165_525_060_596_439), // sqrt(0 + 4 + 144) = sqrt(148)
                (assignments_average, 0.0),              // space count = 1 (1 unit)
                (branches_average, 2.0),
                (conditions_average, 12.0),
                (assignments_min, 0.0),
                (branches_min, 2.0),
                (conditions_min, 12.0),
                (assignments_max, 0.0),
                (branches_max, 2.0),
                (conditions_max, 12.0)
            ]
        );
    }

    // GMetrics, a Groovy source code analyzer, provides the following definition of unary conditional expression:
    // "These are cases where a single variable/field/value is treated as a boolean value.
    // Examples include `if (x)` and `return !ready`."
    // According to this definition, unary conditional expressions are counted also in function return values.
    // Source: https://dx42.github.io/gmetrics/metrics/AbcMetric.html
    // Examples: https://github.com/dx42/gmetrics/blob/master/src/test/groovy/org/gmetrics/metric/abc/AbcMetric_MethodTest.groovy
    #[test]
    fn java_return_with_conditions() {
        check_metrics!(
            "class A {
                boolean m1() {
                    return !(z >= 0);       // +1c
                }
                boolean m2() {
                    return (((!x)));        // +1c
                }
                boolean m3() {
                    return x && y;          // +2c
                }
                boolean m4() {
                    return y || (z < 0);    // +2c
                }
                boolean m5() {
                    return x || y ?         // +3c (two unary conditions and one ?)
                        true : false;
                }
            }",
            "foo.java",
            JavaParser,
            abc,
            [
                (assignments_sum, 0.0),
                (branches_sum, 0.0),
                (conditions_sum, 9.0),
                (magnitude_sum, 9.0),       // sqrt(0 + 0 + 81) = sqrt(81)
                (assignments_average, 0.0), // space count = 7 (1 unit, 1 class and 5 methods)
                (branches_average, 0.0),
                (conditions_average, 1.285_714_285_714_285_8),
                (assignments_min, 0.0),
                (branches_min, 0.0),
                (conditions_min, 0.0),
                (assignments_max, 0.0),
                (branches_max, 0.0),
                (conditions_max, 3.0)
            ]
        );
    }

    // Variables, method invocations, and true or false values
    // inside return statements are not counted as conditions
    #[test]
    fn java_return_without_conditions() {
        check_metrics!(
            "class A {
                boolean m1() {
                    return x;
                }
                boolean m2() {
                    return (x);
                }
                boolean m3() {
                    return y.m();   // +1b
                }
                boolean m4() {
                    return false;
                }
                void m5() {
                    return;
                }
            }",
            "foo.java",
            JavaParser,
            abc,
            [
                (assignments_sum, 0.0),
                (branches_sum, 1.0),
                (conditions_sum, 0.0),
                (magnitude_sum, 1.0),       // sqrt(0 + 1 + 0) = sqrt(1)
                (assignments_average, 0.0), // space count = 7 (1 unit, 1 class and 5 methods)
                (branches_average, 0.142_857_142_857_142_85),
                (conditions_average, 0.0),
                (assignments_min, 0.0),
                (branches_min, 0.0),
                (conditions_min, 0.0),
                (assignments_max, 0.0),
                (branches_max, 1.0),
                (conditions_max, 0.0)
            ]
        );
    }

    // Variables, method invocations, and true or false values
    // in lambda expression return values are not counted as conditions
    #[test]
    fn java_lambda_expressions_return_with_conditions() {
        check_metrics!(
            "
            Predicate<Boolean> p1 = a -> a;                         // +1a
            Predicate<Boolean> p2 = b -> true;                      // +1a
            Predicate<Boolean> p3 = c -> m();                       // +1a
            Predicate<Integer> p4 = d -> d > 10;                    // +1a +1c
            Predicate<Boolean> p5 = (e) -> !e;                      // +1a +1c
            Predicate<Boolean> p6 = (f) -> !((!f));                 // +1a +1c
            Predicate<Boolean> p7 = (g) -> !g && true;              // +1a +2c
            BiPredicate<Boolean, Boolean> bp1 = (h, i) -> !h && !i; // +1a +2c
            BiPredicate<Boolean, Boolean> bp2 = (j, k) -> {
                return j || k;                                      // +1a +2c
            };
            ",
            "foo.java",
            JavaParser,
            abc,
            [
                (assignments_sum, 9.0),
                (branches_sum, 1.0),
                (conditions_sum, 9.0),
                (magnitude_sum, 12.767_145_334_803_704), // sqrt(81 + 1 + 81) = sqrt(163)
                (assignments_average, 9.0),              // space count = 1 (1 unit)
                (branches_average, 1.0),
                (conditions_average, 9.0),
                (assignments_min, 9.0),
                (branches_min, 1.0),
                (conditions_min, 9.0),
                (assignments_max, 9.0),
                (branches_max, 1.0),
                (conditions_max, 9.0)
            ]
        );
    }

    #[test]
    fn java_for_with_variable_declaration() {
        check_metrics!(
            "
            for ( int i1 = 0; !(!(!(!a))); i1++ ) {}                // +2a +1c
            for ( int i2 = 0; !B.m(); i2++ ) {}                     // +2a +1b +1c
            for ( int i3 = 0; a || false; i3++ ) {}                 // +2a +2c
            for ( int i4 = 0; a && B.m() ? true : false; i4++ ) {}  // +2a +1b +3c
            for ( int i5 = 0; true; i5++ ) {}                       // +2a +1c
            ",
            "foo.java",
            JavaParser,
            abc,
            [
                (assignments_sum, 10.0),
                (branches_sum, 2.0),
                (conditions_sum, 8.0),
                (magnitude_sum, 12.961_481_396_815_72), // sqrt(100 + 4 + 64) = sqrt(168)
                (assignments_average, 10.0),            // space count = 1 (1 unit)
                (branches_average, 2.0),
                (conditions_average, 8.0),
                (assignments_min, 10.0),
                (branches_min, 2.0),
                (conditions_min, 8.0),
                (assignments_max, 10.0),
                (branches_max, 2.0),
                (conditions_max, 8.0)
            ]
        );
    }

    #[test]
    fn java_for_without_variable_declaration() {
        check_metrics!(
            "class A{
                void m1() {
                    for (i = 0; x < y; i++) {}          // +2a +1c
                    for (i = 0; ((x < y)); i++) {}      // +2a +1c
                    for (i = 0; !(!(x < y)); i++) {}    // +2a +1c
                    for (i = 0; true; i++) {}           // +2a +1c
                }
                void m2() {
                    for ( ; true; ) {}  // +1c
                }
                void m3() {
                    for ( ; ; ) {}      // +1c (one implicit unary condition set to true)
                }
            }",
            "foo.java",
            JavaParser,
            abc,
            [
                (assignments_sum, 8.0),
                (branches_sum, 0.0),
                (conditions_sum, 6.0),
                (magnitude_sum, 10.0),      // sqrt(64 + 0 + 36) = sqrt(100)
                (assignments_average, 1.6), // space count = 5 (1 unit, 1 class and 3 methods)
                (branches_average, 0.0),
                (conditions_average, 1.2),
                (assignments_min, 0.0),
                (branches_min, 0.0),
                (conditions_min, 0.0),
                (assignments_max, 8.0),
                (branches_max, 0.0),
                (conditions_max, 4.0)
            ]
        );
    }

    // Variables, method invocations, and true or false values
    // in ternary expression return values are not counted as conditions
    #[test]
    fn java_ternary_conditions() {
        check_metrics!(
            "
            a = true;                                   // +1a
            b = a ? true : false;                       // +1a +2c
            c = ((((a)))) ? !false : !b;                // +1a +4c
            d = !this.m() ? !!a : (false);              // +1a +1b +3c
            e = !(a) && b ? ((c)) : !d;                 // +1a +4c
            if ( this.m() ? a : !this.m() ) {}          // +2b +3c
            if ( x > 0 ? !(false) : this.m() ) {}       // +1b +3c
            if ( x > 0 && x != 3 ? !(a) : (!(b)) ) {}   // +5c
            ",
            "foo.java",
            JavaParser,
            abc,
            [
                (assignments_sum, 5.0),
                (branches_sum, 4.0),
                (conditions_sum, 24.0),
                (magnitude_sum, 24.839_484_696_748_443), // sqrt(25 + 16 + 576) = sqrt(617)
                (assignments_average, 5.0),              // space count = 1 (1 unit)
                (branches_average, 4.0),
                (conditions_average, 24.0),
                (assignments_min, 5.0),
                (branches_min, 4.0),
                (conditions_min, 24.0),
                (assignments_max, 5.0),
                (branches_max, 4.0),
                (conditions_max, 24.0)
            ]
        );
    }
}
