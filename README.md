# rcrawl

A command line program which recursively searches for a named file or directory - built with rust

```_
Recursively searches for a given file or directory and prints its full path to stdout

USAGE:
    rcrawl [FLAGS] [OPTIONS] <dirname>

FLAGS:
    -f, --full       A flag indicating that full rather than relative paths should be returned
    -h, --help       Prints help information
    -s, --single     A flag indicating only the first found item should be output (equivalent to --max_items 1)
    -V, --version    Prints version information

OPTIONS:
    -d, --max_depth <max_depth>    The maximum depth to recursively search to
    -i, --max_items <max_items>    The maximum number of results to return

ARGS:
    <dirname>    The name of the file or directory to search for
```
