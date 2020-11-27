mod jx_token;
mod scanner;

use jx_token::Token;
use scanner::scan_token;

fn main() {
    let filename = String::from("test.jx");
    let result = scanner::scan_file(&filename);

    match result {
        Ok(tokens) => {
            for t in tokens {
                println!("{}", t.to_str());
            }
        }
        Err(err) => println!("{}", err),
    }
}
