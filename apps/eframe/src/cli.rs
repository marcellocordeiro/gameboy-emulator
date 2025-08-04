use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Set the device model to DMG
    #[arg(short, long, default_value_t = false)]
    pub dmg: bool,

    /// Optional bootrom path
    #[arg(short, long)]
    pub bootrom: Option<String>,

    /// Optional ROM path (will show the file picker if not provided)
    pub rom: Option<String>,
}

pub fn parse_args() -> Cli {
    Cli::parse()
}
