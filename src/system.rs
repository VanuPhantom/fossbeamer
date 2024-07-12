use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::common::Error;

#[cfg(debug_assertions)]
const CPU_INFO_PATH: &'static str = "./cpuinfo";

#[cfg(not(debug_assertions))]
const CPU_INFO_PATH: &'static str = "/proc/cpuinfo";

pub(crate) fn get_cpu_serial() -> Result<String, Error> {
    let file =
        File::open(CPU_INFO_PATH).map_err(|e| Error::FileIoError(CPU_INFO_PATH.into(), e))?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    while let Some(Ok(line)) = lines.next() {
        if line.starts_with("Serial") {
            let (_, serial) = line
                .split_once(':')
                .ok_or(Error::ParsingError(CPU_INFO_PATH.into()))?;

            return Ok(serial.trim().to_string());
        }
    }

    Err(Error::ParsingError(CPU_INFO_PATH.into()))
}
