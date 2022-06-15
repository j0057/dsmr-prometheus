use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version)]
pub struct CLI {
    #[clap(short='c', long)]
    pub connect: Option<String>,

    #[clap(short='s', long)]
    pub serial: Option<String>,

    #[clap(short='l', long)]
    pub listen: Option<String>,
}

impl CLI {
    pub fn new() -> Self {
        return Self::parse();
    }
}
