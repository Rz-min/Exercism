use time::PrimitiveDateTime as DateTime;
use chrono::*;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {

    let gigasecond = time::Duration::new(1_000_000_000, 0);

    start + gigasecond
}
