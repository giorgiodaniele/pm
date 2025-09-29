use std::{
    env::home_dir, fs, io::{self, Write}, path::PathBuf, process
};

use rpassword;

mod err;
mod models;

use err::AppError;
use models::vault::Vault;

fn main() {
    // Print a nice header once at startup
    print_header();

    if let Err(e) = run() {
        eprintln!("pm> {}", e);
    }
}

fn run() -> Result<(), AppError> {
    //
    // Path: ~/.pm/vault.json
    //
    let mut path = home_dir().ok_or_else(|| {
        AppError::IOError(std::io::Error::new(
            io::ErrorKind::NotFound,
            "Home directory not found",
        ))
    })?;
    path.push(".pm");
    path.push("vault.json");

    //
    // Ask for the password (hidden input)
    //
    let secret = rpassword::prompt_password("pm> enter secret: ")
        .map_err(AppError::IOError)?;

    //
    // If the keystore exists, open it
    // Otherwise, create a new one
    //
    let mut vault = if path.exists() {
        let v = Vault::load(&path, &secret)?;
        println!("pm> hello!");
        v
    } else {
        println!("pm> no vault found, creating a new one...");
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(AppError::IOError)?;
        }
        let mut v = Vault::new();
        v.save(&path, &secret)?;
        v
    };

    //
    // Command loop
    //
    let mut buffer = String::new();
    loop {
        print!("pm> ");
        io::stdout().flush().unwrap();

        buffer.clear();
        if io::stdin().read_line(&mut buffer).is_err() {
            continue;
        }

        let mut parts = buffer.trim().split_whitespace();
        match parts.next() {
            Some("add") => {
                let name = parts.next();
                let pass = parts.next();

                // Description may be a sequence of strings
                // each separated by a space. So, description
                // should be taken until it reaches the ENTER
                // key.
                let mut desc = String::new();
                while let Some(s) = parts.next() {
                    desc.push_str(s);
                    desc.push(' ');
                };

                match (name, pass, desc) {
                    (Some(name), Some(pass), desc) => {
                        vault.add_entry(
                            name.to_string(),
                            pass.to_string(),
                            desc.to_string(),
                        );
                        vault.save(&path, &secret)?;
                        println!("pm> added entry for {}", name);
                    }
                    _ => println!("pm> usage: add <name> <pass> [desc]"),
                }
            }
            Some("get") => {
                if let Some(s) = parts.next() {
                    match vault.get_entry(s) {
                        Some(entry) => {
                            println!("pm> name={}, pass={} desc={}", entry.name, entry.pass, entry.desc)
                        }
                        None => println!("pm> no entry found"),
                    }
                } else {
                    println!("pm> usage: get <name>");
                }
            }
            Some("list") => {
                let entries = vault.get_all();
                if entries.is_empty() {
                    println!("pm> empty");
                } else {
                    for entry in entries {
                        println!("pm> name={}, pass={} desc={}", entry.name, entry.pass, entry.desc)
                    }
                }
            }
            Some("exit") | Some("quit") => {
                vault.save(&path, &secret)?;
                println!("pm> goodbye!");
                break;
            }
            Some("help") => {
                print_help();
            }
            Some(cmd) => {
                println!("pm> {} is not a command", cmd);
                print_help();
            }
            None => {
                print_help();
            }
        }
    }

    Ok(())
}

fn print_help() {
    println!("pm> available commands:");
    println!("      add <name> <pass> [desc]   - add a new entry (e.g., 'add github.com mypassword')");
    println!("      get <name>                 - retrieve a password (e.g., 'get github.com')");
    println!("      list                       - list all entries (decrypted)");
    println!("      exit | quit                - exit the program");
}

fn print_header() {

    let n = 110;

    // Separator line
    println!("{}", "=".repeat(n));

    // Centered subtitle
    let title = "Password Manager (pm)";
    let width = n;
    let padding = (width - title.len()) / 2;
    println!("{}{}", " ".repeat(padding), title);

    println!("{}", "=".repeat(n));

    println!("add <name> <pass> [desc]   - add a new entry (e.g., 'add github.com mypassword')");
    println!("get <name>                 - retrieve a password (e.g., 'get github.com')");
    println!("list                       - list all entries (decrypted)");
    println!("help                       - list available commands");
    println!("exit | quit                - exit the program");

    // Closing separator
    println!("{}", "=".repeat(n));

    // One clean line with instructions
    // println!("Type 'help' for available commands");
}