use core::fmt;

pub struct Parser;

const GET_CMD  : &str = "get";
const DEL_CMD  : &str = "del";
const SET_CMD  : &str = "set";
const ALL_CMD  : &str = "all";
const EXT_CMD  : &str = "exit";
const HELP_CMD : &str = "help";

pub enum Command {
    Get {
        /// An ID associated to the secret
        name: String,
    },
    Set {
        /// An ID associated to the secret
        name: String,
        /// Username associated to the secret
        username: String,
        // Password associated to the secret
        password: String,
    },
    Del {
        /// An ID associated to the secret
        name: String,
    },
    All,
    Exit,
    Help,
}

#[derive(Debug)]
pub enum ParserError {
    UnknownCommand,
    MissingArgument,
    InvalidArgument,
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::UnknownCommand  => write!(f, "unknown command"),
            ParserError::MissingArgument => write!(f, "missing argument"),
            ParserError::InvalidArgument => write!(f, "invalid argument"),
        }
    }
}

impl Parser {
    pub fn parse(input: &str) -> Result<Command, ParserError> {

        let pts: Vec<_>   = input.split_whitespace().collect();
        let cmd   = pts[0].to_lowercase();
        let args = &pts[1..];

        match cmd.as_str() {
            GET_CMD  => {
                if args.len() != 1 { 
                    return Err(ParserError::MissingArgument);
                }
                let name = args[0];
                if name.is_empty() {
                    return Err(ParserError::InvalidArgument);
                }
                return Ok(Command::Get { name: name.to_string() })
            },
            DEL_CMD  => {
                if args.len() != 1 { 
                    return Err(ParserError::MissingArgument);
                }
                let name = args[0];
                if name.is_empty() {
                    return Err(ParserError::InvalidArgument);
                }
                return Ok(Command::Del { name: name.to_string() })
            },
            SET_CMD  => {
                if args.len() != 3 { 
                    return Err(ParserError::MissingArgument);
                }
                let name = args[0];
                let user = args[1];
                let pass = args[2];
                if name.is_empty() || user.is_empty() || pass.is_empty() {
                    return Err(ParserError::InvalidArgument);
                }
                return Ok(Command::Set { 
                    name:     name.to_string(), 
                    username: user.to_string(), 
                    password: pass.to_string() })
            }
            ALL_CMD  => return Ok(Command::All),
            EXT_CMD  => return Ok(Command::Exit),
            HELP_CMD => return Ok(Command::Help),
            _        => return Err(ParserError::UnknownCommand),
        }
    }
}
