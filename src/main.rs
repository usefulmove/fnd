use ignore::{WalkBuilder, DirEntry};
use regex::Regex;
use std::env;
use std::path::Path;

const RELEASE_STATE: &str = "a";

fn main() {
    // enable or disable backtrace on error
    env::set_var("RUST_BACKTRACE", "0");

    // set default search mode
    let mode: SearchMode = SearchMode::Regex;

    // set default search directory
    let mut dir: String = String::from("./");

    // set default verbosity
    let mut verbose: bool = false;

    // don't show hidden files by default
    let mut ignore_hidden: bool = true;

    // get command arguments
    let mut args: Vec<String> = env::args().collect();

    let color_theme = coq::Theme::new();

    args.remove(0); // remove the program name from arguments list

    // if no arguments are passed, behave as if help flag was passed
    if args.is_empty() {
        args.push(String::from("--help"));
    }

    let mut search_expr: String = String::from("");
    while !args.is_empty() {
        match args[0].as_str() {
            "--dir" | "-d" => {
                // set search directory
                args.remove(0); // remove flag
                dir = args.remove(0); // save directory argument

                if verbose {
                    println!(
                        "  {}",
                        color_theme.color_rgb(
                            "  verbose mode",
                            &color_theme.orange_sherbet_bold
                        ),
                    );
                }
            }
            "--help" => {
                // display command usage information
                show_help();
                return;
            }
            "--hidden" | "-h" => {
                args.remove(0); // remove flag
                ignore_hidden = false; // show hidden files
                if verbose {
                    println!(
                        "  {}",
                        color_theme.color_rgb(
                            "  searching hidden files",
                            &color_theme.orange_sherbet_bold
                        ),
                    );
                }
            }
            "--version" => {
                // display version information
                println!(
                    "  {} {}{}",
                    color_theme.color_rgb(
                        "fnd",
                        &color_theme.grey_mouse
                    ),
                    color_theme.color_rgb(
                        env!("CARGO_PKG_VERSION"),
                        &color_theme.blue_smurf_bold
                    ),
                    color_theme.color_rgb(
                        RELEASE_STATE,
                        &color_theme.white_bold
                    ),
                );
                return;
            }
            "--verbose" | "-v" => {
                args.remove(0); // remove flag
                println!(
                    "  {}",
                    color_theme.color_rgb(
                        "  verbose mode",
                        &color_theme.orange_sherbet_bold
                    ),
                );
                verbose = true;
            }
            _ => {
                if args.len() > 1 {
                    eprintln!(
                        "  {}: incorrect use of command arguments",
                        color_theme.color_rgb(
                            "error",
                            &color_theme.red_bold
                        ),
                    );
                    std::process::exit(exit_code::USAGE_ERROR);
                }
                if args.len() == 1 {
                    search_expr = args.remove(0); // get search expression

                    if verbose {
                        println!(
                            "  search expression is \"{}\"",
                            color_theme.color_rgb(
                                &search_expr,
                                &color_theme.blue_smurf_bold
                            ),
                        );
                    }
                }
            }
        }
    }

    /* fnd core */
    match mode {
        SearchMode::Regex => {
            /* regular expression ( regex ) search */
            let reg_exp: String = search_expr;

            match Regex::new(&reg_exp) {
                Ok(re) => {
                    for entry in WalkBuilder::new(&dir).hidden(ignore_hidden).build() {
                        let entry: DirEntry = entry.unwrap();
                        let path: &Path = entry.path();
                        let path_str: &str = path.to_str().unwrap();

                        let no_match: String = String::from("");

                        let matching_term: String = match re.captures(path_str) {
                            Some(match_return) => match_return[0].to_string(),
                            None => no_match.clone(),
                        };

                        if matching_term != no_match {
                            println!(
                                "  {}",
                                coq::highlight(
                                    path_str,
                                    &matching_term,
                                    &color_theme.blue_smurf_bold,
                                ),
                            );
                        }
                    }
                }
                Err(e) => {
                    println!(
                        "  {}: {}",
                        color_theme.color_rgb(
                            "error",
                            &color_theme.red_bold
                        ),
                        e,
                    );
                }
            }
        }
        /*
        SearchMode::Simple => {
            /* simple search */
            let search_term: String = args[0].clone();

            for entry in WalkBuilder::new("./").hidden(ignore_hidden).build() {
                let entry: DirEntry = entry.unwrap();
                let path: &Path = entry.path();
                let path_str: &str = path.to_str().unwrap();
                if path_str.contains(&search_term) {
                    println!(
                        "  {}",
                        coq::highlight(
                            path_str,
                            &search_term,
                            &color_theme.blue_smurf_bold,
                        ),
                    );
                }
            }
        }
        */
    }

    std::process::exit(exit_code::SUCCESS);

}

fn show_help() {
    println!("  ( oh hai! need halp? )");
    //TODO
}

enum SearchMode {
    Regex,
}