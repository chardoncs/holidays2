use chrono::{Datelike, NaiveDate};

use crate::{DATA, Error, Holiday, Result, prelude::*};
use std::collections::VecDeque;

#[derive(Debug)]
pub struct Iter {
    since: NaiveDate,
    until: NaiveDate,
    buf: VecDeque<Holiday>,
}

impl std::iter::Iterator for Iter {
    type Item = Holiday;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.buf.pop_front()?;

        if next.date < self.since {
            return self.next();
        }

        if next.date < self.until {
            Some(next)
        } else {
            None
        }
    }
}

/// Iterate holidays by dates.
#[allow(dead_code)]
pub fn iter(country: Country, since: NaiveDate, until: NaiveDate) -> Result<Iter> {
    let mut buf = VecDeque::new();

    let Some(map) = DATA.get(&country) else {
        return Err(Error::CountryNotAvailable);
    };

    let mut y = since.year();
    while y <= until.year() {
        let Some(year_map) = map.get(&y) else {
            break;
        };

        buf.extend(year_map.values().cloned());

        y += 1;
    }

    Ok(Iter { since, until, buf })
}
