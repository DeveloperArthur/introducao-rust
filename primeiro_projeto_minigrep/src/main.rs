/* trazendo o código que está em lib.rs no escopo de crate binária em main.rs */
extern crate primeiro_projeto_minigrep;

use std::env;
use std::process;
use primeiro_projeto_minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = primeiro_projeto_minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

/* rode o programa com estes comandos:
cargo run <palavra para buscar> poema.txt

ou CASE_INSENSITIVE=1 cargo run <palavra para buscar> poema.txt
caso queira com case insensitive
 */