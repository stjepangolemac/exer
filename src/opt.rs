use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Command {
    /// Initializes a new exer project
    Init,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "exer", about = "Generates sites in no time")]
pub struct Options {
    #[structopt(subcommand)]
    pub command: Command,
}

pub fn get() -> Options {
    Options::from_args()
}
