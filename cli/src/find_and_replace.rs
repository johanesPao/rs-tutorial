use std::env;
use text_colorizer::*;

#[derive(Debug)]
#[allow(dead_code)]
struct Arguments {
    pattern: String,
    replace: String,
    input_file: String,
    output_file: String,
}

fn print_help() {
    eprintln!(
        "{} - replace a string with a new string",
        "Find and Replace".green()
    );
    eprintln!("Usage: <target string> <replacement string> <INPUT FILE> <OUTPUT FILE>");
}

fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 4 {
        print_help();
        eprintln!(
            "{} wrong number of arguments given. Expected 4, got {}",
            "Error".red().bold(),
            args.len()
        );
        std::process::exit(1);
    }

    Arguments {
        pattern: args[0].clone(),
        replace: args[1].clone(),
        input_file: args[2].clone(),
        output_file: args[3].clone(),
    }
}

pub fn run() {
    let args = parse_args();
    println!("{:?}", args)
}
