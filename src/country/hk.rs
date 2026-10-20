
//! Hong Kong
use super::*;

/// Generate holiday map for Hong Kong.
#[allow(unused_mut, unused_variables)]
pub fn build(years: &Option<&std::ops::Range<Year>>) -> Result<HashMap<Year, BTreeMap<NaiveDate, Holiday>>> {
  let mut map = HashMap::new();

  build_year(
    years,
    2001,
    vec![

      (NaiveDate::from_ymd_res(2001, 1, 1)?, "一月一日"),
      (NaiveDate::from_ymd_res(2001, 1, 24)?, "農曆年初一"),
      (NaiveDate::from_ymd_res(2001, 1, 25)?, "農曆年初二"),
      (NaiveDate::from_ymd_res(2001, 1, 26)?, "農曆年初三"),
      (NaiveDate::from_ymd_res(2001, 4, 5)?, "清明節"),
      (NaiveDate::from_ymd_res(2001, 5, 1)?, "勞動節"),
      (NaiveDate::from_ymd_res(2001, 6, 25)?, "端午節"),
      (NaiveDate::from_ymd_res(2001, 7, 1)?, "香港特別行政區成立紀念日"),
      (NaiveDate::from_ymd_res(2001, 7, 2)?, "香港特別行政區成立紀念日（補假）"),
      (NaiveDate::from_ymd_res(2001, 10, 2)?, "中秋節翌日"),
      (NaiveDate::from_ymd_res(2001, 10, 25)?, "重陽節"),
      (NaiveDate::from_ymd_res(2001, 10, 1)?, "國慶日"),
      (NaiveDate::from_ymd_res(2001, 12, 25)?, "聖誕節"),
    ],
    &mut map,
    Country::HK,
    "Hong Kong",
  );

  build_year(
    years,
    2002,
    vec![

      (NaiveDate::from_ymd_res(2002, 1, 1)?, "一月一日"),
      (NaiveDate::from_ymd_res(2002, 2, 12)?, "農曆年初一"),
      (NaiveDate::from_ymd_res(2002, 2, 13)?, "農曆年初二"),
      (NaiveDate::from_ymd_res(2002, 2, 14)?, "農曆年初三"),
      (NaiveDate::from_ymd_res(2002, 4, 5)?, "清明節"),
      (NaiveDate::from_ymd_res(2002, 5, 1)?, "勞動節"),
      (NaiveDate::from_ymd_res(2002, 6, 15)?, "端午節"),
      (NaiveDate::from_ymd_res(2002, 7, 1)?, "香港特別行政區成立紀念日"),
      (NaiveDate::from_ymd_res(2002, 9, 21)?, "中秋節"),
      (NaiveDate::from_ymd_res(2002, 10, 14)?, "重陽節"),
      (NaiveDate::from_ymd_res(2002, 10, 1)?, "國慶日"),
      (NaiveDate::from_ymd_res(2002, 12, 25)?, "聖誕節"),
    ],
    &mut map,
    Country::HK,
    "Hong Kong",
  );

  build_year(
    years,
    2003,
    vec![

      (NaiveDate::from_ymd_res(2003, 1, 1)?, "一月一日"),
      (NaiveDate::from_ymd_res(2003, 1, 31)?, "農曆年初一的前一日"),
      (NaiveDate::from_ymd_res(2003, 2, 1)?, "農曆年初一"),
      (NaiveDate::from_ymd_res(2003, 2, 3)?, "農曆年初三"),
      (NaiveDate::from_ymd_res(2003, 4, 5)?, "清明節"),
      (NaiveDate::from_ymd_res(2003, 5, 1)?, "勞動節"),
      (NaiveDate::from_ymd_res(2003, 6, 4)?, "端午節"),
      (NaiveDate::from_ymd_res(2003, 7, 1)?, "香港特別行政區成立紀念日"),
      (NaiveDate::from_ymd_res(2003, 9, 12)?, "中秋節翌日"),
      (NaiveDate::from_ymd_res(2003, 10, 4)?, "重陽節"),
      (NaiveDate::from_ymd_res(2003, 10, 1)?, "國慶日"),
      (NaiveDate::from_ymd_res(2003, 12, 25)?, "聖誕節"),
    ],
    &mut map,
    Country::HK,
    "Hong Kong",
  );

  build_year(
    years,
    2004,
    vec![

      (NaiveDate::from_ymd_res(2004, 1, 1)?, "一月一日"),
      (NaiveDate::from_ymd_res(2004, 1, 22)?, "農曆年初一"),
      (NaiveDate::from_ymd_res(2004, 1, 23)?, "農曆年初二"),
      (NaiveDate::from_ymd_res(2004, 1, 24)?, "農曆年初三"),
      (NaiveDate::from_ymd_res(2004, 4, 4)?, "清明節"),
      (NaiveDate::from_ymd_res(2004, 4, 5)?, "清明節（補假）"),
      (NaiveDate::from_ymd_res(2004, 5, 1)?, "勞動節"),
      (NaiveDate::from_ymd_res(2004, 6, 22)?, "端午節"),
      (NaiveDate::from_ymd_res(2004, 7, 1)?, "香港特別行政區成立紀念日"),
      (NaiveDate::from_ymd_res(2004, 9, 29)?, "中秋節翌日"),
      (NaiveDate::from_ymd_res(2004, 10, 22)?, "重陽節"),
      (NaiveDate::from_ymd_res(2004, 10, 1)?, "國慶日"),
      (NaiveDate::from_ymd_res(2004, 12, 25)?, "聖誕節"),
    ],
    &mut map,
    Country::HK,
    "Hong Kong",
  );

  build_year(
    years,
    2005,
    vec![

      (NaiveDate::from_ymd_res(2005, 1, 1)?, "一月一日"),
      (NaiveDate::from_ymd_res(2005, 2, 9)?, "農曆年初一"),
      (NaiveDate::from_ymd_res(2005, 2, 10)?, "農曆年初二"),
      (NaiveDate::from_ymd_res(2005, 2, 11)?, "農曆年初三"),
      (NaiveDate::from_ymd_res(2005, 4, 5)?, "清明節"),
      (NaiveDate::from_ymd_res(2005, 5, 1)?, "勞動節"),
      (NaiveDate::from_ymd_res(2005, 5, 2)?, "勞動節（補假）"),
      (NaiveDate::from_ymd_res(2005, 6, 11)?, "端午節"),
      (NaiveDate::from_ymd_res(2005, 7, 1)?, "香港特別行政區成立紀念日"),
      (NaiveDate::from_ymd_res(2005, 9, 19)?, "中秋節翌日"),
      (NaiveDate::from_ymd_res(2005, 10, 11)?, "重陽節"),
      (NaiveDate::from_ymd_res(2005, 10, 1)?, "國慶日"),
      (NaiveDate::from_ymd_res(2005, 12, 25)?, "聖誕節"),
      (NaiveDate::from_ymd_res(2005, 12, 26)?, "聖誕節（補假）"),
    ],
    &mut map,
    Country::HK,
    "Hong Kong",
  );

  build_year(
    years,
    2006,
    vec![

      (NaiveDate::from_ymd_res(2006, 1, 1)?, "一月一日"),
      (NaiveDate::from_ymd_res(2006, 1, 2)?, "一月一日（補假）"),
      (NaiveDate::from_ymd_res(2006, 1, 28)?, "農曆年初一的前一日"),
      (NaiveDate::from_ymd_res(2006, 1, 30)?, "農曆年初二"),
      (NaiveDate::from_ymd_res(2006, 1, 31)?, "農曆年初三"),
      (NaiveDate::from_ymd_res(2006, 4, 5)?, "清明節"),
      (NaiveDate::from_ymd_res(2006, 5, 1)?, "勞動節"),
      (NaiveDate::from_ymd_res(2006, 5, 31)?, "端午節"),
      (NaiveDate::from_ymd_res(2006, 7, 1)?, "香港特別行政區成立紀念日"),
      (NaiveDate::from_ymd_res(2006, 10, 7)?, "中秋節翌日"),
      (NaiveDate::from_ymd_res(2006, 10, 30)?, "重陽節"),
      (NaiveDate::from_ymd_res(2006, 10, 1)?, "國慶日"),
      (NaiveDate::from_ymd_res(2006, 10, 2)?, "國慶日（補假）"),
      (NaiveDate::from_ymd_res(2006, 12, 25)?, "聖誕節"),
    ],
    &mut map,
    Country::HK,
    "Hong Kong",
  );

  build_year(
    years,
    2007,
    vec![

      (NaiveDate::from_ymd_res(2007, 1, 1)?, "一月一日"),
      (NaiveDate::from_ymd_res(2007, 2, 17)?, "農曆年初一的前一日"),
      (NaiveDate::from_ymd_res(2007, 2, 19)?, "農曆年初二"),
      (NaiveDate::from_ymd_res(2007, 2, 20)?, "農曆年初三"),
      (NaiveDate::from_ymd_res(2007, 4, 5)?, "清明節"),
      (NaiveDate::from_ymd_res(2007, 5, 1)?, "勞動節"),
      (NaiveDate::from_ymd_res(2007, 6, 19)?, "端午節"),
      (NaiveDate::from_ymd_res(2007, 7, 1)?, "香港特別行政區成立紀念日"),
      (NaiveDate::from_ymd_res(2007, 7, 2)?, "香港特別行政區成立紀念日（補假）"),
      (NaiveDate::from_ymd_res(2007, 9, 26)?, "中秋節翌日"),
      (NaiveDate::from_ymd_res(2007, 10, 19)?, "重陽節"),
      (NaiveDate::from_ymd_res(2007, 10, 1)?, "國慶日"),
      (NaiveDate::from_ymd_res(2007, 12, 25)?, "聖誕節"),
    ],
    &mut map,
    Country::HK,
    "Hong Kong",
  );

  build_year(
    years,
    2008,
    vec![

      (NaiveDate::from_ymd_res(2008, 1, 1)?, "一月一日"),
      (NaiveDate::from_ymd_res(2008, 2, 7)?, "農曆年初一"),
      (NaiveDate::from_ymd_res(2008, 2, 8)?, "農曆年初二"),
      (NaiveDate::from_ymd_res(2008, 2, 9)?, "農曆年初三"),
      (NaiveDate::from_ymd_res(2008, 4, 4)?, "清明節"),
      (NaiveDate::from_ymd_res(2008, 5, 1)?, "勞動節"),
      (NaiveDate::from_ymd_res(2008, 6, 8)?, "端午節"),
      (NaiveDate::from_ymd_res(2008, 6, 9)?, "端午節（補假）"),
      (NaiveDate::from_ymd_res(2008, 7, 1)?, "香港特別行政區成立紀念日"),
      (NaiveDate::from_ymd_res(2008, 9, 15)?, "中秋節翌日"),
      (NaiveDate::from_ymd_res(2008, 10, 7)?, "重陽節"),
      (NaiveDate::from_ymd_res(2008, 10, 1)?, "國慶日"),
      (NaiveDate::from_ymd_res(2008, 12, 25)?, "聖誕節"),
    ],
    &mut map,
    Country::HK,
    "Hong Kong",
  );

  build_year(
    years,
    2009,
    vec![

      (NaiveDate::from_ymd_res(2009, 1, 1)?, "一月一日"),
      (NaiveDate::from_ymd_res(2009, 1, 26)?, "農曆年初一"),
      (NaiveDate::from_ymd_res(2009, 1, 27)?, "農曆年初二"),
      (NaiveDate::from_ymd_res(2009, 1, 28)?, "農曆年初三"),
      (NaiveDate::from_ymd_res(2009, 4, 4)?, "清明節"),
      (NaiveDate::from_ymd_res(2009, 5, 1)?, "勞動節"),
      (NaiveDate::from_ymd_res(2009, 5, 28)?, "端午節"),
      (NaiveDate::from_ymd_res(2009, 7, 1)?, "香港特別行政區成立紀念日"),
      (NaiveDate::from_ymd_res(2009, 10, 3)?, "中秋節"),
      (NaiveDate::from_ymd_res(2009, 10, 26)?, "重陽節"),
      (NaiveDate::from_ymd_res(2009, 10, 1)?, "國慶日"),
      (NaiveDate::from_ymd_res(2009, 12, 25)?, "聖誕節"),
    ],
    &mut map,
    Country::HK,
    "Hong Kong",
  );

  build_year(
    years,
    2010,
    vec![

      (NaiveDate::from_ymd_res(2010, 1, 1)?, "一月一日"),
      (NaiveDate::from_ymd_res(2010, 2, 13)?, "農曆年初一的前一日"),
      (NaiveDate::from_ymd_res(2010, 2, 15)?, "農曆年初二"),
      (NaiveDate::from_ymd_res(2010, 2, 16)?, "農曆年初三"),
      (NaiveDate::from_ymd_res(2010, 4, 5)?, "清明節"),
      (NaiveDate::from_ymd_res(2010, 5, 1)?, "勞動節"),
      (NaiveDate::from_ymd_res(2010, 6, 16)?, "端午節"),
      (NaiveDate::from_ymd_res(2010, 7, 1)?, "香港特別行政區成立紀念日"),
      (NaiveDate::from_ymd_res(2010, 9, 23)?, "中秋節翌日"),
      (NaiveDate::from_ymd_res(2010, 10, 16)?, "重陽節"),
      (NaiveDate::from_ymd_res(2010, 10, 1)?, "國慶日"),
      (NaiveDate::from_ymd_res(2010, 12, 25)?, "聖誕節"),
    ],
    &mut map,
    Country::HK,
    "Hong Kong",
  );

  build_year(
    years,
    2011,
    vec![

      (NaiveDate::from_ymd_res(2011, 1, 1)?, "一月一日"),
      (NaiveDate::from_ymd_res(2011, 2, 3)?, "農曆年初一"),
      (NaiveDate::from_ymd_res(2011, 2, 4)?, "農曆年初二"),
      (NaiveDate::from_ymd_res(2011, 2, 5)?, "農曆年初三"),
      (NaiveDate::from_ymd_res(2011, 4, 5)?, "清明節"),
      (NaiveDate::from_ymd_res(2011, 5, 1)?, "勞動節"),
      (NaiveDate::from_ymd_res(2011, 5, 2)?, "勞動節（補假）"),
      (NaiveDate::from_ymd_res(2011, 6, 6)?, "端午節"),
      (NaiveDate::from_ymd_res(2011, 7, 1)?, "香港特別行政區成立紀念日"),
      (NaiveDate::from_ymd_res(2011, 9, 13)?, "中秋節翌日"),
      (NaiveDate::from_ymd_res(2011, 10, 5)?, "重陽節"),
      (NaiveDate::from_ymd_res(2011, 10, 1)?, "國慶日"),
      (NaiveDate::from_ymd_res(2011, 12, 25)?, "聖誕節"),
      (NaiveDate::from_ymd_res(2011, 12, 26)?, "聖誕節（補假）"),
    ],
    &mut map,
    Country::HK,
    "Hong Kong",
  );

  build_year(
    years,
    2012,
    vec![

      (NaiveDate::from_ymd_res(2012, 1, 1)?, "一月一日"),
      (NaiveDate::from_ymd_res(2012, 1, 2)?, "一月一日（補假）"),
      (NaiveDate::from_ymd_res(2012, 1, 23)?, "農曆年初一"),
      (NaiveDate::from_ymd_res(2012, 1, 24)?, "農曆年初二"),
      (NaiveDate::from_ymd_res(2012, 1, 25)?, "農曆年初三"),
      (NaiveDate::from_ymd_res(2012, 4, 4)?, "清明節"),
      (NaiveDate::from_ymd_res(2012, 5, 1)?, "勞動節"),
      (NaiveDate::from_ymd_res(2012, 6, 23)?, "端午節"),
      (NaiveDate::from_ymd_res(2012, 7, 1)?, "香港特別行政區成立紀念日"),
      (NaiveDate::from_ymd_res(2012, 7, 2)?, "香港特別行政區成立紀念日（補假）"),
      (NaiveDate::from_ymd_res(2012, 10, 1)?, "中秋節翌日; 國慶日"),
      (NaiveDate::from_ymd_res(2012, 10, 23)?, "重陽節"),
      (NaiveDate::from_ymd_res(2012, 10, 2)?, "國慶日（補假）"),
      (NaiveDate::from_ymd_res(2012, 12, 25)?, "聖誕節"),
    ],
    &mut map,
    Country::HK,
    "Hong Kong",
  );

  build_year(
    years,
    2013,
    vec![

      (NaiveDate::from_ymd_res(2013, 1, 1)?, "一月一日"),
      (NaiveDate::from_ymd_res(2013, 2, 13)?, "農曆年初四"),
      (NaiveDate::from_ymd_res(2013, 2, 11)?, "農曆年初二"),
      (NaiveDate::from_ymd_res(2013, 2, 12)?, "農曆年初三"),
      (NaiveDate::from_ymd_res(2013, 4, 4)?, "清明節"),
      (NaiveDate::from_ymd_res(2013, 5, 1)?, "勞動節"),
      (NaiveDate::from_ymd_res(2013, 6, 12)?, "端午節"),
      (NaiveDate::from_ymd_res(2013, 7, 1)?, "香港特別行政區成立紀念日"),
      (NaiveDate::from_ymd_res(2013, 9, 20)?, "中秋節翌日"),
      (NaiveDate::from_ymd_res(2013, 10, 13)?, "重陽節"),
      (NaiveDate::from_ymd_res(2013, 10, 14)?, "重陽節（補假）"),
      (NaiveDate::from_ymd_res(2013, 10, 1)?, "國慶日"),
      (NaiveDate::from_ymd_res(2013, 12, 25)?, "聖誕節"),
    ],
    &mut map,
    Country::HK,
    "Hong Kong",
  );

  build_year(
    years,
    2014,
    vec![

      (NaiveDate::from_ymd_res(2014, 1, 1)?, "一月一日"),
      (NaiveDate::from_ymd_res(2014, 2, 3)?, "農曆年初四"),
      (NaiveDate::from_ymd_res(2014, 1, 31)?, "農曆年初一"),
      (NaiveDate::from_ymd_res(2014, 2, 1)?, "農曆年初二"),
      (NaiveDate::from_ymd_res(2014, 4, 5)?, "清明節"),
      (NaiveDate::from_ymd_res(2014, 5, 1)?, "勞動節"),
      (NaiveDate::from_ymd_res(2014, 6, 2)?, "端午節"),
      (NaiveDate::from_ymd_res(2014, 7, 1)?, "香港特別行政區成立紀念日"),
      (NaiveDate::from_ymd_res(2014, 9, 9)?, "中秋節翌日"),
      (NaiveDate::from_ymd_res(2014, 10, 2)?, "重陽節"),
      (NaiveDate::from_ymd_res(2014, 10, 1)?, "國慶日"),
      (NaiveDate::from_ymd_res(2014, 12, 25)?, "聖誕節"),
    ],
    &mut map,
    Country::HK,
    "Hong Kong",
  );

  build_year(
    years,
    2015,
    vec![

      (NaiveDate::from_ymd_res(2015, 1, 1)?, "一月一日"),
      (NaiveDate::from_ymd_res(2015, 2, 19)?, "農曆年初一"),
      (NaiveDate::from_ymd_res(2015, 2, 20)?, "農曆年初二"),
      (NaiveDate::from_ymd_res(2015, 2, 21)?, "農曆年初三"),
      (NaiveDate::from_ymd_res(2015, 4, 5)?, "清明節"),
      (NaiveDate::from_ymd_res(2015, 4, 6)?, "清明節（補假）"),
      (NaiveDate::from_ymd_res(2015, 5, 1)?, "勞動節"),
      (NaiveDate::from_ymd_res(2015, 6, 20)?, "端午節"),
      (NaiveDate::from_ymd_res(2015, 7, 1)?, "香港特別行政區成立紀念日"),
      (NaiveDate::from_ymd_res(2015, 9, 28)?, "中秋節翌日"),
      (NaiveDate::from_ymd_res(2015, 10, 21)?, "重陽節"),
      (NaiveDate::from_ymd_res(2015, 10, 1)?, "國慶日"),
      (NaiveDate::from_ymd_res(2015, 12, 25)?, "聖誕節"),
      (NaiveDate::from_ymd_res(2015, 9, 3)?, "中國人民抗日戰爭勝利70周年紀念日"),
    ],
    &mut map,
    Country::HK,
    "Hong Kong",
  );

  build_year(
    years,
    2016,
    vec![

      (NaiveDate::from_ymd_res(2016, 1, 1)?, "一月一日"),
      (NaiveDate::from_ymd_res(2016, 2, 8)?, "農曆年初一"),
      (NaiveDate::from_ymd_res(2016, 2, 9)?, "農曆年初二"),
      (NaiveDate::from_ymd_res(2016, 2, 10)?, "農曆年初三"),
      (NaiveDate::from_ymd_res(2016, 4, 4)?, "清明節"),
      (NaiveDate::from_ymd_res(2016, 5, 1)?, "勞動節"),
      (NaiveDate::from_ymd_res(2016, 5, 2)?, "勞動節（補假）"),
      (NaiveDate::from_ymd_res(2016, 6, 9)?, "端午節"),
      (NaiveDate::from_ymd_res(2016, 7, 1)?, "香港特別行政區成立紀念日"),
      (NaiveDate::from_ymd_res(2016, 9, 16)?, "中秋節翌日"),
      (NaiveDate::from_ymd_res(2016, 10, 9)?, "重陽節"),
      (NaiveDate::from_ymd_res(2016, 10, 10)?, "重陽節（補假）"),
      (NaiveDate::from_ymd_res(2016, 10, 1)?, "國慶日"),
      (NaiveDate::from_ymd_res(2016, 12, 25)?, "聖誕節"),
      (NaiveDate::from_ymd_res(2016, 12, 26)?, "聖誕節（補假）"),
    ],
    &mut map,
    Country::HK,
    "Hong Kong",
  );

  build_year(
    years,
    2017,
    vec![

      (NaiveDate::from_ymd_res(2017, 1, 1)?, "一月一日"),
      (NaiveDate::from_ymd_res(2017, 1, 2)?, "一月一日（補假）"),
      (NaiveDate::from_ymd_res(2017, 1, 31)?, "農曆年初四"),
      (NaiveDate::from_ymd_res(2017, 1, 28)?, "農曆年初一"),
      (NaiveDate::from_ymd_res(2017, 1, 30)?, "農曆年初三"),
      (NaiveDate::from_ymd_res(2017, 4, 4)?, "清明節"),
      (NaiveDate::from_ymd_res(2017, 5, 1)?, "勞動節"),
      (NaiveDate::from_ymd_res(2017, 5, 30)?, "端午節"),
      (NaiveDate::from_ymd_res(2017, 7, 1)?, "香港特別行政區成立紀念日"),
      (NaiveDate::from_ymd_res(2017, 10, 5)?, "中秋節翌日"),
      (NaiveDate::from_ymd_res(2017, 10, 28)?, "重陽節"),
      (NaiveDate::from_ymd_res(2017, 10, 1)?, "國慶日"),
      (NaiveDate::from_ymd_res(2017, 10, 2)?, "國慶日（補假）"),
      (NaiveDate::from_ymd_res(2017, 12, 25)?, "聖誕節"),
    ],
    &mut map,
    Country::HK,
    "Hong Kong",
  );

  build_year(
    years,
    2018,
    vec![

      (NaiveDate::from_ymd_res(2018, 1, 1)?, "一月一日"),
      (NaiveDate::from_ymd_res(2018, 2, 19)?, "農曆年初四"),
      (NaiveDate::from_ymd_res(2018, 2, 16)?, "農曆年初一"),
      (NaiveDate::from_ymd_res(2018, 2, 17)?, "農曆年初二"),
      (NaiveDate::from_ymd_res(2018, 4, 5)?, "清明節"),
      (NaiveDate::from_ymd_res(2018, 5, 1)?, "勞動節"),
      (NaiveDate::from_ymd_res(2018, 6, 18)?, "端午節"),
      (NaiveDate::from_ymd_res(2018, 7, 1)?, "香港特別行政區成立紀念日"),
      (NaiveDate::from_ymd_res(2018, 7, 2)?, "香港特別行政區成立紀念日（補假）"),
      (NaiveDate::from_ymd_res(2018, 9, 25)?, "中秋節翌日"),
      (NaiveDate::from_ymd_res(2018, 10, 17)?, "重陽節"),
      (NaiveDate::from_ymd_res(2018, 10, 1)?, "國慶日"),
      (NaiveDate::from_ymd_res(2018, 12, 25)?, "聖誕節"),
    ],
    &mut map,
    Country::HK,
    "Hong Kong",
  );

  build_year(
    years,
    2019,
    vec![

      (NaiveDate::from_ymd_res(2019, 1, 1)?, "一月一日"),
      (NaiveDate::from_ymd_res(2019, 2, 5)?, "農曆年初一"),
      (NaiveDate::from_ymd_res(2019, 2, 6)?, "農曆年初二"),
      (NaiveDate::from_ymd_res(2019, 2, 7)?, "農曆年初三"),
      (NaiveDate::from_ymd_res(2019, 4, 5)?, "清明節"),
      (NaiveDate::from_ymd_res(2019, 5, 1)?, "勞動節"),
      (NaiveDate::from_ymd_res(2019, 6, 7)?, "端午節"),
      (NaiveDate::from_ymd_res(2019, 7, 1)?, "香港特別行政區成立紀念日"),
      (NaiveDate::from_ymd_res(2019, 9, 14)?, "中秋節翌日"),
      (NaiveDate::from_ymd_res(2019, 10, 7)?, "重陽節"),
      (NaiveDate::from_ymd_res(2019, 10, 1)?, "國慶日"),
      (NaiveDate::from_ymd_res(2019, 12, 25)?, "聖誕節"),
    ],
    &mut map,
    Country::HK,
    "Hong Kong",
  );

  build_year(
    years,
    2020,
    vec![

      (NaiveDate::from_ymd_res(2020, 1, 1)?, "一月一日"),
      (NaiveDate::from_ymd_res(2020, 1, 28)?, "農曆年初四"),
      (NaiveDate::from_ymd_res(2020, 1, 25)?, "農曆年初一"),
      (NaiveDate::from_ymd_res(2020, 1, 27)?, "農曆年初三"),
      (NaiveDate::from_ymd_res(2020, 4, 4)?, "清明節"),
      (NaiveDate::from_ymd_res(2020, 5, 1)?, "勞動節"),
      (NaiveDate::from_ymd_res(2020, 6, 25)?, "端午節"),
      (NaiveDate::from_ymd_res(2020, 7, 1)?, "香港特別行政區成立紀念日"),
      (NaiveDate::from_ymd_res(2020, 10, 2)?, "中秋節翌日"),
      (NaiveDate::from_ymd_res(2020, 10, 25)?, "重陽節"),
      (NaiveDate::from_ymd_res(2020, 10, 26)?, "重陽節（補假）"),
      (NaiveDate::from_ymd_res(2020, 10, 1)?, "國慶日"),
      (NaiveDate::from_ymd_res(2020, 12, 25)?, "聖誕節"),
    ],
    &mut map,
    Country::HK,
    "Hong Kong",
  );

  build_year(
    years,
    2021,
    vec![

      (NaiveDate::from_ymd_res(2021, 1, 1)?, "一月一日"),
      (NaiveDate::from_ymd_res(2021, 2, 15)?, "農曆年初四"),
      (NaiveDate::from_ymd_res(2021, 2, 12)?, "農曆年初一"),
      (NaiveDate::from_ymd_res(2021, 2, 13)?, "農曆年初二"),
      (NaiveDate::from_ymd_res(2021, 4, 4)?, "清明節"),
      (NaiveDate::from_ymd_res(2021, 4, 5)?, "清明節（補假）"),
      (NaiveDate::from_ymd_res(2021, 5, 1)?, "勞動節"),
      (NaiveDate::from_ymd_res(2021, 6, 14)?, "端午節"),
      (NaiveDate::from_ymd_res(2021, 7, 1)?, "香港特別行政區成立紀念日"),
      (NaiveDate::from_ymd_res(2021, 9, 22)?, "中秋節翌日"),
      (NaiveDate::from_ymd_res(2021, 10, 14)?, "重陽節"),
      (NaiveDate::from_ymd_res(2021, 10, 1)?, "國慶日"),
      (NaiveDate::from_ymd_res(2021, 12, 25)?, "聖誕節"),
    ],
    &mut map,
    Country::HK,
    "Hong Kong",
  );

  build_year(
    years,
    2022,
    vec![

      (NaiveDate::from_ymd_res(2022, 1, 1)?, "一月一日"),
      (NaiveDate::from_ymd_res(2022, 2, 1)?, "農曆年初一"),
      (NaiveDate::from_ymd_res(2022, 2, 2)?, "農曆年初二"),
      (NaiveDate::from_ymd_res(2022, 2, 3)?, "農曆年初三"),
      (NaiveDate::from_ymd_res(2022, 4, 5)?, "清明節"),
      (NaiveDate::from_ymd_res(2022, 5, 8)?, "佛誕"),
      (NaiveDate::from_ymd_res(2022, 5, 9)?, "佛誕（補假）"),
      (NaiveDate::from_ymd_res(2022, 5, 1)?, "勞動節"),
      (NaiveDate::from_ymd_res(2022, 5, 2)?, "勞動節（補假）"),
      (NaiveDate::from_ymd_res(2022, 6, 3)?, "端午節"),
      (NaiveDate::from_ymd_res(2022, 7, 1)?, "香港特別行政區成立紀念日"),
      (NaiveDate::from_ymd_res(2022, 9, 12)?, "中秋節後第二日"),
      (NaiveDate::from_ymd_res(2022, 10, 4)?, "重陽節"),
      (NaiveDate::from_ymd_res(2022, 10, 1)?, "國慶日"),
      (NaiveDate::from_ymd_res(2022, 12, 25)?, "聖誕節"),
      (NaiveDate::from_ymd_res(2022, 12, 26)?, "聖誕節（補假）"),
    ],
    &mut map,
    Country::HK,
    "Hong Kong",
  );

  build_year(
    years,
    2023,
    vec![

      (NaiveDate::from_ymd_res(2023, 1, 1)?, "一月一日"),
      (NaiveDate::from_ymd_res(2023, 1, 2)?, "一月一日（補假）"),
      (NaiveDate::from_ymd_res(2023, 1, 25)?, "農曆年初四"),
      (NaiveDate::from_ymd_res(2023, 1, 23)?, "農曆年初二"),
      (NaiveDate::from_ymd_res(2023, 1, 24)?, "農曆年初三"),
      (NaiveDate::from_ymd_res(2023, 4, 5)?, "清明節"),
      (NaiveDate::from_ymd_res(2023, 5, 26)?, "佛誕"),
      (NaiveDate::from_ymd_res(2023, 5, 1)?, "勞動節"),
      (NaiveDate::from_ymd_res(2023, 6, 22)?, "端午節"),
      (NaiveDate::from_ymd_res(2023, 7, 1)?, "香港特別行政區成立紀念日"),
      (NaiveDate::from_ymd_res(2023, 9, 30)?, "中秋節翌日"),
      (NaiveDate::from_ymd_res(2023, 10, 23)?, "重陽節"),
      (NaiveDate::from_ymd_res(2023, 10, 1)?, "國慶日"),
      (NaiveDate::from_ymd_res(2023, 10, 2)?, "國慶日（補假）"),
      (NaiveDate::from_ymd_res(2023, 12, 25)?, "聖誕節"),
    ],
    &mut map,
    Country::HK,
    "Hong Kong",
  );

  build_year(
    years,
    2024,
    vec![

      (NaiveDate::from_ymd_res(2024, 1, 1)?, "一月一日"),
      (NaiveDate::from_ymd_res(2024, 2, 13)?, "農曆年初四"),
      (NaiveDate::from_ymd_res(2024, 2, 10)?, "農曆年初一"),
      (NaiveDate::from_ymd_res(2024, 2, 12)?, "農曆年初三"),
      (NaiveDate::from_ymd_res(2024, 4, 4)?, "清明節"),
      (NaiveDate::from_ymd_res(2024, 5, 15)?, "佛誕"),
      (NaiveDate::from_ymd_res(2024, 5, 1)?, "勞動節"),
      (NaiveDate::from_ymd_res(2024, 6, 10)?, "端午節"),
      (NaiveDate::from_ymd_res(2024, 7, 1)?, "香港特別行政區成立紀念日"),
      (NaiveDate::from_ymd_res(2024, 9, 18)?, "中秋節翌日"),
      (NaiveDate::from_ymd_res(2024, 10, 11)?, "重陽節"),
      (NaiveDate::from_ymd_res(2024, 10, 1)?, "國慶日"),
      (NaiveDate::from_ymd_res(2024, 12, 26)?, "聖誕節後第一個周日"),
      (NaiveDate::from_ymd_res(2024, 12, 25)?, "聖誕節"),
    ],
    &mut map,
    Country::HK,
    "Hong Kong",
  );

  build_year(
    years,
    2025,
    vec![

      (NaiveDate::from_ymd_res(2025, 1, 1)?, "一月一日"),
      (NaiveDate::from_ymd_res(2025, 1, 29)?, "農曆年初一"),
      (NaiveDate::from_ymd_res(2025, 1, 30)?, "農曆年初二"),
      (NaiveDate::from_ymd_res(2025, 1, 31)?, "農曆年初三"),
      (NaiveDate::from_ymd_res(2025, 4, 4)?, "清明節"),
      (NaiveDate::from_ymd_res(2025, 5, 5)?, "佛誕"),
      (NaiveDate::from_ymd_res(2025, 5, 1)?, "勞動節"),
      (NaiveDate::from_ymd_res(2025, 5, 31)?, "端午節"),
      (NaiveDate::from_ymd_res(2025, 7, 1)?, "香港特別行政區成立紀念日"),
      (NaiveDate::from_ymd_res(2025, 10, 7)?, "中秋節翌日"),
      (NaiveDate::from_ymd_res(2025, 10, 29)?, "重陽節"),
      (NaiveDate::from_ymd_res(2025, 10, 1)?, "國慶日"),
      (NaiveDate::from_ymd_res(2025, 12, 26)?, "聖誕節後第一個周日"),
      (NaiveDate::from_ymd_res(2025, 12, 25)?, "聖誕節"),
    ],
    &mut map,
    Country::HK,
    "Hong Kong",
  );

  build_year(
    years,
    2026,
    vec![

      (NaiveDate::from_ymd_res(2026, 1, 1)?, "一月一日"),
      (NaiveDate::from_ymd_res(2026, 2, 17)?, "農曆年初一"),
      (NaiveDate::from_ymd_res(2026, 2, 18)?, "農曆年初二"),
      (NaiveDate::from_ymd_res(2026, 2, 19)?, "農曆年初三"),
      (NaiveDate::from_ymd_res(2026, 4, 6)?, "復活節星期一"),
      (NaiveDate::from_ymd_res(2026, 4, 5)?, "清明節"),
      (NaiveDate::from_ymd_res(2026, 4, 7)?, "清明節（補假）"),
      (NaiveDate::from_ymd_res(2026, 5, 24)?, "佛誕"),
      (NaiveDate::from_ymd_res(2026, 5, 25)?, "佛誕（補假）"),
      (NaiveDate::from_ymd_res(2026, 5, 1)?, "勞動節"),
      (NaiveDate::from_ymd_res(2026, 6, 19)?, "端午節"),
      (NaiveDate::from_ymd_res(2026, 7, 1)?, "香港特別行政區成立紀念日"),
      (NaiveDate::from_ymd_res(2026, 9, 26)?, "中秋節翌日"),
      (NaiveDate::from_ymd_res(2026, 10, 18)?, "重陽節"),
      (NaiveDate::from_ymd_res(2026, 10, 19)?, "重陽節（補假）"),
      (NaiveDate::from_ymd_res(2026, 10, 1)?, "國慶日"),
      (NaiveDate::from_ymd_res(2026, 12, 26)?, "聖誕節後第一個周日"),
      (NaiveDate::from_ymd_res(2026, 12, 25)?, "聖誕節"),
    ],
    &mut map,
    Country::HK,
    "Hong Kong",
  );

  build_year(
    years,
    2027,
    vec![

      (NaiveDate::from_ymd_res(2027, 1, 1)?, "一月一日"),
      (NaiveDate::from_ymd_res(2027, 2, 9)?, "農曆年初四"),
      (NaiveDate::from_ymd_res(2027, 2, 6)?, "農曆年初一"),
      (NaiveDate::from_ymd_res(2027, 2, 8)?, "農曆年初三"),
      (NaiveDate::from_ymd_res(2027, 3, 29)?, "復活節星期一"),
      (NaiveDate::from_ymd_res(2027, 4, 5)?, "清明節"),
      (NaiveDate::from_ymd_res(2027, 5, 13)?, "佛誕"),
      (NaiveDate::from_ymd_res(2027, 5, 1)?, "勞動節"),
      (NaiveDate::from_ymd_res(2027, 6, 9)?, "端午節"),
      (NaiveDate::from_ymd_res(2027, 7, 1)?, "香港特別行政區成立紀念日"),
      (NaiveDate::from_ymd_res(2027, 9, 16)?, "中秋節翌日"),
      (NaiveDate::from_ymd_res(2027, 10, 8)?, "重陽節"),
      (NaiveDate::from_ymd_res(2027, 10, 1)?, "國慶日"),
      (NaiveDate::from_ymd_res(2027, 12, 26)?, "聖誕節後第一個周日"),
      (NaiveDate::from_ymd_res(2027, 12, 27)?, "聖誕節後第一個周日（補假）"),
      (NaiveDate::from_ymd_res(2027, 12, 25)?, "聖誕節"),
    ],
    &mut map,
    Country::HK,
    "Hong Kong",
  );

  build_year(
    years,
    2028,
    vec![

      (NaiveDate::from_ymd_res(2028, 1, 1)?, "一月一日"),
      (NaiveDate::from_ymd_res(2028, 1, 26)?, "農曆年初一"),
      (NaiveDate::from_ymd_res(2028, 1, 27)?, "農曆年初二"),
      (NaiveDate::from_ymd_res(2028, 1, 28)?, "農曆年初三"),
      (NaiveDate::from_ymd_res(2028, 4, 14)?, "耶穌受難節"),
      (NaiveDate::from_ymd_res(2028, 4, 17)?, "復活節星期一"),
      (NaiveDate::from_ymd_res(2028, 4, 4)?, "清明節"),
      (NaiveDate::from_ymd_res(2028, 5, 2)?, "佛誕"),
      (NaiveDate::from_ymd_res(2028, 5, 1)?, "勞動節"),
      (NaiveDate::from_ymd_res(2028, 5, 28)?, "端午節"),
      (NaiveDate::from_ymd_res(2028, 5, 29)?, "端午節（補假）"),
      (NaiveDate::from_ymd_res(2028, 7, 1)?, "香港特別行政區成立紀念日"),
      (NaiveDate::from_ymd_res(2028, 10, 4)?, "中秋節翌日"),
      (NaiveDate::from_ymd_res(2028, 10, 26)?, "重陽節"),
      (NaiveDate::from_ymd_res(2028, 10, 1)?, "國慶日"),
      (NaiveDate::from_ymd_res(2028, 10, 2)?, "國慶日（補假）"),
      (NaiveDate::from_ymd_res(2028, 12, 26)?, "聖誕節後第一個周日"),
      (NaiveDate::from_ymd_res(2028, 12, 25)?, "聖誕節"),
    ],
    &mut map,
    Country::HK,
    "Hong Kong",
  );

  build_year(
    years,
    2029,
    vec![

      (NaiveDate::from_ymd_res(2029, 1, 1)?, "一月一日"),
      (NaiveDate::from_ymd_res(2029, 2, 13)?, "農曆年初一"),
      (NaiveDate::from_ymd_res(2029, 2, 14)?, "農曆年初二"),
      (NaiveDate::from_ymd_res(2029, 2, 15)?, "農曆年初三"),
      (NaiveDate::from_ymd_res(2029, 3, 30)?, "耶穌受難節"),
      (NaiveDate::from_ymd_res(2029, 4, 2)?, "復活節星期一"),
      (NaiveDate::from_ymd_res(2029, 4, 4)?, "清明節"),
      (NaiveDate::from_ymd_res(2029, 5, 20)?, "佛誕"),
      (NaiveDate::from_ymd_res(2029, 5, 21)?, "佛誕（補假）"),
      (NaiveDate::from_ymd_res(2029, 5, 1)?, "勞動節"),
      (NaiveDate::from_ymd_res(2029, 6, 16)?, "端午節"),
      (NaiveDate::from_ymd_res(2029, 7, 1)?, "香港特別行政區成立紀念日"),
      (NaiveDate::from_ymd_res(2029, 7, 2)?, "香港特別行政區成立紀念日（補假）"),
      (NaiveDate::from_ymd_res(2029, 9, 24)?, "中秋節後第二日"),
      (NaiveDate::from_ymd_res(2029, 10, 16)?, "重陽節"),
      (NaiveDate::from_ymd_res(2029, 10, 1)?, "國慶日"),
      (NaiveDate::from_ymd_res(2029, 12, 26)?, "聖誕節後第一個周日"),
      (NaiveDate::from_ymd_res(2029, 12, 25)?, "聖誕節"),
    ],
    &mut map,
    Country::HK,
    "Hong Kong",
  );

  build_year(
    years,
    2030,
    vec![

      (NaiveDate::from_ymd_res(2030, 1, 1)?, "一月一日"),
      (NaiveDate::from_ymd_res(2030, 2, 6)?, "農曆年初四"),
      (NaiveDate::from_ymd_res(2030, 2, 4)?, "農曆年初二"),
      (NaiveDate::from_ymd_res(2030, 2, 5)?, "農曆年初三"),
      (NaiveDate::from_ymd_res(2030, 4, 19)?, "耶穌受難節"),
      (NaiveDate::from_ymd_res(2030, 4, 20)?, "耶穌受難節翌日"),
      (NaiveDate::from_ymd_res(2030, 4, 22)?, "復活節星期一"),
      (NaiveDate::from_ymd_res(2030, 4, 5)?, "清明節"),
      (NaiveDate::from_ymd_res(2030, 5, 9)?, "佛誕"),
      (NaiveDate::from_ymd_res(2030, 5, 1)?, "勞動節"),
      (NaiveDate::from_ymd_res(2030, 6, 5)?, "端午節"),
      (NaiveDate::from_ymd_res(2030, 7, 1)?, "香港特別行政區成立紀念日"),
      (NaiveDate::from_ymd_res(2030, 9, 13)?, "中秋節翌日"),
      (NaiveDate::from_ymd_res(2030, 10, 5)?, "重陽節"),
      (NaiveDate::from_ymd_res(2030, 10, 1)?, "國慶日"),
      (NaiveDate::from_ymd_res(2030, 12, 26)?, "聖誕節後第一個周日"),
      (NaiveDate::from_ymd_res(2030, 12, 25)?, "聖誕節"),
    ],
    &mut map,
    Country::HK,
    "Hong Kong",
  );

  Ok(map)
}