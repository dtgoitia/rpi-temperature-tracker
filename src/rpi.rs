use chrono::{DateTime, Utc};
use std::{fs, io};

const TEMPERATURE_FILE: &str = "/sys/class/thermal/thermal_zone0/temp";

type Temperature = i64;

pub struct TemperatureReading {
    pub t: DateTime<Utc>,
    pub temperature: Temperature,
}

pub fn read_temperature() -> io::Result<TemperatureReading> {
    let t = chrono::Utc::now();

    let content = fs::read_to_string(TEMPERATURE_FILE)?;

    let temperature = content
        .trim()
        .parse::<Temperature>()
        .map_err(|_| {
            format!(
                "expected a positive integer in temperature file, but got instead: {}",
                content
            )
        })
        .map_err(|custom_error_msg| io::Error::new(io::ErrorKind::InvalidData, custom_error_msg))?;

    let reading = TemperatureReading { temperature, t };

    Ok(reading)
}
