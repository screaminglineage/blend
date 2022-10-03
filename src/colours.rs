use blend::{self, RgbColour};
use std::process;

pub fn get_rgbcolour(colour: &str) -> RgbColour {
    // Checks if colour is a hex code
    // and if so tries to get the RGB value
    match colour.chars().next() {
        Some('#') => {
            if let Some(clr) = blend::hex_to_rgb(colour) {
                return clr;
            } else {
                eprintln!("Error! Invalid RGB hex code entered\nHelp: Prefix a colour hex code with a `#` (like `#7afab4` or `#3A05EB`)");
                process::exit(1);
            }
        }

        // Predefined Common Colours
        _ => match colour.to_lowercase().as_ref() {
            "red" => RgbColour::new(255, 0, 0),
            "green" => RgbColour::new(0, 255, 0),
            "blue" => RgbColour::new(0, 0, 255),
            "white" => RgbColour::new(255, 255, 255),
            "black" => RgbColour::new(0, 0, 0),
            "gray" => RgbColour::new(128, 128, 128),
            "grey" => RgbColour::new(128, 128, 128),
            "yellow" => RgbColour::new(255, 255, 0),
            "cyan" => RgbColour::new(0, 255, 255),
            "magenta" => RgbColour::new(255, 0, 255),
            "violet" => RgbColour::new(75, 0, 130),
            "orange" => RgbColour::new(255, 82, 0),
            "brown" => RgbColour::new(139, 69, 19),
            "dark green" => RgbColour::new(0, 100, 0),

            _ => {
                eprintln!("Error! Unsupported Colour: `{}`\nHelp: Use the `--help` option to see all supported colours\nHelp: Prefix a colour hex code with a `#` (like `#7afab4` or `#3A05EB`)", colour.to_lowercase());
                process::exit(1);
            }
        },
    }
}
