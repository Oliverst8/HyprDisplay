use clap::Parser;

/// A command-line tool for managing monitor configurations with Hyprland.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct Args {
    /// Monitor mode to switch to, supports mirror extend left/right
    #[arg(short, long)]
    pub(crate) mode: Option<String>,

    /// Switch to the next monitor mode
    #[arg(short, long)]
    pub(crate) next_mode: bool,

    /// Whether to run setup
    #[arg(long)]
    pub(crate) setup: bool,

    /// Whether to show selection menu
    #[arg(long)]
    pub(crate) menu: bool,
}