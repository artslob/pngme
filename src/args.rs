use ::clap::{AppSettings, Clap};

// TODO add descriptions for arg values

#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
#[clap(version = "1.0", author = "artslob <artslob@yandex.ru>")]
pub struct Opts {
    #[clap(subcommand)]
    sub_cmd: SubCommand,
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
    file_path: String,
    /// Chunk type is 4 ASCII letters
    chunk_type: String,
    /// Just any text
    message: String,
    /// Save result to new file
    output_file: Option<String>,
}

/// Searches for a message hidden in a PNG file and prints the message if one is found
#[derive(Clap)]
pub struct Decode {
    /// Path to png file
    file_path: String,
    /// Chunk type is 4 ASCII letters
    chunk_type: String, // TODO maybe set default to some type?
}

/// Removes a chunk from a PNG file and saves the result
// TODO: print removed message and handle saving to another file
#[derive(Clap)]
pub struct Remove {
    /// Path to png file
    file_path: String,
    /// Chunk type is 4 ASCII letters
    chunk_type: String,
    /// Save result to new file
    output_file: Option<String>,
}

/// Prints all of the chunks in a PNG file
#[derive(Clap)]
pub struct Print {
    /// Path to png file
    file_path: String,
}

pub fn parse_cli() -> Opts {
    Opts::parse()
}
