#[derive(Debug)]
pub(crate) enum RawDuration {
    Hours(f64),
    Minutes(f64),
    Seconds(f64),
}
