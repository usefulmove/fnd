use regex::Regex;
use std::env;
use std::path::Path;
use ignore::{WalkBuilder, DirEntry};

mod poc;

const RELEASE_STATE: &str = "h";

fn main() {
    // enable or disable backtrace on error
    env::set_var("RUST_BACKTRACE", "0");

    // set default search mode
    let mode: SearchMode = SearchMode::Regex;

    // don't show hidden files by default
    let mut ignore_hidden: bool = true;

    // get command arguments
    let mut args: Vec<String> = env::args().collect();

    //println!("{:#?}", args); // debug
    let col = poc::Theme::new();

    // if no arguments are passed, behave as if help flag was passed
    /*
    if args.len() <= 1 {
        args.push("--help".to_string());
    }
    */

    match args[1].as_str() {
        "--help" => {
            // display command usage information
            show_help();
            return;
        }
        "--hidden" | "-h" => {
            // show hidden files
            ignore_hidden = false;

            // remove flag argument
            args.remove(1);
        }
        "--version" => {
            // display version information
            println!(
                "  {} {}{}",
                col.color_rgb("fnd", &col.grey_mouse),
                col.color_rgb(env!("CARGO_PKG_VERSION"), &col.blue_smurf_bold),
                col.color_rgb(RELEASE_STATE, &col.white_bold),
            );
            return;
        }
        _ => ()
    }

    /* fnd core */
    match mode {
        SearchMode::Regex => {
            /* regular expression ( regex ) search */
            let reg_exp: String = args[1].clone();

            match Regex::new(&reg_exp) {
                Ok(re) => {
                    for entry in WalkBuilder::new("./").hidden(ignore_hidden).build() {
                        let entry: DirEntry = entry.unwrap();
                        let path: &Path = entry.path();
                        let path_str: &str = path.to_str().unwrap();

                        //println!("  debug:path:{}", path_str);

                        let no_match: String = "".to_string();

                        let matching_term: String = match re.captures(path_str) {
                            Some(match_return) => match_return[0].to_string(),
                            None => no_match.clone(),
                        };

                        if matching_term != no_match {
                            //println!("  debug:{}", col.color_rgb(path_str, &col.blue_smurf_bold));
                            println!(
                                "  {}",
                                poc::highlight(
                                    path_str,
                                    &matching_term,
                                    &col.blue_smurf_bold,
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
        /*
        SearchMode::Simple => {
            /* simple search */
            let search_term: String = args[1].clone();

            for entry in WalkBuilder::new("./").hidden(ignore_hidden).build() {
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
                            &col.blue_smurf_bold,
                        ),
                    );
                }
            }
        }
        */
    }


    std::process::exit(0);
}

fn show_help() {
    println!("  ( oh hai! need halp? )");
    //TODO
}

enum SearchMode {
    Regex,
}