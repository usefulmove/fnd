use colored::*;
use regex::Regex;

pub fn color_rgb(message: &str, r: u8, g: u8, b: u8) -> ColoredString {
    message.truecolor(r, g, b)
}

pub fn color_rgb_bold(message: &str, r: u8, g: u8, b: u8) -> ColoredString {
    message.truecolor(r, g, b).bold()
}

pub fn color_red_bold(message: &str) -> ColoredString {
    message.truecolor(241, 95, 78).bold()
}

pub fn _color_orange_sherbet_bold(message: &str) -> ColoredString {
    message.truecolor(239, 157, 110).bold()
}

pub fn color_yellow_canary_bold(message: &str) -> ColoredString {
    message.truecolor(255, 252, 103).bold()
}

pub fn color_green_eggs_bold(message: &str) -> ColoredString {
    message.truecolor(135, 255, 175).bold()
}

pub fn color_blue_smurf(message: &str) -> ColoredString {
    message.truecolor(0, 128, 255)
}

pub fn color_blue_smurf_bold(message: &str) -> ColoredString {
    message.truecolor(0, 128, 255).bold()
}

pub fn color_blue_coffee_bold(message: &str) -> ColoredString {
    message.truecolor(0, 192, 255).bold()
}

pub fn color_white_bold(message: &str) -> ColoredString {
    message.truecolor(249, 247, 236).bold()
}

pub fn color_white(message: &str) -> ColoredString {
    message.truecolor(249, 247, 236)
}

pub fn color_grey_mouse(message: &str) -> ColoredString {
    message.truecolor(155, 155, 155)
}

pub fn color_charcoal_cream(message: &str) -> ColoredString {
    message.truecolor(102, 102, 102)
}

pub fn color_blank(_message: &str) -> ColoredString {
    "".truecolor(0, 0, 0)
}

pub fn highlight(output_string: &str, highlight_term: &str, f_color: fn(&str) -> colored::ColoredString) -> String {
    /* find the highlight term in the output string and format the output 
     * string to emphasize the highlight term in the output string
     */

    let tmp: String = output_string.clone().to_string();
    let elements: Vec<&str> = tmp.split(&highlight_term).collect::<Vec<&str>>();

    //print!("{:#?}", elements); // debug

    // construct highlighted output
    let mut o: String = String::new();

    for i in 0..elements.len() {
        if i < (elements.len() - 1) {
            o.push_str(
                &format!(
                    "{}{}",
                    color_grey_mouse(elements[i]),
                    f_color(highlight_term),
                )
            );
        } else {
            o.push_str(
                &format!(
                    "{}",
                    color_grey_mouse(elements[i]),
                )
            );
        }
    }

    o
}

pub fn highlight_filename(output_string: &str) -> String {
    /* highlight everything following the last "/"
     */

    let re: Regex = Regex::new(r"/([^/]+)$").unwrap();

    let filename: String = match re.captures(output_string) {
        Some(n) => n[1].to_string(),
        None => "".to_string(),
    };

    highlight(output_string, &filename, color_white)
}