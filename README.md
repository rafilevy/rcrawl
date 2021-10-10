# rcrawl 1.2.1

Searches for a given file or directory using multiple threads and prints its path to stdout.

## Usage:
`rcrawl [FLAGS] [OPTIONS] <root_directory> <filename>`

## Flags:

- `-a, --all`
  
  A flag indicating wether to search all files including hidden files

- `-h, --help`

  Prints help information

- `-R, --regex` 

  A flag indicating wether the search expression is a regular expression

- `-r, --relative`
  
  A flag indicating that relative rather than full files paths should be returned
    
- `-s, --single`
  
  A flag indicating only the first found item should be output (equivalent to --max_items 1)
    
- `-V, --version`
  
  Prints version information
    
- `-v, --verbose`
  
  A flag indicating wether to print verbose information

## Options:
- `-d, --max_depth <max_depth>`
  
  The maximum depth to recursively search to

- `-i, --max_items <max_items>`
  
  The maximum number of results to return

- `-t, --threads <threads>`
  
  The number of threads to use to search for 

## Args:
- `<root_directory>`

  The name of the root directory to start searching from
- `<filename>`

  The name of the file/directory to search for


Author: [Rafi Levy](rafilevy.co.uk)
