pub mod attribute;
pub mod telegram;
pub mod exporter;
pub mod cli;

use std::net::TcpStream;
use std::fs::File;
use std::io::{BufReader, Read};
use std::time::Duration;

use log::{debug, info, error};

use telegram::Telegram;
use cli::{CLI, Source};

fn is_interactive() -> bool {
    unsafe {
        libc::isatty(1) == 1
    }
}

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
        .filter_level(cli.verbosity.log_level_filter())
        .format_timestamp(is_interactive().then(|| env_logger::fmt::TimestampPrecision::Millis))
        .target(env_logger::Target::Stdout)
        .init();

    // say something
    info!("{} {} starting", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    info!("Reading from {}", cli.source());
    info!("Prometheus listening on http://{}/", cli.listen);
    debug!("{cli:?}");

    // start prometheus_exporter
    exporter::start(&cli.listen)?;

    // connect to source and start source-specific main loop
    match cli.source() {
        Source::Socket(ref host) => {
            let source = TcpStream::connect(host)
                .map_err(|e| format!("Error connecting to {host}: {e}"))?;
            main_loop(source)?;
        },
        Source::Serial(ref tty, bps) => {
            let source = serialport::new(tty, bps)
                .timeout(Duration::from_secs(5))
                .open()
                .map_err(|e| format!("Error opening serial port {tty}: {e}"))?;
            main_loop(source)?;
        },
        Source::File(ref path) => {
            let source = File::options().read(true).open(path)
                .map_err(|e| format!("Error opening {path:?}: {e}"))?;
            main_loop(source)?;
        },
    }

    Ok(())
}

fn main() {
    if let Err(e) = try_main() {
        error!("Exiting with error: {e}");
        std::process::exit(1);
    }
}
