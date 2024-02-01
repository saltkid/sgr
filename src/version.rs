pub fn execute() {
    println!("{}", env!("CARGO_PKG_VERSION"));
}

pub fn help(verbose: bool) {
    println!("  sgr version");
}
