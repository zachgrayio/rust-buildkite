//! Debug logging utilities for proc macros.

macro_rules! debug_log {
    ($module:expr, $($arg:tt)*) => {
        if std::env::var("RUST_LOG").is_ok() {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap();
            let secs = now.as_secs();
            let datetime = crate::debug::chrono_lite(secs);
            eprintln!("[{} DEBUG rust_buildkite_macros::{}] {}", datetime, $module, format_args!($($arg)*));
        }
    };
}

pub(crate) use debug_log;

pub fn chrono_lite(secs: u64) -> String {
    const SECS_PER_MIN: u64 = 60;
    const SECS_PER_HOUR: u64 = 3600;
    const SECS_PER_DAY: u64 = 86400;
    const DAYS_PER_YEAR: u64 = 365;
    const DAYS_PER_4Y: u64 = 1461;
    const DAYS_PER_100Y: u64 = 36524;
    const DAYS_PER_400Y: u64 = 146097;

    let mut days = secs / SECS_PER_DAY;
    let time_of_day = secs % SECS_PER_DAY;

    let hours = time_of_day / SECS_PER_HOUR;
    let minutes = (time_of_day % SECS_PER_HOUR) / SECS_PER_MIN;
    let seconds = time_of_day % SECS_PER_MIN;

    let mut year = 1970i32;
    let cycles_400 = days / DAYS_PER_400Y;
    days %= DAYS_PER_400Y;
    year += (cycles_400 * 400) as i32;

    let cycles_100 = days / DAYS_PER_100Y;
    days %= DAYS_PER_100Y;
    year += (cycles_100 * 100) as i32;

    let cycles_4 = days / DAYS_PER_4Y;
    days %= DAYS_PER_4Y;
    year += (cycles_4 * 4) as i32;

    let years_remaining = days / DAYS_PER_YEAR;
    days %= DAYS_PER_YEAR;
    year += years_remaining as i32;

    let is_leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    let days_in_months: [u64; 12] = if is_leap {
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };

    let mut month = 0u32;
    for (i, &d) in days_in_months.iter().enumerate() {
        if days < d {
            month = (i + 1) as u32;
            break;
        }
        days -= d;
    }
    let day = days + 1;

    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        year, month, day, hours, minutes, seconds
    )
}
