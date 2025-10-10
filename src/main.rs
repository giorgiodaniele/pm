use std::io::{self, Write};
use crate::model::store::Store;

mod model;
mod parser;
mod constant;

fn main() {

    // Inspect home dir
    let home = homedir::my_home()
        .expect("expected existing home dir")
        .unwrap();

    // Generate or load the store from disk
    let fold = home.join(".pm");
    let file = fold.join("secrets.json");
    let mut store = Store::open(file);


    println!("{}", constant::HEADER);

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        if io::stdin().read_line(&mut line).is_err() {
            println!("error: failed to read from stdin");
            continue;
        }

        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // Parse command using struct Parser
        match parser::Parser::parse(line) {
            Ok(cmd) => match cmd {
                parser::Command::Get { name } => {
                    match store.get_secret(&name) {
                        Some(secret) => println!("{}", secret),
                        None => println!("error: {} not found", name),
                    }
                },
                parser::Command::Set { name, username, password } => {
                    store.add_secret(name.to_string(), username.to_string(), password.to_string());
                },
                parser::Command::Del { name } => {
                    store.del_secret(name.to_string())
                },
                parser::Command::All => {
                    for secret in store.get_secrets() {
                        println!("{}", secret);
                    }
                },
                parser::Command::Exit => {
                    store.save();
                    break;
                },
                parser::Command::Help => println!("{}", constant::HELPER),
            },
            Err(err) => println!("{}", err),
        }
    }
}
