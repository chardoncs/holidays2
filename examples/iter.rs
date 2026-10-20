use chrono::NaiveDate;
use holidays2::Country;

fn main() -> anyhow::Result<()> {
    let s = NaiveDate::from_ymd_opt(2022, 1, 1).expect("Invalid date");
    let u = NaiveDate::from_ymd_opt(2023, 1, 1).expect("Invalid date");

    for holiday in holidays2::iter(Country::JP, s, u)?.map(|h| h.date) {
        println!("{holiday:?}",);
    }

    Ok(())
}
