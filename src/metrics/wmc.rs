use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;
use std::fmt;

use crate::checker::Checker;
use crate::*;

// FIX ME: New Java switches are not correctly recognised by tree-sitter-java version 0.19.0
// However, the issue has already been addressed and resolved upstream on the tree-sitter-java GitHub repository
// Upstream issue: https://github.com/tree-sitter/tree-sitter-java/issues/69
// Upstream PR which resolves the issue: https://github.com/tree-sitter/tree-sitter-java/pull/78

/// The `Wmc` metric.
///
/// This metric sums the cyclomatic complexities of all the methods defined in a class.
/// The `Wmc` (Weighted Methods per Class) is an object-oriented metric for classes.
///
/// Original paper and definition:
/// <https://www.researchgate.net/publication/3187649_Kemerer_CF_A_metric_suite_for_object_oriented_design_IEEE_Trans_Softw_Eng_206_476-493>
#[derive(Debug, Clone, Default)]
pub struct Stats {
    cyclomatic: f64,
    class_wmc: f64,
    interface_wmc: f64,
    class_wmc_sum: f64,
    interface_wmc_sum: f64,
    space_kind: SpaceKind,
}

impl Serialize for Stats {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut st = serializer.serialize_struct("wmc", 3)?;
        st.serialize_field("classes", &self.class_wmc_sum())?;
        st.serialize_field("interfaces", &self.interface_wmc_sum())?;
        st.serialize_field("total", &self.total_wmc())?;
        st.end()
    }
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "classes: {}, interfaces: {}, total: {}",
            self.class_wmc_sum(),
            self.interface_wmc_sum(),
            self.total_wmc()
        )
    }
}

impl Stats {
    /// Merges a second `Wmc` metric into the first one
    pub fn merge(&mut self, other: &Stats) {
        use SpaceKind::*;

        // Merges the cyclomatic complexity of a method
        // into the `Wmc` metric value of a class or interface
        if let Function = other.space_kind {
            match self.space_kind {
                Class => self.class_wmc += other.cyclomatic,
                Interface => self.interface_wmc += other.cyclomatic,
                _ => {}
            }
        }

        self.class_wmc_sum += other.class_wmc_sum;
        self.interface_wmc_sum += other.interface_wmc_sum;
    }

    /// Returns the `Wmc` metric value of the classes in a space.
    #[inline(always)]
    pub fn class_wmc(&self) -> f64 {
        self.class_wmc
    }

    /// Returns the `Wmc` metric value of the interfaces in a space.
    #[inline(always)]
    pub fn interface_wmc(&self) -> f64 {
        self.interface_wmc
    }

    /// Returns the sum of the `Wmc` metric values of the classes in a space.
    #[inline(always)]
    pub fn class_wmc_sum(&self) -> f64 {
        self.class_wmc_sum
    }

    /// Returns the sum of the `Wmc` metric values of the interfaces in a space.
    #[inline(always)]
    pub fn interface_wmc_sum(&self) -> f64 {
        self.interface_wmc_sum
    }

    /// Returns the total `Wmc` metric value in a space.
    #[inline(always)]
    pub fn total_wmc(&self) -> f64 {
        self.class_wmc_sum() + self.interface_wmc_sum()
    }

    // Accumulates the `Wmc` metric values
    // of classes and interfaces into the sums
    #[inline(always)]
    pub(crate) fn compute_sum(&mut self) {
        self.class_wmc_sum += self.class_wmc;
        self.interface_wmc_sum += self.interface_wmc;
    }

    // Checks if the `Wmc` metric is disabled
    #[inline(always)]
    pub(crate) fn is_disabled(&self) -> bool {
        matches!(self.space_kind, SpaceKind::Function | SpaceKind::Unknown)
    }
}

#[doc(hidden)]
pub trait Wmc
where
    Self: Checker,
{
    fn compute(_space_kind: SpaceKind, _cyclomatic: &cyclomatic::Stats, _stats: &mut Stats) {}
}

impl Wmc for PythonCode {}
impl Wmc for MozjsCode {}
impl Wmc for JavascriptCode {}
impl Wmc for TypescriptCode {}
impl Wmc for TsxCode {}
impl Wmc for RustCode {}
impl Wmc for CppCode {}
impl Wmc for PreprocCode {}
impl Wmc for CcommentCode {}

impl Wmc for JavaCode {
    fn compute(space_kind: SpaceKind, cyclomatic: &cyclomatic::Stats, stats: &mut Stats) {
        use SpaceKind::*;

        if let Unit | Class | Interface | Function = space_kind {
            if stats.space_kind == Unknown {
                stats.space_kind = space_kind;
            }
            if space_kind == Function {
                // Saves the cyclomatic complexity of the method
                stats.cyclomatic = cyclomatic.cyclomatic_sum();
            }
        }
    }
}

impl Wmc for KotlinCode {}

#[cfg(test)]
mod tests {
    use crate::tools::check_metrics;

    use super::*;

    #[test]
    fn java_single_class() {
        check_metrics::<JavaParser>(
            "public class Example { // wmc = 13

                public boolean m1(boolean a, boolean b) { // +1
                    boolean r = false;
                    if (a && b == a || b) { // +3
                        r = true;
                    }
                    return r;
                }

                public boolean m2(int n) { // +1
                    for (int i = 0; i < n; i++) { // +1
                        int j = n;
                        while (j > i) { // +1
                            j--;
                        }
                    }
                    return (n % 2 == 0) ? true : false; // +1
                }

                public int m3(int x, int y, int z) { // +1
                    int ret;
                    try {
                        z = x/y + y/x;
                    } catch (ArithmeticException e) { // +1
                        z = (x == 0) ? -1 : -2; // +1
                    }
                    switch (z) {
                        case -1: // +1
                            ret = y * y;
                            break;
                        case -2: // +1
                            ret = x * x;
                            break;
                        default:
                            ret = x + y;
                    }
                    return ret;
                }
            }",
            "foo.java",
            |metric| {
                // 1 class
                insta::assert_json_snapshot!(
                    metric.wmc,
                    @r###"
                    {
                      "classes": 13.0,
                      "interfaces": 0.0,
                      "total": 13.0
                    }"###
                );
            },
        );
    }

    // Constructors are considered as methods
    // Reference: https://pdepend.org/documentation/software-metrics/weighted-method-count.html
    #[test]
    fn java_multiple_classes() {
        check_metrics::<JavaParser>(
            "public class MainClass { // wmc = 3
                private int a;
                public MainClass() { // +1
                    a = 0;
                }
                public void setA(int n) { // +1
                    a = n;
                }
                public int getA() { // +1
                    return a;
                }
            }

            class TopLevelClass { // wmc = 2
                private int b;
                public TopLevelClass() { // +1
                    b = 0;
                }
                public int getB() { // +1
                    return b;
                }
            }",
            "foo.java",
            |metric| {
                // 2 classes (3 + 2)
                insta::assert_json_snapshot!(
                    metric.wmc,
                    @r###"
                    {
                      "classes": 5.0,
                      "interfaces": 0.0,
                      "total": 5.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn java_static_nested_class() {
        check_metrics::<JavaParser>(
            "public class TopLevelClass { // wmc = 0
                public static class StaticNestedClass { // wmc = 1
                    private void m() { // +1
                        System.out.println(\"Test\");
                    }
                }
            }",
            "foo.java",
            |metric| {
                // 2 classes (0 + 2)
                insta::assert_json_snapshot!(
                    metric.wmc,
                    @r###"
                    {
                      "classes": 1.0,
                      "interfaces": 0.0,
                      "total": 1.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn java_nested_inner_classes() {
        check_metrics::<JavaParser>(
            "public class TopLevelClass { // wmc = 2
                private int a;

                class InnerClassBefore { // wmc = 1
                    private boolean b = (a % 2 == 0) ? true : false;
                    public boolean getB() { // +1
                        return b;
                    }
                }

                public TopLevelClass(int n) { // +1
                    if (a != n) { // +1
                        a = n;
                    }
                }

                class InnerClassAfter { // wmc = 2
                    private int c = a;

                    public int getC() { // +1
                        return c;
                    }
                    public void setC(int n) { // +1
                        c = n;
                    }

                    class InnerClass1 { // wmc = 1
                        private int p1;
                        class InnerClass2 { // wmc = 1
                            private int p2;
                            public int getP2() { // +1
                                return p2;
                            }
                            class InnerClass3 { // wmc = 2
                                private int p3;
                                public int getP3() { // +1
                                    return p3;
                                }
                                public void setP3(int n) { // +1
                                    p3 = n;
                                }
                            }
                        }
                        public void setP1(int n) { // +1
                            p1 = n;
                        }
                    }
                }
            }",
            "foo.java",
            |metric| {
                // 6 classes (2 + 1 + 2 + 1 + 1 + 2)
                insta::assert_json_snapshot!(
                    metric.wmc,
                    @r###"
                    {
                      "classes": 9.0,
                      "interfaces": 0.0,
                      "total": 9.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn java_local_inner_class() {
        check_metrics::<JavaParser>(
            "import java.util.LinkedList;
            import java.util.List;

            public final class FinalClass { // wmc = 5
                private int a = 1;
                public void test() { // +1
                    final List<String> localList = new LinkedList<String>();

                    class LocalInnerClass { // +1, wmc = 2
                        private int b = (a == 1) ? 1 : 0; // +1
                        public void print() { // +1
                            for ( String s : localList ) { // +1
                                System.out.println(s);
                            }
                        }
                    }
                }
            }",
            "foo.java",
            |metric| {
                // 2 classes (5 + 2)
                insta::assert_json_snapshot!(
                    metric.wmc,
                    @r###"
                    {
                      "classes": 7.0,
                      "interfaces": 0.0,
                      "total": 7.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn java_anonymous_inner_class() {
        check_metrics::<JavaParser>(
            "abstract class AbstractClass { // wmc = 1
                abstract void m1(); // +1
            }
            public class TopLevelClass{ // wmc = 3
                public void m(){ // +1
                    AbstractClass ac1 = new AbstractClass() {
                        void m1() { // +1
                            for (int i = 0; i < 5; i++) { // +1
                                System.out.println(\"Test 1: \" + i);
                            }
                        }
                    };
                    ac1.m1();
                }
            }",
            "foo.java",
            |metric| {
                // 2 classes (1 + 3)
                insta::assert_json_snapshot!(
                    metric.wmc,
                    @r###"
                    {
                      "classes": 4.0,
                      "interfaces": 0.0,
                      "total": 4.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn java_nested_anonymous_inner_classes() {
        check_metrics::<JavaParser>(
            "abstract class AbstractClass{ // wmc = 2
                abstract void m1(); // +1
                abstract void m2(); // +1
            }
            public class TopLevelClass{ // wmc = 6
                public void m(){ // +1

                    AbstractClass ac1 = new AbstractClass() {
                        void m1() { // +1
                            for (int i = 0; i < 5; i++) { // +1
                                System.out.println(\"Test 1: \" + i);
                            }
                        }
                        void m2() { // +1
                            AbstractClass ac2 = new AbstractClass() {
                                void m1() { // +1
                                    System.out.println(\"Test A\");
                                }
                                void m2() { // +1
                                    System.out.println(\"Test B\");
                                }
                            };
                            ac2.m2();
                            System.out.println(\"Test 2\");
                        }
                    };
                    ac1.m1();
                }
            }",
            "foo.java",
            |metric| {
                // 2 classes (2 + 6)
                insta::assert_json_snapshot!(
                    metric.wmc,
                    @r###"
                    {
                      "classes": 8.0,
                      "interfaces": 0.0,
                      "total": 8.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn java_lambda_expression() {
        check_metrics::<JavaParser>(
            "import java.util.ArrayList;

            public class TopLevelClass { // wmc = 2
                private ArrayList<Integer> numbers;

                public void m1() { // +1
                    numbers = new ArrayList<Integer>();
                    numbers.add(1);
                    numbers.add(2);
                    numbers.add(3);
                }

                public void m2() { // +1
                    numbers.forEach( (n) -> { System.out.println(n); } );
                }
            }",
            "foo.java",
            |metric| {
                // 1 class
                insta::assert_json_snapshot!(
                    metric.wmc,
                    @r###"
                    {
                      "classes": 2.0,
                      "interfaces": 0.0,
                      "total": 2.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn java_single_interface() {
        check_metrics::<JavaParser>(
            "interface Example { // wmc = 6
                default boolean m1(boolean a, boolean b) { // +1
                    return (a && b == a || b); // +2
                }
                default int m2(int n) { // +1
                    return (n != 0) ? 1/n : n; // +1
                };
                void m3(); // +1
            }",
            "foo.java",
            |metric| {
                // 1 interface
                insta::assert_json_snapshot!(
                    metric.wmc,
                    @r###"
                    {
                      "classes": 0.0,
                      "interfaces": 6.0,
                      "total": 6.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn java_multiple_interfaces() {
        check_metrics::<JavaParser>(
            "interface FirstInterface { // wmc = 1
                int a = 0;
                default int getA() { // +1
                    return a;
                }
            }

            interface SecondInterface { // wmc = 2
                void setB(int n); // +1
                int getB(); // +1
            }",
            "foo.java",
            |metric| {
                // 2 interfaces (1 + 2)
                insta::assert_json_snapshot!(
                    metric.wmc,
                    @r###"
                    {
                      "classes": 0.0,
                      "interfaces": 3.0,
                      "total": 3.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn java_nested_inner_interfaces() {
        check_metrics::<JavaParser>(
            "interface TopLevelInterface { // wmc = 1
                interface InnerInterfaceBefore { // wmc = 1
                    void m1(); // +1
                }

                void m2(); // +1

                interface InnerInterfaceAfter { // wmc = 2
                    void m3(); // +1
                    interface InnerInterface { // wmc = 1
                        void m4(); // +1
                    }
                    void m5(); // +1
                }
            }",
            "foo.java",
            |metric| {
                // 4 interfaces (1 + 1 + 2 + 1)
                insta::assert_json_snapshot!(
                    metric.wmc,
                    @r###"
                    {
                      "classes": 0.0,
                      "interfaces": 5.0,
                      "total": 5.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn java_class_in_interface() {
        check_metrics::<JavaParser>(
            "interface TopLevelInterface { // wmc = 2
                int getA(); // +1
                boolean getB(); // +1

                class InnerClass { // wmc = 2
                    float c;
                    double d;
                    float getC() { // +1
                        return c;
                    }
                    double getD() { // +1
                        return d;
                    }
                }
            }",
            "foo.java",
            |metric| {
                // 1 class 1 interface
                insta::assert_json_snapshot!(
                    metric.wmc,
                    @r###"
                    {
                      "classes": 2.0,
                      "interfaces": 2.0,
                      "total": 4.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn java_interface_in_class() {
        check_metrics::<JavaParser>(
            "class TopLevelClass { // wmc = 2
                int a;
                boolean b;
                int getA() { // +1
                    return a;
                }
                boolean getB() { // +1
                    return b;
                }

                interface InnerInterface { // wmc = 2
                    float getC(); // +1
                    double getD(); // +1
                }
            }",
            "foo.java",
            |metric| {
                // 1 class 1 interface
                insta::assert_json_snapshot!(
                    metric.wmc,
                    @r###"
                    {
                      "classes": 2.0,
                      "interfaces": 2.0,
                      "total": 4.0
                    }"###
                );
            },
        );
    }
}
