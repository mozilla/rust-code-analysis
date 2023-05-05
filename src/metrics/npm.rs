use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;
use std::fmt;

use crate::checker::Checker;
use crate::langs::*;
use crate::node::Node;
use crate::*;

/// The `Npm` metric.
///
/// This metric counts the number of public methods
/// of classes/interfaces.
#[derive(Clone, Debug, Default)]
pub struct Stats {
    class_npm: usize,
    interface_npm: usize,
    class_nm: usize,
    interface_nm: usize,
    class_npm_sum: usize,
    interface_npm_sum: usize,
    class_nm_sum: usize,
    interface_nm_sum: usize,
    is_class_space: bool,
}

impl Serialize for Stats {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut st = serializer.serialize_struct("npm", 9)?;
        st.serialize_field("classes", &self.class_npm_sum())?;
        st.serialize_field("interfaces", &self.interface_npm_sum())?;
        st.serialize_field("class_methods", &self.class_nm_sum())?;
        st.serialize_field("interface_methods", &self.interface_nm_sum())?;
        st.serialize_field("classes_average", &self.class_coa())?;
        st.serialize_field("interfaces_average", &self.interface_coa())?;
        st.serialize_field("total", &self.total_npm())?;
        st.serialize_field("total_methods", &self.total_nm())?;
        st.serialize_field("average", &self.total_coa())?;
        st.end()
    }
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "classes: {}, interfaces: {}, class_methods: {}, interface_methods: {}, classes_average: {}, interfaces_average: {}, total: {}, total_methods: {}, average: {}",
            self.class_npm_sum(),
            self.interface_npm_sum(),
            self.class_nm_sum(),
            self.interface_nm_sum(),
            self.class_coa(),
            self.interface_coa(),
            self.total_npm(),
            self.total_nm(),
            self.total_coa()
        )
    }
}

impl Stats {
    /// Merges a second `Npm` metric into the first one
    pub fn merge(&mut self, other: &Stats) {
        self.class_npm_sum += other.class_npm_sum;
        self.interface_npm_sum += other.interface_npm_sum;
        self.class_nm_sum += other.class_nm_sum;
        self.interface_nm_sum += other.interface_nm_sum;
    }

    /// Returns the number of class public methods in a space.
    #[inline(always)]
    pub fn class_npm(&self) -> f64 {
        self.class_npm as f64
    }

    /// Returns the number of interface public methods in a space.
    #[inline(always)]
    pub fn interface_npm(&self) -> f64 {
        self.interface_npm as f64
    }

    /// Returns the number of class methods in a space.
    #[inline(always)]
    pub fn class_nm(&self) -> f64 {
        self.class_nm as f64
    }

    /// Returns the number of interface methods in a space.
    #[inline(always)]
    pub fn interface_nm(&self) -> f64 {
        self.interface_nm as f64
    }

    /// Returns the number of class public methods sum in a space.
    #[inline(always)]
    pub fn class_npm_sum(&self) -> f64 {
        self.class_npm_sum as f64
    }

    /// Returns the number of interface public methods sum in a space.
    #[inline(always)]
    pub fn interface_npm_sum(&self) -> f64 {
        self.interface_npm_sum as f64
    }

    /// Returns the number of class methods sum in a space.
    #[inline(always)]
    pub fn class_nm_sum(&self) -> f64 {
        self.class_nm_sum as f64
    }

    /// Returns the number of interface methods sum in a space.
    #[inline(always)]
    pub fn interface_nm_sum(&self) -> f64 {
        self.interface_nm_sum as f64
    }

    /// Returns the class `Coa` metric value
    ///
    /// The `Class Operation Accessibility` metric value for a class
    /// is computed by dividing the `Npm` value of the class
    /// by the total number of methods defined in the class.
    ///
    /// This metric is an adaptation of the `Classified Operation Accessibility` (`COA`)
    /// security metric for not classified methods.
    /// Paper: <https://ieeexplore.ieee.org/abstract/document/5381538>
    #[inline(always)]
    pub fn class_coa(&self) -> f64 {
        self.class_npm_sum() / self.class_nm_sum()
    }

    /// Returns the interface `Coa` metric value
    ///
    /// The `Class Operation Accessibility` metric value for an interface
    /// is computed by dividing the `Npm` value of the interface
    /// by the total number of methods defined in the interface.
    ///
    /// This metric is an adaptation of the `Classified Operation Accessibility` (`COA`)
    /// security metric for not classified methods.
    /// Paper: <https://ieeexplore.ieee.org/abstract/document/5381538>
    #[inline(always)]
    pub fn interface_coa(&self) -> f64 {
        // For the Java language it's not necessary to compute the metric value
        // The metric value in Java can only be 1.0 or f64:NAN
        if self.interface_npm_sum == self.interface_nm_sum && self.interface_npm_sum != 0 {
            1.0
        } else {
            self.interface_npm_sum() / self.interface_nm_sum()
        }
    }

    /// Returns the total `Coa` metric value
    ///
    /// The total `Class Operation Accessibility` metric value
    /// is computed by dividing the total `Npm` value
    /// by the total number of methods.
    ///
    /// This metric is an adaptation of the `Classified Operation Accessibility` (`COA`)
    /// security metric for not classified methods.
    /// Paper: <https://ieeexplore.ieee.org/abstract/document/5381538>
    #[inline(always)]
    pub fn total_coa(&self) -> f64 {
        self.total_npm() / self.total_nm()
    }

    /// Returns the total number of public methods in a space.
    #[inline(always)]
    pub fn total_npm(&self) -> f64 {
        self.class_npm_sum() + self.interface_npm_sum()
    }

    /// Returns the total number of methods in a space.
    #[inline(always)]
    pub fn total_nm(&self) -> f64 {
        self.class_nm_sum() + self.interface_nm_sum()
    }

    // Accumulates the number of class and interface
    // public and not public methods into the sums
    #[inline(always)]
    pub(crate) fn compute_sum(&mut self) {
        self.class_npm_sum += self.class_npm;
        self.interface_npm_sum += self.interface_npm;
        self.class_nm_sum += self.class_nm;
        self.interface_nm_sum += self.interface_nm;
    }

    // Checks if the `Npm` metric is disabled
    #[inline(always)]
    pub(crate) fn is_disabled(&self) -> bool {
        !self.is_class_space
    }
}

pub trait Npm
where
    Self: Checker,
{
    fn compute(node: &Node, stats: &mut Stats);
}

impl Npm for JavaCode {
    fn compute(node: &Node, stats: &mut Stats) {
        use Java::*;

        // Enables the `Npm` metric if computing stats of a class space
        if Self::is_func_space(node) && stats.is_disabled() {
            stats.is_class_space = true;
        }

        match node.object().kind_id().into() {
            ClassBody => {
                stats.class_nm += node
                    .children()
                    .filter(|node| Self::is_func(node))
                    .map(|method| {
                        // The first child node contains the list of method modifiers
                        // There are several modifiers that may be part of a method declaration
                        // Source: https://docs.oracle.com/javase/tutorial/reflect/member/methodModifiers.html
                        if let Some(modifiers) = method.object().child(0) {
                            // Looks for the `public` keyword in the list of method modifiers
                            if matches!(modifiers.kind_id().into(), Modifiers)
                                && Node::new(modifiers)
                                    .first_child(|id| id == Public)
                                    .is_some()
                            {
                                stats.class_npm += 1;
                            }
                        }
                    })
                    .count();
            }
            // All methods in an interface are implicitly public
            // Source: https://docs.oracle.com/javase/tutorial/java/IandI/interfaceDef.html
            InterfaceBody => {
                // Children nodes are filtered because Java interfaces
                // can contain methods but also constants and nested types
                // Source: https://docs.oracle.com/javase/tutorial/java/IandI/createinterface.html
                stats.interface_nm += node.children().filter(|node| Self::is_func(node)).count();
                stats.interface_npm = stats.interface_nm;
            }
            _ => {}
        }
    }
}

implement_metric_trait!(
    Npm,
    PythonCode,
    MozjsCode,
    JavascriptCode,
    TypescriptCode,
    TsxCode,
    RustCode,
    CppCode,
    PreprocCode,
    CcommentCode,
    KotlinCode
);

#[cfg(test)]
mod tests {
    use crate::tools::check_metrics;

    use super::*;

    #[test]
    fn java_constructors() {
        check_metrics::<JavaParser>(
            "class X {
                X() {}
                private X(int a) {}
                protected X(int a, int b) {}
                public X(int a, int b, int c) {}    // +1
            }",
            "foo.java",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.npm,
                    @r###"
                    {
                      "classes": 1.0,
                      "interfaces": 0.0,
                      "class_methods": 4.0,
                      "interface_methods": 0.0,
                      "classes_average": 0.25,
                      "interfaces_average": null,
                      "total": 1.0,
                      "total_methods": 4.0,
                      "average": 0.25
                    }"###
                );
            },
        );
    }

    #[test]
    fn java_methods_returning_primitive_types() {
        check_metrics::<JavaParser>(
            "class X {
                public byte a() {}      // +1
                public short b() {}     // +1
                public int c() {}       // +1
                public long d() {}      // +1
                public float e() {}     // +1
                public double f() {}    // +1
                public boolean g() {}   // +1
                public char h() {}      // +1
                byte i() {}
                short j() {}
                int k() {}
                long l() {}
                float m() {}
                double n() {}
                boolean o() {}
                char p() {}
            }",
            "foo.java",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.npm,
                    @r###"
                    {
                      "classes": 8.0,
                      "interfaces": 0.0,
                      "class_methods": 16.0,
                      "interface_methods": 0.0,
                      "classes_average": 0.5,
                      "interfaces_average": null,
                      "total": 8.0,
                      "total_methods": 16.0,
                      "average": 0.5
                    }"###
                );
            },
        );
    }

    #[test]
    fn java_methods_returning_arrays() {
        check_metrics::<JavaParser>(
            "class X {
                public byte[] a() {}    // +1
                public short[] b() {}   // +1
                public int[] c() {}     // +1
                public long[] d() {}    // +1
                public float[] e() {}   // +1
                public double[] f() {}  // +1
                public boolean[] g() {} // +1
                public char[] h() {}    // +1
                byte[] i() {}
                short[] j() {}
                int[] k() {}
                long[] l() {}
                float[] m() {}
                double[] n() {}
                boolean[] o() {}
                char[] p() {}
            }",
            "foo.java",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.npm,
                    @r###"
                    {
                      "classes": 8.0,
                      "interfaces": 0.0,
                      "class_methods": 16.0,
                      "interface_methods": 0.0,
                      "classes_average": 0.5,
                      "interfaces_average": null,
                      "total": 8.0,
                      "total_methods": 16.0,
                      "average": 0.5
                    }"###
                );
            },
        );
    }

    #[test]
    fn java_methods_returning_objects() {
        check_metrics::<JavaParser>(
            "class X {
                public Integer[] a() {} // +1
                public Integer b() {}   // +1
                public String[] c() {}  // +1
                public String d() {}    // +1
                public Y[] e() {}       // +1
                public Y f() {}         // +1
                Integer[] g() {}
                Integer h() {}
                String[] i() {}
                String j() {}
                Y[] k() {}
                Y l() {}
            }",
            "foo.java",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.npm,
                    @r###"
                    {
                      "classes": 6.0,
                      "interfaces": 0.0,
                      "class_methods": 12.0,
                      "interface_methods": 0.0,
                      "classes_average": 0.5,
                      "interfaces_average": null,
                      "total": 6.0,
                      "total_methods": 12.0,
                      "average": 0.5
                    }"###
                );
            },
        );
    }

    #[test]
    fn java_methods_with_generic_types() {
        check_metrics::<JavaParser>(
            "class X {
                public <T, S extends T> void a(T x, S y) {} // +1
                public <T, S> int b(T x, S y) {}            // +1
                public <T> boolean c(T x) {}                // +1
                public <T> ArrayList<T> d() {}              // +1
                public Y<String> e() {}                     // +1
                <T, S extends T> void f(T x, S y) {}
                <T, S> int g(T x, S y) {}
                <T> boolean h(T x) {}
                <T> ArrayList<T> i() {}
                Y<String> j() {}
            }",
            "foo.java",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.npm,
                    @r###"
                    {
                      "classes": 5.0,
                      "interfaces": 0.0,
                      "class_methods": 10.0,
                      "interface_methods": 0.0,
                      "classes_average": 0.5,
                      "interfaces_average": null,
                      "total": 5.0,
                      "total_methods": 10.0,
                      "average": 0.5
                    }"###
                );
            },
        );
    }

    #[test]
    fn java_method_modifiers() {
        check_metrics::<JavaParser>(
            "abstract class X {
                public static final synchronized strictfp void a() {}   // +1
                static public final synchronized strictfp void b() {}   // +1
                static final public synchronized strictfp void c() {}   // +1
                static final synchronized public strictfp void d() {}   // +1
                static final synchronized strictfp public void e() {}   // +1
                protected static final synchronized native void f();
                static protected final synchronized native void g();
                static final protected synchronized native void h();
                static final synchronized protected native void i();
                static final synchronized native protected void j();
                abstract public void k();                               // +1
                abstract void l();
            }",
            "foo.java",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.npm,
                    @r###"
                    {
                      "classes": 6.0,
                      "interfaces": 0.0,
                      "class_methods": 12.0,
                      "interface_methods": 0.0,
                      "classes_average": 0.5,
                      "interfaces_average": null,
                      "total": 6.0,
                      "total_methods": 12.0,
                      "average": 0.5
                    }"###
                );
            },
        );
    }

    #[test]
    fn java_classes() {
        check_metrics::<JavaParser>(
            "class X {
                public void a() {}  // +1
                public void b() {}  // +1
                private void c() {}
            }
            class Y {
                private void d() {}
                private void e() {}
                public void f() {}  // +1
            }",
            "foo.java",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.npm,
                    @r###"
                    {
                      "classes": 3.0,
                      "interfaces": 0.0,
                      "class_methods": 6.0,
                      "interface_methods": 0.0,
                      "classes_average": 0.5,
                      "interfaces_average": null,
                      "total": 3.0,
                      "total_methods": 6.0,
                      "average": 0.5
                    }"###
                );
            },
        );
    }

    #[test]
    fn java_nested_inner_classes() {
        check_metrics::<JavaParser>(
            "class X {
                public void a() {}          // +1
                class Y {
                    public void b() {}      // +1
                    class Z {
                        public void c() {}  // +1
                    }
                }
            }",
            "foo.java",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.npm,
                    @r###"
                    {
                      "classes": 3.0,
                      "interfaces": 0.0,
                      "class_methods": 3.0,
                      "interface_methods": 0.0,
                      "classes_average": 1.0,
                      "interfaces_average": null,
                      "total": 3.0,
                      "total_methods": 3.0,
                      "average": 1.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn java_local_inner_classes() {
        check_metrics::<JavaParser>(
            "class X {
                public void a() {                   // +1
                    class Y {
                        public void b() {           // +1
                            class Z {
                                public void c() {}  // +1
                            }
                        }
                    }
                }
            }",
            "foo.java",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.npm,
                    @r###"
                    {
                      "classes": 3.0,
                      "interfaces": 0.0,
                      "class_methods": 3.0,
                      "interface_methods": 0.0,
                      "classes_average": 1.0,
                      "interfaces_average": null,
                      "total": 3.0,
                      "total_methods": 3.0,
                      "average": 1.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn java_anonymous_inner_classes() {
        check_metrics::<JavaParser>(
            "abstract class X {
                public abstract void a();   // +1
            }
            abstract class Y {
                abstract void b();
            }
            class Z {
                public void c(){            // +1
                    X x = new X() {
                        @Override
                        public void a() {}  // +1
                    };
                    Y y = new Y() {
                        @Override
                        void b() {}
                    };
                }
            }",
            "foo.java",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.npm,
                    @r###"
                    {
                      "classes": 3.0,
                      "interfaces": 0.0,
                      "class_methods": 5.0,
                      "interface_methods": 0.0,
                      "classes_average": 0.6,
                      "interfaces_average": null,
                      "total": 3.0,
                      "total_methods": 5.0,
                      "average": 0.6
                    }"###
                );
            },
        );
    }

    #[test]
    fn java_interface() {
        check_metrics::<JavaParser>(
            "interface X {
                public int a(); // +1
                boolean b();    // +1
                void c();       // +1
            }",
            "foo.java",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.npm,
                    @r###"
                    {
                      "classes": 0.0,
                      "interfaces": 3.0,
                      "class_methods": 0.0,
                      "interface_methods": 3.0,
                      "classes_average": null,
                      "interfaces_average": 1.0,
                      "total": 3.0,
                      "total_methods": 3.0,
                      "average": 1.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn java_interfaces_and_class() {
        check_metrics::<JavaParser>(
            "interface X {
                void a();           // +1
            }
            interface Y extends X {
                void b();           // +1
                void c();           // +1
            }
            class Z implements Y {
                @Override
                public void a() {}  // +1
                @Override
                public void b() {}  // +1
                @Override
                public void c() {}  // +1
                void d() {}
                void e() {}
            }",
            "foo.java",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.npm,
                    @r###"
                    {
                      "classes": 3.0,
                      "interfaces": 3.0,
                      "class_methods": 5.0,
                      "interface_methods": 3.0,
                      "classes_average": 0.6,
                      "interfaces_average": 1.0,
                      "total": 6.0,
                      "total_methods": 8.0,
                      "average": 0.75
                    }"###
                );
            },
        );
    }
}
