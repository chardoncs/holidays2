use chrono::NaiveDate;
use holidays2::Country;

fn main() -> anyhow::Result<()> {
    // NOTE: The holiday database is built on first access and holds the holidays of all the
    // compiled-in countries and generated years in memory. Consider enabling only the
    // `country-*` features you need, or use `holidays2::build` to limit the years.
    let d = NaiveDate::from_ymd_opt(2022, 1, 1).expect("Invalid date");
    println!(
        "Is {d} a holiday in Japan? Answer is {}",
        holidays2::contains(Country::JP, d)?
    );

    println!("{:?}", holidays2::get(Country::JP, d)?.unwrap());

    Ok(())
}
