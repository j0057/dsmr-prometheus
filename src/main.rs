pub mod attribute;
pub mod telegram;
pub mod exporter;
pub mod cli;

use std::net::TcpStream;
use std::io::{BufReader, Read};

use log::{debug, error};

use telegram::Telegram;
use cli::CLI;

fn main_loop<S: Read>(source: S) -> Result<(), String> {
    let mut reader = BufReader::new(source);

    loop {
        let telegram = Telegram::from(&mut reader)
            .map_err(|e| format!("Error reading frame: {e}"))?;

        exporter::export(&telegram.elements);

        debug!("{telegram:?}");
    }
}

fn try_main() -> Result<(), String> {
    // parse program arguments
    let cli = CLI::new()
        .map_err(|e| {
            println!("{e}"); // logger is not yet initialized at this point
            format!("Error parsing arguments: {e}")
        })?;

    // initialize logger
    env_logger::Builder::from_default_env()
        .target(env_logger::Target::Stdout)
        .init();

    debug!("{cli:?}");

    // start prometheus_exporter
    exporter::start(cli.listen)?;

    // connect to TCP source
    if let Some(host) = cli.connect {
        let source = TcpStream::connect(host.clone())
            .map_err(|e| format!("Error connecting to {host}: {e}"))?;
        main_loop(source)?;
    }

    // connect to serial source
    else if let Some(_tty) = cli.serial {
        // TODO: implement serial source
        todo!()
    }

    // TODO: implement file source

    // should never happen
    else {
        unreachable!();
    }

    Ok(())
}

fn main() {
    if let Err(e) = try_main() {
        error!("Exiting with error: {e}");
        std::process::exit(1);
    }
}
