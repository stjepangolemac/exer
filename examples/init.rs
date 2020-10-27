use exer_internals::error::ExerResult;
use exer_internals::opt::{Command, Options};
use exer_internals::run;

fn main() -> ExerResult<()> {
    let cwd = std::env::current_dir()?;
    let options = Options {
        command: Command::Init,
    };

    run(&cwd, &options)?;

    Ok(())
}
