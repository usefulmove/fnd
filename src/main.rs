use std::env;

const RELEASE_STATE: &str = "a";

fn main() {
    // enable or disable backtrace on error
    env::set_var("RUST_BACKTRACE", "0");

    // get command arguments
    let mut args: Vec<String> = env::args().collect();

    // if no arguments are passed, behave as if help flag was passed
    if args.len() <= 1 {
        args.push("help".to_string());
    }

    println!("hola");
    println!("{:#?}", args);

    match args[1].as_str() {
        "--help" | "help" => {
            // display command usage information
            show_help();
            return;
        }
        _ => ()
    }

}

fn show_help() {
    //TODO
}
