# Lines of Code (LoC)

[Lines of code](https://en.wikipedia.org/wiki/Source_lines_of_code) is a software metric that gives an indication of the size of some source code by counting the lines of the source code.

## Background

```rust
/*
Instruction: Implement factorial function
For extra credits, do not use mutable state or a imperative loop like `for` or `while`.
 */

/// Factorial: n! = n*(n-1)*(n-2)*(n-3)...3*2*1
fn factorial(num: u64) -> u64 {
    
    // use `product` on `Iterator`
    (1..=num).product()
}
```

The example above will be used to illustrate each of the **LoC** metrics described below.

### SLOC

A straight count of all lines in the file including code, comments, and blank lines.  
METRIC VALUE: 5

### PLOC

A count of the instruction lines of code contained in the source code. This would include any brackets or similar syntax on a new line. 
Note that comments and blank lines are not counted in this.   
METRIC VALUE: 3

### LLOC

The "logical" lines is a count of the number of statements in the code. Note that what a statement is depends on the language.  
In the above example there is only a single statement which id the function call of `product` with the `Iterator` as its argument.  
METRIC VALUE: 1

### CLOC

A count of the comments in the code. The type of comment does not matter ie single line, block, or doc.  
METRIC VALUE: 2

### BLANK

Last but not least, this metric counts the blank lines present in a code.
METRIC VALUE: 

## Implementation

To implement the LoC related metrics described above you need to implement the `Loc` trait for the language you want to support.

This requires implementing the `compute` function.
See [/src/metrics/loc.rs](https://github.com/mozilla/rust-code-analysis/blob/master/src/metrics/loc.rs) for where to implement, as well as examples from other languages.

As you implement tests, you may find yourself running into an off by 1 error for SLOC count. 
For most code we expect the `SpaceKind` to be `SpaceKind::Unit` but the default is `SpaceKind::Unknown`.
To solve this you need to implement the `Getter` trait in [src/getter.rs](https://github.com/mozilla/rust-code-analysis/blob/master/src/getter.rs). Specifically, the `get_space_kind` function.