use regex::Regex;
use std::env;
use std::path::Path;
use walkdir::{WalkDir, DirEntry};

mod poc;

const RELEASE_STATE: &str = "e";

fn main() {
    // enable or disable backtrace on error
    env::set_var("RUST_BACKTRACE", "0");

    let mut mode: SearchMode = SearchMode::Simple;

    // get command arguments
    let mut args: Vec<String> = env::args().collect();

    //println!("{:#?}", args); // debug

    if args.len() <= 1 { // no arguments
        for entry in WalkDir::new("./") {
            let entry: DirEntry = entry.unwrap();
            let path: &Path = entry.path();
            let path_str: &str = path.to_str().unwrap();
            println!(
                "  {}",
                poc::highlight_filename(path_str),
            );
        }
        return;
    }

    match args[1].as_str() {
        "--help" => {
            // display command usage information
            show_help();
            return;
        }
        "--version" => {
            // display version information
            println!(
                "  {} {}{}",
                poc::color_grey_mouse("fnd"),
                poc::color_blue_smurf_bold(env!("CARGO_PKG_VERSION")),
                poc::color_white_bold(RELEASE_STATE),
            );
            return;
        }
        "--regex" | "-e" => {
            // regular expression search
            mode = SearchMode::Regex;

            // remove flag argument
            args.remove(1);
        }
        _ => ()
    }

    /* fnd core */
    match mode {
        SearchMode::Simple => {
            /* simple search */
            let search_term: String = args[1].clone();

            for entry in WalkDir::new("./") {
                let entry: DirEntry = entry.unwrap();
                let path: &Path = entry.path();
                let path_str: &str = path.to_str().unwrap();
                if path_str.contains(&search_term) {
                    //println!("  {}", path_str); // debug
                    println!(
                        "  {}",
                        poc::highlight(
                            path_str,
                            &search_term,
                            poc::color_blue_smurf_bold,
                        ),
                    );
                }
            }
        }
        SearchMode::Regex => {
            /* regular expression ( regex ) search */
            let reg_exp: String = args[1].clone();

            match Regex::new(&reg_exp) {
                Ok(re) => {
                    for entry in WalkDir::new("./") {
                        let entry: DirEntry = entry.unwrap();
                        let path: &Path = entry.path();
                        let path_str: &str = path.to_str().unwrap();

                        //println!("  path:{}", path_str); // debug remove

                        let matching_term: String = match re.captures(path_str) {
                            //Some(n) => { println!("  n:{:#?}", n); n[0].to_string() } // debug remove
                            Some(n) => n[0].to_string(),
                            None => "".to_string(),
                        };

                        if matching_term != "".to_string() {
                            //println!("  {}", path_str); // debug
                            println!(
                                "  {}",
                                poc::highlight(
                                    path_str,
                                    &matching_term,
                                    poc::color_blue_smurf_bold,
                                ),
                            );
                        }
                    }
                }
                Err(e) => {
                    println!("  error!: {}", e);
                }
            }
        }
        SearchMode::Glob => {
            println!("  file glob mode under construction"); // debug remove
        }
    }


    std::process::exit(0);
}

fn show_help() {
    //TODO
    println!("  ( oh! hola. need halp? )");
}

enum SearchMode {
    Glob,
    Regex,
    Simple,
}