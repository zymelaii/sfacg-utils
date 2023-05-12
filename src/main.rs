use sfutils::cli::*;

use anyhow::Result;

fn main() -> Result<()> {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("auth", matches)) => match matches.subcommand() {
            Some(("login", matches)) => handle_auth_login(matches)?,
            Some(("logout", matches)) => handle_auth_logout(matches)?,
            Some(("status", matches)) => handle_auth_status(matches)?,
            Some(("refresh", matches)) => handle_auth_refresh(matches)?,
            _ => unreachable!(),
        },
        _ => unreachable!(),
    };

    Ok(())
}
