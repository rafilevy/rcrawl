# rcrawl 1.1.5

A command line program which recursively searches for a named file or directory - built with rust
(Similar to the UNIX find command)

Recursively searches for a given file or directory and prints its full path to stdout

```_
USAGE:
    rcrawl [FLAGS] [OPTIONS] <filename>

FLAGS:
    -h, --help        Prints help information
    -r, --relative    A flag indicating that relative rather than full files paths should be returned
    -s, --single      A flag indicating only the first found item should be output (equivalent to --max_items 1)
    -V, --version     Prints version information

OPTIONS:
    -d, --max_depth <max_depth>    The maximum depth to recursively search to
    -i, --max_items <max_items>    The maximum number of results to return

ARGS:
    <filename>    The name of the file/directory to search for
```
