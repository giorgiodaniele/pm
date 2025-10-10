pub const HELPER : &str = r#"
Usage
  pm <command> [args]

Command:
  get: use this to get a secret
  set: SET use this to set a new secret
  del: DEL use this to delete a secret
  all: ALL use this for list all secrets

Synopsis:
  get <name>
  set <name> <username> <password>
  del <name>
  all

"#;

pub const HEADER : &str = r#"
---------------------------------------
|       Password Manager v0.0.1       |
---------------------------------------
  Examples:
    set www.example.com balenablu 133434
    get www.example.com
  Use 'help' for more info
"#;