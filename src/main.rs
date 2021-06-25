use crate::args::SubCommand;
use std::convert::TryInto;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let opts = crate::args::parse_cli();
    // TODO merge match to 1 result?
    match opts.sub_cmd {
        SubCommand::Encode(cmd) => {
            crate::commands::encode(cmd)?;
        }
        SubCommand::Decode(cmd) => {
            crate::commands::decode(cmd)?;
        }
        SubCommand::Remove(cmd) => {
            crate::commands::remove(cmd)?;
        }
        SubCommand::Print(cmd) => {
            crate::commands::print(cmd)?;
        }
    };
    Ok(())
}
