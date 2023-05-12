use sfutils::Proxy;

use anyhow::{bail, Result};
use clap::{arg, Command};
use colored::*;
use directories::ProjectDirs;
use std::fs;
use std::io::Read;

fn cli() -> Command {
    let auth = Command::new("auth")
        .about("Authenticate sfutils with boluobao")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("login")
                .about("Authenticate with a boluobao host")
                .arg(arg!(--username -U <USERNAME> "The user to authenticate"))
                .arg(
                    arg!(--account -u <ACCOUNT> "The account to authenticate with")
                        .required_unless_present("username"),
                )
                .arg(
                    arg!(--password -p <PASSWORD> "The password to authenticate with")
                        .required_unless_present("username"),
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

fn local_storage() -> Result<toml::Table> {
    let dirs = ProjectDirs::from("", "", "sfutils").unwrap();
    let data_dir = dirs.data_local_dir();
    if !data_dir.exists() {
        fs::create_dir_all(data_dir)?;
    }

    let data_file = data_dir.join("auth.toml");
    let mut file = fs::OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(data_file)?;

    let mut buffer = String::default();
    file.read_to_string(&mut buffer)?;

    Ok(buffer.parse::<toml::Table>().unwrap())
}

fn get_authenticated_users() -> Result<Vec<String>> {
    Ok(local_storage()?.keys().into_iter().cloned().collect())
}

fn cleanup_auth() {
    let _ = fs::remove_file(
        ProjectDirs::from("", "", "sfutils")
            .unwrap()
            .data_local_dir()
            .join("auth.toml"),
    );
}

fn update_auth(profile: &sfutils::api::types::User, password: &str) -> Result<String> {
    let mut data = local_storage()?;
    let mut auth = toml::Table::new();

    auth.insert(
        "email".to_string(),
        toml::Value::from(profile.email.to_owned()),
    );

    auth.insert(
        "phone".to_string(),
        toml::Value::from(profile.phoneNum.to_owned()),
    );

    auth.insert(
        "password".to_string(),
        toml::Value::from(password.to_owned()),
    );

    data.insert(profile.nickName.to_owned(), toml::Value::from(auth));

    let data_file = ProjectDirs::from("", "", "sfutils")
        .unwrap()
        .data_local_dir()
        .join("auth.toml");
    fs::write(data_file, data.to_string())?;

    Ok(profile.nickName.to_owned())
}

fn remove_auth(users: &Vec<String>) -> Result<()> {
    let mut data = local_storage()?;

    users.iter().for_each(|user| {
        data.remove(user);
    });

    let data_file = ProjectDirs::from("", "", "sfutils")
        .unwrap()
        .data_local_dir()
        .join("auth.toml");
    fs::write(data_file, data.to_string())?;

    Ok(())
}

fn get_secrets_of(username: &str) -> Result<(String, String)> {
    match local_storage()?.get(username) {
        Some(value) => {
            let account = value.get("email").unwrap().as_str().unwrap().to_string();
            let password = value.get("password").unwrap().as_str().unwrap().to_string();
            Ok((account, password))
        }
        None => bail!("unknown user"),
    }
}

fn main() -> Result<()> {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("auth", matches)) => match matches.subcommand() {
            Some(("login", matches)) => {
                let account: String;
                let password: String;
                if let Some(username) = matches.get_one::<String>("username") {
                    match get_secrets_of(&username) {
                        Ok(secrets) => {
                            (account, password) = secrets;
                        }
                        Err(_) => {
                            let hint = "Unknown user to be authenticated";
                            eprintln!("{}: {username}", hint.bold().red());
                            bail!(hint.to_lowercase());
                        }
                    }
                } else {
                    account = matches.get_one::<String>("account").unwrap().to_owned();
                    password = matches.get_one::<String>("password").unwrap().to_owned();
                }

                let mut proxy = Proxy::default();
                if let Some(msg) = proxy.login(&account, &password) {
                    let hint = "Authentication failed";
                    eprintln!("{}: {msg}", hint.bold().red());
                    bail!(hint);
                } else if let Ok(profile) = proxy.profile() {
                    let user = update_auth(&profile, &password).unwrap();
                    println!("Logged in to boluobao as {}", user.bold());
                } else {
                    println!("{proxy:#?}");
                }
            }
            Some(("logout", matches)) => {
                if matches.get_flag("all") {
                    cleanup_auth()
                } else {
                    let users: Vec<String> = matches.get_many("USER").unwrap().cloned().collect();
                    let _ = remove_auth(&users);
                };
            }
            Some(("status", matches)) => match matches.subcommand() {
                Some(("list", _)) => {
                    for user in get_authenticated_users().unwrap() {
                        println!("{}", user.bold())
                    }
                }
                Some(("view", matches)) => {
                    let user = matches.get_one::<String>("USER").unwrap();
                    let users = get_authenticated_users().unwrap();
                    if users.contains(user) {
                        let (account, password) = get_secrets_of(&user).unwrap();
                        let mut proxy = Proxy::default();
                        proxy.login(&account, &password);
                        println!("{} {:#?}", user.bold(), proxy.profile().unwrap());
                    } else {
                        let hint = "Unknown user";
                        eprintln!("{}: {}", hint.bold().red(), user);
                        bail!(hint);
                    }
                }
                _ => unreachable!(),
            },
            Some(("refresh", _)) => {}
            _ => unreachable!(),
        },
        _ => unreachable!(),
    };

    Ok(())
}
