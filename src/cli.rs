use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Add colours to mix together
    ///
    /// Accepts hexadecimal colour codes (Both uppercase and lowercase)
    /// as well as some commonly used colours as arguments
    ///
    /// Supported Colours are: red, blue, green, white, black, gray, 
    /// yellow, cyan, magenta, violet, orange, brown and dark green
    #[arg(short, long)]
    pub colour: Vec<String>,

    /// Mix a specified number of random colours
    #[arg(short, long, value_name = "NUM", 
          value_parser = clap::value_parser!(u32).range(1..),
    )]
    pub random: Option<u32>,

    /// Show verbose output
    #[arg(short, long)]
    pub verbose: bool,
}
