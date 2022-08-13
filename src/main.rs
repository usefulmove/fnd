use::colored::*;
use std::env;
use std::fs;
use std::path::Path;

const RELEASE_STATE: &str = "a";

fn main() {
    // enable or disable backtrace on error
    env::set_var("RUST_BACKTRACE", "0");

    // get command arguments
    let mut args: Vec<String> = env::args().collect();

    // if no arguments are passed, behave as if help flag was passed : TODO - remove to allow "fnd" same as "fnd ."?
    if args.len() <= 1 {
        args.push("help".to_string());
    }

    println!("{:#?}", args);

    match args[1].as_str() {
        "--help" | "help" => {
            // display command usage information
            show_help();
            return;
        }
        "--version" | "version" => {
            // display version information
            println!("{}{}", env!("CARGO_PKG_VERSION"), RELEASE_STATE);
            return;
        }
        _ => ()
    }

    println!("execute fnd here: ( % fnd {} )", args[1]);

    // 0.2.0 - simple search
    let search_term: String = args[1].clone();

    println!("simple search: {}", search_term);





}

fn show_help() {
    //TODO
    println!("\"( oh! hola. need some halp? )\"");
}
