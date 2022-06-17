use clap::{Parser, ArgGroup};

#[derive(Parser, Debug)]
#[clap(author, version)]
#[clap(group(ArgGroup::new("source").required(true)))]
pub struct CLI {
    #[clap(short, long, group="source")]
    pub connect: Option<String>,

    #[clap(short, long, group="source")]
    pub serial: Option<String>,

    #[clap(short, long, default_value="0.0.0.0:9194")]
    pub listen: String,
}

impl CLI {
    pub fn new() -> Result<Self, clap::Error> {
        return Self::try_parse();
    }
}
