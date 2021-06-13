mod lib;
mod cli;
mod kube;

use clap::{App, load_yaml};
use termion::color;
use cli::helper::error::CError;

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let mut app = App::from(yaml);
    let matches = app.clone().get_matches();

    let res = match matches.subcommand() {
        Some(("generate", args)) => cli::generate::run(args),
        Some(("diff", args)) => cli::diff::run(args),
        Some(("verify", args)) => cli::dry::run(args),
        _ => app.print_help().map_err(CError::from)
    };

    if let Err(err) = res {
        println!("{}❌ Something went wrong: {}", color::Fg(color::Red), err.message);
    }
}
