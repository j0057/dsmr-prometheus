use std::io::{BufReader, Read, BufRead};

use anyhow::anyhow;

use log::{debug, warn};
use crc16::{State, ARC};

use crate::attribute::Attribute;

#[derive(Debug)]
pub struct Telegram {
    pub header: String,
    pub elements: Vec<Attribute>
}

impl Telegram {
    fn new<T: AsRef<str>>(data: &[T]) -> Result<Self, anyhow::Error> {
        let result = Telegram {
            header: data[0].as_ref()[1..].trim_end().into(),
            elements: data.iter()
                .skip(2)
                .map(|e| e.as_ref().trim_end().parse())
                .collect::<Result<Vec<Attribute>, anyhow::Error>>()?,
        };
        Ok(result)
    }

    pub fn from<S: Read>(reader: &mut BufReader<S>) -> Result<Telegram, anyhow::Error> {
        let mut result = vec![];
        let mut crc16 = State::<ARC>::new();

        loop {
            // read a line
            let mut line = String::new();
            if reader.read_line(&mut line)? == 0 {
                return Err(anyhow!("Unexpected EOF reached"));
            }

            // line is not last line: update CRC16-ARC and store it
            if ! line.starts_with('!') {
                crc16.update(line.as_bytes());
                result.push(line);
                continue;
            }
            else {
                crc16.update(b"!");
            }

            // bad CRC16-ARC: complain and reset and attempt again
            if line != format!("!{:04X}\r\n", crc16.get()) {
                debug!("{result:?} {line:?} {:04X}", crc16.get());
                warn!("CRC mismatch; resyncing");
                result = vec![];
                crc16 = State::<ARC>::new();
                // TODO: prevent endless loop here by trying a reasonable number of times
                continue;
            }

            // good CRC16-ARC: instantiate new Telegram
            return Telegram::new(&result);
        }
    }
}
