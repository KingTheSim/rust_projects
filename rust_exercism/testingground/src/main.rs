use time::PrimitiveDateTime as DateTime;
use time::Duration;
use time::OffsetDateTime;
fn main() {
    let now_odt = OffsetDateTime::now_utc();
    let now_pdt = DateTime::new(now_odt.date(), now_odt.time());

    after(now_pdt);
}
// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let jump = 1000 * 1000 * 1000;
    let start = start + Duration::seconds(jump);
    println!("What time is a gigasecond later than {start}");
    start
}