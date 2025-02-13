use std::{path::Path, process, time::Duration};

mod io;
mod rpi;
mod scheduler;

// Data will be quickly appended into this file.
const HEAD_FILE: &str = "~/.local/share/rpi-temperature-tracker/head";
const HEAD_FILE_HEADER: &str = "datetime,temp_mC";
const SAMPLING_INTERVAL: Milliseconds = 200;
const MAX_LINES_PER_HEAD_FILE: HeadFileLineCount = 300; // 1-min worth of data

// When the HEAD file grows enough, it's atomically moved to this directory for
// off-band processing
const ARCHIVES_DIR: &str = "~/.local/share/rpi-temperature-tracker/archives/raw";

type HeadFileLineCount = io::LineCount;
type Milliseconds = u64;

fn main() -> () {
    if let Err(error) = initialize_data_directories() {
        return exit_with_error(error);
    };

    let interval = Duration::from_millis(SAMPLING_INTERVAL);
    if let Err(error) = scheduler::run_on_interval(action, interval) {
        return exit_with_error(error);
    };
}

fn exit_with_error(message: String) {
    println!("{}", message);
    process::exit(-1);
}

fn initialize_data_directories() -> Result<(), String> {
    if Path::new(ARCHIVES_DIR).exists() {
        return Ok(());
    }

    io::create_nested_dirs(ARCHIVES_DIR).map_err(|err| {
        format!(
            "failed to create data root directory at {}, reason: {}",
            ARCHIVES_DIR, err,
        )
    })?;

    Ok(())
}

fn action() -> Result<(), String> {
    let reading = rpi::read_temperature().map_err(|err| err.to_string())?;

    let count = write_to_head(reading)?;
    if count >= MAX_LINES_PER_HEAD_FILE {
        archive_head()?;
    }

    Ok(())
}

trait ToCsv {
    fn to_csv(&self) -> String;
}

impl ToCsv for rpi::TemperatureReading {
    fn to_csv(&self) -> String {
        return format!("{},{}", self.t.to_rfc3339(), self.temperature);
    }
}

fn write_to_head(reading: rpi::TemperatureReading) -> Result<HeadFileLineCount, String> {
    let line = reading.to_csv();
    let line_count_after_update = io::append_to_file(HEAD_FILE, &line, HEAD_FILE_HEADER)
        .map_err(|err| format!("failed to append file to {}, reason: {}", HEAD_FILE, err))?;
    Ok(line_count_after_update)
}

fn archive_head() -> Result<(), String> {
    let now_timestamp = chrono::Utc::now().timestamp();
    let old_dir = HEAD_FILE;
    let new_dir = format!("{}/archived-on-{}", ARCHIVES_DIR, now_timestamp);
    io::move_dir(old_dir, &new_dir)
        .map_err(|err| format!("failed to archive head file, reason: {}", err))?;
    Ok(())
}
