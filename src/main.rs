use sfutils::Proxy;

use clap::{arg, Arg, ArgAction, Command};
use colored::*;

fn cli() -> Command {
    let auth = Command::new("auth")
        .about("Authenticate sfutils with boluobao")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("login")
                .about("Authenticate with a boluobao host")
                .arg(arg!(--account -u <ACCOUNT> "The account to authenticate with").required(true))
                .arg(
                    arg!(--password -p <PASSWORD> "The password to authenticate with")
                        .required(true),
                )
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("logout")
                .about("Log out of a boluobao host")
                .arg(arg!(--all -a "Logout all authenticated users"))
                .arg(arg!([USER]... "Users to logout"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("status")
                .about("View authentication status")
                .subcommand(Command::new("list").about("List authenticated users"))
                .subcommand(
                    Command::new("view")
                        .about("Display the information of an authenticated user")
                        .arg(arg!(<USER> "The user to display"))
                        .arg_required_else_help(true),
                )
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("refresh")
                .about("Refresh stored authentication credentials")
                .arg_required_else_help(true),
        );

    Command::new("sfutils")
        .about("An efficent yet powerful cli for boluobao")
        .version("0.1.0")
        .author("Zymelaii Ryer")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(auth)
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("auth", matches)) => match matches.subcommand() {
            Some(("login", matches)) => {
                let account = matches.get_one::<String>("account").expect("requires");
                let password = matches.get_one::<String>("password").expect("requires");

                let mut proxy = Proxy::default();
                if let Some(msg) = proxy.login(account, password) {
                    eprintln!("{}: {}", "Authentication failed".bold().red(), msg);
                    return;
                } else {
                    // TODO: store credential to local storage
                    println!("{proxy:#?}");
                }
            }
            Some(("logout", matches)) => {
                let users: Vec<&String> = if matches.get_flag("all") {
                    // TODO: get all local credentials
                    vec![]
                } else {
                    matches.get_many("USER").into_iter().flatten().collect()
                };
                for user in users {
                    // TODO: logout and clean up local storage
                }
            }
            Some(("status", matches)) => {
                match matches.subcommand() {
                    Some(("list", matches)) => {
                        // TODO: get all local credentials
                    }
                    Some(("view", matches)) => {
                        let user = matches.get_one::<String>("USER").unwrap();
                        // TODO: validate user and display informations
                    }
                    _ => unreachable!(),
                }
            }
            Some(("refresh", matches)) => {}
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}
