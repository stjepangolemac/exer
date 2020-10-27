use crate::error::ExerResult;
use crate::templating::Templator;
use std::fs::DirBuilder;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use tera::Context;

/// Prompts the user and returns the answer
fn prompt(msg: &str) -> ExerResult<String> {
    println!("{}", msg);
    stdout().flush()?;

    let mut answer = String::new();
    stdin().read_line(&mut answer)?;
    answer.pop();

    Ok(answer)
}

/// Initializes the exer project in the directory
pub fn init(cwd: &Path) -> ExerResult<()> {
    let templator = Templator::new()?;

    let name = prompt("What is the site name?")?;
    let author = prompt("What is the site author?")?;

    let mut ctx = Context::new();
    ctx.insert("name", &name);
    ctx.insert("author", &author);

    templator.generate_config(cwd, &ctx);

    DirBuilder::new().recursive(true).create(cwd)?;

    Ok(())
}
