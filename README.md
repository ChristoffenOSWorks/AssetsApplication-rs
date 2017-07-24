# AssetsApplication-rs
THIS TOOK ME 3 DAYS. THE C ONE TOOK 3 MONTHS. A Rust port of the original. Minus all the GNU jargon and whatnot (argparse, argv, and argc all pale in straightforwardness in comparison to clap).

## How it works
Make sure you have the prereqs of the original (cairo + corresponding devel dependencies, the same for Gtk3). Oh, and did I mention you need to use rustup to install the build tools? I recommend nightly, but that's because I use nightly for projects that depend on other crates. 

Once this is complete, screw using a build system. Cargo takes care of compiling and deps, so no Makefiles.

To compile **AND** run, two words and a flag from the root of this directory:

```
cargo run --release
```

Cargo interfaces with ```rustc``` to compile everything and pulls down deps from crates.io. The above command will do all of this and then produce an optimized binary for your machine. It will then run it.

## Explanation
So, why am I beating a dead horse over a year later? This was to see if I could pull it off better in Rust. For the CLI in Rust, here's what ```--help``` will output:

```
The Christoffen OSWorks, LLC. Assets Application 

Re-written from C to Rust. Rust is a great language, cleaner for the job than C in soooo many cases.

USAGE:
    assets_application [FLAGS] [SUBCOMMAND]

FLAGS:
    -c, --colored-all         Generate all colored triangles, and the logo
    -C, --colored-no-logo     Generate all colored triangles, except for the logo
    -h, --help                Prints help information
    -o, --outlined-all        Generate all outlined triangles, and the logo
    -O, --outlined-no-logo    Generate all outlined triangles, except for the logo
    -V, --version             Prints version information

SUBCOMMANDS:
    colored-all         Generate all colored triangles, and the logo
    colored-no-logo     Generate all colored triangles, except for the logo
    help                Prints this message or the help of the given subcommand(s)
    outlined-all        Generate all outlined triangles, and the logo
    outlined-no-logo    Generate all outlined triangles, except for the logo
```

Now, for the one using argparse in C:

```
Usage: Assets [OPTION...]

  -c, --colored-all          Generate all colored triangles, and the logo

  -C, --colored-no-logo      Generate all colored triangles, except for the
                             logo

  -F, --frankenlogos         Generate the Frankenlogos (don't ask; just do)

  -o, --outlined-all         Generate all outlined triangles, and the logo

  -O, --outlined-no-logo     Generate all outlined triangles, except for the
                             logo

  -?, --help                 Give this help list
      --usage                Give a short usage message
  -V, --version              Print program version

Report bugs to M. Gage Morgan <gage@christoffen.com>.
```

The layout of the ```clap``` crate is easy, and their docs even show about 14 different ways to do the same things. In contrast, C's argparse is mortifyingly terrible to decipher. Also, it doesn't have Git-style subcommands XD

This is just pre-gaming for when I have to drop down to C++. That will be happening while I am at college, and a majority of any projects I'm satisfied with will probably get pushed over to my personal account, https://github.com/MGageMorgan.

I still don't know everything about programming, and tbh I probably never will. For example, C is a much easier language to write a parser/lexer for via bison and yacc. Those two tools can be difficult to learn in and of themselves, but once you get over them, you can use them and LLVM IR to (possibly) make acompiler that compiles everything down to bytecode. 

Consider this a response to our pipe dream: ```hypox```. I don't have time currently to make that a reality. 

## Christoffen Status
Our **SIX-YEAR-PLAN** involves planning activity dropping from 2018-2022, and having been put back on the rails after 2024. This is to ensure Clark and I have jobs in our qualified fields, and we will continue operating Christoffen as a side project until profits increase enough to allow us to quit our "real jobs" aka *life support*.
