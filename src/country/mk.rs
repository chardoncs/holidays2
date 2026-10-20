//! North Macedonia
use super::*;

/// Generate holiday map for North Macedonia.
#[allow(unused_mut, unused_variables)]
pub fn build(
    years: &Option<&std::ops::Range<Year>>,
) -> Result<HashMap<Year, BTreeMap<NaiveDate, Holiday>>> {
    let mut map = HashMap::new();

    build_year(
        years,
        2001,
        vec![
            (NaiveDate::from_ymd_res(2001, 1, 1)?, "Нова Година"),
            (NaiveDate::from_ymd_res(2001, 1, 2)?, "Нова Година"),
            (NaiveDate::from_ymd_res(2001, 5, 1)?, "Ден на трудот"),
            (NaiveDate::from_ymd_res(2001, 5, 2)?, "Ден на трудот"),
            (NaiveDate::from_ymd_res(2001, 8, 2)?, "Ден на Републиката"),
            (NaiveDate::from_ymd_res(2001, 9, 8)?, "Ден на независноста"),
            (
                NaiveDate::from_ymd_res(2001, 10, 11)?,
                "Ден на народното востание",
            ),
        ],
        &mut map,
        Country::MK,
        "North Macedonia",
    );

    build_year(
        years,
        2002,
        vec![
            (NaiveDate::from_ymd_res(2002, 1, 1)?, "Нова Година"),
            (NaiveDate::from_ymd_res(2002, 1, 2)?, "Нова Година"),
            (NaiveDate::from_ymd_res(2002, 5, 1)?, "Ден на трудот"),
            (NaiveDate::from_ymd_res(2002, 5, 2)?, "Ден на трудот"),
            (NaiveDate::from_ymd_res(2002, 8, 2)?, "Ден на Републиката"),
            (NaiveDate::from_ymd_res(2002, 9, 8)?, "Ден на независноста"),
            (
                NaiveDate::from_ymd_res(2002, 9, 9)?,
                "Ден на независноста (неработен ден)",
            ),
            (
                NaiveDate::from_ymd_res(2002, 10, 11)?,
                "Ден на народното востание",
            ),
        ],
        &mut map,
        Country::MK,
        "North Macedonia",
    );

    build_year(
        years,
        2003,
        vec![
            (NaiveDate::from_ymd_res(2003, 1, 1)?, "Нова Година"),
            (NaiveDate::from_ymd_res(2003, 1, 2)?, "Нова Година"),
            (NaiveDate::from_ymd_res(2003, 5, 1)?, "Ден на трудот"),
            (NaiveDate::from_ymd_res(2003, 5, 2)?, "Ден на трудот"),
            (NaiveDate::from_ymd_res(2003, 8, 2)?, "Ден на Републиката"),
            (NaiveDate::from_ymd_res(2003, 9, 8)?, "Ден на независноста"),
            (
                NaiveDate::from_ymd_res(2003, 10, 11)?,
                "Ден на народното востание",
            ),
        ],
        &mut map,
        Country::MK,
        "North Macedonia",
    );

    build_year(
        years,
        2004,
        vec![
            (NaiveDate::from_ymd_res(2004, 1, 1)?, "Нова Година"),
            (NaiveDate::from_ymd_res(2004, 1, 2)?, "Нова Година"),
            (NaiveDate::from_ymd_res(2004, 5, 1)?, "Ден на трудот"),
            (NaiveDate::from_ymd_res(2004, 5, 2)?, "Ден на трудот"),
            (
                NaiveDate::from_ymd_res(2004, 5, 3)?,
                "Ден на трудот (неработен ден)",
            ),
            (NaiveDate::from_ymd_res(2004, 8, 2)?, "Ден на Републиката"),
            (NaiveDate::from_ymd_res(2004, 9, 8)?, "Ден на независноста"),
            (
                NaiveDate::from_ymd_res(2004, 10, 11)?,
                "Ден на народното востание",
            ),
        ],
        &mut map,
        Country::MK,
        "North Macedonia",
    );

    build_year(
        years,
        2005,
        vec![
            (NaiveDate::from_ymd_res(2005, 1, 1)?, "Нова Година"),
            (NaiveDate::from_ymd_res(2005, 1, 2)?, "Нова Година"),
            (
                NaiveDate::from_ymd_res(2005, 1, 3)?,
                "Нова Година (неработен ден)",
            ),
            (NaiveDate::from_ymd_res(2005, 5, 1)?, "Ден на трудот"),
            (
                NaiveDate::from_ymd_res(2005, 5, 2)?,
                "Ден на трудот; Ден на трудот (неработен ден)",
            ),
            (NaiveDate::from_ymd_res(2005, 8, 2)?, "Ден на Републиката"),
            (NaiveDate::from_ymd_res(2005, 9, 8)?, "Ден на независноста"),
            (
                NaiveDate::from_ymd_res(2005, 10, 11)?,
                "Ден на народното востание",
            ),
        ],
        &mut map,
        Country::MK,
        "North Macedonia",
    );

    build_year(
        years,
        2006,
        vec![
            (NaiveDate::from_ymd_res(2006, 1, 1)?, "Нова Година"),
            (
                NaiveDate::from_ymd_res(2006, 1, 2)?,
                "Нова Година; Нова Година (неработен ден)",
            ),
            (NaiveDate::from_ymd_res(2006, 5, 1)?, "Ден на трудот"),
            (NaiveDate::from_ymd_res(2006, 5, 2)?, "Ден на трудот"),
            (NaiveDate::from_ymd_res(2006, 8, 2)?, "Ден на Републиката"),
            (NaiveDate::from_ymd_res(2006, 9, 8)?, "Ден на независноста"),
            (
                NaiveDate::from_ymd_res(2006, 10, 11)?,
                "Ден на народното востание",
            ),
        ],
        &mut map,
        Country::MK,
        "North Macedonia",
    );

    build_year(
        years,
        2007,
        vec![
            (NaiveDate::from_ymd_res(2007, 1, 1)?, "Нова Година"),
            (NaiveDate::from_ymd_res(2007, 1, 2)?, "Нова Година"),
            (NaiveDate::from_ymd_res(2007, 4, 9)?, "Велигден"),
            (NaiveDate::from_ymd_res(2007, 5, 1)?, "Ден на трудот"),
            (
                NaiveDate::from_ymd_res(2007, 5, 24)?,
                "Светите Кирил и Методиј - Ден на сесловенските просветители",
            ),
            (NaiveDate::from_ymd_res(2007, 8, 2)?, "Ден на Републиката"),
            (NaiveDate::from_ymd_res(2007, 9, 8)?, "Ден на независноста"),
            (
                NaiveDate::from_ymd_res(2007, 10, 11)?,
                "Ден на народното востание",
            ),
            (
                NaiveDate::from_ymd_res(2007, 10, 23)?,
                "Ден на македонската револуционерна борба",
            ),
            (
                NaiveDate::from_ymd_res(2007, 12, 8)?,
                "Свети Климент Охридски",
            ),
            (
                NaiveDate::from_ymd_res(2007, 10, 13)?,
                "Рамазан Бајрам (проценето)",
            ),
        ],
        &mut map,
        Country::MK,
        "North Macedonia",
    );

    build_year(
        years,
        2008,
        vec![
            (NaiveDate::from_ymd_res(2008, 1, 1)?, "Нова Година"),
            (NaiveDate::from_ymd_res(2008, 1, 7)?, "Божиќ"),
            (NaiveDate::from_ymd_res(2008, 4, 28)?, "Велигден"),
            (NaiveDate::from_ymd_res(2008, 5, 1)?, "Ден на трудот"),
            (
                NaiveDate::from_ymd_res(2008, 5, 24)?,
                "Светите Кирил и Методиј - Ден на сесловенските просветители",
            ),
            (NaiveDate::from_ymd_res(2008, 8, 2)?, "Ден на Републиката"),
            (NaiveDate::from_ymd_res(2008, 9, 8)?, "Ден на независноста"),
            (
                NaiveDate::from_ymd_res(2008, 10, 11)?,
                "Ден на народното востание",
            ),
            (
                NaiveDate::from_ymd_res(2008, 10, 23)?,
                "Ден на македонската револуционерна борба",
            ),
            (
                NaiveDate::from_ymd_res(2008, 12, 8)?,
                "Свети Климент Охридски",
            ),
            (
                NaiveDate::from_ymd_res(2008, 10, 1)?,
                "Рамазан Бајрам (проценето)",
            ),
        ],
        &mut map,
        Country::MK,
        "North Macedonia",
    );

    build_year(
        years,
        2009,
        vec![
            (NaiveDate::from_ymd_res(2009, 1, 1)?, "Нова Година"),
            (NaiveDate::from_ymd_res(2009, 1, 7)?, "Божиќ"),
            (NaiveDate::from_ymd_res(2009, 4, 20)?, "Велигден"),
            (NaiveDate::from_ymd_res(2009, 5, 1)?, "Ден на трудот"),
            (
                NaiveDate::from_ymd_res(2009, 5, 24)?,
                "Светите Кирил и Методиј - Ден на сесловенските просветители",
            ),
            (
                NaiveDate::from_ymd_res(2009, 5, 25)?,
                "Светите Кирил и Методиј - Ден на сесловенските просветители (неработен ден)",
            ),
            (NaiveDate::from_ymd_res(2009, 8, 2)?, "Ден на Републиката"),
            (
                NaiveDate::from_ymd_res(2009, 8, 3)?,
                "Ден на Републиката (неработен ден)",
            ),
            (NaiveDate::from_ymd_res(2009, 9, 8)?, "Ден на независноста"),
            (
                NaiveDate::from_ymd_res(2009, 10, 11)?,
                "Ден на народното востание",
            ),
            (
                NaiveDate::from_ymd_res(2009, 10, 12)?,
                "Ден на народното востание (неработен ден)",
            ),
            (
                NaiveDate::from_ymd_res(2009, 10, 23)?,
                "Ден на македонската револуционерна борба",
            ),
            (
                NaiveDate::from_ymd_res(2009, 12, 8)?,
                "Свети Климент Охридски",
            ),
            (
                NaiveDate::from_ymd_res(2009, 9, 20)?,
                "Рамазан Бајрам (проценето)",
            ),
            (
                NaiveDate::from_ymd_res(2009, 9, 21)?,
                "Рамазан Бајрам (неработен ден, проценето)",
            ),
        ],
        &mut map,
        Country::MK,
        "North Macedonia",
    );

    build_year(
        years,
        2010,
        vec![
            (NaiveDate::from_ymd_res(2010, 1, 1)?, "Нова Година"),
            (NaiveDate::from_ymd_res(2010, 1, 7)?, "Божиќ"),
            (NaiveDate::from_ymd_res(2010, 4, 5)?, "Велигден"),
            (NaiveDate::from_ymd_res(2010, 5, 1)?, "Ден на трудот"),
            (
                NaiveDate::from_ymd_res(2010, 5, 24)?,
                "Светите Кирил и Методиј - Ден на сесловенските просветители",
            ),
            (NaiveDate::from_ymd_res(2010, 8, 2)?, "Ден на Републиката"),
            (NaiveDate::from_ymd_res(2010, 9, 8)?, "Ден на независноста"),
            (
                NaiveDate::from_ymd_res(2010, 10, 11)?,
                "Ден на народното востание",
            ),
            (
                NaiveDate::from_ymd_res(2010, 10, 23)?,
                "Ден на македонската револуционерна борба",
            ),
            (
                NaiveDate::from_ymd_res(2010, 12, 8)?,
                "Свети Климент Охридски",
            ),
            (NaiveDate::from_ymd_res(2010, 9, 9)?, "Рамазан Бајрам"),
        ],
        &mut map,
        Country::MK,
        "North Macedonia",
    );

    build_year(
        years,
        2011,
        vec![
            (NaiveDate::from_ymd_res(2011, 1, 1)?, "Нова Година"),
            (NaiveDate::from_ymd_res(2011, 1, 7)?, "Божиќ"),
            (NaiveDate::from_ymd_res(2011, 4, 25)?, "Велигден"),
            (NaiveDate::from_ymd_res(2011, 5, 1)?, "Ден на трудот"),
            (
                NaiveDate::from_ymd_res(2011, 5, 2)?,
                "Ден на трудот (неработен ден)",
            ),
            (
                NaiveDate::from_ymd_res(2011, 5, 24)?,
                "Светите Кирил и Методиј - Ден на сесловенските просветители",
            ),
            (NaiveDate::from_ymd_res(2011, 8, 2)?, "Ден на Републиката"),
            (NaiveDate::from_ymd_res(2011, 9, 8)?, "Ден на независноста"),
            (
                NaiveDate::from_ymd_res(2011, 10, 11)?,
                "Ден на народното востание",
            ),
            (
                NaiveDate::from_ymd_res(2011, 10, 23)?,
                "Ден на македонската револуционерна борба",
            ),
            (
                NaiveDate::from_ymd_res(2011, 10, 24)?,
                "Ден на македонската револуционерна борба (неработен ден)",
            ),
            (
                NaiveDate::from_ymd_res(2011, 12, 8)?,
                "Свети Климент Охридски",
            ),
            (NaiveDate::from_ymd_res(2011, 8, 30)?, "Рамазан Бајрам"),
        ],
        &mut map,
        Country::MK,
        "North Macedonia",
    );

    build_year(
        years,
        2012,
        vec![
            (NaiveDate::from_ymd_res(2012, 1, 1)?, "Нова Година"),
            (
                NaiveDate::from_ymd_res(2012, 1, 2)?,
                "Нова Година (неработен ден)",
            ),
            (NaiveDate::from_ymd_res(2012, 1, 7)?, "Божиќ"),
            (NaiveDate::from_ymd_res(2012, 4, 16)?, "Велигден"),
            (NaiveDate::from_ymd_res(2012, 5, 1)?, "Ден на трудот"),
            (
                NaiveDate::from_ymd_res(2012, 5, 24)?,
                "Светите Кирил и Методиј - Ден на сесловенските просветители",
            ),
            (NaiveDate::from_ymd_res(2012, 8, 2)?, "Ден на Републиката"),
            (NaiveDate::from_ymd_res(2012, 9, 8)?, "Ден на независноста"),
            (
                NaiveDate::from_ymd_res(2012, 10, 11)?,
                "Ден на народното востание",
            ),
            (
                NaiveDate::from_ymd_res(2012, 10, 23)?,
                "Ден на македонската револуционерна борба",
            ),
            (
                NaiveDate::from_ymd_res(2012, 12, 8)?,
                "Свети Климент Охридски",
            ),
            (NaiveDate::from_ymd_res(2012, 8, 19)?, "Рамазан Бајрам"),
            (
                NaiveDate::from_ymd_res(2012, 8, 20)?,
                "Рамазан Бајрам (неработен ден)",
            ),
        ],
        &mut map,
        Country::MK,
        "North Macedonia",
    );

    build_year(
        years,
        2013,
        vec![
            (NaiveDate::from_ymd_res(2013, 1, 1)?, "Нова Година"),
            (NaiveDate::from_ymd_res(2013, 1, 7)?, "Божиќ"),
            (NaiveDate::from_ymd_res(2013, 5, 6)?, "Велигден"),
            (NaiveDate::from_ymd_res(2013, 5, 1)?, "Ден на трудот"),
            (
                NaiveDate::from_ymd_res(2013, 5, 24)?,
                "Светите Кирил и Методиј - Ден на сесловенските просветители",
            ),
            (NaiveDate::from_ymd_res(2013, 8, 2)?, "Ден на Републиката"),
            (NaiveDate::from_ymd_res(2013, 9, 8)?, "Ден на независноста"),
            (
                NaiveDate::from_ymd_res(2013, 9, 9)?,
                "Ден на независноста (неработен ден)",
            ),
            (
                NaiveDate::from_ymd_res(2013, 10, 11)?,
                "Ден на народното востание",
            ),
            (
                NaiveDate::from_ymd_res(2013, 10, 23)?,
                "Ден на македонската револуционерна борба",
            ),
            (
                NaiveDate::from_ymd_res(2013, 12, 8)?,
                "Свети Климент Охридски",
            ),
            (
                NaiveDate::from_ymd_res(2013, 12, 9)?,
                "Свети Климент Охридски (неработен ден)",
            ),
            (NaiveDate::from_ymd_res(2013, 8, 8)?, "Рамазан Бајрам"),
        ],
        &mut map,
        Country::MK,
        "North Macedonia",
    );

    build_year(
        years,
        2014,
        vec![
            (NaiveDate::from_ymd_res(2014, 1, 1)?, "Нова Година"),
            (NaiveDate::from_ymd_res(2014, 1, 7)?, "Божиќ"),
            (NaiveDate::from_ymd_res(2014, 4, 21)?, "Велигден"),
            (NaiveDate::from_ymd_res(2014, 5, 1)?, "Ден на трудот"),
            (
                NaiveDate::from_ymd_res(2014, 5, 24)?,
                "Светите Кирил и Методиј - Ден на сесловенските просветители",
            ),
            (NaiveDate::from_ymd_res(2014, 8, 2)?, "Ден на Републиката"),
            (NaiveDate::from_ymd_res(2014, 9, 8)?, "Ден на независноста"),
            (
                NaiveDate::from_ymd_res(2014, 10, 11)?,
                "Ден на народното востание",
            ),
            (
                NaiveDate::from_ymd_res(2014, 10, 23)?,
                "Ден на македонската револуционерна борба",
            ),
            (
                NaiveDate::from_ymd_res(2014, 12, 8)?,
                "Свети Климент Охридски",
            ),
            (NaiveDate::from_ymd_res(2014, 7, 28)?, "Рамазан Бајрам"),
        ],
        &mut map,
        Country::MK,
        "North Macedonia",
    );

    build_year(
        years,
        2015,
        vec![
            (NaiveDate::from_ymd_res(2015, 1, 1)?, "Нова Година"),
            (NaiveDate::from_ymd_res(2015, 1, 7)?, "Божиќ"),
            (NaiveDate::from_ymd_res(2015, 4, 13)?, "Велигден"),
            (NaiveDate::from_ymd_res(2015, 5, 1)?, "Ден на трудот"),
            (
                NaiveDate::from_ymd_res(2015, 5, 24)?,
                "Светите Кирил и Методиј - Ден на сесловенските просветители",
            ),
            (
                NaiveDate::from_ymd_res(2015, 5, 25)?,
                "Светите Кирил и Методиј - Ден на сесловенските просветители (неработен ден)",
            ),
            (NaiveDate::from_ymd_res(2015, 8, 2)?, "Ден на Републиката"),
            (
                NaiveDate::from_ymd_res(2015, 8, 3)?,
                "Ден на Републиката (неработен ден)",
            ),
            (NaiveDate::from_ymd_res(2015, 9, 8)?, "Ден на независноста"),
            (
                NaiveDate::from_ymd_res(2015, 10, 11)?,
                "Ден на народното востание",
            ),
            (
                NaiveDate::from_ymd_res(2015, 10, 12)?,
                "Ден на народното востание (неработен ден)",
            ),
            (
                NaiveDate::from_ymd_res(2015, 10, 23)?,
                "Ден на македонската револуционерна борба",
            ),
            (
                NaiveDate::from_ymd_res(2015, 12, 8)?,
                "Свети Климент Охридски",
            ),
            (NaiveDate::from_ymd_res(2015, 7, 17)?, "Рамазан Бајрам"),
        ],
        &mut map,
        Country::MK,
        "North Macedonia",
    );

    build_year(
        years,
        2016,
        vec![
            (NaiveDate::from_ymd_res(2016, 1, 1)?, "Нова Година"),
            (NaiveDate::from_ymd_res(2016, 1, 7)?, "Божиќ"),
            (
                NaiveDate::from_ymd_res(2016, 5, 2)?,
                "Велигден; Ден на трудот (неработен ден)",
            ),
            (NaiveDate::from_ymd_res(2016, 5, 1)?, "Ден на трудот"),
            (
                NaiveDate::from_ymd_res(2016, 5, 24)?,
                "Светите Кирил и Методиј - Ден на сесловенските просветители",
            ),
            (NaiveDate::from_ymd_res(2016, 8, 2)?, "Ден на Републиката"),
            (NaiveDate::from_ymd_res(2016, 9, 8)?, "Ден на независноста"),
            (
                NaiveDate::from_ymd_res(2016, 10, 11)?,
                "Ден на народното востание",
            ),
            (
                NaiveDate::from_ymd_res(2016, 10, 23)?,
                "Ден на македонската револуционерна борба",
            ),
            (
                NaiveDate::from_ymd_res(2016, 10, 24)?,
                "Ден на македонската револуционерна борба (неработен ден)",
            ),
            (
                NaiveDate::from_ymd_res(2016, 12, 8)?,
                "Свети Климент Охридски",
            ),
            (NaiveDate::from_ymd_res(2016, 7, 5)?, "Рамазан Бајрам"),
        ],
        &mut map,
        Country::MK,
        "North Macedonia",
    );

    build_year(
        years,
        2017,
        vec![
            (NaiveDate::from_ymd_res(2017, 1, 1)?, "Нова Година"),
            (
                NaiveDate::from_ymd_res(2017, 1, 2)?,
                "Нова Година (неработен ден)",
            ),
            (NaiveDate::from_ymd_res(2017, 1, 7)?, "Божиќ"),
            (NaiveDate::from_ymd_res(2017, 4, 17)?, "Велигден"),
            (NaiveDate::from_ymd_res(2017, 5, 1)?, "Ден на трудот"),
            (
                NaiveDate::from_ymd_res(2017, 5, 24)?,
                "Светите Кирил и Методиј - Ден на сесловенските просветители",
            ),
            (NaiveDate::from_ymd_res(2017, 8, 2)?, "Ден на Републиката"),
            (NaiveDate::from_ymd_res(2017, 9, 8)?, "Ден на независноста"),
            (
                NaiveDate::from_ymd_res(2017, 10, 11)?,
                "Ден на народното востание",
            ),
            (
                NaiveDate::from_ymd_res(2017, 10, 23)?,
                "Ден на македонската револуционерна борба",
            ),
            (
                NaiveDate::from_ymd_res(2017, 12, 8)?,
                "Свети Климент Охридски",
            ),
            (NaiveDate::from_ymd_res(2017, 6, 25)?, "Рамазан Бајрам"),
            (
                NaiveDate::from_ymd_res(2017, 6, 26)?,
                "Рамазан Бајрам (неработен ден)",
            ),
        ],
        &mut map,
        Country::MK,
        "North Macedonia",
    );

    build_year(
        years,
        2018,
        vec![
            (NaiveDate::from_ymd_res(2018, 1, 1)?, "Нова Година"),
            (NaiveDate::from_ymd_res(2018, 1, 7)?, "Божиќ"),
            (
                NaiveDate::from_ymd_res(2018, 1, 8)?,
                "Божиќ (неработен ден)",
            ),
            (NaiveDate::from_ymd_res(2018, 4, 9)?, "Велигден"),
            (NaiveDate::from_ymd_res(2018, 5, 1)?, "Ден на трудот"),
            (
                NaiveDate::from_ymd_res(2018, 5, 24)?,
                "Светите Кирил и Методиј - Ден на сесловенските просветители",
            ),
            (NaiveDate::from_ymd_res(2018, 8, 2)?, "Ден на Републиката"),
            (NaiveDate::from_ymd_res(2018, 9, 8)?, "Ден на независноста"),
            (
                NaiveDate::from_ymd_res(2018, 10, 11)?,
                "Ден на народното востание",
            ),
            (
                NaiveDate::from_ymd_res(2018, 10, 23)?,
                "Ден на македонската револуционерна борба",
            ),
            (
                NaiveDate::from_ymd_res(2018, 12, 8)?,
                "Свети Климент Охридски",
            ),
            (NaiveDate::from_ymd_res(2018, 6, 15)?, "Рамазан Бајрам"),
        ],
        &mut map,
        Country::MK,
        "North Macedonia",
    );

    build_year(
        years,
        2019,
        vec![
            (NaiveDate::from_ymd_res(2019, 1, 1)?, "Нова Година"),
            (NaiveDate::from_ymd_res(2019, 1, 7)?, "Божиќ"),
            (NaiveDate::from_ymd_res(2019, 4, 29)?, "Велигден"),
            (NaiveDate::from_ymd_res(2019, 5, 1)?, "Ден на трудот"),
            (
                NaiveDate::from_ymd_res(2019, 5, 24)?,
                "Светите Кирил и Методиј - Ден на сесловенските просветители",
            ),
            (NaiveDate::from_ymd_res(2019, 8, 2)?, "Ден на Републиката"),
            (NaiveDate::from_ymd_res(2019, 9, 8)?, "Ден на независноста"),
            (
                NaiveDate::from_ymd_res(2019, 9, 9)?,
                "Ден на независноста (неработен ден)",
            ),
            (
                NaiveDate::from_ymd_res(2019, 10, 11)?,
                "Ден на народното востание",
            ),
            (
                NaiveDate::from_ymd_res(2019, 10, 23)?,
                "Ден на македонската револуционерна борба",
            ),
            (
                NaiveDate::from_ymd_res(2019, 12, 8)?,
                "Свети Климент Охридски",
            ),
            (
                NaiveDate::from_ymd_res(2019, 12, 9)?,
                "Свети Климент Охридски (неработен ден)",
            ),
            (NaiveDate::from_ymd_res(2019, 6, 4)?, "Рамазан Бајрам"),
        ],
        &mut map,
        Country::MK,
        "North Macedonia",
    );

    build_year(
        years,
        2020,
        vec![
            (NaiveDate::from_ymd_res(2020, 1, 1)?, "Нова Година"),
            (NaiveDate::from_ymd_res(2020, 1, 7)?, "Божиќ"),
            (NaiveDate::from_ymd_res(2020, 4, 20)?, "Велигден"),
            (NaiveDate::from_ymd_res(2020, 5, 1)?, "Ден на трудот"),
            (
                NaiveDate::from_ymd_res(2020, 5, 24)?,
                "Рамазан Бајрам; Светите Кирил и Методиј - Ден на сесловенските просветители",
            ),
            (
                NaiveDate::from_ymd_res(2020, 5, 25)?,
                "Рамазан Бајрам (неработен ден); Светите Кирил и Методиј - Ден на сесловенските просветители (неработен ден)",
            ),
            (NaiveDate::from_ymd_res(2020, 8, 2)?, "Ден на Републиката"),
            (
                NaiveDate::from_ymd_res(2020, 8, 3)?,
                "Ден на Републиката (неработен ден)",
            ),
            (NaiveDate::from_ymd_res(2020, 9, 8)?, "Ден на независноста"),
            (
                NaiveDate::from_ymd_res(2020, 10, 11)?,
                "Ден на народното востание",
            ),
            (
                NaiveDate::from_ymd_res(2020, 10, 12)?,
                "Ден на народното востание (неработен ден)",
            ),
            (
                NaiveDate::from_ymd_res(2020, 10, 23)?,
                "Ден на македонската револуционерна борба",
            ),
            (
                NaiveDate::from_ymd_res(2020, 12, 8)?,
                "Свети Климент Охридски",
            ),
        ],
        &mut map,
        Country::MK,
        "North Macedonia",
    );

    build_year(
        years,
        2021,
        vec![
            (NaiveDate::from_ymd_res(2021, 1, 1)?, "Нова Година"),
            (NaiveDate::from_ymd_res(2021, 1, 7)?, "Божиќ"),
            (NaiveDate::from_ymd_res(2021, 5, 3)?, "Велигден"),
            (NaiveDate::from_ymd_res(2021, 5, 1)?, "Ден на трудот"),
            (
                NaiveDate::from_ymd_res(2021, 5, 24)?,
                "Светите Кирил и Методиј - Ден на сесловенските просветители",
            ),
            (NaiveDate::from_ymd_res(2021, 8, 2)?, "Ден на Републиката"),
            (NaiveDate::from_ymd_res(2021, 9, 8)?, "Ден на независноста"),
            (
                NaiveDate::from_ymd_res(2021, 10, 11)?,
                "Ден на народното востание",
            ),
            (
                NaiveDate::from_ymd_res(2021, 10, 23)?,
                "Ден на македонската револуционерна борба",
            ),
            (
                NaiveDate::from_ymd_res(2021, 12, 8)?,
                "Свети Климент Охридски",
            ),
            (NaiveDate::from_ymd_res(2021, 5, 13)?, "Рамазан Бајрам"),
        ],
        &mut map,
        Country::MK,
        "North Macedonia",
    );

    build_year(
        years,
        2022,
        vec![
            (NaiveDate::from_ymd_res(2022, 1, 1)?, "Нова Година"),
            (NaiveDate::from_ymd_res(2022, 1, 7)?, "Божиќ"),
            (NaiveDate::from_ymd_res(2022, 4, 25)?, "Велигден"),
            (NaiveDate::from_ymd_res(2022, 5, 1)?, "Ден на трудот"),
            (
                NaiveDate::from_ymd_res(2022, 5, 2)?,
                "Ден на трудот (неработен ден); Рамазан Бајрам",
            ),
            (
                NaiveDate::from_ymd_res(2022, 5, 24)?,
                "Светите Кирил и Методиј - Ден на сесловенските просветители",
            ),
            (NaiveDate::from_ymd_res(2022, 8, 2)?, "Ден на Републиката"),
            (NaiveDate::from_ymd_res(2022, 9, 8)?, "Ден на независноста"),
            (
                NaiveDate::from_ymd_res(2022, 10, 11)?,
                "Ден на народното востание",
            ),
            (
                NaiveDate::from_ymd_res(2022, 10, 23)?,
                "Ден на македонската револуционерна борба",
            ),
            (
                NaiveDate::from_ymd_res(2022, 10, 24)?,
                "Ден на македонската револуционерна борба (неработен ден)",
            ),
            (
                NaiveDate::from_ymd_res(2022, 12, 8)?,
                "Свети Климент Охридски",
            ),
        ],
        &mut map,
        Country::MK,
        "North Macedonia",
    );

    build_year(
        years,
        2023,
        vec![
            (NaiveDate::from_ymd_res(2023, 1, 1)?, "Нова Година"),
            (
                NaiveDate::from_ymd_res(2023, 1, 2)?,
                "Нова Година (неработен ден)",
            ),
            (NaiveDate::from_ymd_res(2023, 1, 7)?, "Божиќ"),
            (NaiveDate::from_ymd_res(2023, 4, 17)?, "Велигден"),
            (NaiveDate::from_ymd_res(2023, 5, 1)?, "Ден на трудот"),
            (
                NaiveDate::from_ymd_res(2023, 5, 24)?,
                "Светите Кирил и Методиј - Ден на сесловенските просветители",
            ),
            (NaiveDate::from_ymd_res(2023, 8, 2)?, "Ден на Републиката"),
            (NaiveDate::from_ymd_res(2023, 9, 8)?, "Ден на независноста"),
            (
                NaiveDate::from_ymd_res(2023, 10, 11)?,
                "Ден на народното востание",
            ),
            (
                NaiveDate::from_ymd_res(2023, 10, 23)?,
                "Ден на македонската револуционерна борба",
            ),
            (
                NaiveDate::from_ymd_res(2023, 12, 8)?,
                "Свети Климент Охридски",
            ),
            (NaiveDate::from_ymd_res(2023, 4, 21)?, "Рамазан Бајрам"),
        ],
        &mut map,
        Country::MK,
        "North Macedonia",
    );

    build_year(
        years,
        2024,
        vec![
            (NaiveDate::from_ymd_res(2024, 1, 1)?, "Нова Година"),
            (NaiveDate::from_ymd_res(2024, 1, 7)?, "Божиќ"),
            (
                NaiveDate::from_ymd_res(2024, 1, 8)?,
                "Божиќ (неработен ден)",
            ),
            (NaiveDate::from_ymd_res(2024, 5, 6)?, "Велигден"),
            (NaiveDate::from_ymd_res(2024, 5, 1)?, "Ден на трудот"),
            (
                NaiveDate::from_ymd_res(2024, 5, 24)?,
                "Светите Кирил и Методиј - Ден на сесловенските просветители",
            ),
            (NaiveDate::from_ymd_res(2024, 8, 2)?, "Ден на Републиката"),
            (NaiveDate::from_ymd_res(2024, 9, 8)?, "Ден на независноста"),
            (
                NaiveDate::from_ymd_res(2024, 9, 9)?,
                "Ден на независноста (неработен ден)",
            ),
            (
                NaiveDate::from_ymd_res(2024, 10, 11)?,
                "Ден на народното востание",
            ),
            (
                NaiveDate::from_ymd_res(2024, 10, 23)?,
                "Ден на македонската револуционерна борба",
            ),
            (
                NaiveDate::from_ymd_res(2024, 12, 8)?,
                "Свети Климент Охридски",
            ),
            (
                NaiveDate::from_ymd_res(2024, 12, 9)?,
                "Свети Климент Охридски (неработен ден)",
            ),
            (NaiveDate::from_ymd_res(2024, 4, 10)?, "Рамазан Бајрам"),
            (NaiveDate::from_ymd_res(2024, 4, 24)?, "Ден на изборите"),
            (NaiveDate::from_ymd_res(2024, 5, 8)?, "Ден на изборите"),
        ],
        &mut map,
        Country::MK,
        "North Macedonia",
    );

    build_year(
        years,
        2025,
        vec![
            (NaiveDate::from_ymd_res(2025, 1, 1)?, "Нова Година"),
            (NaiveDate::from_ymd_res(2025, 1, 7)?, "Божиќ"),
            (NaiveDate::from_ymd_res(2025, 4, 21)?, "Велигден"),
            (NaiveDate::from_ymd_res(2025, 5, 1)?, "Ден на трудот"),
            (
                NaiveDate::from_ymd_res(2025, 5, 24)?,
                "Светите Кирил и Методиј - Ден на сесловенските просветители",
            ),
            (NaiveDate::from_ymd_res(2025, 8, 2)?, "Ден на Републиката"),
            (NaiveDate::from_ymd_res(2025, 9, 8)?, "Ден на независноста"),
            (
                NaiveDate::from_ymd_res(2025, 10, 11)?,
                "Ден на народното востание",
            ),
            (
                NaiveDate::from_ymd_res(2025, 10, 23)?,
                "Ден на македонската револуционерна борба",
            ),
            (
                NaiveDate::from_ymd_res(2025, 12, 8)?,
                "Свети Климент Охридски",
            ),
            (NaiveDate::from_ymd_res(2025, 3, 30)?, "Рамазан Бајрам"),
            (
                NaiveDate::from_ymd_res(2025, 3, 31)?,
                "Рамазан Бајрам (неработен ден)",
            ),
        ],
        &mut map,
        Country::MK,
        "North Macedonia",
    );

    build_year(
        years,
        2026,
        vec![
            (NaiveDate::from_ymd_res(2026, 1, 1)?, "Нова Година"),
            (NaiveDate::from_ymd_res(2026, 1, 7)?, "Божиќ"),
            (NaiveDate::from_ymd_res(2026, 4, 13)?, "Велигден"),
            (NaiveDate::from_ymd_res(2026, 5, 1)?, "Ден на трудот"),
            (
                NaiveDate::from_ymd_res(2026, 5, 24)?,
                "Светите Кирил и Методиј - Ден на сесловенските просветители",
            ),
            (
                NaiveDate::from_ymd_res(2026, 5, 25)?,
                "Светите Кирил и Методиј - Ден на сесловенските просветители (неработен ден)",
            ),
            (NaiveDate::from_ymd_res(2026, 8, 2)?, "Ден на Републиката"),
            (
                NaiveDate::from_ymd_res(2026, 8, 3)?,
                "Ден на Републиката (неработен ден)",
            ),
            (NaiveDate::from_ymd_res(2026, 9, 8)?, "Ден на независноста"),
            (
                NaiveDate::from_ymd_res(2026, 10, 11)?,
                "Ден на народното востание",
            ),
            (
                NaiveDate::from_ymd_res(2026, 10, 12)?,
                "Ден на народното востание (неработен ден)",
            ),
            (
                NaiveDate::from_ymd_res(2026, 10, 23)?,
                "Ден на македонската револуционерна борба",
            ),
            (
                NaiveDate::from_ymd_res(2026, 12, 8)?,
                "Свети Климент Охридски",
            ),
            (
                NaiveDate::from_ymd_res(2026, 3, 20)?,
                "Рамазан Бајрам (проценето)",
            ),
        ],
        &mut map,
        Country::MK,
        "North Macedonia",
    );

    build_year(
        years,
        2027,
        vec![
            (NaiveDate::from_ymd_res(2027, 1, 1)?, "Нова Година"),
            (NaiveDate::from_ymd_res(2027, 1, 7)?, "Божиќ"),
            (NaiveDate::from_ymd_res(2027, 5, 3)?, "Велигден"),
            (NaiveDate::from_ymd_res(2027, 5, 1)?, "Ден на трудот"),
            (
                NaiveDate::from_ymd_res(2027, 5, 24)?,
                "Светите Кирил и Методиј - Ден на сесловенските просветители",
            ),
            (NaiveDate::from_ymd_res(2027, 8, 2)?, "Ден на Републиката"),
            (NaiveDate::from_ymd_res(2027, 9, 8)?, "Ден на независноста"),
            (
                NaiveDate::from_ymd_res(2027, 10, 11)?,
                "Ден на народното востание",
            ),
            (
                NaiveDate::from_ymd_res(2027, 10, 23)?,
                "Ден на македонската револуционерна борба",
            ),
            (
                NaiveDate::from_ymd_res(2027, 12, 8)?,
                "Свети Климент Охридски",
            ),
            (
                NaiveDate::from_ymd_res(2027, 3, 9)?,
                "Рамазан Бајрам (проценето)",
            ),
        ],
        &mut map,
        Country::MK,
        "North Macedonia",
    );

    build_year(
        years,
        2028,
        vec![
            (NaiveDate::from_ymd_res(2028, 1, 1)?, "Нова Година"),
            (NaiveDate::from_ymd_res(2028, 1, 7)?, "Божиќ"),
            (NaiveDate::from_ymd_res(2028, 4, 17)?, "Велигден"),
            (NaiveDate::from_ymd_res(2028, 5, 1)?, "Ден на трудот"),
            (
                NaiveDate::from_ymd_res(2028, 5, 24)?,
                "Светите Кирил и Методиј - Ден на сесловенските просветители",
            ),
            (NaiveDate::from_ymd_res(2028, 8, 2)?, "Ден на Републиката"),
            (NaiveDate::from_ymd_res(2028, 9, 8)?, "Ден на независноста"),
            (
                NaiveDate::from_ymd_res(2028, 10, 11)?,
                "Ден на народното востание",
            ),
            (
                NaiveDate::from_ymd_res(2028, 10, 23)?,
                "Ден на македонската револуционерна борба",
            ),
            (
                NaiveDate::from_ymd_res(2028, 12, 8)?,
                "Свети Климент Охридски",
            ),
            (
                NaiveDate::from_ymd_res(2028, 2, 26)?,
                "Рамазан Бајрам (проценето)",
            ),
        ],
        &mut map,
        Country::MK,
        "North Macedonia",
    );

    build_year(
        years,
        2029,
        vec![
            (NaiveDate::from_ymd_res(2029, 1, 1)?, "Нова Година"),
            (NaiveDate::from_ymd_res(2029, 1, 7)?, "Божиќ"),
            (
                NaiveDate::from_ymd_res(2029, 1, 8)?,
                "Божиќ (неработен ден)",
            ),
            (NaiveDate::from_ymd_res(2029, 4, 9)?, "Велигден"),
            (NaiveDate::from_ymd_res(2029, 5, 1)?, "Ден на трудот"),
            (
                NaiveDate::from_ymd_res(2029, 5, 24)?,
                "Светите Кирил и Методиј - Ден на сесловенските просветители",
            ),
            (NaiveDate::from_ymd_res(2029, 8, 2)?, "Ден на Републиката"),
            (NaiveDate::from_ymd_res(2029, 9, 8)?, "Ден на независноста"),
            (
                NaiveDate::from_ymd_res(2029, 10, 11)?,
                "Ден на народното востание",
            ),
            (
                NaiveDate::from_ymd_res(2029, 10, 23)?,
                "Ден на македонската револуционерна борба",
            ),
            (
                NaiveDate::from_ymd_res(2029, 12, 8)?,
                "Свети Климент Охридски",
            ),
            (
                NaiveDate::from_ymd_res(2029, 2, 14)?,
                "Рамазан Бајрам (проценето)",
            ),
        ],
        &mut map,
        Country::MK,
        "North Macedonia",
    );

    build_year(
        years,
        2030,
        vec![
            (NaiveDate::from_ymd_res(2030, 1, 1)?, "Нова Година"),
            (NaiveDate::from_ymd_res(2030, 1, 7)?, "Божиќ"),
            (NaiveDate::from_ymd_res(2030, 4, 29)?, "Велигден"),
            (NaiveDate::from_ymd_res(2030, 5, 1)?, "Ден на трудот"),
            (
                NaiveDate::from_ymd_res(2030, 5, 24)?,
                "Светите Кирил и Методиј - Ден на сесловенските просветители",
            ),
            (NaiveDate::from_ymd_res(2030, 8, 2)?, "Ден на Републиката"),
            (NaiveDate::from_ymd_res(2030, 9, 8)?, "Ден на независноста"),
            (
                NaiveDate::from_ymd_res(2030, 9, 9)?,
                "Ден на независноста (неработен ден)",
            ),
            (
                NaiveDate::from_ymd_res(2030, 10, 11)?,
                "Ден на народното востание",
            ),
            (
                NaiveDate::from_ymd_res(2030, 10, 23)?,
                "Ден на македонската револуционерна борба",
            ),
            (
                NaiveDate::from_ymd_res(2030, 12, 8)?,
                "Свети Климент Охридски",
            ),
            (
                NaiveDate::from_ymd_res(2030, 12, 9)?,
                "Свети Климент Охридски (неработен ден)",
            ),
            (
                NaiveDate::from_ymd_res(2030, 2, 4)?,
                "Рамазан Бајрам (проценето)",
            ),
        ],
        &mut map,
        Country::MK,
        "North Macedonia",
    );

    Ok(map)
}
