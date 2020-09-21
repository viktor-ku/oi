use lib_duration::{duration, Duration};
use pretty_assertions::assert_eq;
mod utils;

#[test]
fn sub_hours() {
    assert_eq!(duration("2h -1 hour", now!()), Ok(Duration::new(1, 0, 0)));
}

#[test]
fn sub_minutes() {
    assert_eq!(
        duration("30 minutes -10m", now!()),
        Ok(Duration::new(0, 20, 0))
    );
}

#[test]
fn sub_seconds() {
    assert_eq!(duration("50 sec -1s", now!()), Ok(Duration::new(0, 0, 49)));
}

#[test]
fn sub_1s_from_1s() {
    assert_eq!(duration("1s -1s", now!()), Ok(Duration::new(0, 0, 0)));
}

#[test]
fn below_zero() {
    assert_eq!(duration("1s -2s", now!()), Ok(Duration::new(0, 0, 0)));
}

#[test]
fn way_below_zero() {
    assert_eq!(
        duration("1s -200 hours -100m -400s", now!()),
        Ok(Duration::new(0, 0, 0))
    );
}

#[test]
fn sub_1s_from_1m() {
    assert_eq!(duration("1m -1s", now!()), Ok(Duration::new(0, 0, 59)));
}

#[test]
fn sub_1s_from_1h() {
    assert_eq!(duration("1h -1s", now!()), Ok(Duration::new(0, 59, 59)));
}

#[test]
fn sub_1m_from_1h() {
    assert_eq!(duration("1h -1m", now!()), Ok(Duration::new(0, 59, 0)));
}

#[test]
fn sub_trio() {
    assert_eq!(
        duration("2h -1h -1m -1s", now!()),
        Ok(Duration::new(0, 58, 59))
    );
}

#[test]
fn sub_seconds_from_exact_time() {
    assert_eq!(
        duration("oh wow at 1pm -45 secs", &time! {11:00-30}),
        Ok(Duration::new(1, 58, 45))
    );
}

#[test]
fn sub_minutes_from_exact_time() {
    assert_eq!(
        duration("look at him go at 10:45 -5 min", &time! {10:10}),
        Ok(Duration::new(0, 30, 0))
    );
}

#[test]
fn sub_hours_from_exact_time() {
    assert_eq!(
        duration("should be -1h starting at 22", &time! {12:00}),
        Ok(Duration::new(9, 0, 0))
    );
}

#[test]
fn sub_trio_from_exact_time() {
    assert_eq!(
        duration(
            r###"
                event was planned to begin at 6pm
                but they moved it 1 hour and 30 minutes further
                I need exactly -2 hours -25 minutes to get there
                and additional -300 seconds to get to the classroom
                reserve another -.5h to spare while on my way
            "###,
            &time! {14:00}
        ),
        Ok(Duration::new(2, 30, 0))
    );
}

#[test]
fn sub_then_exact_time() {
    assert_eq!(
        duration("-1h at 23", &time! {20:00}),
        Ok(Duration::new(2, 0, 0))
    );
}

#[test]
fn minus_10h_at_23() {
    assert_eq!(
        duration("-10 hours at 23", &time! {23:30}),
        Ok(Duration::new(13, 30, 0))
    );
}

#[test]
fn minus_10h_at_10_pm() {
    assert_eq!(
        duration("-10h at 10pm", &time! {00:00}),
        Ok(Duration::new(12, 0, 0))
    );
}

#[test]
fn minus_10h_at_10_am() {
    assert_eq!(
        duration("-10h at 10am", &time! {00:00}),
        Ok(Duration::new(0, 0, 0))
    );
}

#[test]
fn minus_20h_at_10_am() {
    assert_eq!(
        duration("-20h at 10am", &time! {00:00}),
        Ok(Duration::new(0, 0, 0))
    );
}
