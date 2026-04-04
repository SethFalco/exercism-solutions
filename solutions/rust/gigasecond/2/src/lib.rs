use time::{PrimitiveDateTime as DateTime, Duration};

const GIGASECOND: Duration = Duration::seconds(1000000000);

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    start + GIGASECOND
}
