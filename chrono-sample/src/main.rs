extern crate chrono;

use chrono::{DateTime, FixedOffset, Local, Utc};

fn main() {
    let utc: DateTime<Utc> = Utc::now();
    println!("Now UTC  : {:?}", utc);

    let local: DateTime<Local> = Local::now();
    println!("Now Local: {:?}", local);

    let time = DateTime::parse_from_rfc2822("Mon, 28 Jan 2008 10:30:00 +0900").unwrap();
    println!("Datetime : {:?}", time);

    let tz = FixedOffset::east(9 * 3600);
    let new_utc = utc.with_timezone(&tz);
    println!("New UTC  : {:?}", new_utc);

    let fmt = new_utc.format("%Y/%m/%d %H:%M:%S%z");
    println!("Formatted date: {}", fmt);
}
