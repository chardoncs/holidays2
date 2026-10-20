use chrono::NaiveDate;
use holidays2::{Country, Error, NaiveDateExt, build, contains, data, get, iter};

#[test]
fn get_holiday() -> anyhow::Result<()> {
    let d = NaiveDate::from_ymd_res(2022, 1, 1)?;
    assert_eq!("元日", get(Country::JP, d)?.unwrap().name);
    assert!(contains(Country::JP, d)?);

    let d = NaiveDate::from_ymd_res(2023, 1, 1)?;
    assert_eq!("元日", get(Country::JP, d)?.unwrap().name);
    assert!(contains(Country::JP, d)?);

    let d = NaiveDate::from_ymd_res(2022, 1, 1)?;
    assert_eq!("New Year's Day", get(Country::US, d)?.unwrap().name);
    assert!(contains(Country::US, d)?);

    Ok(())
}

#[test]
fn get_none_for_non_holiday() -> anyhow::Result<()> {
    // Not a public holiday in Japan.
    let d = NaiveDate::from_ymd_res(2022, 1, 2)?;
    assert!(get(Country::JP, d)?.is_none());
    assert!(!contains(Country::JP, d)?);

    Ok(())
}

#[test]
fn get_year_not_available() -> anyhow::Result<()> {
    // The database is generated from 2001 to 2030.
    let d = NaiveDate::from_ymd_res(1990, 1, 1)?;
    assert_eq!(Error::YearNotAvailable, get(Country::JP, d).unwrap_err());
    assert_eq!(
        Error::YearNotAvailable,
        contains(Country::JP, d).unwrap_err()
    );

    Ok(())
}

#[test]
fn data_is_shared() {
    // There is no initialization step: the database is built on first access.
    assert!(data().contains_key(&Country::JP));
    assert!(data().contains_key(&Country::US));
}

#[test]
fn build_by_year() -> anyhow::Result<()> {
    let map = build(Some(2022..2023))?;

    let jp = map.get(&Country::JP).expect("JP should be built");
    assert!(jp.contains_key(&2022));
    assert!(!jp.contains_key(&2023));

    let us = map.get(&Country::US).expect("US should be built");
    assert!(us.contains_key(&2022));
    assert!(!us.contains_key(&2023));

    Ok(())
}

#[test]
fn build_all_years() -> anyhow::Result<()> {
    let map = build(None)?;

    // Building without a range gives the same database as the shared one.
    assert_eq!(map.len(), data().len());
    assert!(map[&Country::JP].contains_key(&2030));

    Ok(())
}

#[test]
fn iterate() -> anyhow::Result<()> {
    let s = NaiveDate::from_ymd_res(2022, 1, 1)?;
    let u = NaiveDate::from_ymd_res(2023, 1, 1)?;
    assert_eq!(
        vec![
            NaiveDate::from_ymd_res(2022, 1, 1)?,
            NaiveDate::from_ymd_res(2022, 1, 10)?,
            NaiveDate::from_ymd_res(2022, 2, 11)?,
            NaiveDate::from_ymd_res(2022, 2, 23)?,
            NaiveDate::from_ymd_res(2022, 3, 21)?,
            NaiveDate::from_ymd_res(2022, 4, 29)?,
            NaiveDate::from_ymd_res(2022, 5, 3)?,
            NaiveDate::from_ymd_res(2022, 5, 4)?,
            NaiveDate::from_ymd_res(2022, 5, 5)?,
            NaiveDate::from_ymd_res(2022, 7, 18)?,
            NaiveDate::from_ymd_res(2022, 8, 11)?,
            NaiveDate::from_ymd_res(2022, 9, 19)?,
            NaiveDate::from_ymd_res(2022, 9, 23)?,
            NaiveDate::from_ymd_res(2022, 10, 10)?,
            NaiveDate::from_ymd_res(2022, 11, 3)?,
            NaiveDate::from_ymd_res(2022, 11, 23)?,
        ],
        iter(Country::JP, s, u)?.map(|h| h.date).collect::<Vec<_>>()
    );

    Ok(())
}

#[test]
fn iterate_without_holidays() -> anyhow::Result<()> {
    let s = NaiveDate::from_ymd_res(1990, 1, 1)?;
    let u = NaiveDate::from_ymd_res(1991, 1, 1)?;
    assert_eq!(
        Vec::<NaiveDate>::new(),
        iter(Country::JP, s, u)?.map(|h| h.date).collect::<Vec<_>>()
    );

    Ok(())
}
