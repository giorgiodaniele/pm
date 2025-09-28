use std::{fs, path::PathBuf, io::{self, Write}, process};
use std::env::home_dir;
use sha2::{Sha256, Digest};

mod err;
mod models;
use err::AppError;
use models::vault::SecretStore;

fn main() {
    if let Err(e) = run() {
        eprintln!("pm> {}", e);
        process::exit(1);
    }
}

fn run() -> Result<(), AppError> {

    // Path: ~/.pm/vault.json
    let mut path = PathBuf::from(home_dir().unwrap());
    path.push(".pm");
    path.push("vault.json");

    // Ask for the master password
    print!("pm> enter your master password: ");
    io::stdout().flush().unwrap();

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).map_err(AppError::IOError)?;
    let pass = buffer.trim().to_string();
    let hash = Sha256::digest(pass.as_bytes());

    // If the keystore exists, password from user is
    // going to be used to enter it

    let mut vault = if path.exists() {
        let v = SecretStore::read(&path, pass)?;
        println!("pm> hello!");
        v
    } else {

        // If the keystore does not exist, password from
        // user is going to be uses to setup master 
        // password, which is the only secret the user 
        // is supposed to know

        println!("pm> no vault found, creating a new one...");
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(AppError::IOError)?;
        }
        let v = SecretStore::new(hash.into());
        v.write(&path)?;
        v
    };

    loop {
        // Await for a command
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
                    (Some(s), Some(p)) => {
                        vault.insert(s.to_string(), p.to_string()).write(&path)?;
                        println!("pm> added entry for {}", s);
                    }
                    _ => {
                        println!("pm> usage: add <name> <pass>")
                    }
                }
            }
            Some("get") => {
                if let Some(s) = parts.next() {
                    match vault.search(s) {
                        Some(entry) => {
                            println!("pm> name={}, pass={}", entry.name, entry.pass)
                        }
                        None => {
                            println!("pm> empty")
                        }
                    }
                } else {
                    println!("pm> usage: get <name>");
                }
            }
            Some("list") => {
                let tot = vault.vals.iter().count();
                if tot == 0 {
                    println!("pm> empty")
                    
                } else {
                    vault.vals.iter().for_each(|entry| {
                        println!("name={}, pass={}", entry.name, entry.pass);
                    });
                }
            }
            Some("exit") | Some("quit") => {
                println!("pm> goodbye!");
                break;
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
    println!("      add <name> <pass>   - add a new entry (e.g., ''add github.com hellofans'')");
    println!("      get <name>          - retrieve a password (e.g., ''get github.com'')");
    println!("      list                - list all entries");
    println!("      exit | quit         - exit the program");
}
