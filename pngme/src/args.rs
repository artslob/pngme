use ::clap::{AppSettings, Clap};
use std::path::PathBuf;

#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
#[clap(version = env!("CARGO_PKG_VERSION"))]
#[clap(about = "Utility that lets you hide secret messages in PNG files")]
pub struct Opts {
    #[clap(subcommand)]
    pub(crate) sub_cmd: SubCommand,
}

#[derive(Clap)]
pub enum SubCommand {
    Encode(Encode),
    Decode(Decode),
    Remove(Remove),
    Print(Print),
}

/// Encodes a message into a PNG file and saves the result
#[derive(Clap)]
pub struct Encode {
    /// Path to png file
    pub(crate) file_path: PathBuf,
    /// Chunk type is 4 ASCII letters
    pub(crate) chunk_type: String,
    /// Just any text
    pub(crate) message: Option<String>,
    /// Save png file with added chunk to new file
    #[clap(short, long)]
    pub(crate) output_file: Option<PathBuf>,
}

/// Searches for a message hidden in a PNG file and prints the message if one is found
#[derive(Clap)]
pub struct Decode {
    /// Path to png file
    pub(crate) file_path: PathBuf,
    /// Chunk type is 4 ASCII letters
    pub(crate) chunk_type: String,
    /// Set when data should go as raw unescaped bytes to stdout
    #[clap(long)]
    pub(crate) raw: bool,
}

/// Removes a chunk from a PNG file and saves the result
#[derive(Clap)]
pub struct Remove {
    /// Path to png file
    pub(crate) file_path: PathBuf,
    /// Chunk type is 4 ASCII letters
    pub(crate) chunk_type: String,
    /// Save png file with removed chunk to new file
    #[clap(short, long)]
    pub(crate) output_file: Option<PathBuf>,
    /// Set when data should go as raw unescaped bytes to stdout
    #[clap(long)]
    pub(crate) raw: bool,
}

/// Prints all of the chunks in a PNG file
#[derive(Clap)]
pub struct Print {
    /// Path to png file
    pub(crate) file_path: PathBuf,
    /// Get detailed information about chunks
    #[clap(long)]
    pub(crate) verbose: bool,
}

pub fn parse_cli() -> Opts {
    Opts::parse()
}
