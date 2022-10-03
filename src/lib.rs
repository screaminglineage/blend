use colored::Colorize;
use rand::Rng;
use std::fmt::{self, Display};

const HORIZONTAL_MAX: u8 = 5;

#[derive(Debug, PartialEq)]
pub struct RgbColour {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl RgbColour {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        RgbColour { red, green, blue }
    }

    pub fn rgb_string(&self) -> String {
        format!("({}, {}, {})", self.red, self.green, self.blue)
    }
}

impl Display for RgbColour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} #{:x}{:x}{:x}",
            "   ".on_truecolor(self.red, self.green, self.blue),
            self.red,
            self.green,
            self.blue
        )
    }
}

pub fn hex_to_rgb(hex_colour: &str) -> Option<RgbColour> {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    if hex_colour.len() != 7 {
        return None;
    }

    for i in 0..3 {
        // Takes 2 at a time, skipping the first character (#)
        let fact = 2 * i;
        let hex_colour = &hex_colour[(1 + fact)..=(2 + fact)];
        let rgb_colour = u8::from_str_radix(hex_colour, 16).ok()?;

        if i == 0 {
            red = rgb_colour
        } else if i == 1 {
            green = rgb_colour
        } else if i == 2 {
            blue = rgb_colour
        }
    }
    Some(RgbColour::new(red, green, blue))
}

pub fn mix_colours(colours: &[RgbColour]) -> RgbColour {
    let num_colours = colours.len() as u32;
    let mut r_sum: u32 = 0;
    let mut g_sum: u32 = 0;
    let mut b_sum: u32 = 0;

    for clr in colours {
        r_sum += clr.red as u32;
        g_sum += clr.green as u32;
        b_sum += clr.blue as u32;
    }

    let red = (r_sum / num_colours) as u8;
    let green = (g_sum / num_colours) as u8;
    let blue = (b_sum / num_colours) as u8;

    RgbColour::new(red, green, blue)
}

pub fn rand_colours(num: u32) -> Vec<RgbColour> {
    let mut rng = rand::thread_rng();
    (0..num)
        .map(|_| RgbColour::new(rng.gen::<u8>(), rng.gen::<u8>(), rng.gen::<u8>()))
        .collect()
}

pub fn print_colours(colours: Vec<RgbColour>, mixed_colour: RgbColour, show_rgb: bool) {
    let len = colours.len();

    if show_rgb {
        // Printing Vertically
        println!("\t{} {}", colours[0], colours[0].rgb_string());
        for i in 1..len {
            println!("+\t{} {}", colours[i], colours[i].rgb_string());
        }
        println!("\n=\t{} {}", mixed_colour, mixed_colour.rgb_string());
    } else {
        // Printing Horizontally
        if len <= HORIZONTAL_MAX as usize {
            print!("{}", colours[0]);
            for i in 1..len {
                print!(" + {}", colours[i]);
            }
            println!(" = {}", mixed_colour);

        // Printing Vertically
        } else {
            println!("\t{}", colours[0]);
            for i in 1..len {
                println!("+\t{}", colours[i]);
            }
            println!("\n=\t{}", mixed_colour);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn colour_mixer() {
        let col_1 = hex_to_rgb("#ffed00").unwrap();
        let col_2 = hex_to_rgb("#ff0000").unwrap();
        let col_3 = hex_to_rgb("#ff00ab").unwrap();
        let col_4 = hex_to_rgb("#0047ab").unwrap();
        let col_5 = hex_to_rgb("#00edff").unwrap();
        let col_6 = hex_to_rgb("#00b500").unwrap();
        let col_7 = hex_to_rgb("#ffffff").unwrap();
        let col_8 = hex_to_rgb("#000000").unwrap();

        let mixed = mix_colours(&[col_1, col_2, col_3, col_4, col_5, col_6, col_7, col_8]);
        assert_eq!(
            mixed,
            RgbColour {
                red: 127,
                green: 122,
                blue: 106
            }
        );
    }

    #[test]
    fn rand_colour() {
        let num = 10;
        let colours = rand_colours(num);
        for clr in colours {
            println!("{}", clr);
        }
    }
}
