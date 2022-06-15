use std::io::{self, BufReader, Read, BufRead};

use crc16::{State, ARC};

use crate::attribute::Attribute;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("I/O error reading frame: {0}")]
    IO(#[from] io::Error),
    #[error("Reached EOF reading from source")]
    EOF,
    #[error("Attribute error: {0}")]
    Attribute(String)
}

#[derive(Debug)]
pub struct Telegram {
    pub header: String,
    pub elements: Vec<Attribute>
}

impl Telegram {
    fn new(data: Vec<String>) -> Result<Self, Error> {
        let result = Telegram {
            header: data[0][1..].trim_end().into(),
            elements: data.iter()
                .skip(2)
                .map(|e| e.trim_end().parse())
                .collect::<Result<Vec<Attribute>, String>>()
                .map_err(|s| Error::Attribute(s))?,
        };
        Ok(result)
    }

    pub fn from<S: Read>(reader: &mut BufReader<S>) -> Result<Telegram, Error> {
        let mut result = vec![];
        let mut crc16 = State::<ARC>::new();

        loop {
            // read a line
            let mut line = String::new();
            if reader.read_line(&mut line)? == 0 {
                return Err(Error::EOF);
            }

            // line is not last line: update CRC16-ARC and store it
            if ! line.starts_with('!') {
                crc16.update(line.as_bytes());
                result.push(line);
                continue;
            }

            // bad CRC16-ARC: complain and reset and attempt again
            crc16.update(b"!");
            if line != format!("!{:04X}\r\n", crc16.get()) {
                eprintln!("{result:?} {line:?} {:04X}", crc16.get());
                eprintln!("CRC mismatch; resyncing");
                result = vec![];
                crc16 = State::<ARC>::new();
                // TODO: prevent endless loop here by trying a reasonable number of times
                continue;
            }

            // good CRC16-ARC: instantiate new Telegram
            return Ok(Telegram::new(result)?);
        }
    }
}
