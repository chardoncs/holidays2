//! Sweden
use super::*;

/// Generate holiday map for Sweden.
#[allow(unused_mut, unused_variables)]
pub fn build(
    years: &Option<&std::ops::Range<Year>>,
) -> Result<HashMap<Year, BTreeMap<NaiveDate, Holiday>>> {
    let mut map = HashMap::new();

    build_year(
        years,
        2001,
        vec![
            (NaiveDate::from_ymd_res(2001, 1, 1)?, "Nyårsdagen"),
            (NaiveDate::from_ymd_res(2001, 1, 6)?, "Trettondedag jul"),
            (NaiveDate::from_ymd_res(2001, 4, 13)?, "Långfredagen"),
            (NaiveDate::from_ymd_res(2001, 4, 15)?, "Påskdagen"),
            (NaiveDate::from_ymd_res(2001, 4, 16)?, "Annandag påsk"),
            (NaiveDate::from_ymd_res(2001, 5, 1)?, "Första maj"),
            (
                NaiveDate::from_ymd_res(2001, 5, 24)?,
                "Kristi himmelsfärdsdag",
            ),
            (NaiveDate::from_ymd_res(2001, 6, 3)?, "Pingstdagen"),
            (NaiveDate::from_ymd_res(2001, 6, 4)?, "Annandag pingst"),
            (NaiveDate::from_ymd_res(2001, 6, 23)?, "Midsommardagen"),
            (NaiveDate::from_ymd_res(2001, 11, 3)?, "Alla helgons dag"),
            (NaiveDate::from_ymd_res(2001, 12, 25)?, "Juldagen"),
            (NaiveDate::from_ymd_res(2001, 12, 26)?, "Annandag jul"),
        ],
        &mut map,
        Country::SE,
        "Sweden",
    );

    build_year(
        years,
        2002,
        vec![
            (NaiveDate::from_ymd_res(2002, 1, 1)?, "Nyårsdagen"),
            (NaiveDate::from_ymd_res(2002, 1, 6)?, "Trettondedag jul"),
            (NaiveDate::from_ymd_res(2002, 3, 29)?, "Långfredagen"),
            (NaiveDate::from_ymd_res(2002, 3, 31)?, "Påskdagen"),
            (NaiveDate::from_ymd_res(2002, 4, 1)?, "Annandag påsk"),
            (NaiveDate::from_ymd_res(2002, 5, 1)?, "Första maj"),
            (
                NaiveDate::from_ymd_res(2002, 5, 9)?,
                "Kristi himmelsfärdsdag",
            ),
            (NaiveDate::from_ymd_res(2002, 5, 19)?, "Pingstdagen"),
            (NaiveDate::from_ymd_res(2002, 5, 20)?, "Annandag pingst"),
            (NaiveDate::from_ymd_res(2002, 6, 22)?, "Midsommardagen"),
            (NaiveDate::from_ymd_res(2002, 11, 2)?, "Alla helgons dag"),
            (NaiveDate::from_ymd_res(2002, 12, 25)?, "Juldagen"),
            (NaiveDate::from_ymd_res(2002, 12, 26)?, "Annandag jul"),
        ],
        &mut map,
        Country::SE,
        "Sweden",
    );

    build_year(
        years,
        2003,
        vec![
            (NaiveDate::from_ymd_res(2003, 1, 1)?, "Nyårsdagen"),
            (NaiveDate::from_ymd_res(2003, 1, 6)?, "Trettondedag jul"),
            (NaiveDate::from_ymd_res(2003, 4, 18)?, "Långfredagen"),
            (NaiveDate::from_ymd_res(2003, 4, 20)?, "Påskdagen"),
            (NaiveDate::from_ymd_res(2003, 4, 21)?, "Annandag påsk"),
            (NaiveDate::from_ymd_res(2003, 5, 1)?, "Första maj"),
            (
                NaiveDate::from_ymd_res(2003, 5, 29)?,
                "Kristi himmelsfärdsdag",
            ),
            (NaiveDate::from_ymd_res(2003, 6, 8)?, "Pingstdagen"),
            (NaiveDate::from_ymd_res(2003, 6, 9)?, "Annandag pingst"),
            (NaiveDate::from_ymd_res(2003, 6, 21)?, "Midsommardagen"),
            (NaiveDate::from_ymd_res(2003, 11, 1)?, "Alla helgons dag"),
            (NaiveDate::from_ymd_res(2003, 12, 25)?, "Juldagen"),
            (NaiveDate::from_ymd_res(2003, 12, 26)?, "Annandag jul"),
        ],
        &mut map,
        Country::SE,
        "Sweden",
    );

    build_year(
        years,
        2004,
        vec![
            (NaiveDate::from_ymd_res(2004, 1, 1)?, "Nyårsdagen"),
            (NaiveDate::from_ymd_res(2004, 1, 6)?, "Trettondedag jul"),
            (NaiveDate::from_ymd_res(2004, 4, 9)?, "Långfredagen"),
            (NaiveDate::from_ymd_res(2004, 4, 11)?, "Påskdagen"),
            (NaiveDate::from_ymd_res(2004, 4, 12)?, "Annandag påsk"),
            (NaiveDate::from_ymd_res(2004, 5, 1)?, "Första maj"),
            (
                NaiveDate::from_ymd_res(2004, 5, 20)?,
                "Kristi himmelsfärdsdag",
            ),
            (NaiveDate::from_ymd_res(2004, 5, 30)?, "Pingstdagen"),
            (NaiveDate::from_ymd_res(2004, 5, 31)?, "Annandag pingst"),
            (NaiveDate::from_ymd_res(2004, 6, 26)?, "Midsommardagen"),
            (NaiveDate::from_ymd_res(2004, 11, 6)?, "Alla helgons dag"),
            (NaiveDate::from_ymd_res(2004, 12, 25)?, "Juldagen"),
            (NaiveDate::from_ymd_res(2004, 12, 26)?, "Annandag jul"),
        ],
        &mut map,
        Country::SE,
        "Sweden",
    );

    build_year(
        years,
        2005,
        vec![
            (NaiveDate::from_ymd_res(2005, 1, 1)?, "Nyårsdagen"),
            (NaiveDate::from_ymd_res(2005, 1, 6)?, "Trettondedag jul"),
            (NaiveDate::from_ymd_res(2005, 3, 25)?, "Långfredagen"),
            (NaiveDate::from_ymd_res(2005, 3, 27)?, "Påskdagen"),
            (NaiveDate::from_ymd_res(2005, 3, 28)?, "Annandag påsk"),
            (NaiveDate::from_ymd_res(2005, 5, 1)?, "Första maj"),
            (
                NaiveDate::from_ymd_res(2005, 5, 5)?,
                "Kristi himmelsfärdsdag",
            ),
            (NaiveDate::from_ymd_res(2005, 6, 6)?, "Nationaldagen"),
            (NaiveDate::from_ymd_res(2005, 5, 15)?, "Pingstdagen"),
            (NaiveDate::from_ymd_res(2005, 6, 25)?, "Midsommardagen"),
            (NaiveDate::from_ymd_res(2005, 11, 5)?, "Alla helgons dag"),
            (NaiveDate::from_ymd_res(2005, 12, 25)?, "Juldagen"),
            (NaiveDate::from_ymd_res(2005, 12, 26)?, "Annandag jul"),
        ],
        &mut map,
        Country::SE,
        "Sweden",
    );

    build_year(
        years,
        2006,
        vec![
            (NaiveDate::from_ymd_res(2006, 1, 1)?, "Nyårsdagen"),
            (NaiveDate::from_ymd_res(2006, 1, 6)?, "Trettondedag jul"),
            (NaiveDate::from_ymd_res(2006, 4, 14)?, "Långfredagen"),
            (NaiveDate::from_ymd_res(2006, 4, 16)?, "Påskdagen"),
            (NaiveDate::from_ymd_res(2006, 4, 17)?, "Annandag påsk"),
            (NaiveDate::from_ymd_res(2006, 5, 1)?, "Första maj"),
            (
                NaiveDate::from_ymd_res(2006, 5, 25)?,
                "Kristi himmelsfärdsdag",
            ),
            (NaiveDate::from_ymd_res(2006, 6, 6)?, "Nationaldagen"),
            (NaiveDate::from_ymd_res(2006, 6, 4)?, "Pingstdagen"),
            (NaiveDate::from_ymd_res(2006, 6, 24)?, "Midsommardagen"),
            (NaiveDate::from_ymd_res(2006, 11, 4)?, "Alla helgons dag"),
            (NaiveDate::from_ymd_res(2006, 12, 25)?, "Juldagen"),
            (NaiveDate::from_ymd_res(2006, 12, 26)?, "Annandag jul"),
        ],
        &mut map,
        Country::SE,
        "Sweden",
    );

    build_year(
        years,
        2007,
        vec![
            (NaiveDate::from_ymd_res(2007, 1, 1)?, "Nyårsdagen"),
            (NaiveDate::from_ymd_res(2007, 1, 6)?, "Trettondedag jul"),
            (NaiveDate::from_ymd_res(2007, 4, 6)?, "Långfredagen"),
            (NaiveDate::from_ymd_res(2007, 4, 8)?, "Påskdagen"),
            (NaiveDate::from_ymd_res(2007, 4, 9)?, "Annandag påsk"),
            (NaiveDate::from_ymd_res(2007, 5, 1)?, "Första maj"),
            (
                NaiveDate::from_ymd_res(2007, 5, 17)?,
                "Kristi himmelsfärdsdag",
            ),
            (NaiveDate::from_ymd_res(2007, 6, 6)?, "Nationaldagen"),
            (NaiveDate::from_ymd_res(2007, 5, 27)?, "Pingstdagen"),
            (NaiveDate::from_ymd_res(2007, 6, 23)?, "Midsommardagen"),
            (NaiveDate::from_ymd_res(2007, 11, 3)?, "Alla helgons dag"),
            (NaiveDate::from_ymd_res(2007, 12, 25)?, "Juldagen"),
            (NaiveDate::from_ymd_res(2007, 12, 26)?, "Annandag jul"),
        ],
        &mut map,
        Country::SE,
        "Sweden",
    );

    build_year(
        years,
        2008,
        vec![
            (NaiveDate::from_ymd_res(2008, 1, 1)?, "Nyårsdagen"),
            (NaiveDate::from_ymd_res(2008, 1, 6)?, "Trettondedag jul"),
            (NaiveDate::from_ymd_res(2008, 3, 21)?, "Långfredagen"),
            (NaiveDate::from_ymd_res(2008, 3, 23)?, "Påskdagen"),
            (NaiveDate::from_ymd_res(2008, 3, 24)?, "Annandag påsk"),
            (
                NaiveDate::from_ymd_res(2008, 5, 1)?,
                "Första maj; Kristi himmelsfärdsdag",
            ),
            (NaiveDate::from_ymd_res(2008, 6, 6)?, "Nationaldagen"),
            (NaiveDate::from_ymd_res(2008, 5, 11)?, "Pingstdagen"),
            (NaiveDate::from_ymd_res(2008, 6, 21)?, "Midsommardagen"),
            (NaiveDate::from_ymd_res(2008, 11, 1)?, "Alla helgons dag"),
            (NaiveDate::from_ymd_res(2008, 12, 25)?, "Juldagen"),
            (NaiveDate::from_ymd_res(2008, 12, 26)?, "Annandag jul"),
        ],
        &mut map,
        Country::SE,
        "Sweden",
    );

    build_year(
        years,
        2009,
        vec![
            (NaiveDate::from_ymd_res(2009, 1, 1)?, "Nyårsdagen"),
            (NaiveDate::from_ymd_res(2009, 1, 6)?, "Trettondedag jul"),
            (NaiveDate::from_ymd_res(2009, 4, 10)?, "Långfredagen"),
            (NaiveDate::from_ymd_res(2009, 4, 12)?, "Påskdagen"),
            (NaiveDate::from_ymd_res(2009, 4, 13)?, "Annandag påsk"),
            (NaiveDate::from_ymd_res(2009, 5, 1)?, "Första maj"),
            (
                NaiveDate::from_ymd_res(2009, 5, 21)?,
                "Kristi himmelsfärdsdag",
            ),
            (NaiveDate::from_ymd_res(2009, 6, 6)?, "Nationaldagen"),
            (NaiveDate::from_ymd_res(2009, 5, 31)?, "Pingstdagen"),
            (NaiveDate::from_ymd_res(2009, 6, 20)?, "Midsommardagen"),
            (NaiveDate::from_ymd_res(2009, 10, 31)?, "Alla helgons dag"),
            (NaiveDate::from_ymd_res(2009, 12, 25)?, "Juldagen"),
            (NaiveDate::from_ymd_res(2009, 12, 26)?, "Annandag jul"),
        ],
        &mut map,
        Country::SE,
        "Sweden",
    );

    build_year(
        years,
        2010,
        vec![
            (NaiveDate::from_ymd_res(2010, 1, 1)?, "Nyårsdagen"),
            (NaiveDate::from_ymd_res(2010, 1, 6)?, "Trettondedag jul"),
            (NaiveDate::from_ymd_res(2010, 4, 2)?, "Långfredagen"),
            (NaiveDate::from_ymd_res(2010, 4, 4)?, "Påskdagen"),
            (NaiveDate::from_ymd_res(2010, 4, 5)?, "Annandag påsk"),
            (NaiveDate::from_ymd_res(2010, 5, 1)?, "Första maj"),
            (
                NaiveDate::from_ymd_res(2010, 5, 13)?,
                "Kristi himmelsfärdsdag",
            ),
            (NaiveDate::from_ymd_res(2010, 6, 6)?, "Nationaldagen"),
            (NaiveDate::from_ymd_res(2010, 5, 23)?, "Pingstdagen"),
            (NaiveDate::from_ymd_res(2010, 6, 26)?, "Midsommardagen"),
            (NaiveDate::from_ymd_res(2010, 11, 6)?, "Alla helgons dag"),
            (NaiveDate::from_ymd_res(2010, 12, 25)?, "Juldagen"),
            (NaiveDate::from_ymd_res(2010, 12, 26)?, "Annandag jul"),
        ],
        &mut map,
        Country::SE,
        "Sweden",
    );

    build_year(
        years,
        2011,
        vec![
            (NaiveDate::from_ymd_res(2011, 1, 1)?, "Nyårsdagen"),
            (NaiveDate::from_ymd_res(2011, 1, 6)?, "Trettondedag jul"),
            (NaiveDate::from_ymd_res(2011, 4, 22)?, "Långfredagen"),
            (NaiveDate::from_ymd_res(2011, 4, 24)?, "Påskdagen"),
            (NaiveDate::from_ymd_res(2011, 4, 25)?, "Annandag påsk"),
            (NaiveDate::from_ymd_res(2011, 5, 1)?, "Första maj"),
            (
                NaiveDate::from_ymd_res(2011, 6, 2)?,
                "Kristi himmelsfärdsdag",
            ),
            (NaiveDate::from_ymd_res(2011, 6, 6)?, "Nationaldagen"),
            (NaiveDate::from_ymd_res(2011, 6, 12)?, "Pingstdagen"),
            (NaiveDate::from_ymd_res(2011, 6, 25)?, "Midsommardagen"),
            (NaiveDate::from_ymd_res(2011, 11, 5)?, "Alla helgons dag"),
            (NaiveDate::from_ymd_res(2011, 12, 25)?, "Juldagen"),
            (NaiveDate::from_ymd_res(2011, 12, 26)?, "Annandag jul"),
        ],
        &mut map,
        Country::SE,
        "Sweden",
    );

    build_year(
        years,
        2012,
        vec![
            (NaiveDate::from_ymd_res(2012, 1, 1)?, "Nyårsdagen"),
            (NaiveDate::from_ymd_res(2012, 1, 6)?, "Trettondedag jul"),
            (NaiveDate::from_ymd_res(2012, 4, 6)?, "Långfredagen"),
            (NaiveDate::from_ymd_res(2012, 4, 8)?, "Påskdagen"),
            (NaiveDate::from_ymd_res(2012, 4, 9)?, "Annandag påsk"),
            (NaiveDate::from_ymd_res(2012, 5, 1)?, "Första maj"),
            (
                NaiveDate::from_ymd_res(2012, 5, 17)?,
                "Kristi himmelsfärdsdag",
            ),
            (NaiveDate::from_ymd_res(2012, 6, 6)?, "Nationaldagen"),
            (NaiveDate::from_ymd_res(2012, 5, 27)?, "Pingstdagen"),
            (NaiveDate::from_ymd_res(2012, 6, 23)?, "Midsommardagen"),
            (NaiveDate::from_ymd_res(2012, 11, 3)?, "Alla helgons dag"),
            (NaiveDate::from_ymd_res(2012, 12, 25)?, "Juldagen"),
            (NaiveDate::from_ymd_res(2012, 12, 26)?, "Annandag jul"),
        ],
        &mut map,
        Country::SE,
        "Sweden",
    );

    build_year(
        years,
        2013,
        vec![
            (NaiveDate::from_ymd_res(2013, 1, 1)?, "Nyårsdagen"),
            (NaiveDate::from_ymd_res(2013, 1, 6)?, "Trettondedag jul"),
            (NaiveDate::from_ymd_res(2013, 3, 29)?, "Långfredagen"),
            (NaiveDate::from_ymd_res(2013, 3, 31)?, "Påskdagen"),
            (NaiveDate::from_ymd_res(2013, 4, 1)?, "Annandag påsk"),
            (NaiveDate::from_ymd_res(2013, 5, 1)?, "Första maj"),
            (
                NaiveDate::from_ymd_res(2013, 5, 9)?,
                "Kristi himmelsfärdsdag",
            ),
            (NaiveDate::from_ymd_res(2013, 6, 6)?, "Nationaldagen"),
            (NaiveDate::from_ymd_res(2013, 5, 19)?, "Pingstdagen"),
            (NaiveDate::from_ymd_res(2013, 6, 22)?, "Midsommardagen"),
            (NaiveDate::from_ymd_res(2013, 11, 2)?, "Alla helgons dag"),
            (NaiveDate::from_ymd_res(2013, 12, 25)?, "Juldagen"),
            (NaiveDate::from_ymd_res(2013, 12, 26)?, "Annandag jul"),
        ],
        &mut map,
        Country::SE,
        "Sweden",
    );

    build_year(
        years,
        2014,
        vec![
            (NaiveDate::from_ymd_res(2014, 1, 1)?, "Nyårsdagen"),
            (NaiveDate::from_ymd_res(2014, 1, 6)?, "Trettondedag jul"),
            (NaiveDate::from_ymd_res(2014, 4, 18)?, "Långfredagen"),
            (NaiveDate::from_ymd_res(2014, 4, 20)?, "Påskdagen"),
            (NaiveDate::from_ymd_res(2014, 4, 21)?, "Annandag påsk"),
            (NaiveDate::from_ymd_res(2014, 5, 1)?, "Första maj"),
            (
                NaiveDate::from_ymd_res(2014, 5, 29)?,
                "Kristi himmelsfärdsdag",
            ),
            (NaiveDate::from_ymd_res(2014, 6, 6)?, "Nationaldagen"),
            (NaiveDate::from_ymd_res(2014, 6, 8)?, "Pingstdagen"),
            (NaiveDate::from_ymd_res(2014, 6, 21)?, "Midsommardagen"),
            (NaiveDate::from_ymd_res(2014, 11, 1)?, "Alla helgons dag"),
            (NaiveDate::from_ymd_res(2014, 12, 25)?, "Juldagen"),
            (NaiveDate::from_ymd_res(2014, 12, 26)?, "Annandag jul"),
        ],
        &mut map,
        Country::SE,
        "Sweden",
    );

    build_year(
        years,
        2015,
        vec![
            (NaiveDate::from_ymd_res(2015, 1, 1)?, "Nyårsdagen"),
            (NaiveDate::from_ymd_res(2015, 1, 6)?, "Trettondedag jul"),
            (NaiveDate::from_ymd_res(2015, 4, 3)?, "Långfredagen"),
            (NaiveDate::from_ymd_res(2015, 4, 5)?, "Påskdagen"),
            (NaiveDate::from_ymd_res(2015, 4, 6)?, "Annandag påsk"),
            (NaiveDate::from_ymd_res(2015, 5, 1)?, "Första maj"),
            (
                NaiveDate::from_ymd_res(2015, 5, 14)?,
                "Kristi himmelsfärdsdag",
            ),
            (NaiveDate::from_ymd_res(2015, 6, 6)?, "Nationaldagen"),
            (NaiveDate::from_ymd_res(2015, 5, 24)?, "Pingstdagen"),
            (NaiveDate::from_ymd_res(2015, 6, 20)?, "Midsommardagen"),
            (NaiveDate::from_ymd_res(2015, 10, 31)?, "Alla helgons dag"),
            (NaiveDate::from_ymd_res(2015, 12, 25)?, "Juldagen"),
            (NaiveDate::from_ymd_res(2015, 12, 26)?, "Annandag jul"),
        ],
        &mut map,
        Country::SE,
        "Sweden",
    );

    build_year(
        years,
        2016,
        vec![
            (NaiveDate::from_ymd_res(2016, 1, 1)?, "Nyårsdagen"),
            (NaiveDate::from_ymd_res(2016, 1, 6)?, "Trettondedag jul"),
            (NaiveDate::from_ymd_res(2016, 3, 25)?, "Långfredagen"),
            (NaiveDate::from_ymd_res(2016, 3, 27)?, "Påskdagen"),
            (NaiveDate::from_ymd_res(2016, 3, 28)?, "Annandag påsk"),
            (NaiveDate::from_ymd_res(2016, 5, 1)?, "Första maj"),
            (
                NaiveDate::from_ymd_res(2016, 5, 5)?,
                "Kristi himmelsfärdsdag",
            ),
            (NaiveDate::from_ymd_res(2016, 6, 6)?, "Nationaldagen"),
            (NaiveDate::from_ymd_res(2016, 5, 15)?, "Pingstdagen"),
            (NaiveDate::from_ymd_res(2016, 6, 25)?, "Midsommardagen"),
            (NaiveDate::from_ymd_res(2016, 11, 5)?, "Alla helgons dag"),
            (NaiveDate::from_ymd_res(2016, 12, 25)?, "Juldagen"),
            (NaiveDate::from_ymd_res(2016, 12, 26)?, "Annandag jul"),
        ],
        &mut map,
        Country::SE,
        "Sweden",
    );

    build_year(
        years,
        2017,
        vec![
            (NaiveDate::from_ymd_res(2017, 1, 1)?, "Nyårsdagen"),
            (NaiveDate::from_ymd_res(2017, 1, 6)?, "Trettondedag jul"),
            (NaiveDate::from_ymd_res(2017, 4, 14)?, "Långfredagen"),
            (NaiveDate::from_ymd_res(2017, 4, 16)?, "Påskdagen"),
            (NaiveDate::from_ymd_res(2017, 4, 17)?, "Annandag påsk"),
            (NaiveDate::from_ymd_res(2017, 5, 1)?, "Första maj"),
            (
                NaiveDate::from_ymd_res(2017, 5, 25)?,
                "Kristi himmelsfärdsdag",
            ),
            (NaiveDate::from_ymd_res(2017, 6, 6)?, "Nationaldagen"),
            (NaiveDate::from_ymd_res(2017, 6, 4)?, "Pingstdagen"),
            (NaiveDate::from_ymd_res(2017, 6, 24)?, "Midsommardagen"),
            (NaiveDate::from_ymd_res(2017, 11, 4)?, "Alla helgons dag"),
            (NaiveDate::from_ymd_res(2017, 12, 25)?, "Juldagen"),
            (NaiveDate::from_ymd_res(2017, 12, 26)?, "Annandag jul"),
        ],
        &mut map,
        Country::SE,
        "Sweden",
    );

    build_year(
        years,
        2018,
        vec![
            (NaiveDate::from_ymd_res(2018, 1, 1)?, "Nyårsdagen"),
            (NaiveDate::from_ymd_res(2018, 1, 6)?, "Trettondedag jul"),
            (NaiveDate::from_ymd_res(2018, 3, 30)?, "Långfredagen"),
            (NaiveDate::from_ymd_res(2018, 4, 1)?, "Påskdagen"),
            (NaiveDate::from_ymd_res(2018, 4, 2)?, "Annandag påsk"),
            (NaiveDate::from_ymd_res(2018, 5, 1)?, "Första maj"),
            (
                NaiveDate::from_ymd_res(2018, 5, 10)?,
                "Kristi himmelsfärdsdag",
            ),
            (NaiveDate::from_ymd_res(2018, 6, 6)?, "Nationaldagen"),
            (NaiveDate::from_ymd_res(2018, 5, 20)?, "Pingstdagen"),
            (NaiveDate::from_ymd_res(2018, 6, 23)?, "Midsommardagen"),
            (NaiveDate::from_ymd_res(2018, 11, 3)?, "Alla helgons dag"),
            (NaiveDate::from_ymd_res(2018, 12, 25)?, "Juldagen"),
            (NaiveDate::from_ymd_res(2018, 12, 26)?, "Annandag jul"),
        ],
        &mut map,
        Country::SE,
        "Sweden",
    );

    build_year(
        years,
        2019,
        vec![
            (NaiveDate::from_ymd_res(2019, 1, 1)?, "Nyårsdagen"),
            (NaiveDate::from_ymd_res(2019, 1, 6)?, "Trettondedag jul"),
            (NaiveDate::from_ymd_res(2019, 4, 19)?, "Långfredagen"),
            (NaiveDate::from_ymd_res(2019, 4, 21)?, "Påskdagen"),
            (NaiveDate::from_ymd_res(2019, 4, 22)?, "Annandag påsk"),
            (NaiveDate::from_ymd_res(2019, 5, 1)?, "Första maj"),
            (
                NaiveDate::from_ymd_res(2019, 5, 30)?,
                "Kristi himmelsfärdsdag",
            ),
            (NaiveDate::from_ymd_res(2019, 6, 6)?, "Nationaldagen"),
            (NaiveDate::from_ymd_res(2019, 6, 9)?, "Pingstdagen"),
            (NaiveDate::from_ymd_res(2019, 6, 22)?, "Midsommardagen"),
            (NaiveDate::from_ymd_res(2019, 11, 2)?, "Alla helgons dag"),
            (NaiveDate::from_ymd_res(2019, 12, 25)?, "Juldagen"),
            (NaiveDate::from_ymd_res(2019, 12, 26)?, "Annandag jul"),
        ],
        &mut map,
        Country::SE,
        "Sweden",
    );

    build_year(
        years,
        2020,
        vec![
            (NaiveDate::from_ymd_res(2020, 1, 1)?, "Nyårsdagen"),
            (NaiveDate::from_ymd_res(2020, 1, 6)?, "Trettondedag jul"),
            (NaiveDate::from_ymd_res(2020, 4, 10)?, "Långfredagen"),
            (NaiveDate::from_ymd_res(2020, 4, 12)?, "Påskdagen"),
            (NaiveDate::from_ymd_res(2020, 4, 13)?, "Annandag påsk"),
            (NaiveDate::from_ymd_res(2020, 5, 1)?, "Första maj"),
            (
                NaiveDate::from_ymd_res(2020, 5, 21)?,
                "Kristi himmelsfärdsdag",
            ),
            (NaiveDate::from_ymd_res(2020, 6, 6)?, "Nationaldagen"),
            (NaiveDate::from_ymd_res(2020, 5, 31)?, "Pingstdagen"),
            (NaiveDate::from_ymd_res(2020, 6, 20)?, "Midsommardagen"),
            (NaiveDate::from_ymd_res(2020, 10, 31)?, "Alla helgons dag"),
            (NaiveDate::from_ymd_res(2020, 12, 25)?, "Juldagen"),
            (NaiveDate::from_ymd_res(2020, 12, 26)?, "Annandag jul"),
        ],
        &mut map,
        Country::SE,
        "Sweden",
    );

    build_year(
        years,
        2021,
        vec![
            (NaiveDate::from_ymd_res(2021, 1, 1)?, "Nyårsdagen"),
            (NaiveDate::from_ymd_res(2021, 1, 6)?, "Trettondedag jul"),
            (NaiveDate::from_ymd_res(2021, 4, 2)?, "Långfredagen"),
            (NaiveDate::from_ymd_res(2021, 4, 4)?, "Påskdagen"),
            (NaiveDate::from_ymd_res(2021, 4, 5)?, "Annandag påsk"),
            (NaiveDate::from_ymd_res(2021, 5, 1)?, "Första maj"),
            (
                NaiveDate::from_ymd_res(2021, 5, 13)?,
                "Kristi himmelsfärdsdag",
            ),
            (NaiveDate::from_ymd_res(2021, 6, 6)?, "Nationaldagen"),
            (NaiveDate::from_ymd_res(2021, 5, 23)?, "Pingstdagen"),
            (NaiveDate::from_ymd_res(2021, 6, 26)?, "Midsommardagen"),
            (NaiveDate::from_ymd_res(2021, 11, 6)?, "Alla helgons dag"),
            (NaiveDate::from_ymd_res(2021, 12, 25)?, "Juldagen"),
            (NaiveDate::from_ymd_res(2021, 12, 26)?, "Annandag jul"),
        ],
        &mut map,
        Country::SE,
        "Sweden",
    );

    build_year(
        years,
        2022,
        vec![
            (NaiveDate::from_ymd_res(2022, 1, 1)?, "Nyårsdagen"),
            (NaiveDate::from_ymd_res(2022, 1, 6)?, "Trettondedag jul"),
            (NaiveDate::from_ymd_res(2022, 4, 15)?, "Långfredagen"),
            (NaiveDate::from_ymd_res(2022, 4, 17)?, "Påskdagen"),
            (NaiveDate::from_ymd_res(2022, 4, 18)?, "Annandag påsk"),
            (NaiveDate::from_ymd_res(2022, 5, 1)?, "Första maj"),
            (
                NaiveDate::from_ymd_res(2022, 5, 26)?,
                "Kristi himmelsfärdsdag",
            ),
            (NaiveDate::from_ymd_res(2022, 6, 6)?, "Nationaldagen"),
            (NaiveDate::from_ymd_res(2022, 6, 5)?, "Pingstdagen"),
            (NaiveDate::from_ymd_res(2022, 6, 25)?, "Midsommardagen"),
            (NaiveDate::from_ymd_res(2022, 11, 5)?, "Alla helgons dag"),
            (NaiveDate::from_ymd_res(2022, 12, 25)?, "Juldagen"),
            (NaiveDate::from_ymd_res(2022, 12, 26)?, "Annandag jul"),
        ],
        &mut map,
        Country::SE,
        "Sweden",
    );

    build_year(
        years,
        2023,
        vec![
            (NaiveDate::from_ymd_res(2023, 1, 1)?, "Nyårsdagen"),
            (NaiveDate::from_ymd_res(2023, 1, 6)?, "Trettondedag jul"),
            (NaiveDate::from_ymd_res(2023, 4, 7)?, "Långfredagen"),
            (NaiveDate::from_ymd_res(2023, 4, 9)?, "Påskdagen"),
            (NaiveDate::from_ymd_res(2023, 4, 10)?, "Annandag påsk"),
            (NaiveDate::from_ymd_res(2023, 5, 1)?, "Första maj"),
            (
                NaiveDate::from_ymd_res(2023, 5, 18)?,
                "Kristi himmelsfärdsdag",
            ),
            (NaiveDate::from_ymd_res(2023, 6, 6)?, "Nationaldagen"),
            (NaiveDate::from_ymd_res(2023, 5, 28)?, "Pingstdagen"),
            (NaiveDate::from_ymd_res(2023, 6, 24)?, "Midsommardagen"),
            (NaiveDate::from_ymd_res(2023, 11, 4)?, "Alla helgons dag"),
            (NaiveDate::from_ymd_res(2023, 12, 25)?, "Juldagen"),
            (NaiveDate::from_ymd_res(2023, 12, 26)?, "Annandag jul"),
        ],
        &mut map,
        Country::SE,
        "Sweden",
    );

    build_year(
        years,
        2024,
        vec![
            (NaiveDate::from_ymd_res(2024, 1, 1)?, "Nyårsdagen"),
            (NaiveDate::from_ymd_res(2024, 1, 6)?, "Trettondedag jul"),
            (NaiveDate::from_ymd_res(2024, 3, 29)?, "Långfredagen"),
            (NaiveDate::from_ymd_res(2024, 3, 31)?, "Påskdagen"),
            (NaiveDate::from_ymd_res(2024, 4, 1)?, "Annandag påsk"),
            (NaiveDate::from_ymd_res(2024, 5, 1)?, "Första maj"),
            (
                NaiveDate::from_ymd_res(2024, 5, 9)?,
                "Kristi himmelsfärdsdag",
            ),
            (NaiveDate::from_ymd_res(2024, 6, 6)?, "Nationaldagen"),
            (NaiveDate::from_ymd_res(2024, 5, 19)?, "Pingstdagen"),
            (NaiveDate::from_ymd_res(2024, 6, 22)?, "Midsommardagen"),
            (NaiveDate::from_ymd_res(2024, 11, 2)?, "Alla helgons dag"),
            (NaiveDate::from_ymd_res(2024, 12, 25)?, "Juldagen"),
            (NaiveDate::from_ymd_res(2024, 12, 26)?, "Annandag jul"),
        ],
        &mut map,
        Country::SE,
        "Sweden",
    );

    build_year(
        years,
        2025,
        vec![
            (NaiveDate::from_ymd_res(2025, 1, 1)?, "Nyårsdagen"),
            (NaiveDate::from_ymd_res(2025, 1, 6)?, "Trettondedag jul"),
            (NaiveDate::from_ymd_res(2025, 4, 18)?, "Långfredagen"),
            (NaiveDate::from_ymd_res(2025, 4, 20)?, "Påskdagen"),
            (NaiveDate::from_ymd_res(2025, 4, 21)?, "Annandag påsk"),
            (NaiveDate::from_ymd_res(2025, 5, 1)?, "Första maj"),
            (
                NaiveDate::from_ymd_res(2025, 5, 29)?,
                "Kristi himmelsfärdsdag",
            ),
            (NaiveDate::from_ymd_res(2025, 6, 6)?, "Nationaldagen"),
            (NaiveDate::from_ymd_res(2025, 6, 8)?, "Pingstdagen"),
            (NaiveDate::from_ymd_res(2025, 6, 21)?, "Midsommardagen"),
            (NaiveDate::from_ymd_res(2025, 11, 1)?, "Alla helgons dag"),
            (NaiveDate::from_ymd_res(2025, 12, 25)?, "Juldagen"),
            (NaiveDate::from_ymd_res(2025, 12, 26)?, "Annandag jul"),
        ],
        &mut map,
        Country::SE,
        "Sweden",
    );

    build_year(
        years,
        2026,
        vec![
            (NaiveDate::from_ymd_res(2026, 1, 1)?, "Nyårsdagen"),
            (NaiveDate::from_ymd_res(2026, 1, 6)?, "Trettondedag jul"),
            (NaiveDate::from_ymd_res(2026, 4, 3)?, "Långfredagen"),
            (NaiveDate::from_ymd_res(2026, 4, 5)?, "Påskdagen"),
            (NaiveDate::from_ymd_res(2026, 4, 6)?, "Annandag påsk"),
            (NaiveDate::from_ymd_res(2026, 5, 1)?, "Första maj"),
            (
                NaiveDate::from_ymd_res(2026, 5, 14)?,
                "Kristi himmelsfärdsdag",
            ),
            (NaiveDate::from_ymd_res(2026, 6, 6)?, "Nationaldagen"),
            (NaiveDate::from_ymd_res(2026, 5, 24)?, "Pingstdagen"),
            (NaiveDate::from_ymd_res(2026, 6, 20)?, "Midsommardagen"),
            (NaiveDate::from_ymd_res(2026, 10, 31)?, "Alla helgons dag"),
            (NaiveDate::from_ymd_res(2026, 12, 25)?, "Juldagen"),
            (NaiveDate::from_ymd_res(2026, 12, 26)?, "Annandag jul"),
        ],
        &mut map,
        Country::SE,
        "Sweden",
    );

    build_year(
        years,
        2027,
        vec![
            (NaiveDate::from_ymd_res(2027, 1, 1)?, "Nyårsdagen"),
            (NaiveDate::from_ymd_res(2027, 1, 6)?, "Trettondedag jul"),
            (NaiveDate::from_ymd_res(2027, 3, 26)?, "Långfredagen"),
            (NaiveDate::from_ymd_res(2027, 3, 28)?, "Påskdagen"),
            (NaiveDate::from_ymd_res(2027, 3, 29)?, "Annandag påsk"),
            (NaiveDate::from_ymd_res(2027, 5, 1)?, "Första maj"),
            (
                NaiveDate::from_ymd_res(2027, 5, 6)?,
                "Kristi himmelsfärdsdag",
            ),
            (NaiveDate::from_ymd_res(2027, 6, 6)?, "Nationaldagen"),
            (NaiveDate::from_ymd_res(2027, 5, 16)?, "Pingstdagen"),
            (NaiveDate::from_ymd_res(2027, 6, 26)?, "Midsommardagen"),
            (NaiveDate::from_ymd_res(2027, 11, 6)?, "Alla helgons dag"),
            (NaiveDate::from_ymd_res(2027, 12, 25)?, "Juldagen"),
            (NaiveDate::from_ymd_res(2027, 12, 26)?, "Annandag jul"),
        ],
        &mut map,
        Country::SE,
        "Sweden",
    );

    build_year(
        years,
        2028,
        vec![
            (NaiveDate::from_ymd_res(2028, 1, 1)?, "Nyårsdagen"),
            (NaiveDate::from_ymd_res(2028, 1, 6)?, "Trettondedag jul"),
            (NaiveDate::from_ymd_res(2028, 4, 14)?, "Långfredagen"),
            (NaiveDate::from_ymd_res(2028, 4, 16)?, "Påskdagen"),
            (NaiveDate::from_ymd_res(2028, 4, 17)?, "Annandag påsk"),
            (NaiveDate::from_ymd_res(2028, 5, 1)?, "Första maj"),
            (
                NaiveDate::from_ymd_res(2028, 5, 25)?,
                "Kristi himmelsfärdsdag",
            ),
            (NaiveDate::from_ymd_res(2028, 6, 6)?, "Nationaldagen"),
            (NaiveDate::from_ymd_res(2028, 6, 4)?, "Pingstdagen"),
            (NaiveDate::from_ymd_res(2028, 6, 24)?, "Midsommardagen"),
            (NaiveDate::from_ymd_res(2028, 11, 4)?, "Alla helgons dag"),
            (NaiveDate::from_ymd_res(2028, 12, 25)?, "Juldagen"),
            (NaiveDate::from_ymd_res(2028, 12, 26)?, "Annandag jul"),
        ],
        &mut map,
        Country::SE,
        "Sweden",
    );

    build_year(
        years,
        2029,
        vec![
            (NaiveDate::from_ymd_res(2029, 1, 1)?, "Nyårsdagen"),
            (NaiveDate::from_ymd_res(2029, 1, 6)?, "Trettondedag jul"),
            (NaiveDate::from_ymd_res(2029, 3, 30)?, "Långfredagen"),
            (NaiveDate::from_ymd_res(2029, 4, 1)?, "Påskdagen"),
            (NaiveDate::from_ymd_res(2029, 4, 2)?, "Annandag påsk"),
            (NaiveDate::from_ymd_res(2029, 5, 1)?, "Första maj"),
            (
                NaiveDate::from_ymd_res(2029, 5, 10)?,
                "Kristi himmelsfärdsdag",
            ),
            (NaiveDate::from_ymd_res(2029, 6, 6)?, "Nationaldagen"),
            (NaiveDate::from_ymd_res(2029, 5, 20)?, "Pingstdagen"),
            (NaiveDate::from_ymd_res(2029, 6, 23)?, "Midsommardagen"),
            (NaiveDate::from_ymd_res(2029, 11, 3)?, "Alla helgons dag"),
            (NaiveDate::from_ymd_res(2029, 12, 25)?, "Juldagen"),
            (NaiveDate::from_ymd_res(2029, 12, 26)?, "Annandag jul"),
        ],
        &mut map,
        Country::SE,
        "Sweden",
    );

    build_year(
        years,
        2030,
        vec![
            (NaiveDate::from_ymd_res(2030, 1, 1)?, "Nyårsdagen"),
            (NaiveDate::from_ymd_res(2030, 1, 6)?, "Trettondedag jul"),
            (NaiveDate::from_ymd_res(2030, 4, 19)?, "Långfredagen"),
            (NaiveDate::from_ymd_res(2030, 4, 21)?, "Påskdagen"),
            (NaiveDate::from_ymd_res(2030, 4, 22)?, "Annandag påsk"),
            (NaiveDate::from_ymd_res(2030, 5, 1)?, "Första maj"),
            (
                NaiveDate::from_ymd_res(2030, 5, 30)?,
                "Kristi himmelsfärdsdag",
            ),
            (NaiveDate::from_ymd_res(2030, 6, 6)?, "Nationaldagen"),
            (NaiveDate::from_ymd_res(2030, 6, 9)?, "Pingstdagen"),
            (NaiveDate::from_ymd_res(2030, 6, 22)?, "Midsommardagen"),
            (NaiveDate::from_ymd_res(2030, 11, 2)?, "Alla helgons dag"),
            (NaiveDate::from_ymd_res(2030, 12, 25)?, "Juldagen"),
            (NaiveDate::from_ymd_res(2030, 12, 26)?, "Annandag jul"),
        ],
        &mut map,
        Country::SE,
        "Sweden",
    );

    Ok(map)
}
