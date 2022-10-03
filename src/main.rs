use blend::{self, RgbColour};
use clap::{CommandFactory, Parser};
use std::{env, io, process};

mod cli;
use cli::Cli;

mod colours;
use colours::get_rgbcolour;

fn main() -> io::Result<()> {
    //demo();
    let cli = Cli::parse();

    // Show Help if no Arguments Given
    if env::args().count() == 1 {
        let mut cmd = Cli::command();
        cmd.print_help()?;
        process::exit(0);
    }

    // Getting Colours and Converting to RGB
    let mut colours: Vec<RgbColour> = cli.colour.iter().map(|hex| get_rgbcolour(hex)).collect();

    // Generating Random Colours
    if let Some(num) = cli.random {
        let mut random_colours = blend::rand_colours(num);
        colours.append(&mut random_colours);
    }

    // Printing Colours
    let show_rgb = match cli.verbose {
        true => true,
        _ => false,
    };

    // Mixing Colours
    if colours.len() > 1 {
        let mixed_colour = blend::mix_colours(&colours);
        blend::print_colours(colours, mixed_colour, show_rgb);

    // Single Colour
    } else if colours.len() == 1 && show_rgb {
        println!("{} {}", colours[0], colours[0].rgb_string());
    } else if colours.len() == 1 {
        println!("{}", colours[0]);
    }

    Ok(())
}

fn _demo() {
    use blend::hex_to_rgb as ha;

    let colours = vec![
        ha("#3a05eb").unwrap(),
        ha("#044a52").unwrap(),
        ha("#057d15").unwrap(),
        ha("#b40bde").unwrap(),
    ];
    let mixed = blend::mix_colours(&colours);

    for clr in colours {
        print!("{} + ", clr);
    }
    println!("= {}", mixed);
}
