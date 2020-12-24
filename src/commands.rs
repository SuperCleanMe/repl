mod exit;
mod use_db;
mod meminfo;

pub fn parse(input: String, loc: &String) -> String {
    match input.to_lowercase().as_str() {
        "exit" => {
            exit::exit(0);
        }
        _ => {
            println!("\x1b[32m{}\x1b[0m > \x1b[31mError\x1b[0m > Unkown command '{}'", loc, input)
        }
    }

    loc.clone()
}