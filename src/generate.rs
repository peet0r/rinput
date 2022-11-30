use anyhow::Result;
use evdev::Key;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::utils::get_key_from_str;

pub fn parse_file(path: String) -> Result<Vec<Key>> {
    //Open file
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Get list of keys
    let mut keys = Vec::new();
    for line in reader.lines().flatten() {
        keys.push(get_key_from_str(&line)?);
    }

    Ok(keys)
}
