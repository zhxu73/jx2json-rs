mod jx_token;
mod scanner;

use std::{env, process::exit};

fn main() {
    let filename = parse_args();
    let result = scanner::scan_file(&filename);

    match result {
        Ok(tokens) => {
            for t in tokens {
                println!("{}", t);
            }
        }
        Err(err) => println!("{}", err),
    }
}

fn parse_args() -> String {
    let args: Vec<String> = env::args().collect();
    if args[1..].is_empty() {
        eprintln!("no args supplied");
        print_help();
        exit(-1)
    }
    for arg in &args[1..] {
        match arg.as_str() {
            "--help" | "-h" => print_help(),
            // filename
            _ => return arg.clone(),
        };
    }
    eprintln!("needs filename");
    exit(-1);
}

fn print_help() {
    println!("jx2json [--help|-h] [jx-filename]");
}
