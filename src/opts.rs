use structopt::StructOpt;

#[derive(StructOpt, Debug, PartialEq)]
pub struct Opt {
    #[structopt(short, long = "debug", short = "d")]
    pub debug: bool,
    #[structopt(subcommand)]
    pub action: Action,
}

#[derive(StructOpt, Debug, PartialEq)]
pub enum Action {
    CreateNote { name: String },
    RemoveNote,
    EnterTui,
    EnterGui,
}
