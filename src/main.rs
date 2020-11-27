mod jx_token;
mod scanner;

fn main() {
    let filename = String::from("test.jx");
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
