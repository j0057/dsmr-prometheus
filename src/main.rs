pub mod attribute;
pub mod telegram;
pub mod cli;

use std::net::TcpStream;
use std::io::{BufReader, Read};

use lazy_static::lazy_static;
use prometheus_exporter::prometheus;

use attribute::Attribute;
use telegram::Telegram;

lazy_static! {
    static ref ELECTRICITY_DELIVERED: prometheus::GaugeVec
        = prometheus::register_gauge_vec!("electricity_delivered", "Meter reading electricity delivered to client (kWh)", &["tariff"]).unwrap();

    static ref ELECTRICITY_RECEIVED: prometheus::GaugeVec
        = prometheus::register_gauge_vec!("electricity_received", "Meter reading electricity delivered by client (kWh)", &["tariff"]).unwrap();

    static ref TARIFF_INDICATOR: prometheus::IntGauge
        = prometheus::register_int_gauge!("tariff_indicator", "Tariff indicator electricity").unwrap();

    static ref ACTUAL_POWER_DELIVERED: prometheus::Gauge
        = prometheus::register_gauge!("actual_power_delivered", "Actual electricity power delivered (+P) (kW)").unwrap();

    static ref ACTUAL_POWER_RECEIVED: prometheus::Gauge
        = prometheus::register_gauge!("actual_power_received", "Actual electricity power received (-P) (kW)").unwrap();

    static ref INSTANT_VOLTAGE: prometheus::GaugeVec
        = prometheus::register_gauge_vec!("instant_voltage", "Instantaneous voltage by phase (V)", &["phase"]).unwrap();

    static ref INSTANT_CURRENT: prometheus::GaugeVec
        = prometheus::register_gauge_vec!("instant_current", "Instantaneous current by phase (A)", &["phase"]).unwrap();

    static ref INSTANT_POWER_DELIVERED: prometheus::GaugeVec
        = prometheus::register_gauge_vec!("instant_power_delivered", "Instantaneous active power delivered by phase (kW)", &["phase"]).unwrap();

    static ref INSTANT_POWER_RECEIVED: prometheus::GaugeVec
        = prometheus::register_gauge_vec!("instant_power_received", "Instantaneous active power received by phase (kW)", &["phase"]).unwrap();

    static ref GAS_DELIVERED: prometheus::Gauge
        = prometheus::register_gauge!("gas_delivered", "Gas delivered to client (m³)").unwrap();
}

fn main_loop<S: Read>(source: S) -> Result<(), String> {
    let mut reader = BufReader::new(source);

    loop {
        let telegram = Telegram::from(&mut reader)
            .map_err(|e| format!("Error reading frame: {e}"))?;

        for attr in &telegram.elements {
            match *attr {
                Attribute::ElectricityDelivered(tariff, kwh)        => ELECTRICITY_DELIVERED.with_label_values(&[&tariff.to_string()]).set(kwh),
                Attribute::ElectricityReceived(tariff, kwh)         => ELECTRICITY_RECEIVED.with_label_values(&[&tariff.to_string()]).set(kwh),
                Attribute::TariffIndicator(tariff)                  => TARIFF_INDICATOR.set(tariff),
                Attribute::ActualPowerDelivered(kw)                 => ACTUAL_POWER_DELIVERED.set(kw),
                Attribute::ActualPowerReceived(kw)                  => ACTUAL_POWER_RECEIVED.set(kw),
                Attribute::InstantVoltage(phase, v)                 => INSTANT_VOLTAGE.with_label_values(&[&phase.to_string()]).set(v),
                Attribute::InstantCurrent(phase, a)                 => INSTANT_CURRENT.with_label_values(&[&phase.to_string()]).set(a),
                Attribute::InstantPowerDelivered(phase, kw)         => INSTANT_POWER_DELIVERED.with_label_values(&[&phase.to_string()]).set(kw),
                Attribute::InstantPowerReceived(phase, kw)          => INSTANT_POWER_RECEIVED.with_label_values(&[&phase.to_string()]).set(kw),
                Attribute::GasDelivered(_, _, m3)                   => GAS_DELIVERED.set(m3),
                _                                                   => ()
            }
        }

        println!("{telegram:?}");
    }
}

fn main() -> Result<(), String> {
    let cli = cli::CLI::new();

    println!("{cli:?}");

    let bind = cli.listen.parse()
        .map_err(|e| format!("Cannot parse binding address {listen:?}: {e:?}"))?;
    let _exporter = prometheus_exporter::start(bind)
        .map_err(|e| format!("Error starting Prometheus exporter: {e:?}"))?;

    if let Some(host) = cli.connect {
        let source = TcpStream::connect(host.clone())
            .map_err(|e| format!("Error connecting to {host}: {e:?}"))?;
        main_loop(source)?;
    }
    else if let Some(_tty) = cli.serial {
        todo!()
    }
    else {
        unreachable!();
    }

    Ok(())
}
