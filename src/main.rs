mod lib;
mod cli;
mod kube;

use clap::{App, load_yaml};
use termion::color;

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();

    let res = match matches.subcommand() {
        Some(("generate", args)) => cli::generate::run(args),
        Some(("diff", args)) => cli::diff::run(args),
        _ => Ok(())
    };

    if let Err(err) = res {
        println!("{} an error occurred: {}", color::Fg(color::Red), err.message);
    }
}
