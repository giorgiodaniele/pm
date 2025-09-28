use std::{fs, path::PathBuf, io::{self, Write}, process};
use homedir;

mod err;
mod models;
use err::AppError;
use models::vault::SecretStore;

fn main() {
    // Print a nice header once at startup
    print_header();

    if let Err(e) = run() {
        eprintln!("pm> {}", e);
        process::exit(1);
    }
}

fn run() -> Result<(), AppError> {
    //
    // Path: ~/.pm/vault.json
    //
    let mut path = homedir::my_home().unwrap().ok_or(AppError::IOError(io::Error::new(
        io::ErrorKind::NotFound,
        "Could not determine home directory",
    )))?;
    path.push(".pm");
    path.push("vault.json");

    //
    // Ask for the password
    //
    print!("pm> enter secret: ");
    io::stdout().flush().unwrap();

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).map_err(AppError::IOError)?;
    let secret = buffer.trim().to_string();

    //
    // If the keystore exists, open it
    // Otherwise, create a new one
    //
    let mut vault = if path.exists() {
        let v = SecretStore::load(&path, &secret)?;
        println!("pm> hello!");
        v
    } else {
        println!("pm> no vault found, creating a new one...");
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(AppError::IOError)?;
        }
        let mut v = SecretStore::new();
        v.save(&path, &secret)?;   // <-- creates an empty encrypted vault
        v
    };

    //
    // Command loop
    //
    loop {
        print!("pm> ");
        io::stdout().flush().unwrap();

        buffer.clear();
        if io::stdin().read_line(&mut buffer).is_err() {
            continue;
        }

        let input = buffer.trim();
        let mut parts = input.split_whitespace();
        let cmd = parts.next();

        match cmd {
            Some("add") => {
                let name = parts.next();
                let pass = parts.next();
                match (name, pass) {
                    (Some(name), Some(password)) => {
                        vault.add_entry(name.to_string(), password.to_string());
                        println!("pm> added entry for {}", name);
                    }
                    _ => println!("pm> usage: add <name> <pass>"),
                }
            }
            Some("get") => {
                if let Some(s) = parts.next() {
                    match vault.get_entry(s) {
                        Some(entry) => println!("pm> name={}, pass={}", entry.name, entry.password),
                        None => println!("pm> empty"),
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
                        println!("name={}, pass={}", entry.name, entry.password);
                    }
                }
            }
            Some("exit") | Some("quit") => {
                vault.save(&path, &secret)?;
                println!("pm> goodbye!");
                break;
            }
            Some(cmd) => {
                vault.save(&path, &secret)?;
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
    println!("      add <name> <pass>   - add a new entry (e.g., 'add github.com mypassword')");
    println!("      get <name>          - retrieve a password (e.g., 'get github.com')");
    println!("      list                - list all entries (decrypted)");
    println!("      exit | quit         - exit the program");
}

fn print_header() {
    // Separator line
    println!("\n{}", "=".repeat(40));

    // Centered subtitle
    let title = "Simple Password Manager (pm)";
    let width = 40;
    let padding = (width - title.len()) / 2;
    println!("{}{}", " ".repeat(padding), title);

    // Closing separator
    println!("{}", "=".repeat(40));
    println!();
}
