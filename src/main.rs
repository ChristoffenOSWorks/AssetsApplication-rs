mod triangles;
mod controls;
mod ui;

extern crate cairo;
extern crate gtk;
extern crate clap;

use std::fs;
use clap::{Arg, App, SubCommand};

fn main() {
    let matches =
        App::new("The Christoffen OSWorks, LLC. Assets Application\n")
        .about("\nRe-written from C to Rust. Rust is a great language, cleaner for the job than C in soooo many cases.")
        .arg(Arg::with_name("colored-all")
            .help("Generate all colored triangles, and the logo")
            .short("c")
            .long("colored-all"))

        .arg(Arg::with_name("colored-no-logo")
            .help("Generate all colored triangles, except for the logo")
            .short("C")
            .long("colored-no-logo"))

        .arg(Arg::with_name("outlined-all")
            .help("Generate all outlined triangles, and the logo")
            .short("o")
            .long("outlined-all"))

        .arg(Arg::with_name("outlined-no-logo")
            .help("Generate all outlined triangles, except for the logo")
            .short("O")
            .long("outlined-no-logo"))

        .subcommand(SubCommand::with_name("colored-all")
            .about("Generate all colored triangles, and the logo")
            .version("1.0")
            .author("M. Gage Morgan <gage@christoffen.com>")
            .arg(Arg::with_name("vestibular")
            .help("NOT NEEDED")))

        .subcommand(SubCommand::with_name("colored-no-logo")
            .about("Generate all colored triangles, except for the logo")
            .version("1.0")
            .author("M. Gage Morgan <gage@christoffen.com>")
            .arg(Arg::with_name("vestibular")
            .help("NOT NEEDED")))

        .subcommand(SubCommand::with_name("outlined-all")
            .about("Generate all outlined triangles, and the logo")
            .version("1.0")
            .author("M. Gage Morgan <gage@christoffen.com>")
            .arg(Arg::with_name("vestibular")
            .help("NOT NEEDED")))

        .subcommand(SubCommand::with_name("outlined-no-logo")
            .about("Generate all outlined triangles, except for the logo")
            .version("1.0")
            .author("M. Gage Morgan <gage@christoffen.com>")
            .arg(Arg::with_name("vestibular")
            .help("NOT NEEDED")))
        .get_matches();

    if matches.is_present("colored-all") {
        use controls::all_colored::*;
        all_colored();
    } else if matches.is_present("colored-no-logo") {
        use controls::colored_no_logo::*;
        colored_no_logo();
    } else if matches.is_present("outlined-all") {
        use controls::outlined_all::*;
        outlined_all();
    } else if matches.is_present("outlined-no-logo") {
        use controls::outlined_all::*;
        outlined_no_logo();
    } else {
        use ui::gtk_main;
        gtk_main();
    }
}

pub fn mkdir(dir: &str) -> std::io::Result<()> {
    fs::create_dir(&dir)?;
    Ok(())
}
