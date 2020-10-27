use crate::error::ExerResult;
use crate::opt::Options;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use tera::{Context, Tera};

pub struct Templator {
    tera: Tera,
}

impl Templator {
    pub fn new() -> ExerResult<Self> {
        let mut t = Templator {
            tera: Tera::new("templates/**/*")?,
        };

        t.tera
            .add_raw_template("exer.toml", include_str!("templates/init/exer.toml"))?;

        Ok(t)
    }

    pub fn generate_config(&self, dir: &Path, ctx: &Context) -> ExerResult<()> {
        let destination = dir.join("exer.toml");
        let rendered = self.tera.render("exer.toml", ctx)?;

        let mut f = File::create(destination)?;
        f.write_all(rendered.as_bytes())?;

        Ok(())
    }
}
