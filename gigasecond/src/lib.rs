use chrono::{DateTime, Duration, Utc};

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
	let number10 = 10i64;
	start + Duration::seconds(number10.pow(9))
}
