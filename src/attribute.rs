use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Unit {
    KiloWatt,
    KiloWattHour,
    Ampere,
    Volt,
    CubicMeters,
    Seconds,
}

impl FromStr for Unit {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        match text {
            "kW"    => Ok(Self::KiloWatt),
            "kWh"   => Ok(Self::KiloWattHour),
            "A"     => Ok(Self::Ampere),
            "V"     => Ok(Self::Volt),
            "m3"    => Ok(Self::CubicMeters),
            "s"     => Ok(Self::Seconds),
             _      => Err(format!("Unknown unit {text:?}")),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Attribute {
    Header(String),
    Version(String),
    Timestamp(String),
    EquipmentIdentifier(String),
    ElectricityDelivered(u8, f64),
    ElectricityReceived(u8, f64),
    TariffIndicator(i64),
    ActualPowerDelivered(f64),
    ActualPowerReceived(f64),
    PowerFailures(u32),
    PowerFailuresLong(u32),
    PowerFailureLog(Vec<String>),
    VoltageSags(u8, u16),
    VoltageSwells(u8, u16),
    TextMessage(String),
    InstantVoltage(u8, f64),
    InstantCurrent(u8, f64),
    InstantPowerDelivered(u8, f64),
    InstantPowerReceived(u8, f64),
    GasEquipmentDeviceType(u8, u8),
    GasEquipmentIdentifier(u8, String),
    GasDelivered(u8, String, f64),
    // TODO: heat-pump related stuff
    // TODO: water-related stuff
}

impl Attribute {
    fn parse_hex(line: &str) -> Result<String, String> {
        let bytes = (0..line.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&line[i..i+2], 16))
            .collect::<Result<Vec<u8>, std::num::ParseIntError>>()
            .map_err(|e| format!("Cannot parse {line:?} as hex string: {e:?}"))?;

        String::from_utf8(bytes)
            .map_err(|e| format!("Cannot parse sequence as UTF-8: {e:?}"))
    }

    fn parse_num_unit<T: FromStr>(value: &str) -> Result<(T, String), String>
    where <T as FromStr>::Err: std::fmt::Debug
    {
        let index = value
            .find('*')
            .ok_or_else(|| format!("Cannot find '*' in value {value:?}"))?;
        let (value, unit) = value.split_at(index);
        let value = value.parse()
            .map_err(|e| format!("Error parsing number {value:?}: {e:?}"))?;
        Ok((value, unit.into()))
    }

    fn parse_num<T: FromStr>(value: &str) -> Result<T, String>
    where <T as FromStr>::Err: std::fmt::Debug,
    {
        value
            .parse()
            .map_err(|e| format!("Error parsing number {value:?}: {e:?}"))
    }
}

impl FromStr for Attribute {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        // split before and after first parenthesis
        let delim = line.find('(')
                        .ok_or_else(|| format!("First parenthesis not found in value {line:?}"))?;
        let (obis, values) = line.split_at(delim);

        // parse key to a [u8; 5]
        let key: [u8; 5] = obis.split(&['-', ':', '.'])
                               .map(str::parse::<u8>)
                               .collect::<Result<Vec<u8>, _>>()
                               .map_err(|e| format!("OBIS {obis:?} caused error parsing number: {e:?}"))?
                               .try_into()
                               .map_err(|e| format!("OBIS {obis:?} has wrong number of elements: {e:?}"))?;

        // split values to a Vec<String>
        let value: Vec<String> = values.trim_start_matches('(')
                                       .trim_end_matches(')')
                                       .split(")(")
                                       .map(|s| s.to_owned())
                                       .collect::<Vec<String>>();

        // try to instantiate result
        match key {
            [1, 3, 0, 2, 8]                                         => Ok(Self::Version(value[0].clone())),

            [0, 0, 1, 0, 0]                                         => Ok(Self::Timestamp(value[0].clone())),

            [0, 0, 96, 1, 1]                                        => Ok(Self::EquipmentIdentifier(Self::parse_hex(&value[0])?)),

            [1, 0, 1, 8, n]     if n == 1 || n == 2                 => Self::parse_num_unit(&value[0]).map(|(v, _)| Self::ElectricityDelivered(n, v)),

            [1, 0, 2, 8, n]     if n == 1 || n == 2                 => Self::parse_num_unit(&value[0]).map(|(v, _)| Self::ElectricityReceived(n, v)),

            [0, 0, 96, 14, 0]                                       => Ok(Self::TariffIndicator(Self::parse_num(&value[0])?)),

            [1, 0, 1, 7, 0]                                         => Self::parse_num_unit(&value[0]).map(|(v, _)| Self::ActualPowerDelivered(v)),

            [1, 0, 2, 7, 0]                                         => Self::parse_num_unit(&value[0]).map(|(v, _)| Self::ActualPowerReceived(v)),

            [0, 0, 96, 7, 21]                                       => Ok(Self::PowerFailures(Self::parse_num(&value[0])?)),

            [0, 0, 96, 7, 9]                                        => Ok(Self::PowerFailuresLong(Self::parse_num(&value[0])?)),

            [1, 0, 99, 97, 0]                                       => Ok(Self::PowerFailureLog(value)),

            [1, 0, n, 32, 0]    if n == 32 || n == 52 || n == 72    => Ok(Self::VoltageSags((n - 32) / 20 + 1, Self::parse_num(&value[0])?)),

            [1, 0, n, 36, 0]    if n == 32 || n == 52 || n == 72    => Ok(Self::VoltageSwells((n - 32) / 20 + 1, Self::parse_num(&value[0])?)),

            [0, 0, 96, 13, 0]                                       => Ok(Self::TextMessage(value[0].clone())),

            [1, 0, n, 7, 0]     if n == 32 || n == 52 || n == 72    => Self::parse_num_unit(&value[0]).map(|(v, _)| Self::InstantVoltage((n - 32) / 20 + 1, v)),

            [1, 0, n, 7, 0]     if n == 31 || n == 51 || n == 71    => Self::parse_num_unit(&value[0]).map(|(v, _)| Self::InstantCurrent((n - 31) / 20 + 1, v)),

            [1, 0, n, 7, 0]     if n == 21 || n == 41 || n == 61    => Self::parse_num_unit(&value[0]).map(|(v, _)| Self::InstantPowerDelivered((n - 21) / 20 + 1, v)),

            [1, 0, n, 7, 0]     if n == 22 || n == 42 || n == 62    => Self::parse_num_unit(&value[0]).map(|(v, _)| Self::InstantPowerReceived((n - 22) / 20 + 1, v)),

            [0, n, 24, 1, 0]                                        => Ok(Self::GasEquipmentDeviceType(n, Self::parse_num(&value[0])?)),

            [0, n, 96, 1, 0]                                        => Ok(Self::GasEquipmentIdentifier(n, Self::parse_hex(&value[0])?)),

            [0, n, 24, 2, 1]                                        => Self::parse_num_unit(&value[1]).map(|(v, _)| Self::GasDelivered(n, value[0].clone(), v)),

            _                                                       => Err(format!("Cannot parse OBIS key {key:?}")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Attribute;

    #[test]
    fn test_attribute() -> Result<(), String> {
        let tests = [
            ("1-3:0.2.8(50)",                                               Attribute::Version("50".into())),
            ("0-0:1.0.0(220611162528S)",                                    Attribute::Timestamp("220611162528S".into())),
            ("0-0:96.1.1(4530313233343536373839303132333435)",              Attribute::EquipmentIdentifier("E0123456789012345".into())),
            ("1-0:1.8.1(006024.008*kWh)",                                   Attribute::ElectricityDelivered(1, 6024.008)),
            ("1-0:2.8.1(001375.008*kWh)",                                   Attribute::ElectricityReceived(1, 1375.008)),
            ("0-0:96.14.0(0001)",                                           Attribute::TariffIndicator(1)),
            ("1-0:1.7.0(00.000*kW)",                                        Attribute::ActualPowerDelivered(0.0)),
            ("1-0:2.7.0(03.106*kW)",                                        Attribute::ActualPowerReceived(3.106)),
            ("0-0:96.7.21(00010)",                                          Attribute::PowerFailures(10)),
            ("0-0:96.7.9(00002)",                                           Attribute::PowerFailuresLong(2)),
            ("1-0:99.97.0(1)(0-0:96.7.19)(180228084605W)(0000000486*s)",    Attribute::PowerFailureLog(vec!["1".into(), "0-0:96.7.19".into(), "180228084605W".into(), "0000000486*s".into()])),
            ("1-0:32.32.0(00007)",                                          Attribute::VoltageSags(1, 7)),
            ("1-0:32.36.0(00001)",                                          Attribute::VoltageSwells(1, 1)),
            ("0-0:96.13.0()",                                               Attribute::TextMessage("".into())),
            ("1-0:32.7.0(242.6*V)",                                         Attribute::InstantVoltage(1, 242.6)),
            ("1-0:31.7.0(012*A)",                                           Attribute::InstantCurrent(1, 12)),
            ("1-0:21.7.0(00.000*kW)",                                       Attribute::InstantPowerDelivered(1, 0.0)),
            ("1-0:22.7.0(03.059*kW)",                                       Attribute::InstantPowerReceived(1, 3.059)),
            ("0-1:24.1.0(003)",                                             Attribute::GasEquipmentDeviceType(1, 3)),
            ("0-1:96.1.0(4730313233343536373839303132333435)",              Attribute::GasEquipmentIdentifier(1, "G0123456789012345".into())),
            ("0-1:24.2.1(220611162510S)(03814.705*m3)",                     Attribute::GasDelivered(1, "220611162510S".into(), 03814.705)),
        ];
        for (s, e) in tests {
            assert_eq!(s.parse::<Attribute>()?, e);
        }
        Ok(())
    }
}
