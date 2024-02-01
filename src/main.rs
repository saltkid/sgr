// commands
mod add;
mod help;
mod list;
mod remove;
mod run;
mod version;

// helper functions
mod utils;
use utils::{logln, LogLevel};

fn main() {
    let raw_args: Vec<String> = std::env::args().collect();
    let res = parse_args(&raw_args);
    let (command, arg) = match res {
        Ok(res) => res,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    match command.execute(arg) {
        Ok(res) => println!("{}", res),
        Err(e) => println!("{}", e),
    }
}

fn parse_args(raw_args: &[String]) -> Result<(CMD, Option<&str>), String> {
    if raw_args.len() < 2 {
        return Ok((CMD::Run, None));
    }

    // validate command
    let cmd = match to_command(raw_args[1].as_str()) {
        Ok(cmd) => cmd,
        Err(e) => return Err(e),
    };

    // warn user of unused args
    let args = &raw_args[2..];
    if args.len() > 1 {
        logln(
            LogLevel::Warn,
            format!("args '{:?}' will be unused", args[1..].to_vec()),
        );
    }

    let arg = match args.is_empty() {
        true => None,
        false => Some(args[0].as_str()),
    };

    return Ok((cmd, arg));
}

// add commands here
pub enum CMD {
    Run,
    Add,
    Remove,
    List,
    Help,
    Version,
}

// strategy pattern baby!
impl CMD {
    fn help(&self, verbose: bool) {
        match self {
            CMD::Run => {} // impossible
            CMD::Add => add::help(verbose),
            CMD::Remove => remove::help(verbose),
            CMD::List => list::help(verbose),
            CMD::Help => help::help(verbose),
            CMD::Version => version::help(verbose),
        }
    }

    fn execute(&self, arg: Option<&str>) -> Result<String, String> {
        let mut res: Result<(), String> = Ok(());
        match self {
            // only one with success message: the path to cd to
            CMD::Run => {
                return run::execute();
            }

            // no success messages
            CMD::Version => version::execute(),
            CMD::Help => res = help::execute(arg),
            CMD::Add => res = add::execute(arg),
            CMD::Remove => res = remove::execute(arg),
            CMD::List => res = list::execute(arg, None),
        };

        match res {
            Ok(_) => Ok("".to_string()),
            Err(e) => Err(e),
        }
    }
}

fn to_command(command: &str) -> Result<CMD, String> {
    match command {
        "add" => Ok(CMD::Add),
        "remove" => Ok(CMD::Remove),
        "list" => Ok(CMD::List),
        "help" => Ok(CMD::Help),
        "version" => Ok(CMD::Version),
        _ => Err(format!("unknown command '{}'", command)),
    }
}
