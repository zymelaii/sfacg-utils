use sfutils::Proxy;

use clap::{arg, Command};
use colored::*;

fn cli() -> Command {
    Command::new("sfutils")
        .about("An efficent yet powerful cli for boluobao")
        .version("0.1.0")
        .author("Zymelaii Ryer")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("auth")
                .about("Authenticate sfutils with boluobao")
                .arg(arg!(--account -u <ACCOUNT> "The account to authenticate with"))
                .arg(arg!(--username -U <USERNAME> "The username to authenticate with"))
                .arg(arg!(--password -p <PASSWORD> "The password to authenticate with"))
                .arg_required_else_help(true),
        )
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("auth", submatches)) => {
            let account = submatches.get_one::<String>("account").expect("requires");
            let password = submatches.get_one::<String>("password").expect("requires");

            let mut proxy = Proxy::default();
            if let Some(msg) = proxy.login(account, password) {
                eprintln!("{}: {}", "Authentication failed".bold().red(), msg);
                return;
            } else {
                println!("{proxy:#?}");
            }
        }
        _ => unreachable!(),
    }
}
