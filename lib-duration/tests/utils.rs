#[macro_export]
macro_rules! time {
    ($h:literal:$m:literal-$s:literal) => {{
        use chrono::{Local, TimeZone};
        Local.ymd(2020, 9, 30).and_hms($h, $m, $s)
    }};

    ($h:literal:$m:literal) => {{
        use chrono::{Local, TimeZone};
        Local.ymd(2020, 9, 30).and_hms($h, $m, 0)
    }};
}
