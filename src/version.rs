use crate::utils::StringExt;

pub fn execute() {
    println!("{}", env!("CARGO_PKG_VERSION"));
}

pub fn help(verbose: bool) {
    let title = match verbose {
        true => "version"
            .to_string()
            .pad_right(15)
            .fill_left(2)
            .bold()
            .underline(),
        false => "version".to_string().pad_right(15).fill_left(2).bold(),
    };

    println!(
        "{}{}{}",
        title,
        "prints the current version of",
        " sgr".to_string().bold()
    );
    if verbose {
        println!(
            "\n{}{}",
            "Usage:"
                .to_string()
                .pad_right(15)
                .bold()
                .underline()
                .fill_left(2),
            "sgr version".to_string()
        );
    }
}
