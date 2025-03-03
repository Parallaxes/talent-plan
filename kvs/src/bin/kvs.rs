use clap::Command;
use std::process::exit;

fn main() {
    let matches = Command::new("kvs")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            Command::new("get")
                .about("Get a value from the store")
                .arg(clap::Arg::new("key").required(true)),
        )
        .subcommand(
            Command::new("set")
                .about("Set a value in the store")
                .arg(clap::Arg::new("key").required(true))
                .arg(clap::Arg::new("value").required(true)),
        )
        .subcommand(
            Command::new("rm")
                .about("Remove a value from the store")
                .arg(clap::Arg::new("key").required(true)),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("set", _matches)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        Some(("get", _matches)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        Some(("rm", _matches)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        _ => unreachable!(),
    }
}
