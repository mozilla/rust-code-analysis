# Lines of Code (LoC)

[Lines of code](https://en.wikipedia.org/wiki/Source_lines_of_code) has 5 variants that count different types of code lines.

## Background

```python
def web_socket_transfer_data(request):
    while True:
        line = request.ws_stream.receive_message()
        if line is None:
            return
        code, reason = line.split(' ', 1)
        if code is None or reason is None:
            return
        request.ws_stream.close_connection(int(code), reason)
        
        # close_connection() initiates closing handshake. It validates code
        # and reason. If you want to send a broken close frame for a test,
        # following code will be useful.
        # > data = struct.pack('!H', int(code)) + reason.encode('UTF-8')
        # > request.connection.write(stream.create_close_frame(data))
        # > # Suppress to re-respond client responding close frame.
        # > raise Exception(\"customized server initiated closing handshake\")
```

### SLOC

A straight count of all lines in the file including code, comments, and blank lines.  
In the example above, sloc would be 17.

### PLOC

A count of the "physical" lines of code contained in the source code. This would include any brackets on a new line. 
If it is executable code, it is counted. Note that comments and blank lines are not counted in this.  
In the example above, ploc would be 9.

### LLOC

The "logical" lines are lines of code that actually do something.

```csharp
for(int i = 0; i < 10; i++)
{
  Console.WriteLine(i);
}
```

As an example, the C# above has 4 physical lines of code but only has 2 logical lines. 
The `for` statement and the `Console.WriteLine(i);` are the only lines that really do anything.

### CLOC

A count of the comments in the code. This is immaterial of whether they are single line comment or block comments.
In the Python example, there are 7 comment lines.


### BLANK

This quite obviously, counts the blank lines.  
In the Python example there is 1 blank line.

## Implementation

To implement the LoC related metrics described above you need to implement the `Loc` trait for the language you are adding support to.

This requires implementing the `compute` member. `fn compute(node: &Node, stats: &mut Stats, is_func_space: bool, is_unit: bool)`.
See [/src/metrics/loc.rs](https://github.com/mozilla/rust-code-analysis/blob/master/src/metrics/loc.rs) for where to implement as well as examples from other languages.


TODO: explain using the grammar