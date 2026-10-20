mod build;
mod country;
mod iter;
mod prelude;

pub use prelude::*;

use chrono::{Datelike, NaiveDate};
use std::{
    collections::{BTreeMap, HashMap},
    ops::Range,
    sync::LazyLock,
};

/// Type alias for Holiday map.
pub type HolidayPerCountryMap = HashMap<Year, BTreeMap<NaiveDate, Holiday>>;
pub type HolidayMap = HashMap<Country, HolidayPerCountryMap>;

/// Type alias for Year.
pub type Year = i32;

/// Result type of this crate.
pub type Result<T> = std::result::Result<T, Error>;

/// Holiday database shared by [`get`], [`contains`] and [`iter`].
///
/// It is populated on first access and lives for the rest of the program, so
/// there is no initialization step to call. Which countries are compiled in is
/// decided at build time by the `country-*` Cargo features.
static DATA: LazyLock<HolidayMap> =
    LazyLock::new(|| build::build(None).expect("failed to build the holiday database"));

/// Get the holiday database shared by this crate.
///
/// The database holds every compiled-in country and every generated year and is
/// built on first access. Prefer this over [`build`] unless you need a map
/// limited to a range of years.
pub fn data() -> &'static HolidayMap {
    LazyLock::force(&DATA)
}

/// Build a holiday database from scratch.
///
/// Pass `Some(range)` to only include the given years, or `None` to include
/// every generated year. The database built here is owned by the caller and is
/// independent of the one returned by [`data`], which makes it usable in a
/// single threaded context or when only a few years are needed.
pub fn build(years: Option<Range<Year>>) -> Result<HolidayMap> {
    build::build(years.as_ref())
}

/// Get holiday by ISO 3166-1 alpha-2 country code and date. If the specified country or year is
/// not available, it will return `Err(Error)`. If the specified date is not a holiday, it
/// will return `Ok(None)`. Otherwise, it will return `Ok(Some(Holiday))`.
pub fn get(country: Country, date: NaiveDate) -> Result<Option<Holiday>> {
    let Some(map) = DATA.get(&country) else {
        return Err(Error::CountryNotAvailable);
    };

    let Some(map) = map.get(&date.year()) else {
        return Err(Error::YearNotAvailable);
    };

    Ok(map.get(&date).cloned())
}

/// Check if the specified date is a holiday. If the specified country or year is
/// not available, it will return `Err(Error)`. If the date is not a holiday, it
/// will return `Ok(false)`. Otherwise, it will return `Ok(true)`.
pub fn contains(country: Country, date: NaiveDate) -> Result<bool> {
    let Some(map) = DATA.get(&country) else {
        return Err(Error::CountryNotAvailable);
    };

    let Some(map) = map.get(&date.year()) else {
        return Err(Error::YearNotAvailable);
    };

    Ok(map.get(&date).is_some())
}

/// Represents a holiday.
#[derive(Debug, Clone)]
pub struct Holiday {
    /// two-letter country code defined in ISO 3166-1 alpha-2.
    pub code: Country,
    /// Country name.
    pub country: String,
    /// Date of holiday.
    pub date: chrono::NaiveDate,
    /// Name of holiday.
    pub name: String,
}

impl Holiday {
    fn new(
        code: Country,
        country: impl Into<String>,
        date: NaiveDate,
        name: impl Into<String>,
    ) -> Holiday {
        Holiday {
            code,
            country: country.into(),
            date,
            name: name.into(),
        }
    }
}

/// Error type of this crate.
#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum Error {
    /// Holiday is not available for this country.
    #[error("Holiday is not available for this country")]
    CountryNotAvailable,
    /// Holiday is not available for this year.
    #[error("Holiday is not available for this year")]
    YearNotAvailable,
    /// Unexpexted error occurred.
    #[error("Unexpexted error occurred: {0}")]
    Unexpected(String),
}

/// Helper trait used inside the crate and tests.
#[doc(hidden)]
pub trait NaiveDateExt {
    fn from_ymd_res(year: i32, month: u32, day: u32) -> Result<NaiveDate> {
        NaiveDate::from_ymd_opt(year, month, day)
            .ok_or_else(|| Error::Unexpected(format!("invalid date {year}-{month}-{day}")))
    }
}

impl NaiveDateExt for NaiveDate {}
