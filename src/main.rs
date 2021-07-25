use crate::args::SubCommand;

mod args;
mod commands;

fn main() -> pngme::Result<()> {
    match crate::args::parse_cli().sub_cmd {
        SubCommand::Encode(cmd) => crate::commands::encode(cmd)?,
        SubCommand::Decode(cmd) => crate::commands::decode(cmd)?,
        SubCommand::Remove(cmd) => crate::commands::remove(cmd)?,
        SubCommand::Print(cmd) => crate::commands::print(cmd)?,
    };
    Ok(())
}
