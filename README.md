# Parse GenBank XML Files 

Parse GenBank Genome XML files!

## Install

### From source

From GitHub, assuming you have [Rust installed](https://www.rust-lang.org/tools/install):

```
$ git clone https://github.com/mooreryan/parse-gb-xml.git
$ cd parse-gb-xml
$ cargo build --release
```

Then move or symlink the binary (`./target/release/parse-gb-xml`) somewhere on your path:

```
$ ln -s $(pwd)/target/release/parse-gb-xml $HOME/bin/parse-gb-xml
```

## Getting XML files from NCBI

Try NCBI [E-utilities](https://www.ncbi.nlm.nih.gov/books/NBK25499/).  Assuming the genomes you want have the following IDs (MF417837, MF417838, MF417839), then do something like this:

```
https://eutils.ncbi.nlm.nih.gov/entrez/eutils/efetch.fcgi?db=nuccore&id=MF417837,MF417838,MF417839&rettype=gb&retmode=xml
```

You can then run that output file through this program.

## Usage

See the options.

```
$ parse-gb-xml --help

parse-gb-xml 0.1.0
Parse GB XML files into genome and its peptides

USAGE:
    parse-gb-xml <xml> <genomes> <peptides>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <xml>         Path to input GB XML file
    <genomes>     Path to genome output
    <peptides>    Path to peptide output
```

## License

Dual-licensed to be compatible with the Rust project.

Licensed under the [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0) or the [MIT license](http://opensource.org/licenses/MIT), at your option. This program may not be copied, modified, or distributed except according to those terms.