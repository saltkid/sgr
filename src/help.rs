use crate::{add, list, remove, to_command, utils::StringExt, version};

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
            // H = cursor to top left, 2J = clear screen
            print!("\x1B[H\x1B[2J");
            println!(
                "{} Search Git Repos",
                "sgr:".to_string().pad_right(16).bold().underline()
            );
            print!(
                "{} ",
                "Version:".to_string().pad_right(16).bold().underline()
            );
            version::execute();
            println!(
                "{} {} {} {}\n",
                "Usage:".to_string().pad_right(16).bold().underline(),
                "sgr".to_string().bold(),
                "<optional_command>".to_string().italic(),
                "<optional_arg>".to_string().italic()
            );
            println!("{}: ", "Optional Commands".to_string().bold().underline());
            add::help(false);
            remove::help(false);
            list::help(false);
            help(false);
            version::help(false);
        }
        Some(command) => {
            // H = cursor to top left, 2J = clear screen
            println!("\x1B[H\x1B[2J");
            command.help(true)
        }
    }

    Ok(())
}

pub fn help(verbose: bool) {
    let title = match verbose {
        true => "help"
            .to_string()
            .pad_right(15)
            .fill_left(2)
            .bold()
            .underline(),
        false => "help".to_string().pad_right(15).fill_left(2).bold(),
    };

    println!("{}{}", title, "prints this help message or the verbose");
    println!(
        "{}{}\n",
        "".to_string().pad_right(15).fill_left(2),
        "help message for a specific command if specified"
    );
    if verbose {
        println!("\n{}", "Usage:".to_string().bold().underline().fill_left(2));
        println!("{}", "sgr help".to_string().fill_left(17));
        println!("{}", "sgr help add".to_string().fill_left(17));
        println!("{}", "sgr help help".to_string().fill_left(17));
    }
}
