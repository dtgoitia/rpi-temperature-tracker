use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{prelude::*, BufReader};

use shellexpand;

pub type LineCount = usize;

/// Append `line` to `path` file. If `path` file does not exist, create it and
/// add `header` before appending `line`.
pub fn append_to_file(path: &str, line: &str, header: &str) -> Result<LineCount, std::io::Error> {
    let abs_path = shellexpand::tilde(path).into_owned();
    let file_ro = File::open(&abs_path);

    let count_before: LineCount = match file_ro {
        Ok(file_ro) => {
            let reader = BufReader::new(file_ro);
            reader.lines().count()
        }
        Err(_) => 0,
    };

    let mut file = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open(abs_path)?;

    if count_before == 0 {
        writeln!(file, "{}", header)?;
        return Ok(2);
    }

    writeln!(file, "{}", line)?;

    let count_after = count_before + 1;

    Ok(count_after)
}

pub fn create_nested_dirs(path: &str) -> Result<(), std::io::Error> {
    let abs_path = shellexpand::tilde(path).into_owned();
    fs::create_dir_all(abs_path)
}

pub fn move_dir(old_dir: &str, new_dir: &str) -> Result<(), String> {
    let old_abs_dir = shellexpand::tilde(old_dir).into_owned();
    let new_abs_dir = shellexpand::tilde(new_dir).into_owned();

    fs::rename(old_abs_dir, new_abs_dir).map_err(|err| {
        format!(
            "failed to move {} directory to {}, reason: {}",
            old_dir, new_dir, err,
        )
    })
}
