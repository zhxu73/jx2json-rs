mod ast;
mod jx_token;
mod parser;
mod scanner;
mod symbol_tab;

use std::{env, process::exit};

fn main() {
    let cli_args = parse_args();
    let tokens = match scanner::scan_file(&cli_args.filename) {
        Ok(tokens) => tokens,
        Err(err) => {
            eprintln!("{}", err);
            exit(-1);
        }
    };

    // print tokens
    if cli_args.print_tokens {
        for t in &tokens {
            println!("{}", t);
        }
    }

    let parser_result = match parser::parse_tokens(tokens) {
        Ok(result) => result,
        Err(err) => {
            eprintln!("{}", err);
            exit(-1);
        }
    };

    match parser_result.root.as_ref() {
        ast::AstNode::OBJECT(_) => {
            if cli_args.print_ast {
                println!("{}", parser_result.root);
            }
        }
        _ => eprintln!("ast root is not not object"),
    }
}

struct CLIArgs {
    filename: String,
    print_tokens: bool,
    print_ast: bool,
    json_gen: bool,
    debug: bool,
}

fn parse_args() -> CLIArgs {
    let args: Vec<String> = env::args().collect();
    if args[1..].is_empty() {
        eprintln!("no args supplied");
        print_help();
        exit(-1)
    }
    let mut cli_args = CLIArgs {
        filename: String::from(""),
        print_tokens: false,
        print_ast: false,
        json_gen: true,
        debug: false,
    };
    for arg in &args[1..] {
        match arg.as_str() {
            "--help" | "-h" => print_help(),
            "--print-tokens" => cli_args.print_tokens = true,
            "--print-ast" => cli_args.print_ast = true,
            "--json-gen" => cli_args.json_gen = true,
            "--debug" => cli_args.debug = true,
            // filename
            _ => cli_args.filename = arg.clone(),
        };
    }
    if cli_args.filename.is_empty() {
        eprintln!("needs filename");
        exit(-1);
    }
    cli_args
}

fn print_help() {
    println!("jx2json [--help|-h] [jx-filename]");
}
