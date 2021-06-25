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
    match opts.sub_cmd {
        SubCommand::Encode(_) => {}
        SubCommand::Decode(_) => {}
        SubCommand::Remove(_) => {}
        SubCommand::Print(cmd) => {
            crate::commands::print(cmd)?;
        }
    };
    Ok(())
}
