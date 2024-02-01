use crate::{add, list, remove, to_command, version};

pub fn execute(arg: Option<&str>) -> Result<(), String> {
    let command = match arg {
        Some(arg) => {
            let res = to_command(arg);
            match res {
                Ok(res) => Some(res),
                Err(err) => return Err(err),
            }
        }
        None => None,
    };

    match command {
        None => {
            println!("sgr: Search Git Repos");
            print!("Version: ");
            version::execute();
            println!("Usage: sgr <optional_command> <optional_arg>");
            println!("Optional Commands: ");
            add::help(false);
            remove::help(false);
            list::help(false);
        }
        Some(command) => {
            // H = cursor to top left, 2J = clear screen
            // println!("\x1B[H\x1B[2J");
            command.help(true)
        }
    }

    Ok(())
}

pub fn help(verbose: bool) {
    todo!()
}
