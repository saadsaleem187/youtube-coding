use std::{
    fs::OpenOptions,
    io::{self, Write},
    time::{SystemTime, UNIX_EPOCH},
};

enum LogLevel {
    Info,
    Warn,
    Error,
}

fn main() -> Result<(), io::Error> {
    log(LogLevel::Info, "Application started...")?;
    log(LogLevel::Warn, "Low disk space")?;
    log(LogLevel::Error, "Could not connect to the server")?;

    Ok(())
}

fn timestamp() -> Result<String, io::Error> {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    let secs = duration.as_secs();

    let day_total_secs = 86400;

    let days_since_epoch = secs / day_total_secs;
    let secs_today = secs % day_total_secs;

    let hours = (secs_today / 3600) % 24;
    let minutes = (secs_today / 60) % 60;
    let seconds = secs_today % 60;

    // Approximate year, month and day
    let mut year = 1970;
    let mut remaining_days = days_since_epoch;

    loop {
        let days_in_year = if is_leap_year(year) { 366 } else { 365 };

        if remaining_days >= days_in_year {
            remaining_days -= days_in_year;
            year += 1;
        } else {
            break;
        }
    }

    let days_in_month = if is_leap_year(year) {
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };

    let mut month = 1;

    for &days in &days_in_month {
        if remaining_days >= days {
            remaining_days -= days;
            month += 1;
        } else {
            break;
        }
    }

    let day = remaining_days + 1;

    Ok(format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
        year, month, day, hours, minutes, seconds
    ))
}

fn is_leap_year(year: u64) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

fn log(level: LogLevel, msg: &str) -> Result<(), io::Error> {
    let level_str = match level {
        LogLevel::Info => "[INFO]",
        LogLevel::Warn => "[WARN]",
        LogLevel::Error => "[ERROR]",
    };

    let ts = timestamp()?;

    let final_msg = format!("{} {} {}\n", ts, level_str, msg);

    print!("{}", final_msg);

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("app.log")
        .unwrap();

    file.write_all(final_msg.as_bytes()).unwrap();

    Ok(())
}
