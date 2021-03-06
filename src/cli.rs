use std::path::PathBuf;

use clap::{Parser, ArgGroup, Args};
use clap_verbosity_flag::{Verbosity, InfoLevel};

#[derive(Parser, Debug)]
#[clap(author, version)]
pub struct CLI {
    #[clap(flatten)]
    source: SourceArgs,

    #[clap(short, long, default_value="115200")]
    baud_rate: u32,

    #[clap(short, long, default_value="0.0.0.0:9194")]
    pub listen: String,

    #[clap(flatten)]
    pub verbosity: Verbosity<InfoLevel>,
}

#[derive(Args, Debug)]
#[clap(group(ArgGroup::new("source").required(true)))]
pub struct SourceArgs {
    #[clap(short, long, group="source")]
    pub connect: Option<String>,

    #[clap(short, long, group="source")]
    pub serial: Option<String>,

    #[clap(short, long, group="source")]
    pub file: Option<PathBuf>,
}

pub enum Source {
    Socket(String),
    Serial(String, u32),
    File(PathBuf),
}

impl std::fmt::Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Source::Socket(ref host)            => write!(f, "TCP source {host}"),
            Source::Serial(ref tty, baud_rate)  => write!(f, "serial port source {tty} ({baud_rate} bps)"),
            Source::File(ref path)              => write!(f, "file source {path:?}"),
        }
    }
}

impl CLI {
    pub fn new() -> Result<Self, clap::Error> {
        Self::try_parse()
    }

    pub fn source(&self) -> Source {
        None.xor(self.source.connect.clone().map(Source::Socket))
            .xor(self.source.serial.clone().map(|tty| Source::Serial(tty, self.baud_rate)))
            .xor(self.source.file.clone().map(Source::File))
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_long_1() -> Result<(), clap::Error> {
        CLI::try_parse_from(["./foo", "--connect", "example.com:8000", "--listen", "0.0.0.0:9194"])?;
        Ok(())
    }

    #[test]
    fn test_long_2() -> Result<(), clap::Error> {
        CLI::try_parse_from(["./bar", "--serial", "/dev/ttyS0", "--listen", "0.0.0.0:9194"])?;
        Ok(())
    }

    #[test]
    fn test_short() -> Result<(), clap::Error> {
        CLI::try_parse_from(["./foo", "-c", "example.com:4000", "-l", "0.0.0.0:9194"])?;
        Ok(())
    }

    #[test]
    fn test_source_missing() {
        assert!(CLI::try_parse_from(["./foo", "-l", "0.0.0.0:9194"]).is_err());
    }

    #[test]
    fn test_source_connect() {
        assert!(CLI::try_parse_from(["./foo", "-l", "0.0.0.0:9194", "-c", "example.com:8000"]).is_ok());
    }

    #[test]
    fn test_source_serial() {
        assert!(CLI::try_parse_from(["./foo", "-l", "0.0.0.0:9194", "-s", "/dev/ttyS0"]).is_ok());
    }

    #[test]
    fn test_source_file() {
        assert!(CLI::try_parse_from(["./foo", "-l", "0.0.0.0:9194", "-f", "file.txt"]).is_ok());
    }

    #[test]
    fn test_source_1_too_many() {
        assert!(CLI::try_parse_from(["./foo", "-l", "0.0.0.0:9194", "-c", "example.com:8000", "-s", "/dev/ttyS0"]).is_err());
    }
}
