use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;
use std::fmt;

use crate::checker::Checker;
use crate::langs::*;
use crate::node::Node;
use crate::*;

/// The `Npa` metric.
///
/// This metric counts the number of public attributes
/// of classes/interfaces.
#[derive(Clone, Debug, Default)]
pub struct Stats {
    class_npa: usize,
    interface_npa: usize,
    class_na: usize,
    interface_na: usize,
    class_npa_sum: usize,
    interface_npa_sum: usize,
    class_na_sum: usize,
    interface_na_sum: usize,
    is_class_space: bool,
}

impl Serialize for Stats {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut st = serializer.serialize_struct("npa", 9)?;
        st.serialize_field("classes", &self.class_npa_sum())?;
        st.serialize_field("interfaces", &self.interface_npa_sum())?;
        st.serialize_field("class_attributes", &self.class_na_sum())?;
        st.serialize_field("interface_attributes", &self.interface_na_sum())?;
        st.serialize_field("classes_average", &self.class_cda())?;
        st.serialize_field("interfaces_average", &self.interface_cda())?;
        st.serialize_field("total", &self.total_npa())?;
        st.serialize_field("total_attributes", &self.total_na())?;
        st.serialize_field("average", &self.total_cda())?;
        st.end()
    }
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "classes: {}, interfaces: {}, class_attributes: {}, interface_attributes: {}, classes_average: {}, interfaces_average: {}, total: {}, total_attributes: {}, average: {}",
            self.class_npa_sum(),
            self.interface_npa_sum(),
            self.class_na_sum(),
            self.interface_na_sum(),
            self.class_cda(),
            self.interface_cda(),
            self.total_npa(),
            self.total_na(),
            self.total_cda()
        )
    }
}

impl Stats {
    /// Merges a second `Npa` metric into the first one
    pub fn merge(&mut self, other: &Stats) {
        self.class_npa_sum += other.class_npa_sum;
        self.interface_npa_sum += other.interface_npa_sum;
        self.class_na_sum += other.class_na_sum;
        self.interface_na_sum += other.interface_na_sum;
    }

    /// Returns the number of class public attributes in a space.
    #[inline(always)]
    pub fn class_npa(&self) -> f64 {
        self.class_npa as f64
    }

    /// Returns the number of interface public attributes in a space.
    #[inline(always)]
    pub fn interface_npa(&self) -> f64 {
        self.interface_npa as f64
    }

    /// Returns the number of class attributes in a space.
    #[inline(always)]
    pub fn class_na(&self) -> f64 {
        self.class_na as f64
    }

    /// Returns the number of interface attributes in a space.
    #[inline(always)]
    pub fn interface_na(&self) -> f64 {
        self.interface_na as f64
    }

    /// Returns the number of class public attributes sum in a space.
    #[inline(always)]
    pub fn class_npa_sum(&self) -> f64 {
        self.class_npa_sum as f64
    }

    /// Returns the number of interface public attributes sum in a space.
    #[inline(always)]
    pub fn interface_npa_sum(&self) -> f64 {
        self.interface_npa_sum as f64
    }

    /// Returns the number of class attributes sum in a space.
    #[inline(always)]
    pub fn class_na_sum(&self) -> f64 {
        self.class_na_sum as f64
    }

    /// Returns the number of interface attributes sum in a space.
    #[inline(always)]
    pub fn interface_na_sum(&self) -> f64 {
        self.interface_na_sum as f64
    }

    /// Returns the class `Cda` metric value
    ///
    /// The `Class Data Accessibility` metric value for a class
    /// is computed by dividing the `Npa` value of the class
    /// by the total number of attributes defined in the class.
    ///
    /// This metric is an adaptation of the `Classified Class Data Accessibility` (`CCDA`)
    /// security metric for not classified attributes.
    /// Paper: <https://ieeexplore.ieee.org/abstract/document/5381538>
    #[inline(always)]
    pub fn class_cda(&self) -> f64 {
        self.class_npa_sum() / self.class_na_sum as f64
    }

    /// Returns the interface `Cda` metric value
    ///
    /// The `Class Data Accessibility` metric value for an interface
    /// is computed by dividing the `Npa` value of the interface
    /// by the total number of attributes defined in the interface.
    ///
    /// This metric is an adaptation of the `Classified Class Data Accessibility` (`CCDA`)
    /// security metric for not classified attributes.
    /// Paper: <https://ieeexplore.ieee.org/abstract/document/5381538>
    #[inline(always)]
    pub fn interface_cda(&self) -> f64 {
        // For the Java language it's not necessary to compute the metric value
        // The metric value in Java can only be 1.0 or f64:NAN
        if self.interface_npa_sum == self.interface_na_sum && self.interface_npa_sum != 0 {
            1.0
        } else {
            self.interface_npa_sum() / self.interface_na_sum()
        }
    }

    /// Returns the total `Cda` metric value
    ///
    /// The total `Class Data Accessibility` metric value
    /// is computed by dividing the total `Npa` value
    /// by the total number of attributes.
    ///
    /// This metric is an adaptation of the `Classified Class Data Accessibility` (`CCDA`)
    /// security metric for not classified attributes.
    /// Paper: <https://ieeexplore.ieee.org/abstract/document/5381538>
    #[inline(always)]
    pub fn total_cda(&self) -> f64 {
        self.total_npa() / self.total_na()
    }

    /// Returns the total number of public attributes in a space.
    #[inline(always)]
    pub fn total_npa(&self) -> f64 {
        self.class_npa_sum() + self.interface_npa_sum()
    }

    /// Returns the total number of attributes in a space.
    #[inline(always)]
    pub fn total_na(&self) -> f64 {
        self.class_na_sum() + self.interface_na_sum()
    }

    // Accumulates the number of class and interface
    // public and not public attributes into the sums
    #[inline(always)]
    pub(crate) fn compute_sum(&mut self) {
        self.class_npa_sum += self.class_npa;
        self.interface_npa_sum += self.interface_npa;
        self.class_na_sum += self.class_na;
        self.interface_na_sum += self.interface_na;
    }

    // Checks if the `Npa` metric is disabled
    #[inline(always)]
    pub(crate) fn is_disabled(&self) -> bool {
        !self.is_class_space
    }
}

#[doc(hidden)]
pub trait Npa
where
    Self: Checker,
{
    fn compute(_node: &Node, _stats: &mut Stats) {}
}

impl Npa for PythonCode {}
impl Npa for MozjsCode {}
impl Npa for JavascriptCode {}
impl Npa for TypescriptCode {}
impl Npa for TsxCode {}
impl Npa for RustCode {}
impl Npa for CppCode {}
impl Npa for PreprocCode {}
impl Npa for CcommentCode {}

impl Npa for JavaCode {
    fn compute(node: &Node, stats: &mut Stats) {
        use Java::*;

        // Enables the `Npa` metric if computing stats of a class space
        if Self::is_func_space(node) && stats.is_disabled() {
            stats.is_class_space = true;
        }

        match node.object().kind_id().into() {
            ClassBody => {
                stats.class_na += node
                    .children()
                    .filter(|node| matches!(node.object().kind_id().into(), FieldDeclaration))
                    .map(|declaration| {
                        let attributes = declaration
                            .children()
                            .filter(|n| matches!(n.object().kind_id().into(), VariableDeclarator))
                            .count();
                        // The first child node contains the list of attribute modifiers
                        // There are several modifiers that may be part of a field declaration
                        // Source: https://docs.oracle.com/javase/tutorial/reflect/member/fieldModifiers.html
                        if declaration.object().child(0).map_or(false, |modifiers| {
                            // Looks for the `public` keyword in the list of attribute modifiers
                            matches!(modifiers.kind_id().into(), Modifiers)
                                && Node::new(modifiers)
                                    .first_child(|id| id == Public)
                                    .is_some()
                        }) {
                            stats.class_npa += attributes;
                        }
                        attributes
                    })
                    .sum::<usize>();
            }
            // Every field declaration in the body of an interface is implicitly public, static, and final
            // Source: https://docs.oracle.com/javase/specs/jls/se7/html/jls-9.html
            InterfaceBody => {
                // Children nodes are filtered because Java interfaces
                // can contain constants but also methods and nested types
                // Source: https://docs.oracle.com/javase/tutorial/java/IandI/createinterface.html
                stats.interface_na += node
                    .children()
                    .filter(|node| matches!(node.object().kind_id().into(), ConstantDeclaration))
                    .flat_map(|node| node.children())
                    .filter(|node| matches!(node.object().kind_id().into(), VariableDeclarator))
                    .count();
                stats.interface_npa = stats.interface_na;
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn java_single_attributes() {
        check_metrics!(
            "class X {
                public byte a;      // +1
                public short b;     // +1
                public int c;       // +1
                public long d;      // +1
                public float e;     // +1
                public double f;    // +1
                public boolean g;   // +1
                public char h;      // +1
                byte i;
                short j;
                int k;
                long l;
                float m;
                double n;
                boolean o;
                char p;
            }",
            "foo.java",
            JavaParser,
            npa,
            [
                (class_npa_sum, 8, usize),
                (interface_npa_sum, 0, usize),
                (class_na_sum, 16, usize),
                (interface_na_sum, 0, usize),
                (total_npa, 8, usize),
                (total_na, 16, usize)
            ],
            [
                (class_cda, 0.5),
                (interface_cda, f64::NAN),
                (total_cda, 0.5)
            ]
        );
    }

    #[test]
    fn java_multiple_attributes() {
        check_metrics!(
            "class X {
                public byte a1;                 // +1
                public short b1, b2;            // +2
                public int c1, c2, c3;          // +3
                public long d1, d2, d3, d4;     // +4
                public float e1, e2, e3, e4;    // +4
                public double f1, f2, f3;       // +3
                public boolean g1, g2;          // +2
                public char h1;                 // +1
                byte i1, i2, i3, i4;
                short j1, j2, j3;
                int k1, k2;
                long l1;
                float m1;
                double n1, n2;
                boolean o1, o2, o3;
                char p1, p2, p3, p4;
            }",
            "foo.java",
            JavaParser,
            npa,
            [
                (class_npa_sum, 20, usize),
                (interface_npa_sum, 0, usize),
                (class_na_sum, 40, usize),
                (interface_na_sum, 0, usize),
                (total_npa, 20, usize),
                (total_na, 40, usize)
            ],
            [
                (class_cda, 0.5),
                (interface_cda, f64::NAN),
                (total_cda, 0.5)
            ]
        );
    }

    #[test]
    fn java_initialized_attributes() {
        check_metrics!(
            "class X {
                public byte a1 = 1;                             // +1
                public short b1 = 2, b2;                        // +2
                public int c1, c2 = 3, c3;                      // +3
                public long d1 = 4, d2, d3, d4 = 5;             // +4
                public float e1, e2 = 6.0f, e3 = 7.0f, e4;      // +4
                public double f1 = 8.0, f2 = 9.0, f3 = 10.0;    // +3
                public boolean g1 = true, g2;                   // +2
                public char h1 = 'a';                           // +1
                byte i1 = 1, i2 = 2, i3 = 3, i4 = 4;
                short j1 = 5, j2, j3 = 6;
                int k1, k2 = 7;
                long l1 = 8;
                float m1 = 9.0f;
                double n1, n2 = 10.0;
                boolean o1, o2 = false, o3;
                char p1 = 'a', p2 = 'b', p3 = 'c', p4 = 'd';
            }",
            "foo.java",
            JavaParser,
            npa,
            [
                (class_npa_sum, 20, usize),
                (interface_npa_sum, 0, usize),
                (class_na_sum, 40, usize),
                (interface_na_sum, 0, usize),
                (total_npa, 20, usize),
                (total_na, 40, usize)
            ],
            [
                (class_cda, 0.5),
                (interface_cda, f64::NAN),
                (total_cda, 0.5)
            ]
        );
    }

    #[test]
    fn java_array_attributes() {
        check_metrics!(
            "class X {
                public byte[] a1, a2, a3, a4;                       // +4
                public short b1[], b2[], b3[];                      // +3
                public int[] c1 = { 1 }, c2;                        // +2
                public long d1[] = { 1 };                           // +1
                public float[] e1 = { 1.0f, 2.0f, 3.0f };           // +1
                public double f1[] = { 1.0, 2.0, 3.0 }, f2[];       // +2
                public boolean[] g1 = new boolean[5], g2, g3;       // +3
                public char[] h1 = new char[5], h2[], h3[], h4[];   // +4
                byte[] i1;
                short j1[], j2[];
                int[] k1, k2, k3 = { 1 };
                long l1[], l2[] = { 1 }, l3[] = { 2 }, l4[];
                float[] m1, m2, m3, m4 = { 1.0f, 2.0f, 3.0f };
                double n1[], n2[] = { 1.0, 2.0, 3.0 }, n3[];
                boolean[] o1, o2 = new boolean[5];
                char[] p1 = new char[5];
            }",
            "foo.java",
            JavaParser,
            npa,
            [
                (class_npa_sum, 20, usize),
                (interface_npa_sum, 0, usize),
                (class_na_sum, 40, usize),
                (interface_na_sum, 0, usize),
                (total_npa, 20, usize),
                (total_na, 40, usize)
            ],
            [
                (class_cda, 0.5),
                (interface_cda, f64::NAN),
                (total_cda, 0.5)
            ]
        );
    }

    #[test]
    fn java_object_attributes() {
        check_metrics!(
            "class X {
                public Integer[] a1 = { 1 };                                    // +1
                public Integer b1, b2;                                          // +2
                public String[] c1 = { \"Hello\" }, c2, c3 = { \"World!\" };    // +3
                public String d1[][] = { { \"Hello\" }, { \"World!\" } };       // +1
                public Y[] e1, e2[];                                            // +2
                public Y f1[], f2[][], f3[][][];                                // +3
                Integer[] g1 = { new Integer(1) };
                Integer h1 = new Integer(1), h2 = new Integer(2);
                String[] i1, i2 = { \"Hello World!\" }, i3;
                String j1 = \"Hello World!\";
                Y[] k1[], k2;
                Y l1[][], l2[], l3 = new Y();
            }",
            "foo.java",
            JavaParser,
            npa,
            [
                (class_npa_sum, 12, usize),
                (interface_npa_sum, 0, usize),
                (class_na_sum, 24, usize),
                (interface_na_sum, 0, usize),
                (total_npa, 12, usize),
                (total_na, 24, usize)
            ],
            [
                (class_cda, 0.5),
                (interface_cda, f64::NAN),
                (total_cda, 0.5)
            ]
        );
    }

    #[test]
    fn java_generic_attributes() {
        check_metrics!(
            "class X<T, S extends T> {
                public T a1;                            // +1
                public Entry<T, S> b1, b2[];            // +2
                public ArrayList<T> c1, c2, c3;         // +3
                public HashMap<Long, Double> d1, d2;    // +2
                public TreeSet<String> e1;              // +1
                S f1;
                Entry<S, T> g1[], g2;
                ArrayList<S> h1, h2, h3;
                HashMap<Long, Float> i1, i2;
                TreeSet<Entry<S, T>> j1;
            }",
            "foo.java",
            JavaParser,
            npa,
            [
                (class_npa_sum, 9, usize),
                (interface_npa_sum, 0, usize),
                (class_na_sum, 18, usize),
                (interface_na_sum, 0, usize),
                (total_npa, 9, usize),
                (total_na, 18, usize)
            ],
            [
                (class_cda, 0.5),
                (interface_cda, f64::NAN),
                (total_cda, 0.5)
            ]
        );
    }

    #[test]
    fn java_attribute_modifiers() {
        check_metrics!(
            "class X {
                public transient volatile static int a;     // +1
                transient public volatile static int b;     // +1
                transient volatile public static int c;     // +1
                transient volatile static public int d;     // +1
                public transient static final int e = 1;    // +1
                transient public static final int f = 2;    // +1
                transient static public final int g = 3;    // +1
                transient static final public int h = 4;    // +1
                protected transient volatile static int i;
                transient volatile static protected int j;
                private transient volatile static int k;
                transient volatile static private int l;
                transient volatile static int m;
                transient static final int n = 5;
                static public final int o = 6;              // +1
                final public int p = 7;                     // +1
            }",
            "foo.java",
            JavaParser,
            npa,
            [
                (class_npa_sum, 10, usize),
                (interface_npa_sum, 0, usize),
                (class_na_sum, 16, usize),
                (interface_na_sum, 0, usize),
                (total_npa, 10, usize),
                (total_na, 16, usize)
            ],
            [
                (class_cda, 0.625),
                (interface_cda, f64::NAN),
                (total_cda, 0.625)
            ]
        );
    }

    #[test]
    fn java_classes() {
        check_metrics!(
            "class X {
                public int a;       // +1
                public boolean b;   // +1
                private char c;
            }
            class Y {
                private double d;
                private long e;
                public float f;      // +1
            }",
            "foo.java",
            JavaParser,
            npa,
            [
                (class_npa_sum, 3, usize),
                (interface_npa_sum, 0, usize),
                (class_na_sum, 6, usize),
                (interface_na_sum, 0, usize),
                (total_npa, 3, usize),
                (total_na, 6, usize)
            ],
            [
                (class_cda, 0.5),
                (interface_cda, f64::NAN),
                (total_cda, 0.5)
            ]
        );
    }

    #[test]
    fn java_nested_inner_classes() {
        check_metrics!(
            "class X {
                public int a;           // +1
                class Y {
                    public boolean b;   // +1
                    class Z {
                        public char c;  // +1
                    }
                }
            }",
            "foo.java",
            JavaParser,
            npa,
            [
                (class_npa_sum, 3, usize),
                (interface_npa_sum, 0, usize),
                (class_na_sum, 3, usize),
                (interface_na_sum, 0, usize),
                (total_npa, 3, usize),
                (total_na, 3, usize)
            ],
            [
                (class_cda, 1.0),
                (interface_cda, f64::NAN),
                (total_cda, 1.0)
            ]
        );
    }

    #[test]
    fn java_local_inner_classes() {
        check_metrics!(
            "class X {
                public int a;                   // +1
                void x() {
                    class Y {
                        public boolean b;       // +1
                        void y() {
                            class Z {
                                public char c;  // +1
                                void z() {}
                            }
                        }
                    }
                }
            }",
            "foo.java",
            JavaParser,
            npa,
            [
                (class_npa_sum, 3, usize),
                (interface_npa_sum, 0, usize),
                (class_na_sum, 3, usize),
                (interface_na_sum, 0, usize),
                (total_npa, 3, usize),
                (total_na, 3, usize)
            ],
            [
                (class_cda, 1.0),
                (interface_cda, f64::NAN),
                (total_cda, 1.0)
            ]
        );
    }

    #[test]
    fn java_anonymous_inner_classes() {
        check_metrics!(
            "abstract class X {
                public int a;               // +1
            }
            abstract class Y {
                boolean b;
            }
            class Z {
                public char c;              // +1
                public void z(){
                    X x1 = new X() {
                        public double d;    // +1
                    };
                    Y y1 = new Y() {
                        long e;
                    };
                }
            }",
            "foo.java",
            JavaParser,
            npa,
            [
                (class_npa_sum, 3, usize),
                (interface_npa_sum, 0, usize),
                (class_na_sum, 5, usize),
                (interface_na_sum, 0, usize),
                (total_npa, 3, usize),
                (total_na, 5, usize)
            ],
            [
                (class_cda, 0.6),
                (interface_cda, f64::NAN),
                (total_cda, 0.6)
            ]
        );
    }

    #[test]
    fn java_interface() {
        check_metrics!(
            "interface X {
                public int a = 0;           // +1
                static boolean b = false;   // +1
                final char c = ' ';         // +1
            }",
            "foo.java",
            JavaParser,
            npa,
            [
                (class_npa_sum, 0, usize),
                (interface_npa_sum, 3, usize),
                (class_na_sum, 0, usize),
                (interface_na_sum, 3, usize),
                (total_npa, 3, usize),
                (total_na, 3, usize)
            ],
            [
                (class_cda, f64::NAN),
                (interface_cda, 1.0),
                (total_cda, 1.0)
            ]
        );
    }
}
