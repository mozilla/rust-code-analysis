# Supported Metrics

**rust-code-analysis** implements a series of metrics
- **CC**: it calculates the code complexity examining the
  control flow of a program.
- **SLOC**: it counts the number of lines in a source file.
- **PLOC**: it counts the number of physical lines (instructions) contained in
  a source file.
- **LLOC**: it counts the number of logical lines (statements) contained in
  a source file.
- **CLOC**: it counts the number of comments in a source file.
- **BLANK**: it counts the number of blank lines in a source file.
- **HALSTEAD**: it is a suite that provides a series of information, such as the
  effort required to maintain the analyzed code, the size in bits to store the
  program, the difficulty to understand the code, an estimate of the number of
  bugs present in the codebase, and an estimate of the time needed to
  implement the software.
- **MI**: it is a suite that allows to evaluate the maintainability of a software.
- **NOM**: it counts the number of functions and closures in a file/trait/class.
- **NEXITS**: it counts the number of possible exit points from a method/function.
- **NARGS**: it counts the number of arguments of a function/method.

The metrics above are still **NOT** implemented for C#, CSS, Go, Java and HTML languages.
