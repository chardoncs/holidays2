//! Armenia
use super::*;

/// Generate holiday map for Armenia.
#[allow(unused_mut, unused_variables)]
pub fn build(
    years: &Option<&std::ops::Range<Year>>,
) -> Result<HashMap<Year, BTreeMap<NaiveDate, Holiday>>> {
    let mut map = HashMap::new();

    build_year(
        years,
        2001,
        vec![
            (NaiveDate::from_ymd_res(2001, 1, 1)?, "Նոր տարվա օր"),
            (NaiveDate::from_ymd_res(2001, 1, 2)?, "Նոր տարվա օր"),
            (
                NaiveDate::from_ymd_res(2001, 1, 6)?,
                "Սուրբ Ծնունդ եւ Հայտնություն",
            ),
            (NaiveDate::from_ymd_res(2001, 3, 8)?, "Կանանց միջազգային օր"),
            (
                NaiveDate::from_ymd_res(2001, 4, 7)?,
                "Մայրության և գեղեցկության տոն",
            ),
            (
                NaiveDate::from_ymd_res(2001, 4, 24)?,
                "Ցեղասպանության զոհերի հիշատակի օր",
            ),
            (
                NaiveDate::from_ymd_res(2001, 5, 1)?,
                "Աշխատավորների համերաշխության միջազգային օր",
            ),
            (
                NaiveDate::from_ymd_res(2001, 5, 9)?,
                "Հաղթանակի և Խաղաղության տոն",
            ),
            (NaiveDate::from_ymd_res(2001, 5, 28)?, "Հանրապետության օր"),
            (NaiveDate::from_ymd_res(2001, 7, 5)?, "Սահմանադրության օր"),
            (NaiveDate::from_ymd_res(2001, 9, 21)?, "Անկախության օր"),
            (NaiveDate::from_ymd_res(2001, 12, 31)?, "Նոր տարվա գիշեր"),
        ],
        &mut map,
        Country::AM,
        "Armenia",
    );

    build_year(
        years,
        2002,
        vec![
            (NaiveDate::from_ymd_res(2002, 1, 1)?, "Նոր տարվա օր"),
            (NaiveDate::from_ymd_res(2002, 1, 2)?, "Նոր տարվա օր"),
            (
                NaiveDate::from_ymd_res(2002, 1, 6)?,
                "Սուրբ Ծնունդ եւ Հայտնություն",
            ),
            (NaiveDate::from_ymd_res(2002, 3, 8)?, "Կանանց տոն"),
            (
                NaiveDate::from_ymd_res(2002, 4, 24)?,
                "Ցեղասպանության զոհերի հիշատակի օր",
            ),
            (NaiveDate::from_ymd_res(2002, 5, 1)?, "Աշխատանքի օր"),
            (
                NaiveDate::from_ymd_res(2002, 5, 9)?,
                "Հաղթանակի և Խաղաղության տոն",
            ),
            (NaiveDate::from_ymd_res(2002, 5, 28)?, "Հանրապետության օր"),
            (NaiveDate::from_ymd_res(2002, 7, 5)?, "Սահմանադրության օր"),
            (NaiveDate::from_ymd_res(2002, 9, 21)?, "Անկախության օր"),
            (NaiveDate::from_ymd_res(2002, 12, 31)?, "Նոր տարվա գիշեր"),
        ],
        &mut map,
        Country::AM,
        "Armenia",
    );

    build_year(
        years,
        2003,
        vec![
            (NaiveDate::from_ymd_res(2003, 1, 1)?, "Նոր տարվա օր"),
            (NaiveDate::from_ymd_res(2003, 1, 2)?, "Նոր տարվա օր"),
            (
                NaiveDate::from_ymd_res(2003, 1, 6)?,
                "Սուրբ Ծնունդ եւ Հայտնություն",
            ),
            (NaiveDate::from_ymd_res(2003, 1, 28)?, "Բանակի օր"),
            (NaiveDate::from_ymd_res(2003, 3, 8)?, "Կանանց տոն"),
            (
                NaiveDate::from_ymd_res(2003, 4, 24)?,
                "Ցեղասպանության զոհերի հիշատակի օր",
            ),
            (NaiveDate::from_ymd_res(2003, 5, 1)?, "Աշխատանքի օր"),
            (
                NaiveDate::from_ymd_res(2003, 5, 9)?,
                "Հաղթանակի և Խաղաղության տոն",
            ),
            (NaiveDate::from_ymd_res(2003, 5, 28)?, "Հանրապետության օր"),
            (NaiveDate::from_ymd_res(2003, 7, 5)?, "Սահմանադրության օր"),
            (NaiveDate::from_ymd_res(2003, 9, 21)?, "Անկախության օր"),
            (NaiveDate::from_ymd_res(2003, 12, 31)?, "Նոր տարվա գիշեր"),
            (
                NaiveDate::from_ymd_res(2003, 1, 3)?,
                "Հանգստյան օր (հետաձգվել է 29.12.2002 թվականից)",
            ),
        ],
        &mut map,
        Country::AM,
        "Armenia",
    );

    build_year(
        years,
        2004,
        vec![
            (NaiveDate::from_ymd_res(2004, 1, 1)?, "Նոր տարվա օր"),
            (NaiveDate::from_ymd_res(2004, 1, 2)?, "Նոր տարվա օր"),
            (
                NaiveDate::from_ymd_res(2004, 1, 6)?,
                "Սուրբ Ծնունդ եւ Հայտնություն",
            ),
            (NaiveDate::from_ymd_res(2004, 1, 28)?, "Բանակի օր"),
            (NaiveDate::from_ymd_res(2004, 3, 8)?, "Կանանց տոն"),
            (
                NaiveDate::from_ymd_res(2004, 4, 24)?,
                "Ցեղասպանության զոհերի հիշատակի օր",
            ),
            (NaiveDate::from_ymd_res(2004, 5, 1)?, "Աշխատանքի օր"),
            (
                NaiveDate::from_ymd_res(2004, 5, 9)?,
                "Հաղթանակի և Խաղաղության տոն",
            ),
            (NaiveDate::from_ymd_res(2004, 5, 28)?, "Հանրապետության օր"),
            (NaiveDate::from_ymd_res(2004, 7, 5)?, "Սահմանադրության օր"),
            (NaiveDate::from_ymd_res(2004, 9, 21)?, "Անկախության օր"),
            (NaiveDate::from_ymd_res(2004, 12, 31)?, "Նոր տարվա գիշեր"),
        ],
        &mut map,
        Country::AM,
        "Armenia",
    );

    build_year(
        years,
        2005,
        vec![
            (NaiveDate::from_ymd_res(2005, 1, 1)?, "Նոր տարվա օր"),
            (NaiveDate::from_ymd_res(2005, 1, 2)?, "Նոր տարվա օր"),
            (
                NaiveDate::from_ymd_res(2005, 1, 6)?,
                "Սուրբ Ծնունդ եւ Հայտնություն",
            ),
            (NaiveDate::from_ymd_res(2005, 1, 28)?, "Բանակի օր"),
            (NaiveDate::from_ymd_res(2005, 3, 8)?, "Կանանց տոն"),
            (
                NaiveDate::from_ymd_res(2005, 4, 24)?,
                "Ցեղասպանության զոհերի հիշատակի օր",
            ),
            (NaiveDate::from_ymd_res(2005, 5, 1)?, "Աշխատանքի օր"),
            (
                NaiveDate::from_ymd_res(2005, 5, 9)?,
                "Հաղթանակի և Խաղաղության տոն",
            ),
            (NaiveDate::from_ymd_res(2005, 5, 28)?, "Հանրապետության օր"),
            (NaiveDate::from_ymd_res(2005, 7, 5)?, "Սահմանադրության օր"),
            (NaiveDate::from_ymd_res(2005, 9, 21)?, "Անկախության օր"),
            (NaiveDate::from_ymd_res(2005, 12, 31)?, "Նոր տարվա գիշեր"),
            (
                NaiveDate::from_ymd_res(2005, 1, 3)?,
                "Հանգստյան օր (հետաձգվել է 08.01.2005 թվականից)",
            ),
            (
                NaiveDate::from_ymd_res(2005, 3, 7)?,
                "Հանգստյան օր (հետաձգվել է 05.03.2005 թվականից)",
            ),
            (
                NaiveDate::from_ymd_res(2005, 7, 4)?,
                "Հանգստյան օր (հետաձգվել է 02.07.2005 թվականից)",
            ),
        ],
        &mut map,
        Country::AM,
        "Armenia",
    );

    build_year(
        years,
        2006,
        vec![
            (NaiveDate::from_ymd_res(2006, 1, 1)?, "Նոր տարվա օր"),
            (NaiveDate::from_ymd_res(2006, 1, 2)?, "Նոր տարվա օր"),
            (
                NaiveDate::from_ymd_res(2006, 1, 6)?,
                "Սուրբ Ծնունդ եւ Հայտնություն",
            ),
            (NaiveDate::from_ymd_res(2006, 1, 28)?, "Բանակի օր"),
            (NaiveDate::from_ymd_res(2006, 3, 8)?, "Կանանց տոն"),
            (
                NaiveDate::from_ymd_res(2006, 4, 24)?,
                "Ցեղասպանության զոհերի հիշատակի օր",
            ),
            (NaiveDate::from_ymd_res(2006, 5, 1)?, "Աշխատանքի օր"),
            (
                NaiveDate::from_ymd_res(2006, 5, 9)?,
                "Հաղթանակի և Խաղաղության տոն",
            ),
            (NaiveDate::from_ymd_res(2006, 5, 28)?, "Հանրապետության օր"),
            (NaiveDate::from_ymd_res(2006, 7, 5)?, "Սահմանադրության օր"),
            (NaiveDate::from_ymd_res(2006, 9, 21)?, "Անկախության օր"),
            (NaiveDate::from_ymd_res(2006, 12, 31)?, "Նոր տարվա գիշեր"),
            (
                NaiveDate::from_ymd_res(2006, 1, 3)?,
                "Հանգստյան օր (հետաձգվել է 08.01.2006 թվականից)",
            ),
            (
                NaiveDate::from_ymd_res(2006, 5, 8)?,
                "Հանգստյան օր (հետաձգվել է 06.05.2006 թվականից)",
            ),
        ],
        &mut map,
        Country::AM,
        "Armenia",
    );

    build_year(
        years,
        2007,
        vec![
            (NaiveDate::from_ymd_res(2007, 1, 1)?, "Նոր տարվա օր"),
            (NaiveDate::from_ymd_res(2007, 1, 2)?, "Նոր տարվա օր"),
            (
                NaiveDate::from_ymd_res(2007, 1, 6)?,
                "Սուրբ Ծնունդ եւ Հայտնություն",
            ),
            (NaiveDate::from_ymd_res(2007, 1, 28)?, "Բանակի օր"),
            (NaiveDate::from_ymd_res(2007, 3, 8)?, "Կանանց տոն"),
            (
                NaiveDate::from_ymd_res(2007, 4, 24)?,
                "Ցեղասպանության զոհերի հիշատակի օր",
            ),
            (NaiveDate::from_ymd_res(2007, 5, 1)?, "Աշխատանքի օր"),
            (
                NaiveDate::from_ymd_res(2007, 5, 9)?,
                "Հաղթանակի և Խաղաղության տոն",
            ),
            (NaiveDate::from_ymd_res(2007, 5, 28)?, "Հանրապետության օր"),
            (NaiveDate::from_ymd_res(2007, 7, 5)?, "Սահմանադրության օր"),
            (NaiveDate::from_ymd_res(2007, 9, 21)?, "Անկախության օր"),
            (NaiveDate::from_ymd_res(2007, 12, 31)?, "Նոր տարվա գիշեր"),
            (
                NaiveDate::from_ymd_res(2007, 1, 3)?,
                "Հանգստյան օր (հետաձգվել է 30.12.2006 թվականից)",
            ),
            (
                NaiveDate::from_ymd_res(2007, 1, 4)?,
                "Հանգստյան օր (հետաձգվել է 07.01.2007 թվականից)",
            ),
            (
                NaiveDate::from_ymd_res(2007, 4, 30)?,
                "Հանգստյան օր (հետաձգվել է 28.04.2007 թվականից)",
            ),
        ],
        &mut map,
        Country::AM,
        "Armenia",
    );

    build_year(
        years,
        2008,
        vec![
            (NaiveDate::from_ymd_res(2008, 1, 1)?, "Նոր տարվա օր"),
            (NaiveDate::from_ymd_res(2008, 1, 2)?, "Նոր տարվա օր"),
            (
                NaiveDate::from_ymd_res(2008, 1, 6)?,
                "Սուրբ Ծնունդ եւ Հայտնություն",
            ),
            (NaiveDate::from_ymd_res(2008, 1, 28)?, "Բանակի օր"),
            (NaiveDate::from_ymd_res(2008, 3, 8)?, "Կանանց տոն"),
            (
                NaiveDate::from_ymd_res(2008, 4, 24)?,
                "Ցեղասպանության զոհերի հիշատակի օր",
            ),
            (NaiveDate::from_ymd_res(2008, 5, 1)?, "Աշխատանքի օր"),
            (
                NaiveDate::from_ymd_res(2008, 5, 9)?,
                "Հաղթանակի և Խաղաղության տոն",
            ),
            (NaiveDate::from_ymd_res(2008, 5, 28)?, "Հանրապետության օր"),
            (NaiveDate::from_ymd_res(2008, 7, 5)?, "Սահմանադրության օր"),
            (NaiveDate::from_ymd_res(2008, 9, 21)?, "Անկախության օր"),
            (NaiveDate::from_ymd_res(2008, 12, 31)?, "Նոր տարվա գիշեր"),
            (
                NaiveDate::from_ymd_res(2008, 1, 3)?,
                "Հանգստյան օր (հետաձգվել է 29.12.2007 թվականից)",
            ),
            (
                NaiveDate::from_ymd_res(2008, 1, 4)?,
                "Հանգստյան օր (հետաձգվել է 12.01.2008 թվականից)",
            ),
            (
                NaiveDate::from_ymd_res(2008, 5, 2)?,
                "Հանգստյան օր (հետաձգվել է 04.05.2008 թվականից)",
            ),
            (
                NaiveDate::from_ymd_res(2008, 6, 30)?,
                "Հանգստյան օր (հետաձգվել է 28.06.2008 թվականից)",
            ),
            (
                NaiveDate::from_ymd_res(2008, 8, 18)?,
                "Հանգստյան օր (հետաձգվել է 16.08.2008 թվականից)",
            ),
            (
                NaiveDate::from_ymd_res(2008, 9, 15)?,
                "Հանգստյան օր (հետաձգվել է 13.09.2008 թվականից)",
            ),
        ],
        &mut map,
        Country::AM,
        "Armenia",
    );

    build_year(
        years,
        2009,
        vec![
            (NaiveDate::from_ymd_res(2009, 1, 1)?, "Նոր տարվա օր"),
            (NaiveDate::from_ymd_res(2009, 1, 2)?, "Նոր տարվա օր"),
            (
                NaiveDate::from_ymd_res(2009, 1, 6)?,
                "Սուրբ Ծնունդ եւ Հայտնություն",
            ),
            (NaiveDate::from_ymd_res(2009, 1, 28)?, "Բանակի օր"),
            (NaiveDate::from_ymd_res(2009, 3, 8)?, "Կանանց տոն"),
            (
                NaiveDate::from_ymd_res(2009, 4, 24)?,
                "Ցեղասպանության զոհերի հիշատակի օր",
            ),
            (NaiveDate::from_ymd_res(2009, 5, 1)?, "Աշխատանքի օր"),
            (
                NaiveDate::from_ymd_res(2009, 5, 9)?,
                "Հաղթանակի և Խաղաղության տոն",
            ),
            (NaiveDate::from_ymd_res(2009, 5, 28)?, "Հանրապետության օր"),
            (NaiveDate::from_ymd_res(2009, 7, 5)?, "Սահմանադրության օր"),
            (NaiveDate::from_ymd_res(2009, 9, 21)?, "Անկախության օր"),
            (NaiveDate::from_ymd_res(2009, 12, 31)?, "Նոր տարվա գիշեր"),
            (
                NaiveDate::from_ymd_res(2009, 1, 5)?,
                "Հանգստյան օր (հետաձգվել է 10.01.2009 թվականից)",
            ),
            (
                NaiveDate::from_ymd_res(2009, 1, 7)?,
                "Հանգստյան օր (հետաձգվել է 27.12.2008 թվականից)",
            ),
            (
                NaiveDate::from_ymd_res(2009, 4, 13)?,
                "Հանգստյան օր (հետաձգվել է 18.04.2009 թվականից)",
            ),
            (
                NaiveDate::from_ymd_res(2009, 7, 20)?,
                "Հանգստյան օր (հետաձգվել է 18.07.2009 թվականից)",
            ),
            (
                NaiveDate::from_ymd_res(2009, 8, 17)?,
                "Հանգստյան օր (հետաձգվել է 15.08.2009 թվականից)",
            ),
            (
                NaiveDate::from_ymd_res(2009, 9, 14)?,
                "Հանգստյան օր (հետաձգվել է 12.09.2009 թվականից)",
            ),
        ],
        &mut map,
        Country::AM,
        "Armenia",
    );

    build_year(
        years,
        2010,
        vec![
            (NaiveDate::from_ymd_res(2010, 1, 1)?, "Նոր տարվա օր"),
            (NaiveDate::from_ymd_res(2010, 1, 2)?, "Նոր տարվա օր"),
            (
                NaiveDate::from_ymd_res(2010, 1, 6)?,
                "Սուրբ Ծնունդ եւ Հայտնություն",
            ),
            (NaiveDate::from_ymd_res(2010, 1, 3)?, "նախածննդյան տոներ"),
            (NaiveDate::from_ymd_res(2010, 1, 4)?, "նախածննդյան տոներ"),
            (NaiveDate::from_ymd_res(2010, 1, 5)?, "նախածննդյան տոներ"),
            (NaiveDate::from_ymd_res(2010, 1, 7)?, "Մեռելոց հիշատակի օր"),
            (NaiveDate::from_ymd_res(2010, 1, 28)?, "Բանակի օր"),
            (NaiveDate::from_ymd_res(2010, 3, 8)?, "Կանանց տոն"),
            (
                NaiveDate::from_ymd_res(2010, 4, 24)?,
                "Ցեղասպանության զոհերի հիշատակի օր",
            ),
            (NaiveDate::from_ymd_res(2010, 5, 1)?, "Աշխատանքի օր"),
            (
                NaiveDate::from_ymd_res(2010, 5, 9)?,
                "Հաղթանակի և Խաղաղության տոն",
            ),
            (NaiveDate::from_ymd_res(2010, 5, 28)?, "Հանրապետության օր"),
            (NaiveDate::from_ymd_res(2010, 7, 5)?, "Սահմանադրության օր"),
            (NaiveDate::from_ymd_res(2010, 9, 21)?, "Անկախության օր"),
            (NaiveDate::from_ymd_res(2010, 12, 31)?, "Նոր տարվա գիշեր"),
            (
                NaiveDate::from_ymd_res(2010, 1, 8)?,
                "Հանգստյան օր (հետաձգվել է 26.12.2009 թվականից)",
            ),
            (
                NaiveDate::from_ymd_res(2010, 4, 5)?,
                "Հանգստյան օր (հետաձգվել է 10.04.2010 թվականից)",
            ),
            (
                NaiveDate::from_ymd_res(2010, 7, 12)?,
                "Հանգստյան օր (հետաձգվել է 17.07.2010 թվականից)",
            ),
            (
                NaiveDate::from_ymd_res(2010, 8, 16)?,
                "Հանգստյան օր (հետաձգվել է 21.08.2010 թվականից)",
            ),
            (
                NaiveDate::from_ymd_res(2010, 9, 13)?,
                "Հանգստյան օր (հետաձգվել է 18.09.2010 թվականից)",
            ),
            (
                NaiveDate::from_ymd_res(2010, 9, 20)?,
                "Հանգստյան օր (հետաձգվել է 11.09.2010 թվականից)",
            ),
        ],
        &mut map,
        Country::AM,
        "Armenia",
    );

    build_year(
        years,
        2011,
        vec![
            (NaiveDate::from_ymd_res(2011, 1, 1)?, "Նոր տարվա օր"),
            (NaiveDate::from_ymd_res(2011, 1, 2)?, "Նոր տարվա օր"),
            (
                NaiveDate::from_ymd_res(2011, 1, 6)?,
                "Սուրբ Ծնունդ եւ Հայտնություն",
            ),
            (NaiveDate::from_ymd_res(2011, 1, 3)?, "նախածննդյան տոներ"),
            (NaiveDate::from_ymd_res(2011, 1, 4)?, "նախածննդյան տոներ"),
            (NaiveDate::from_ymd_res(2011, 1, 5)?, "նախածննդյան տոներ"),
            (NaiveDate::from_ymd_res(2011, 1, 7)?, "Մեռելոց հիշատակի օր"),
            (NaiveDate::from_ymd_res(2011, 1, 28)?, "Բանակի օր"),
            (NaiveDate::from_ymd_res(2011, 3, 8)?, "Կանանց տոն"),
            (
                NaiveDate::from_ymd_res(2011, 4, 24)?,
                "Ցեղասպանության զոհերի հիշատակի օր",
            ),
            (NaiveDate::from_ymd_res(2011, 5, 1)?, "Աշխատանքի օր"),
            (
                NaiveDate::from_ymd_res(2011, 5, 9)?,
                "Հաղթանակի և Խաղաղության տոն",
            ),
            (NaiveDate::from_ymd_res(2011, 5, 28)?, "Հանրապետության օր"),
            (NaiveDate::from_ymd_res(2011, 7, 5)?, "Սահմանադրության օր"),
            (NaiveDate::from_ymd_res(2011, 9, 21)?, "Անկախության օր"),
            (NaiveDate::from_ymd_res(2011, 12, 31)?, "Նոր տարվա գիշեր"),
            (
                NaiveDate::from_ymd_res(2011, 3, 7)?,
                "Հանգստյան օր (հետաձգվել է 12.03.2011 թվականից)",
            ),
            (
                NaiveDate::from_ymd_res(2011, 4, 25)?,
                "Հանգստյան օր (հետաձգվել է 30.04.2011 թվականից)",
            ),
            (
                NaiveDate::from_ymd_res(2011, 7, 4)?,
                "Հանգստյան օր (հետաձգվել է 09.07.2011 թվականից)",
            ),
            (
                NaiveDate::from_ymd_res(2011, 8, 1)?,
                "Հանգստյան օր (հետաձգվել է 06.08.2011 թվականից)",
            ),
            (
                NaiveDate::from_ymd_res(2011, 8, 15)?,
                "Հանգստյան օր (հետաձգվել է 20.08.2011 թվականից)",
            ),
            (
                NaiveDate::from_ymd_res(2011, 9, 12)?,
                "Հանգստյան օր (հետաձգվել է 17.09.2011 թվականից)",
            ),
        ],
        &mut map,
        Country::AM,
        "Armenia",
    );

    build_year(
        years,
        2012,
        vec![
            (NaiveDate::from_ymd_res(2012, 1, 1)?, "Նոր տարվա օր"),
            (NaiveDate::from_ymd_res(2012, 1, 2)?, "Նոր տարվա օր"),
            (
                NaiveDate::from_ymd_res(2012, 1, 6)?,
                "Սուրբ Ծնունդ եւ Հայտնություն",
            ),
            (NaiveDate::from_ymd_res(2012, 1, 3)?, "նախածննդյան տոներ"),
            (NaiveDate::from_ymd_res(2012, 1, 4)?, "նախածննդյան տոներ"),
            (NaiveDate::from_ymd_res(2012, 1, 5)?, "նախածննդյան տոներ"),
            (NaiveDate::from_ymd_res(2012, 1, 7)?, "Մեռելոց հիշատակի օր"),
            (NaiveDate::from_ymd_res(2012, 1, 28)?, "Բանակի օր"),
            (NaiveDate::from_ymd_res(2012, 3, 8)?, "Կանանց տոն"),
            (
                NaiveDate::from_ymd_res(2012, 4, 24)?,
                "Ցեղասպանության զոհերի հիշատակի օր",
            ),
            (NaiveDate::from_ymd_res(2012, 5, 1)?, "Աշխատանքի օր"),
            (
                NaiveDate::from_ymd_res(2012, 5, 9)?,
                "Հաղթանակի և Խաղաղության տոն",
            ),
            (NaiveDate::from_ymd_res(2012, 5, 28)?, "Հանրապետության օր"),
            (NaiveDate::from_ymd_res(2012, 7, 5)?, "Սահմանադրության օր"),
            (NaiveDate::from_ymd_res(2012, 9, 21)?, "Անկախության օր"),
            (NaiveDate::from_ymd_res(2012, 12, 31)?, "Նոր տարվա գիշեր"),
            (
                NaiveDate::from_ymd_res(2012, 3, 9)?,
                "Հանգստյան օր (հետաձգվել է 24.03.2012 թվականից)",
            ),
            (
                NaiveDate::from_ymd_res(2012, 4, 9)?,
                "Հանգստյան օր (հետաձգվել է 14.04.2012 թվականից)",
            ),
            (
                NaiveDate::from_ymd_res(2012, 4, 30)?,
                "Հանգստյան օր (հետաձգվել է 28.04.2012 թվականից)",
            ),
            (
                NaiveDate::from_ymd_res(2012, 7, 6)?,
                "Հանգստյան օր (հետաձգվել է 14.07.2012 թվականից)",
            ),
            (
                NaiveDate::from_ymd_res(2012, 7, 16)?,
                "Հանգստյան օր (հետաձգվել է 21.07.2012 թվականից)",
            ),
            (
                NaiveDate::from_ymd_res(2012, 8, 13)?,
                "Հանգստյան օր (հետաձգվել է 18.08.2012 թվականից)",
            ),
            (
                NaiveDate::from_ymd_res(2012, 9, 17)?,
                "Հանգստյան օր (հետաձգվել է 29.09.2012 թվականից)",
            ),
        ],
        &mut map,
        Country::AM,
        "Armenia",
    );

    build_year(
        years,
        2013,
        vec![
            (NaiveDate::from_ymd_res(2013, 1, 1)?, "Նոր տարվա օր"),
            (NaiveDate::from_ymd_res(2013, 1, 2)?, "Նոր տարվա օր"),
            (
                NaiveDate::from_ymd_res(2013, 1, 6)?,
                "Սուրբ Ծնունդ եւ Հայտնություն",
            ),
            (NaiveDate::from_ymd_res(2013, 1, 3)?, "նախածննդյան տոներ"),
            (NaiveDate::from_ymd_res(2013, 1, 4)?, "նախածննդյան տոներ"),
            (NaiveDate::from_ymd_res(2013, 1, 5)?, "նախածննդյան տոներ"),
            (NaiveDate::from_ymd_res(2013, 1, 7)?, "Մեռելոց հիշատակի օր"),
            (NaiveDate::from_ymd_res(2013, 1, 28)?, "Բանակի օր"),
            (NaiveDate::from_ymd_res(2013, 3, 8)?, "Կանանց տոն"),
            (
                NaiveDate::from_ymd_res(2013, 4, 24)?,
                "Ցեղասպանության զոհերի հիշատակի օր",
            ),
            (NaiveDate::from_ymd_res(2013, 5, 1)?, "Աշխատանքի օր"),
            (
                NaiveDate::from_ymd_res(2013, 5, 9)?,
                "Հաղթանակի և Խաղաղության տոն",
            ),
            (NaiveDate::from_ymd_res(2013, 5, 28)?, "Հանրապետության օր"),
            (NaiveDate::from_ymd_res(2013, 7, 5)?, "Սահմանադրության օր"),
            (NaiveDate::from_ymd_res(2013, 9, 21)?, "Անկախության օր"),
            (NaiveDate::from_ymd_res(2013, 12, 31)?, "Նոր տարվա գիշեր"),
            (
                NaiveDate::from_ymd_res(2013, 4, 1)?,
                "Հանգստյան օր (հետաձգվել է 13.04.2013 թվականից)",
            ),
            (
                NaiveDate::from_ymd_res(2013, 5, 10)?,
                "Հանգստյան օր (հետաձգվել է 18.05.2013 թվականից)",
            ),
            (
                NaiveDate::from_ymd_res(2013, 5, 27)?,
                "Հանգստյան օր (հետաձգվել է 01.06.2013 թվականից)",
            ),
            (
                NaiveDate::from_ymd_res(2013, 7, 8)?,
                "Հանգստյան օր (հետաձգվել է 13.07.2013 թվականից)",
            ),
            (
                NaiveDate::from_ymd_res(2013, 8, 19)?,
                "Հանգստյան օր (հետաձգվել է 24.08.2013 թվականից)",
            ),
            (
                NaiveDate::from_ymd_res(2013, 9, 16)?,
                "Հանգստյան օր (հետաձգվել է 28.09.2013 թվականից)",
            ),
            (
                NaiveDate::from_ymd_res(2013, 12, 30)?,
                "Հանգստյան օր (հետաձգվել է 28.12.2013 թվականից)",
            ),
        ],
        &mut map,
        Country::AM,
        "Armenia",
    );

    build_year(
        years,
        2014,
        vec![
            (NaiveDate::from_ymd_res(2014, 1, 1)?, "Նոր տարվա օր"),
            (NaiveDate::from_ymd_res(2014, 1, 2)?, "Նոր տարվա օր"),
            (
                NaiveDate::from_ymd_res(2014, 1, 6)?,
                "Սուրբ Ծնունդ եւ Հայտնություն",
            ),
            (NaiveDate::from_ymd_res(2014, 1, 3)?, "նախածննդյան տոներ"),
            (NaiveDate::from_ymd_res(2014, 1, 4)?, "նախածննդյան տոներ"),
            (NaiveDate::from_ymd_res(2014, 1, 5)?, "նախածննդյան տոներ"),
            (NaiveDate::from_ymd_res(2014, 1, 7)?, "Մեռելոց հիշատակի օր"),
            (NaiveDate::from_ymd_res(2014, 1, 28)?, "Բանակի օր"),
            (NaiveDate::from_ymd_res(2014, 3, 8)?, "Կանանց տոն"),
            (
                NaiveDate::from_ymd_res(2014, 4, 24)?,
                "Ցեղասպանության զոհերի հիշատակի օր",
            ),
            (NaiveDate::from_ymd_res(2014, 5, 1)?, "Աշխատանքի օր"),
            (
                NaiveDate::from_ymd_res(2014, 5, 9)?,
                "Հաղթանակի և Խաղաղության տոն",
            ),
            (NaiveDate::from_ymd_res(2014, 5, 28)?, "Հանրապետության օր"),
            (NaiveDate::from_ymd_res(2014, 7, 5)?, "Սահմանադրության օր"),
            (NaiveDate::from_ymd_res(2014, 9, 21)?, "Անկախության օր"),
            (NaiveDate::from_ymd_res(2014, 12, 31)?, "Նոր տարվա գիշեր"),
            (
                NaiveDate::from_ymd_res(2014, 1, 27)?,
                "Հանգստյան օր (հետաձգվել է 01.02.2014 թվականից)",
            ),
            (
                NaiveDate::from_ymd_res(2014, 4, 21)?,
                "Հանգստյան օր (հետաձգվել է 26.04.2014 թվականից)",
            ),
            (
                NaiveDate::from_ymd_res(2014, 5, 2)?,
                "Հանգստյան օր (հետաձգվել է 31.05.2014 թվականից)",
            ),
            (
                NaiveDate::from_ymd_res(2014, 7, 28)?,
                "Հանգստյան օր (հետաձգվել է 02.08.2014 թվականից)",
            ),
            (
                NaiveDate::from_ymd_res(2014, 8, 18)?,
                "Հանգստյան օր (հետաձգվել է 23.08.2014 թվականից)",
            ),
            (
                NaiveDate::from_ymd_res(2014, 9, 15)?,
                "Հանգստյան օր (հետաձգվել է 27.09.2014 թվականից)",
            ),
        ],
        &mut map,
        Country::AM,
        "Armenia",
    );

    build_year(
        years,
        2015,
        vec![
            (NaiveDate::from_ymd_res(2015, 1, 1)?, "Նոր տարվա օր"),
            (NaiveDate::from_ymd_res(2015, 1, 2)?, "Նոր տարվա օր"),
            (
                NaiveDate::from_ymd_res(2015, 1, 6)?,
                "Սուրբ Ծնունդ եւ Հայտնություն",
            ),
            (NaiveDate::from_ymd_res(2015, 1, 3)?, "նախածննդյան տոներ"),
            (NaiveDate::from_ymd_res(2015, 1, 4)?, "նախածննդյան տոներ"),
            (NaiveDate::from_ymd_res(2015, 1, 5)?, "նախածննդյան տոներ"),
            (NaiveDate::from_ymd_res(2015, 1, 7)?, "Մեռելոց հիշատակի օր"),
            (NaiveDate::from_ymd_res(2015, 1, 28)?, "Բանակի օր"),
            (NaiveDate::from_ymd_res(2015, 3, 8)?, "Կանանց տոն"),
            (
                NaiveDate::from_ymd_res(2015, 4, 24)?,
                "Հայոց ցեղասպանության զոհերի հիշատակի օր",
            ),
            (NaiveDate::from_ymd_res(2015, 5, 1)?, "Աշխատանքի օր"),
            (
                NaiveDate::from_ymd_res(2015, 5, 9)?,
                "Հաղթանակի և Խաղաղության տոն",
            ),
            (NaiveDate::from_ymd_res(2015, 5, 28)?, "Հանրապետության օր"),
            (NaiveDate::from_ymd_res(2015, 7, 5)?, "Սահմանադրության օր"),
            (NaiveDate::from_ymd_res(2015, 9, 21)?, "Անկախության օր"),
            (NaiveDate::from_ymd_res(2015, 12, 31)?, "Նոր տարվա գիշեր"),
            (
                NaiveDate::from_ymd_res(2015, 1, 8)?,
                "Հանգստյան օր (հետաձգվել է 20.12.2014 թվականից)",
            ),
            (
                NaiveDate::from_ymd_res(2015, 1, 9)?,
                "Հանգստյան օր (հետաձգվել է 27.12.2014 թվականից)",
            ),
            (
                NaiveDate::from_ymd_res(2015, 4, 6)?,
                "Հանգստյան օր (հետաձգվել է 11.04.2015 թվականից)",
            ),
            (
                NaiveDate::from_ymd_res(2015, 4, 23)?,
                "Հանգստյան օր (հետաձգվել է 18.04.2015 թվականից)",
            ),
            (
                NaiveDate::from_ymd_res(2015, 7, 13)?,
                "Հանգստյան օր (հետաձգվել է 18.07.2015 թվականից)",
            ),
            (
                NaiveDate::from_ymd_res(2015, 9, 14)?,
                "Հանգստյան օր (հետաձգվել է 19.09.2015 թվականից)",
            ),
        ],
        &mut map,
        Country::AM,
        "Armenia",
    );

    build_year(
        years,
        2016,
        vec![
            (NaiveDate::from_ymd_res(2016, 1, 1)?, "Նոր տարվա օր"),
            (NaiveDate::from_ymd_res(2016, 1, 2)?, "Նոր տարվա օր"),
            (
                NaiveDate::from_ymd_res(2016, 1, 6)?,
                "Սուրբ Ծնունդ եւ Հայտնություն",
            ),
            (NaiveDate::from_ymd_res(2016, 1, 3)?, "նախածննդյան տոներ"),
            (NaiveDate::from_ymd_res(2016, 1, 4)?, "նախածննդյան տոներ"),
            (NaiveDate::from_ymd_res(2016, 1, 5)?, "նախածննդյան տոներ"),
            (NaiveDate::from_ymd_res(2016, 1, 7)?, "Մեռելոց հիշատակի օր"),
            (NaiveDate::from_ymd_res(2016, 1, 28)?, "Բանակի օր"),
            (NaiveDate::from_ymd_res(2016, 3, 8)?, "Կանանց տոն"),
            (
                NaiveDate::from_ymd_res(2016, 4, 24)?,
                "Հայոց ցեղասպանության զոհերի հիշատակի օր",
            ),
            (NaiveDate::from_ymd_res(2016, 5, 1)?, "Աշխատանքի օր"),
            (
                NaiveDate::from_ymd_res(2016, 5, 9)?,
                "Հաղթանակի և Խաղաղության տոն",
            ),
            (NaiveDate::from_ymd_res(2016, 5, 28)?, "Հանրապետության օր"),
            (NaiveDate::from_ymd_res(2016, 7, 5)?, "Սահմանադրության օր"),
            (NaiveDate::from_ymd_res(2016, 9, 21)?, "Անկախության օր"),
            (NaiveDate::from_ymd_res(2016, 12, 31)?, "Նոր տարվա գիշեր"),
            (
                NaiveDate::from_ymd_res(2016, 3, 7)?,
                "Հանգստյան օր (հետաձգվել է 12.03.2016 թվականից)",
            ),
            (
                NaiveDate::from_ymd_res(2016, 3, 28)?,
                "Հանգստյան օր (հետաձգվել է 02.04.2016 թվականից)",
            ),
            (
                NaiveDate::from_ymd_res(2016, 9, 12)?,
                "Հանգստյան օր (հետաձգվել է 24.09.2016 թվականից)",
            ),
        ],
        &mut map,
        Country::AM,
        "Armenia",
    );

    build_year(
        years,
        2017,
        vec![
            (NaiveDate::from_ymd_res(2017, 1, 1)?, "Նոր տարվա օր"),
            (NaiveDate::from_ymd_res(2017, 1, 2)?, "Նոր տարվա օր"),
            (
                NaiveDate::from_ymd_res(2017, 1, 6)?,
                "Սուրբ Ծնունդ եւ Հայտնություն",
            ),
            (NaiveDate::from_ymd_res(2017, 1, 3)?, "նախածննդյան տոներ"),
            (NaiveDate::from_ymd_res(2017, 1, 4)?, "նախածննդյան տոներ"),
            (NaiveDate::from_ymd_res(2017, 1, 5)?, "նախածննդյան տոներ"),
            (NaiveDate::from_ymd_res(2017, 1, 7)?, "Մեռելոց հիշատակի օր"),
            (NaiveDate::from_ymd_res(2017, 1, 28)?, "Բանակի օր"),
            (NaiveDate::from_ymd_res(2017, 3, 8)?, "Կանանց տոն"),
            (
                NaiveDate::from_ymd_res(2017, 4, 24)?,
                "Հայոց ցեղասպանության զոհերի հիշատակի օր",
            ),
            (NaiveDate::from_ymd_res(2017, 5, 1)?, "Աշխատանքի օր"),
            (
                NaiveDate::from_ymd_res(2017, 5, 9)?,
                "Հաղթանակի և Խաղաղության տոն",
            ),
            (NaiveDate::from_ymd_res(2017, 5, 28)?, "Հանրապետության օր"),
            (NaiveDate::from_ymd_res(2017, 7, 5)?, "Սահմանադրության օր"),
            (NaiveDate::from_ymd_res(2017, 9, 21)?, "Անկախության օր"),
            (NaiveDate::from_ymd_res(2017, 12, 31)?, "Նոր տարվա գիշեր"),
            (
                NaiveDate::from_ymd_res(2017, 4, 17)?,
                "Հանգստյան օր (հետաձգվել է 06.05.2017 թվականից)",
            ),
            (
                NaiveDate::from_ymd_res(2017, 5, 8)?,
                "Հանգստյան օր (հետաձգվել է 20.05.2017 թվականից)",
            ),
            (
                NaiveDate::from_ymd_res(2017, 9, 18)?,
                "Հանգստյան օր (հետաձգվել է 23.09.2017 թվականից)",
            ),
        ],
        &mut map,
        Country::AM,
        "Armenia",
    );

    build_year(
        years,
        2018,
        vec![
            (NaiveDate::from_ymd_res(2018, 1, 1)?, "Նոր տարվա օր"),
            (NaiveDate::from_ymd_res(2018, 1, 2)?, "Նոր տարվա օր"),
            (
                NaiveDate::from_ymd_res(2018, 1, 6)?,
                "Սուրբ Ծնունդ եւ Հայտնություն",
            ),
            (NaiveDate::from_ymd_res(2018, 1, 3)?, "նախածննդյան տոներ"),
            (NaiveDate::from_ymd_res(2018, 1, 4)?, "նախածննդյան տոներ"),
            (NaiveDate::from_ymd_res(2018, 1, 5)?, "նախածննդյան տոներ"),
            (NaiveDate::from_ymd_res(2018, 1, 7)?, "Մեռելոց հիշատակի օր"),
            (NaiveDate::from_ymd_res(2018, 1, 28)?, "Բանակի օր"),
            (NaiveDate::from_ymd_res(2018, 3, 8)?, "Կանանց տոն"),
            (
                NaiveDate::from_ymd_res(2018, 4, 24)?,
                "Հայոց ցեղասպանության զոհերի հիշատակի օր",
            ),
            (NaiveDate::from_ymd_res(2018, 5, 1)?, "Աշխատանքի օր"),
            (
                NaiveDate::from_ymd_res(2018, 5, 9)?,
                "Հաղթանակի և Խաղաղության տոն",
            ),
            (NaiveDate::from_ymd_res(2018, 5, 28)?, "Հանրապետության օր"),
            (NaiveDate::from_ymd_res(2018, 7, 5)?, "Սահմանադրության օր"),
            (NaiveDate::from_ymd_res(2018, 9, 21)?, "Անկախության օր"),
            (NaiveDate::from_ymd_res(2018, 12, 31)?, "Նոր տարվա գիշեր"),
            (
                NaiveDate::from_ymd_res(2018, 3, 9)?,
                "Հանգստյան օր (հետաձգվել է 17.03.2018 թվականից)",
            ),
            (
                NaiveDate::from_ymd_res(2018, 4, 2)?,
                "Հանգստյան օր (հետաձգվել է 07.04.2018 թվականից)",
            ),
            (
                NaiveDate::from_ymd_res(2018, 4, 30)?,
                "Հանգստյան օր (հետաձգվել է 05.05.2018 թվականից)",
            ),
            (
                NaiveDate::from_ymd_res(2018, 10, 11)?,
                "Հանգստյան օր (հետաձգվել է 27.10.2018 թվականից)",
            ),
            (
                NaiveDate::from_ymd_res(2018, 10, 12)?,
                "Հանգստյան օր (հետաձգվել է 03.11.2018 թվականից)",
            ),
        ],
        &mut map,
        Country::AM,
        "Armenia",
    );

    build_year(
        years,
        2019,
        vec![
            (NaiveDate::from_ymd_res(2019, 1, 1)?, "Նոր տարվա օր"),
            (NaiveDate::from_ymd_res(2019, 1, 2)?, "Նոր տարվա օր"),
            (
                NaiveDate::from_ymd_res(2019, 1, 6)?,
                "Սուրբ Ծնունդ եւ Հայտնություն",
            ),
            (NaiveDate::from_ymd_res(2019, 1, 3)?, "նախածննդյան տոներ"),
            (NaiveDate::from_ymd_res(2019, 1, 4)?, "նախածննդյան տոներ"),
            (NaiveDate::from_ymd_res(2019, 1, 5)?, "նախածննդյան տոներ"),
            (NaiveDate::from_ymd_res(2019, 1, 7)?, "Մեռելոց հիշատակի օր"),
            (NaiveDate::from_ymd_res(2019, 1, 28)?, "Բանակի օր"),
            (NaiveDate::from_ymd_res(2019, 3, 8)?, "Կանանց տոն"),
            (
                NaiveDate::from_ymd_res(2019, 4, 24)?,
                "Հայոց ցեղասպանության զոհերի հիշատակի օր",
            ),
            (NaiveDate::from_ymd_res(2019, 5, 1)?, "Աշխատանքի օր"),
            (
                NaiveDate::from_ymd_res(2019, 5, 9)?,
                "Հաղթանակի և Խաղաղության տոն",
            ),
            (NaiveDate::from_ymd_res(2019, 5, 28)?, "Հանրապետության օր"),
            (NaiveDate::from_ymd_res(2019, 7, 5)?, "Սահմանադրության օր"),
            (NaiveDate::from_ymd_res(2019, 9, 21)?, "Անկախության օր"),
            (NaiveDate::from_ymd_res(2019, 12, 31)?, "Նոր տարվա գիշեր"),
        ],
        &mut map,
        Country::AM,
        "Armenia",
    );

    build_year(
        years,
        2020,
        vec![
            (NaiveDate::from_ymd_res(2020, 1, 1)?, "Նոր տարվա օր"),
            (NaiveDate::from_ymd_res(2020, 1, 2)?, "Նոր տարվա օր"),
            (
                NaiveDate::from_ymd_res(2020, 1, 6)?,
                "Սուրբ Ծնունդ եւ Հայտնություն",
            ),
            (NaiveDate::from_ymd_res(2020, 1, 3)?, "նախածննդյան տոներ"),
            (NaiveDate::from_ymd_res(2020, 1, 4)?, "նախածննդյան տոներ"),
            (NaiveDate::from_ymd_res(2020, 1, 5)?, "նախածննդյան տոներ"),
            (NaiveDate::from_ymd_res(2020, 1, 7)?, "Մեռելոց հիշատակի օր"),
            (NaiveDate::from_ymd_res(2020, 1, 28)?, "Բանակի օր"),
            (NaiveDate::from_ymd_res(2020, 3, 8)?, "Կանանց տոն"),
            (
                NaiveDate::from_ymd_res(2020, 4, 24)?,
                "Հայոց ցեղասպանության զոհերի հիշատակի օր",
            ),
            (NaiveDate::from_ymd_res(2020, 5, 1)?, "Աշխատանքի օր"),
            (
                NaiveDate::from_ymd_res(2020, 5, 9)?,
                "Հաղթանակի և Խաղաղության տոն",
            ),
            (NaiveDate::from_ymd_res(2020, 5, 28)?, "Հանրապետության օր"),
            (NaiveDate::from_ymd_res(2020, 7, 5)?, "Սահմանադրության օր"),
            (NaiveDate::from_ymd_res(2020, 9, 21)?, "Անկախության օր"),
            (NaiveDate::from_ymd_res(2020, 12, 31)?, "Նոր տարվա գիշեր"),
            (
                NaiveDate::from_ymd_res(2020, 1, 27)?,
                "Հանգստյան օր (հետաձգվել է 01.02.2020 թվականից)",
            ),
            (
                NaiveDate::from_ymd_res(2020, 5, 29)?,
                "Հանգստյան օր (հետաձգվել է 23.05.2020 թվականից)",
            ),
        ],
        &mut map,
        Country::AM,
        "Armenia",
    );

    build_year(
        years,
        2021,
        vec![
            (NaiveDate::from_ymd_res(2021, 1, 1)?, "Նոր տարվա օր"),
            (NaiveDate::from_ymd_res(2021, 1, 2)?, "Նոր տարվա օր"),
            (
                NaiveDate::from_ymd_res(2021, 1, 6)?,
                "Սուրբ Ծնունդ եւ Հայտնություն",
            ),
            (NaiveDate::from_ymd_res(2021, 1, 3)?, "նախածննդյան տոներ"),
            (NaiveDate::from_ymd_res(2021, 1, 4)?, "նախածննդյան տոներ"),
            (NaiveDate::from_ymd_res(2021, 1, 5)?, "նախածննդյան տոներ"),
            (NaiveDate::from_ymd_res(2021, 1, 7)?, "Մեռելոց հիշատակի օր"),
            (NaiveDate::from_ymd_res(2021, 1, 28)?, "Բանակի օր"),
            (NaiveDate::from_ymd_res(2021, 3, 8)?, "Կանանց տոն"),
            (
                NaiveDate::from_ymd_res(2021, 4, 24)?,
                "Հայոց ցեղասպանության զոհերի հիշատակի օր",
            ),
            (NaiveDate::from_ymd_res(2021, 5, 1)?, "Աշխատանքի օր"),
            (
                NaiveDate::from_ymd_res(2021, 5, 9)?,
                "Հաղթանակի և Խաղաղության տոն",
            ),
            (NaiveDate::from_ymd_res(2021, 5, 28)?, "Հանրապետության օր"),
            (NaiveDate::from_ymd_res(2021, 7, 5)?, "Սահմանադրության օր"),
            (NaiveDate::from_ymd_res(2021, 9, 21)?, "Անկախության օր"),
            (NaiveDate::from_ymd_res(2021, 12, 31)?, "Նոր տարվա գիշեր"),
            (
                NaiveDate::from_ymd_res(2021, 9, 20)?,
                "Հանգստյան օր (հետաձգվել է 25.09.2021 թվականից)",
            ),
        ],
        &mut map,
        Country::AM,
        "Armenia",
    );

    build_year(
        years,
        2022,
        vec![
            (NaiveDate::from_ymd_res(2022, 1, 1)?, "Նոր տարվա օր"),
            (NaiveDate::from_ymd_res(2022, 1, 2)?, "Նոր տարվա օր"),
            (
                NaiveDate::from_ymd_res(2022, 1, 6)?,
                "Սուրբ Ծնունդ եւ Հայտնություն",
            ),
            (NaiveDate::from_ymd_res(2022, 1, 28)?, "Բանակի օր"),
            (NaiveDate::from_ymd_res(2022, 3, 8)?, "Կանանց տոն"),
            (
                NaiveDate::from_ymd_res(2022, 4, 24)?,
                "Հայոց ցեղասպանության զոհերի հիշատակի օր",
            ),
            (NaiveDate::from_ymd_res(2022, 5, 1)?, "Աշխատանքի օր"),
            (
                NaiveDate::from_ymd_res(2022, 5, 9)?,
                "Հաղթանակի և Խաղաղության տոն",
            ),
            (NaiveDate::from_ymd_res(2022, 5, 28)?, "Հանրապետության օր"),
            (NaiveDate::from_ymd_res(2022, 7, 5)?, "Սահմանադրության օր"),
            (NaiveDate::from_ymd_res(2022, 9, 21)?, "Անկախության օր"),
            (NaiveDate::from_ymd_res(2022, 12, 31)?, "Նոր տարվա գիշեր"),
        ],
        &mut map,
        Country::AM,
        "Armenia",
    );

    build_year(
        years,
        2023,
        vec![
            (NaiveDate::from_ymd_res(2023, 1, 1)?, "Նոր տարվա օր"),
            (NaiveDate::from_ymd_res(2023, 1, 2)?, "Նոր տարվա օր"),
            (
                NaiveDate::from_ymd_res(2023, 1, 6)?,
                "Սուրբ Ծնունդ եւ Հայտնություն",
            ),
            (NaiveDate::from_ymd_res(2023, 1, 28)?, "Բանակի օր"),
            (NaiveDate::from_ymd_res(2023, 3, 8)?, "Կանանց տոն"),
            (
                NaiveDate::from_ymd_res(2023, 4, 24)?,
                "Հայոց ցեղասպանության զոհերի հիշատակի օր",
            ),
            (NaiveDate::from_ymd_res(2023, 5, 1)?, "Աշխատանքի օր"),
            (
                NaiveDate::from_ymd_res(2023, 5, 9)?,
                "Հաղթանակի և Խաղաղության տոն",
            ),
            (NaiveDate::from_ymd_res(2023, 5, 28)?, "Հանրապետության օր"),
            (NaiveDate::from_ymd_res(2023, 7, 5)?, "Սահմանադրության օր"),
            (NaiveDate::from_ymd_res(2023, 9, 21)?, "Անկախության օր"),
            (NaiveDate::from_ymd_res(2023, 12, 31)?, "Նոր տարվա գիշեր"),
        ],
        &mut map,
        Country::AM,
        "Armenia",
    );

    build_year(
        years,
        2024,
        vec![
            (NaiveDate::from_ymd_res(2024, 1, 1)?, "Նոր տարվա օր"),
            (NaiveDate::from_ymd_res(2024, 1, 2)?, "Նոր տարվա օր"),
            (
                NaiveDate::from_ymd_res(2024, 1, 6)?,
                "Սուրբ Ծնունդ եւ Հայտնություն",
            ),
            (NaiveDate::from_ymd_res(2024, 1, 28)?, "Բանակի օր"),
            (NaiveDate::from_ymd_res(2024, 3, 8)?, "Կանանց տոն"),
            (
                NaiveDate::from_ymd_res(2024, 4, 24)?,
                "Հայոց ցեղասպանության զոհերի հիշատակի օր",
            ),
            (NaiveDate::from_ymd_res(2024, 5, 1)?, "Աշխատանքի օր"),
            (
                NaiveDate::from_ymd_res(2024, 5, 9)?,
                "Հաղթանակի և Խաղաղության տոն",
            ),
            (NaiveDate::from_ymd_res(2024, 5, 28)?, "Հանրապետության օր"),
            (NaiveDate::from_ymd_res(2024, 7, 5)?, "Սահմանադրության օր"),
            (NaiveDate::from_ymd_res(2024, 9, 21)?, "Անկախության օր"),
            (NaiveDate::from_ymd_res(2024, 12, 31)?, "Նոր տարվա գիշեր"),
        ],
        &mut map,
        Country::AM,
        "Armenia",
    );

    build_year(
        years,
        2025,
        vec![
            (NaiveDate::from_ymd_res(2025, 1, 1)?, "Նոր տարվա օր"),
            (NaiveDate::from_ymd_res(2025, 1, 2)?, "Նոր տարվա օր"),
            (
                NaiveDate::from_ymd_res(2025, 1, 6)?,
                "Սուրբ Ծնունդ եւ Հայտնություն",
            ),
            (NaiveDate::from_ymd_res(2025, 1, 28)?, "Բանակի օր"),
            (NaiveDate::from_ymd_res(2025, 3, 8)?, "Կանանց տոն"),
            (
                NaiveDate::from_ymd_res(2025, 4, 24)?,
                "Հայոց ցեղասպանության զոհերի հիշատակի օր",
            ),
            (NaiveDate::from_ymd_res(2025, 5, 1)?, "Աշխատանքի օր"),
            (
                NaiveDate::from_ymd_res(2025, 5, 9)?,
                "Հաղթանակի և Խաղաղության տոն",
            ),
            (NaiveDate::from_ymd_res(2025, 5, 28)?, "Հանրապետության օր"),
            (NaiveDate::from_ymd_res(2025, 7, 5)?, "Սահմանադրության օր"),
            (NaiveDate::from_ymd_res(2025, 9, 21)?, "Անկախության օր"),
            (NaiveDate::from_ymd_res(2025, 12, 31)?, "Նոր տարվա գիշեր"),
        ],
        &mut map,
        Country::AM,
        "Armenia",
    );

    build_year(
        years,
        2026,
        vec![
            (NaiveDate::from_ymd_res(2026, 1, 1)?, "Նոր տարվա օր"),
            (NaiveDate::from_ymd_res(2026, 1, 2)?, "Նոր տարվա օր"),
            (
                NaiveDate::from_ymd_res(2026, 1, 6)?,
                "Սուրբ Ծնունդ եւ Հայտնություն",
            ),
            (
                NaiveDate::from_ymd_res(2026, 1, 27)?,
                "Հայրենիքի պաշտպանության համար զոհվածների հիշատակի օր",
            ),
            (NaiveDate::from_ymd_res(2026, 1, 28)?, "Բանակի օր"),
            (NaiveDate::from_ymd_res(2026, 3, 8)?, "Կանանց տոն"),
            (
                NaiveDate::from_ymd_res(2026, 4, 24)?,
                "Հայոց ցեղասպանության զոհերի հիշատակի օր",
            ),
            (NaiveDate::from_ymd_res(2026, 5, 1)?, "Աշխատանքի օր"),
            (
                NaiveDate::from_ymd_res(2026, 5, 9)?,
                "Հաղթանակի և Խաղաղության տոն",
            ),
            (NaiveDate::from_ymd_res(2026, 5, 28)?, "Հանրապետության օր"),
            (NaiveDate::from_ymd_res(2026, 7, 5)?, "Սահմանադրության օր"),
            (NaiveDate::from_ymd_res(2026, 9, 21)?, "Անկախության օր"),
            (NaiveDate::from_ymd_res(2026, 12, 31)?, "Նոր տարվա գիշեր"),
        ],
        &mut map,
        Country::AM,
        "Armenia",
    );

    build_year(
        years,
        2027,
        vec![
            (NaiveDate::from_ymd_res(2027, 1, 1)?, "Նոր տարվա օր"),
            (NaiveDate::from_ymd_res(2027, 1, 2)?, "Նոր տարվա օր"),
            (
                NaiveDate::from_ymd_res(2027, 1, 6)?,
                "Սուրբ Ծնունդ եւ Հայտնություն",
            ),
            (
                NaiveDate::from_ymd_res(2027, 1, 27)?,
                "Հայրենիքի պաշտպանության համար զոհվածների հիշատակի օր",
            ),
            (NaiveDate::from_ymd_res(2027, 1, 28)?, "Բանակի օր"),
            (NaiveDate::from_ymd_res(2027, 3, 8)?, "Կանանց տոն"),
            (
                NaiveDate::from_ymd_res(2027, 4, 24)?,
                "Հայոց ցեղասպանության զոհերի հիշատակի օր",
            ),
            (NaiveDate::from_ymd_res(2027, 5, 1)?, "Աշխատանքի օր"),
            (
                NaiveDate::from_ymd_res(2027, 5, 9)?,
                "Հաղթանակի և Խաղաղության տոն",
            ),
            (NaiveDate::from_ymd_res(2027, 5, 28)?, "Հանրապետության օր"),
            (NaiveDate::from_ymd_res(2027, 7, 5)?, "Սահմանադրության օր"),
            (NaiveDate::from_ymd_res(2027, 9, 21)?, "Անկախության օր"),
            (NaiveDate::from_ymd_res(2027, 12, 31)?, "Նոր տարվա գիշեր"),
        ],
        &mut map,
        Country::AM,
        "Armenia",
    );

    build_year(
        years,
        2028,
        vec![
            (NaiveDate::from_ymd_res(2028, 1, 1)?, "Նոր տարվա օր"),
            (NaiveDate::from_ymd_res(2028, 1, 2)?, "Նոր տարվա օր"),
            (
                NaiveDate::from_ymd_res(2028, 1, 6)?,
                "Սուրբ Ծնունդ եւ Հայտնություն",
            ),
            (
                NaiveDate::from_ymd_res(2028, 1, 27)?,
                "Հայրենիքի պաշտպանության համար զոհվածների հիշատակի օր",
            ),
            (NaiveDate::from_ymd_res(2028, 1, 28)?, "Բանակի օր"),
            (NaiveDate::from_ymd_res(2028, 3, 8)?, "Կանանց տոն"),
            (
                NaiveDate::from_ymd_res(2028, 4, 24)?,
                "Հայոց ցեղասպանության զոհերի հիշատակի օր",
            ),
            (NaiveDate::from_ymd_res(2028, 5, 1)?, "Աշխատանքի օր"),
            (
                NaiveDate::from_ymd_res(2028, 5, 9)?,
                "Հաղթանակի և Խաղաղության տոն",
            ),
            (NaiveDate::from_ymd_res(2028, 5, 28)?, "Հանրապետության օր"),
            (NaiveDate::from_ymd_res(2028, 7, 5)?, "Սահմանադրության օր"),
            (NaiveDate::from_ymd_res(2028, 9, 21)?, "Անկախության օր"),
            (NaiveDate::from_ymd_res(2028, 12, 31)?, "Նոր տարվա գիշեր"),
        ],
        &mut map,
        Country::AM,
        "Armenia",
    );

    build_year(
        years,
        2029,
        vec![
            (NaiveDate::from_ymd_res(2029, 1, 1)?, "Նոր տարվա օր"),
            (NaiveDate::from_ymd_res(2029, 1, 2)?, "Նոր տարվա օր"),
            (
                NaiveDate::from_ymd_res(2029, 1, 6)?,
                "Սուրբ Ծնունդ եւ Հայտնություն",
            ),
            (
                NaiveDate::from_ymd_res(2029, 1, 27)?,
                "Հայրենիքի պաշտպանության համար զոհվածների հիշատակի օր",
            ),
            (NaiveDate::from_ymd_res(2029, 1, 28)?, "Բանակի օր"),
            (NaiveDate::from_ymd_res(2029, 3, 8)?, "Կանանց տոն"),
            (
                NaiveDate::from_ymd_res(2029, 4, 24)?,
                "Հայոց ցեղասպանության զոհերի հիշատակի օր",
            ),
            (NaiveDate::from_ymd_res(2029, 5, 1)?, "Աշխատանքի օր"),
            (
                NaiveDate::from_ymd_res(2029, 5, 9)?,
                "Հաղթանակի և Խաղաղության տոն",
            ),
            (NaiveDate::from_ymd_res(2029, 5, 28)?, "Հանրապետության օր"),
            (NaiveDate::from_ymd_res(2029, 7, 5)?, "Սահմանադրության օր"),
            (NaiveDate::from_ymd_res(2029, 9, 21)?, "Անկախության օր"),
            (NaiveDate::from_ymd_res(2029, 12, 31)?, "Նոր տարվա գիշեր"),
        ],
        &mut map,
        Country::AM,
        "Armenia",
    );

    build_year(
        years,
        2030,
        vec![
            (NaiveDate::from_ymd_res(2030, 1, 1)?, "Նոր տարվա օր"),
            (NaiveDate::from_ymd_res(2030, 1, 2)?, "Նոր տարվա օր"),
            (
                NaiveDate::from_ymd_res(2030, 1, 6)?,
                "Սուրբ Ծնունդ եւ Հայտնություն",
            ),
            (
                NaiveDate::from_ymd_res(2030, 1, 27)?,
                "Հայրենիքի պաշտպանության համար զոհվածների հիշատակի օր",
            ),
            (NaiveDate::from_ymd_res(2030, 1, 28)?, "Բանակի օր"),
            (NaiveDate::from_ymd_res(2030, 3, 8)?, "Կանանց տոն"),
            (
                NaiveDate::from_ymd_res(2030, 4, 24)?,
                "Հայոց ցեղասպանության զոհերի հիշատակի օր",
            ),
            (NaiveDate::from_ymd_res(2030, 5, 1)?, "Աշխատանքի օր"),
            (
                NaiveDate::from_ymd_res(2030, 5, 9)?,
                "Հաղթանակի և Խաղաղության տոն",
            ),
            (NaiveDate::from_ymd_res(2030, 5, 28)?, "Հանրապետության օր"),
            (NaiveDate::from_ymd_res(2030, 7, 5)?, "Սահմանադրության օր"),
            (NaiveDate::from_ymd_res(2030, 9, 21)?, "Անկախության օր"),
            (NaiveDate::from_ymd_res(2030, 12, 31)?, "Նոր տարվա գիշեր"),
        ],
        &mut map,
        Country::AM,
        "Armenia",
    );

    Ok(map)
}
