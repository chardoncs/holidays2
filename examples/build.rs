use chrono::NaiveDate;
use holidays2::Country;

fn main() -> anyhow::Result<()> {
    // Build an owned database containing 2022 only. This does not touch the shared database
    // used by `holidays2::get` and `holidays2::contains`.
    let map = holidays2::build(Some(2022..2023))?;

    let d = NaiveDate::from_ymd_opt(2022, 1, 1).expect("Invalid date");
    let jp = map.get(&Country::JP).expect("JP should be compiled in");
    let is_holiday = jp.get(&2022).is_some_and(|year| year.contains_key(&d));

    println!("Is {d} a holiday in Japan? Answer is {is_holiday}");

    Ok(())
}
