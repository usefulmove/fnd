use ignore::{WalkBuilder, DirEntry};
use regex::Regex;
use std::env;
use std::path::Path;
use std::process::exit;

const RELEASE_STATE: &str = "b";

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
                        "  search directory: {}",
                        color_theme.color_rgb(
                            &dir,
                            &color_theme.yellow_canary
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
                    println!("  searching hidden files");
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
                println!("  verbose mode");
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
                    exit(exitcode::USAGE);
                }
                if args.len() == 1 {
                    search_expr = args.remove(0); // get search expression

                    if verbose {
                        println!(
                            "  search expression: \"{}\"",
                            color_theme.color_rgb(
                                &search_expr,
                                &color_theme.yellow_canary
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
                    for res in WalkBuilder::new(&dir).hidden(ignore_hidden).build() {
                        let entry: DirEntry = match res {
                            Ok(directory_entry) => directory_entry,
                            Err(e) => {
                                eprintln!(
                                    "  {}: invalid path [{}]: {}",
                                    color_theme.color_rgb(
                                        "error",
                                        &color_theme.red_bold
                                    ),
                                    color_theme.color_rgb(
                                        &dir,
                                        &color_theme.blue_smurf_bold,
                                    ),
                                    e,
                                );
                                exit(exitcode::OSFILE);
                            }
                        };
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
    }

    exit(exitcode::OK);

}

fn show_help() {
    // color theme
    let theme = coq::Theme::new();

    println!();
    println!(
        "{}",
        theme.color_rgb("FND", &theme.cream_bold)
    );
    println!(
        "    {} {} {} {}",
        theme.color_rgb("fnd", &theme.grey_mouse),
        theme.color_rgb("..", &theme.charcoal_cream),
        theme.color_rgb("search for files that match a regular expression", &theme.cream_bold),
        theme.color_rgb(env!("CARGO_PKG_VERSION"), &theme.grey_mouse),
    );
    println!();
    println!(
        "{}",
        theme.color_rgb("USAGE", &theme.cream_bold)
    );
    println!(
        "    {} {} {}",
        theme.color_rgb("fnd", &theme.grey_mouse),
        theme.color_rgb("[OPTIONS]", &theme.cream_bold),
        theme.color_rgb("<regex>", &theme.blue_coffee_bold),
    );
    println!();
    println!(
        "{}",
        theme.color_rgb("OPTIONS", &theme.cream_bold)
    );
    println!(
        "        {}      show version",
        theme.color_rgb("--version", &theme.yellow_canary),
    );
    println!(
        "    {}{} {}          specify search directory",
        theme.color_rgb("-d", &theme.yellow_canary),
        theme.color_rgb(",", &theme.charcoal_cream),
        theme.color_rgb("--dir", &theme.yellow_canary),
    );
    println!(
        "    {}{} {}       search hidden files",
        theme.color_rgb("-h", &theme.yellow_canary),
        theme.color_rgb(",", &theme.charcoal_cream),
        theme.color_rgb("--hidden", &theme.yellow_canary),
    );
    println!(
        "        {}         show help information",
        theme.color_rgb("--help", &theme.yellow_canary),
    );
    println!();
    println!(
        "{}",
        theme.color_rgb("DESCRIPTION", &theme.cream_bold)
    );
    println!(
        "The fnd command searches for files that match a regular expression {}. \
        The non-hidden files in the current directory is searched by default. It \
        is also possible to search a different directory by using the {} option \
        followed by the directory to be searched. The search can be expanded to \
        include hidden files by using the {} option.",
        theme.color_rgb("<regex>", &theme.blue_coffee_bold),
        theme.color_rgb("--dir", &theme.yellow_canary_bold),
        theme.color_rgb("--hidden", &theme.yellow_canary_bold),
    );
    println!();
    println!(
        "    Repository:    {}",
        theme.color_rgb("https://github.com/usefulmove/fnd#readme", &theme.grey_mouse),
    );
    println!();
}

enum SearchMode {
    Regex,
}