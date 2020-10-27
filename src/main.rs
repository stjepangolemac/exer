use exer_internals::error::ExerResult;
use exer_internals::{opt, run};

fn main() -> ExerResult<()> {
    let options = opt::get();
    let cwd = std::env::current_dir()?;

    run(&cwd, &options);

    Ok(())
}
