# Supported Metrics

**rust-code-analysis** implements a series of metrics:

- **ABC**: it measures the size of a source code by counting the number of
Assignments (`A`), Branches (`B`) and Conditions (`C`).
- **BLANK**: it counts the number of blank lines in a source file.
- **CC**: it calculates the _Cyclomatic complexity_ examining the
  control flow of a program.
- **CLOC**: it counts the number of comments in a source file.
- **COGNITIVE**: it calculates the _Cognitive complexity_, measuring how complex
it is to understand a unit of code.
- **HALSTEAD**: it is a suite that provides a series of information, such as the
  effort required to maintain the analyzed code, the size in bits to store the
  program, the difficulty to understand the code, an estimate of the number of
  bugs present in the codebase, and an estimate of the time needed to
  implement the software.
- **LLOC**: it counts the number of logical lines (statements) contained in a
source file.
- **MI**: it is a suite that allows to evaluate the maintainability of a software.
- **NARGS**: it counts the number of arguments of a function/method.
- **NEXITS**: it counts the number of possible exit points from a method/function.
- **NOM**: it counts the number of functions and closures in a file/trait/class.
- **NPA**: it counts the number of public attributes in classes/interfaces.
- **NPM**: it counts the number of public methods in classes/interfaces.
- **PLOC**: it counts the number of physical lines (instructions) contained in
a source file.
- **SLOC**: it counts the number of lines in a source file.
- **WMC**: it sums the _Cyclomatic complexity_ of every method defined in a class.
