use std::{
    fs::File,
    io::{BufReader, Read},
};

use chrono::{TimeZone, Utc};

/// Read file and return the contents as String
/// Path is provided in args, hence errors are unexpected behavior
/// and program will forcibly quit if they are encountered
pub fn read_file(path: &str) -> String {
    let mut result = String::new();

    let file = File::open(path).expect("Unable to read file");
    let mut buf_reader = BufReader::new(file);
    buf_reader
        .read_to_string(&mut result)
        .expect("Failed to read file");

    result
}

/// Takes hour from 24-hr format clock, and converts to Unix Epoch value
pub fn epoch_time(hour: u32) -> i64 {
    let today = Utc::now().date_naive();
    let time = Utc.from_utc_datetime(&today.and_hms_opt(hour, 0, 0).unwrap());

    time.timestamp()
}
