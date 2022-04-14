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
/// Original paper and definition:
/// https://www.researchgate.net/publication/3187649_Kemerer_CF_A_metric_suite_for_object_oriented_design_IEEE_Trans_Softw_Eng_206_476-493
#[derive(Debug, Clone)]
pub struct Stats {
    wmc: f64,
    space_kind: SpaceKind,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            wmc: 0.,
            space_kind: SpaceKind::Unknown,
        }
    }
}

impl Serialize for Stats {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut st = serializer.serialize_struct("wmc", 1)?;
        st.serialize_field(
            if self.space_kind == SpaceKind::Unit {
                "wmc_total"
            } else {
                "wmc"
            },
            &self.wmc(),
        )?;
        st.end()
    }
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "wmc: {}", self.wmc(),)
    }
}

impl Stats {
    /// Returns the `Wmc` metric value
    pub fn wmc(&self) -> f64 {
        self.wmc
    }

    /// Returns the space kind associated to the `Wmc` metric value
    pub fn space_kind(&self) -> SpaceKind {
        self.space_kind
    }

    // Returns true if the `Wmc` metric value should not be visible
    pub fn is_not_class_or_unit(&self) -> bool {
        self.space_kind != SpaceKind::Class && self.space_kind != SpaceKind::Unit
    }
}

#[doc(hidden)]
pub trait Wmc
where
    Self: Checker,
{
    fn compute(_child: &FuncSpace, _parent: &mut FuncSpace) {}
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
    fn compute(child: &FuncSpace, parent: &mut FuncSpace) {
        if child.kind == SpaceKind::Function && parent.kind == SpaceKind::Class {
            parent.metrics.wmc.space_kind = SpaceKind::Class;
            parent.metrics.wmc.wmc += child.metrics.cyclomatic.cyclomatic_sum()
        } else if child.kind == SpaceKind::Class && parent.kind == SpaceKind::Unit {
            parent.metrics.wmc.space_kind = SpaceKind::Unit;
            parent.metrics.wmc.wmc += child.metrics.wmc.wmc
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn java_single_class() {
        check_metrics!(
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
            JavaParser,
            wmc,
            [(wmc, 13.0)] // 1 top level class
        );
    }

    // Constructors are considered as methods
    // Reference: https://pdepend.org/documentation/software-metrics/weighted-method-count.html
    #[test]
    fn java_multiple_classes() {
        check_metrics!(
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
            JavaParser,
            wmc,
            [(wmc, 5.0)] // 2 top level classes (wmc total = 3 + 2)
        );
    }

    #[test]
    fn java_static_nested_class() {
        check_metrics!(
            "public class TopLevelClass { // wmc = 0
                public static class StaticNestedClass { // wmc = 1
                    private void m() { // +1
                        System.out.println(\"Test\");
                    }
                }
            }",
            "foo.java",
            JavaParser,
            wmc,
            [(wmc, 0.0)] // 1 top level class
        );
    }

    #[test]
    fn java_nested_inner_classes() {
        check_metrics!(
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
            JavaParser,
            wmc,
            [(wmc, 2.0)] // 1 top level class
        );
    }

    #[test]
    fn java_local_inner_class() {
        check_metrics!(
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
            JavaParser,
            wmc,
            [(wmc, 5.0)] // 1 top level class
        );
    }

    #[test]
    fn java_anonymous_inner_class() {
        check_metrics!(
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
            JavaParser,
            wmc,
            [(wmc, 4.0)] // 2 top level classes (wmc total = 1 + 3)
        );
    }

    #[test]
    fn java_nested_anonymous_inner_classes() {
        check_metrics!(
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
            JavaParser,
            wmc,
            [(wmc, 8.0)] // 2 top level classes (wmc total = 2 + 6)
        );
    }

    #[test]
    fn java_lambda_expression() {
        check_metrics!(
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
            JavaParser,
            wmc,
            [(wmc, 2.0)] // 1 top level class
        );
    }
}
