use std::io::{self, Write};
use crate::model::store::Store;

mod model;
mod constant;

const GET_CMD  : &str = "get";
const DEL_CMD  : &str = "del";
const SET_CMD  : &str = "set";
const ALL_CMD  : &str = "all";
const EXT_CMD  : &str = "exit";
const HELP_CMD : &str = "help";

fn main() {
    let home = homedir::my_home()
        .expect("expected existing home dir")
        .unwrap();

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

        let pts: Vec<_>   = line.split_whitespace().collect();
        let cmd   = pts[0].to_lowercase();
        let args = &pts[1..];

        match cmd.as_str() {
            SET_CMD  => handle_set(&mut store, args),
            GET_CMD  => handle_get(&store, args),
            ALL_CMD  => handle_all(&store),
            DEL_CMD  => handle_del(&mut store, args),
            HELP_CMD => println!("{}", constant::HELPER),
            EXT_CMD  => {
                store.save();
                break;
            },
            _ => println!("error: unknown command '{}', use command 'help' for more information", cmd),
        }
    }
}

fn handle_set(store: &mut Store, args: &[&str]) {
    if args.len() != 3 {
        println!("error: correct syntax: set <name> <username> <password>");
        return;
    }

    let name = args[0];
    let user = args[1];
    let pass = args[2];

    if name.is_empty() || user.is_empty() || pass.is_empty() {
        println!("error: name and text are required");
        return;
    }

    store.add_secret(name.to_string(), user.to_string(), pass.to_string());
}

fn handle_get(store: &Store, args: &[&str]) {
    if args.len() != 1 {
        println!("error: correct syntax: get <name>");
        return;
    }

    let name = args[0];
    if name.is_empty() {
        println!("error: name is required");
        return;
    }

    match store.get_secret(name) {
        Some(secret) => println!("{}", secret),
        None => println!("error: {} not found", name),
    }
}

fn handle_all(store: &Store) {
    for secret in store.get_secrets() {
        println!("{}", secret);
    }
}

fn handle_del(store: &mut Store, args: &[&str]) {
    if args.len() != 1 {
        println!("error: correct syntax: del <name>");
        return;
    }

    let name = args[0];
    if name.is_empty() {
        println!("error: name is required");
        return;
    }

    store.del_secret(name.to_string());
}
