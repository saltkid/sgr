fn main() {
    let raw_args: Vec<String> = std::env::args().collect();

    if raw_args.len() < 2 {
        let res = run();
        match res {
            Ok(_) => {}
            Err(e) => println!("{}", e),
        }
        return;
    }

    let command = &raw_args[1];
    let args = &raw_args[2..];

    let result = match command.as_str() {
        "add" => add(args),
        "remove" => remove(args),
        "list" => list(args),
        _ => Err("unknown command".to_string()),
    };

    match result {
        Ok(_) => {}
        Err(e) => println!("{}", e),
    }
}

fn run() -> Result<(), String> {
    println!("ran run");
    return Ok(());
}
fn add(args: &[String]) -> Result<(), String> {
    println!("ran add with {:?}", args);
    return Ok(());
}

fn remove(args: &[String]) -> Result<(), String> {
    println!("ran remove with {:?}", args);
    return Ok(());
}

fn list(args: &[String]) -> Result<(), String> {
    println!("ran list with {:?}", args);
    return Ok(());
}
