use crate::args::SubCommand;

mod args;
mod commands;

type Error = Box<dyn std::error::Error + Sync + Send>;
type Result<T> = std::result::Result<T, Error>;

fn main() -> crate::Result<()> {
    match crate::args::parse_cli().sub_cmd {
        SubCommand::Encode(cmd) => crate::commands::encode(cmd)?,
        SubCommand::Decode(cmd) => crate::commands::decode(cmd)?,
        SubCommand::Remove(cmd) => crate::commands::remove(cmd)?,
        SubCommand::Print(cmd) => crate::commands::print(cmd)?,
    };
    Ok(())
}
