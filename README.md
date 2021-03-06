# clap ![Travis-CI](https://travis-ci.org/kbknapp/clap-rs.svg?branch=master)
Command Line Argument Parser written in Rust

A simply library for parsing command line arguments and subcommands when writing command line and console applications.

You can use `clap` to lay out a list of possible valid command line arguments and subcommands, then let `clap` parse the string given by the user at runtime.

When using `clap` you define a set of parameters and rules for your arguments and subcommands, then at runtime `clap` will determine their validity.

`clap` also provides the traditional version and help switches 'for free' by parsing the list of possible valid arguments lazily at runtime, and if not already defined by the developer `clap` will autogenerate all applicable "help" and "version" switches (as well as a "help" subcommand if other subcommands are defined as well).

 After defining a list of possible valid arguments and subcommands, `clap` gives you a list of valid matches that the user supplied at runtime, or informs the user of their error and exits gracefully. You can use this list to determine the functioning of your program.

## Quick Example
 
```rust
// (Full example with comments in examples/myapp.rs)
extern crate clap;
use clap::{Arg, App, SubCommand};

fn main() {
	let matches = App::new("MyApp")
							.version("1.0")
							.author("Kevin K. <kbknapp@gmail.com>")
							.about("Does awesome things")
							.arg(Arg::new("config")
										.short("c")
										.long("config")
										.help("Sets a custom config file")
										.takes_value(true))
							.arg(Arg::new("output")
										.help("Sets an optional output file")
										.index(1))
							.arg(Arg::new("debug")
										.short("d")
	 									.multiple(true)
										.help("Turn debugging information on"))
							.subcommand(SubCommand::new("test")
										.about("controls testing features")
										.arg(Arg::new("verbose")
											.short("v")
											.help("print test information verbosely")))
							.get_matches();
	
	if let Some(o) = matches.value_of("output") {
		println!("Value for output: {}", o);
	}
	 
	if let Some(c) = matches.value_of("config") {
		println!("Value for config: {}", c);
	}
	
	match matches.occurrences_of("debug") {
		0 => println!("Debug mode is off"),
		1 => println!("Debug mode is kind of on"),
		2 => println!("Debug mode is on"),
		3 | _ => println!("Don't be crazy"),
	}
	 
	if let Some(ref matches) = matches.subcommand_matches("test") {
		if matches.is_present("list") {
			println!("Printing testing lists...");
		} else {
			println!("Not printing testing lists...");
		}
	}
	 
	// more porgram logic goes here...
}
```

If you were to compile the above program and run it with the flag `--help` or `-h` (or `help` subcommand, since we defined `test` as a subcommand) the following output woud be presented

```sh
$ myprog --help
MyApp 1.0
Kevin K. <kbknapp@gmail.com>
Does awesome things

USAGE:
	MyApp [FLAGS] [OPTIONS] [POSITIONAL] [SUBCOMMANDS]

FLAGS:
	-d   			Turn debugging information on
	-h,--help		Prints this message
	-v,--version	Prints version information
 
OPTIONS:
	-c,--config <config>		Sets a custom config file

POSITIONAL ARGUMENTS:
	output			Sets an optional output file

SUBCOMMANDS:
	help			Prints this message
	test			Controls testing features
```

## Installation
Add `clap` as a dependecy in your `Cargo.toml` file to use from crates.io:

 ```
 [dependencies]
 clap = "*"
 ```
 Or track the latest on the master branch at github:

```
[dependencies.clap]
git = "https://github.com/kbknapp/clap-rs.git"
```

Then run `cargo build` or `cargo update` for your project.

## Usage

Add `extern crate clap;` to your crate root.

## More Information

You can find complete documentation on the [github-pages site](http://kbknapp.github.io/clap-rs/docs/clap/index.html) for this project.
