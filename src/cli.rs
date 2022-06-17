use clap::{Parser, ArgGroup};

#[derive(Parser, Debug)]
#[clap(author, version)]
#[clap(group(ArgGroup::new("source").required(true)))]
pub struct CLI {
    #[clap(short, long, group="source")]
    pub connect: Option<String>,

    #[clap(short, long, group="source")]
    pub serial: Option<String>,

    #[clap(short, long)]
    pub listen: Option<String>,
}

impl CLI {
    pub fn new() -> Self {
        return Self::parse();
    }
}
