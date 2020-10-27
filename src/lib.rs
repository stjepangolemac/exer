use crate::error::ExerResult;
use crate::opt::Command;
use std::path::Path;

pub mod error;
pub mod opt;
mod tasks;
mod templating;

pub fn run(cwd: &Path, options: &opt::Options) -> ExerResult<()> {
    match options.command {
        Command::Init => tasks::init(cwd),
        _ => Ok(()),
    }?;

    Ok(())
}
