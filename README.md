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
