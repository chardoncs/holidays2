
//! Vietnam
use super::*;

/// Generate holiday map for Vietnam.
#[allow(unused_mut, unused_variables)]
pub fn build(years: &Option<&std::ops::Range<Year>>) -> Result<HashMap<Year, BTreeMap<NaiveDate, Holiday>>> {
  let mut map = HashMap::new();

  build_year(
    years,
    2001,
    vec![

      (NaiveDate::from_ymd_res(2001, 1, 1)?, "Tết Dương lịch"),
      (NaiveDate::from_ymd_res(2001, 1, 23)?, "Giao thừa Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2001, 1, 24)?, "Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2001, 1, 25)?, "Mùng hai Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2001, 1, 26)?, "Mùng ba Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2001, 4, 30)?, "Ngày Chiến thắng"),
      (NaiveDate::from_ymd_res(2001, 5, 1)?, "Ngày Quốc tế Lao động"),
      (NaiveDate::from_ymd_res(2001, 9, 2)?, "Quốc khánh"),
      (NaiveDate::from_ymd_res(2001, 9, 3)?, "Quốc khánh (nghỉ bù)"),
    ],
    &mut map,
    Country::VN,
    "Vietnam",
  );

  build_year(
    years,
    2002,
    vec![

      (NaiveDate::from_ymd_res(2002, 1, 1)?, "Tết Dương lịch"),
      (NaiveDate::from_ymd_res(2002, 2, 11)?, "Giao thừa Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2002, 2, 12)?, "Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2002, 2, 13)?, "Mùng hai Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2002, 2, 14)?, "Mùng ba Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2002, 4, 30)?, "Ngày Chiến thắng"),
      (NaiveDate::from_ymd_res(2002, 5, 1)?, "Ngày Quốc tế Lao động"),
      (NaiveDate::from_ymd_res(2002, 9, 2)?, "Quốc khánh"),
    ],
    &mut map,
    Country::VN,
    "Vietnam",
  );

  build_year(
    years,
    2003,
    vec![

      (NaiveDate::from_ymd_res(2003, 1, 1)?, "Tết Dương lịch"),
      (NaiveDate::from_ymd_res(2003, 1, 31)?, "Giao thừa Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2003, 2, 1)?, "Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2003, 2, 2)?, "Mùng hai Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2003, 2, 3)?, "Mùng ba Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2003, 4, 30)?, "Ngày Chiến thắng"),
      (NaiveDate::from_ymd_res(2003, 5, 1)?, "Ngày Quốc tế Lao động"),
      (NaiveDate::from_ymd_res(2003, 9, 2)?, "Quốc khánh"),
      (NaiveDate::from_ymd_res(2003, 2, 4)?, "Mùng bốn Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2003, 2, 5)?, "Mùng năm Tết Nguyên Đán"),
    ],
    &mut map,
    Country::VN,
    "Vietnam",
  );

  build_year(
    years,
    2004,
    vec![

      (NaiveDate::from_ymd_res(2004, 1, 1)?, "Tết Dương lịch"),
      (NaiveDate::from_ymd_res(2004, 1, 21)?, "Giao thừa Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2004, 1, 22)?, "Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2004, 1, 23)?, "Mùng hai Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2004, 1, 24)?, "Mùng ba Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2004, 4, 30)?, "Ngày Chiến thắng"),
      (NaiveDate::from_ymd_res(2004, 5, 1)?, "Ngày Quốc tế Lao động"),
      (NaiveDate::from_ymd_res(2004, 9, 2)?, "Quốc khánh"),
      (NaiveDate::from_ymd_res(2004, 1, 26)?, "Mùng năm Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2004, 5, 3)?, "Ngày Quốc tế Lao động (nghỉ bù)"),
    ],
    &mut map,
    Country::VN,
    "Vietnam",
  );

  build_year(
    years,
    2005,
    vec![

      (NaiveDate::from_ymd_res(2005, 1, 1)?, "Tết Dương lịch"),
      (NaiveDate::from_ymd_res(2005, 2, 8)?, "Giao thừa Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2005, 2, 9)?, "Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2005, 2, 10)?, "Mùng hai Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2005, 2, 11)?, "Mùng ba Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2005, 4, 30)?, "Ngày Chiến thắng"),
      (NaiveDate::from_ymd_res(2005, 5, 1)?, "Ngày Quốc tế Lao động"),
      (NaiveDate::from_ymd_res(2005, 9, 2)?, "Quốc khánh"),
      (NaiveDate::from_ymd_res(2005, 1, 3)?, "Tết Dương lịch (nghỉ bù)"),
      (NaiveDate::from_ymd_res(2005, 5, 2)?, "Ngày Chiến thắng (nghỉ bù)"),
      (NaiveDate::from_ymd_res(2005, 5, 3)?, "Ngày Quốc tế Lao động (nghỉ bù)"),
    ],
    &mut map,
    Country::VN,
    "Vietnam",
  );

  build_year(
    years,
    2006,
    vec![

      (NaiveDate::from_ymd_res(2006, 1, 1)?, "Tết Dương lịch"),
      (NaiveDate::from_ymd_res(2006, 1, 28)?, "Giao thừa Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2006, 1, 29)?, "Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2006, 1, 30)?, "Mùng hai Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2006, 1, 31)?, "Mùng ba Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2006, 4, 30)?, "Ngày Chiến thắng"),
      (NaiveDate::from_ymd_res(2006, 5, 1)?, "Ngày Quốc tế Lao động"),
      (NaiveDate::from_ymd_res(2006, 9, 2)?, "Quốc khánh"),
      (NaiveDate::from_ymd_res(2006, 2, 1)?, "Mùng bốn Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2006, 2, 2)?, "Mùng năm Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2006, 1, 2)?, "Tết Dương lịch (nghỉ bù)"),
      (NaiveDate::from_ymd_res(2006, 5, 2)?, "Ngày Chiến thắng (nghỉ bù)"),
      (NaiveDate::from_ymd_res(2006, 9, 4)?, "Quốc khánh (nghỉ bù)"),
    ],
    &mut map,
    Country::VN,
    "Vietnam",
  );

  build_year(
    years,
    2007,
    vec![

      (NaiveDate::from_ymd_res(2007, 1, 1)?, "Tết Dương lịch"),
      (NaiveDate::from_ymd_res(2007, 2, 16)?, "Giao thừa Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2007, 2, 17)?, "Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2007, 2, 18)?, "Mùng hai Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2007, 2, 19)?, "Mùng ba Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2007, 4, 26)?, "Ngày Giỗ Tổ Hùng Vương"),
      (NaiveDate::from_ymd_res(2007, 4, 30)?, "Ngày Chiến thắng"),
      (NaiveDate::from_ymd_res(2007, 5, 1)?, "Ngày Quốc tế Lao động"),
      (NaiveDate::from_ymd_res(2007, 9, 2)?, "Quốc khánh"),
      (NaiveDate::from_ymd_res(2007, 2, 20)?, "Mùng bốn Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2007, 2, 21)?, "Mùng năm Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2007, 9, 3)?, "Quốc khánh (nghỉ bù)"),
    ],
    &mut map,
    Country::VN,
    "Vietnam",
  );

  build_year(
    years,
    2008,
    vec![

      (NaiveDate::from_ymd_res(2008, 1, 1)?, "Tết Dương lịch"),
      (NaiveDate::from_ymd_res(2008, 2, 6)?, "Giao thừa Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2008, 2, 7)?, "Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2008, 2, 8)?, "Mùng hai Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2008, 2, 9)?, "Mùng ba Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2008, 4, 15)?, "Ngày Giỗ Tổ Hùng Vương"),
      (NaiveDate::from_ymd_res(2008, 4, 30)?, "Ngày Chiến thắng"),
      (NaiveDate::from_ymd_res(2008, 5, 1)?, "Ngày Quốc tế Lao động"),
      (NaiveDate::from_ymd_res(2008, 9, 2)?, "Quốc khánh"),
      (NaiveDate::from_ymd_res(2008, 2, 11)?, "Mùng năm Tết Nguyên Đán"),
    ],
    &mut map,
    Country::VN,
    "Vietnam",
  );

  build_year(
    years,
    2009,
    vec![

      (NaiveDate::from_ymd_res(2009, 1, 1)?, "Tết Dương lịch"),
      (NaiveDate::from_ymd_res(2009, 1, 25)?, "Giao thừa Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2009, 1, 26)?, "Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2009, 1, 27)?, "Mùng hai Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2009, 1, 28)?, "Mùng ba Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2009, 4, 4)?, "Ngày Giỗ Tổ Hùng Vương"),
      (NaiveDate::from_ymd_res(2009, 4, 30)?, "Ngày Chiến thắng"),
      (NaiveDate::from_ymd_res(2009, 5, 1)?, "Ngày Quốc tế Lao động"),
      (NaiveDate::from_ymd_res(2009, 9, 2)?, "Quốc khánh"),
      (NaiveDate::from_ymd_res(2009, 1, 29)?, "Mùng bốn Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2009, 4, 6)?, "Ngày Giỗ Tổ Hùng Vương (nghỉ bù)"),
    ],
    &mut map,
    Country::VN,
    "Vietnam",
  );

  build_year(
    years,
    2010,
    vec![

      (NaiveDate::from_ymd_res(2010, 1, 1)?, "Tết Dương lịch"),
      (NaiveDate::from_ymd_res(2010, 2, 13)?, "Giao thừa Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2010, 2, 14)?, "Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2010, 2, 15)?, "Mùng hai Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2010, 2, 16)?, "Mùng ba Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2010, 4, 23)?, "Ngày Giỗ Tổ Hùng Vương"),
      (NaiveDate::from_ymd_res(2010, 4, 30)?, "Ngày Chiến thắng"),
      (NaiveDate::from_ymd_res(2010, 5, 1)?, "Ngày Quốc tế Lao động"),
      (NaiveDate::from_ymd_res(2010, 9, 2)?, "Quốc khánh"),
      (NaiveDate::from_ymd_res(2010, 2, 17)?, "Mùng bốn Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2010, 2, 18)?, "Mùng năm Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2010, 5, 3)?, "Ngày Quốc tế Lao động (nghỉ bù)"),
      (NaiveDate::from_ymd_res(2010, 2, 19)?, "Ngày nghỉ (thay cho ngày 27/02/2010)"),
    ],
    &mut map,
    Country::VN,
    "Vietnam",
  );

  build_year(
    years,
    2011,
    vec![

      (NaiveDate::from_ymd_res(2011, 1, 1)?, "Tết Dương lịch"),
      (NaiveDate::from_ymd_res(2011, 2, 2)?, "Giao thừa Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2011, 2, 3)?, "Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2011, 2, 4)?, "Mùng hai Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2011, 2, 5)?, "Mùng ba Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2011, 4, 12)?, "Ngày Giỗ Tổ Hùng Vương"),
      (NaiveDate::from_ymd_res(2011, 4, 30)?, "Ngày Chiến thắng"),
      (NaiveDate::from_ymd_res(2011, 5, 1)?, "Ngày Quốc tế Lao động"),
      (NaiveDate::from_ymd_res(2011, 9, 2)?, "Quốc khánh"),
      (NaiveDate::from_ymd_res(2011, 2, 7)?, "Mùng năm Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2011, 1, 3)?, "Tết Dương lịch (nghỉ bù)"),
      (NaiveDate::from_ymd_res(2011, 5, 2)?, "Ngày Chiến thắng (nghỉ bù)"),
      (NaiveDate::from_ymd_res(2011, 5, 3)?, "Ngày Quốc tế Lao động (nghỉ bù)"),
    ],
    &mut map,
    Country::VN,
    "Vietnam",
  );

  build_year(
    years,
    2012,
    vec![

      (NaiveDate::from_ymd_res(2012, 1, 1)?, "Tết Dương lịch"),
      (NaiveDate::from_ymd_res(2012, 1, 22)?, "Giao thừa Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2012, 1, 23)?, "Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2012, 1, 24)?, "Mùng hai Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2012, 1, 25)?, "Mùng ba Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2012, 3, 31)?, "Ngày Giỗ Tổ Hùng Vương"),
      (NaiveDate::from_ymd_res(2012, 4, 30)?, "Ngày Chiến thắng"),
      (NaiveDate::from_ymd_res(2012, 5, 1)?, "Ngày Quốc tế Lao động"),
      (NaiveDate::from_ymd_res(2012, 9, 2)?, "Quốc khánh"),
      (NaiveDate::from_ymd_res(2012, 1, 26)?, "Mùng bốn Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2012, 1, 2)?, "Tết Dương lịch (nghỉ bù)"),
      (NaiveDate::from_ymd_res(2012, 4, 2)?, "Ngày Giỗ Tổ Hùng Vương (nghỉ bù)"),
      (NaiveDate::from_ymd_res(2012, 9, 3)?, "Quốc khánh (nghỉ bù)"),
      (NaiveDate::from_ymd_res(2012, 1, 27)?, "Ngày nghỉ (thay cho ngày 04/02/2012)"),
    ],
    &mut map,
    Country::VN,
    "Vietnam",
  );

  build_year(
    years,
    2013,
    vec![

      (NaiveDate::from_ymd_res(2013, 1, 1)?, "Tết Dương lịch"),
      (NaiveDate::from_ymd_res(2013, 2, 9)?, "Giao thừa Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2013, 2, 10)?, "Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2013, 2, 11)?, "Mùng hai Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2013, 2, 12)?, "Mùng ba Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2013, 2, 13)?, "Mùng bốn Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2013, 4, 19)?, "Ngày Giỗ Tổ Hùng Vương"),
      (NaiveDate::from_ymd_res(2013, 4, 30)?, "Ngày Chiến thắng"),
      (NaiveDate::from_ymd_res(2013, 5, 1)?, "Ngày Quốc tế Lao động"),
      (NaiveDate::from_ymd_res(2013, 9, 2)?, "Quốc khánh"),
      (NaiveDate::from_ymd_res(2013, 2, 14)?, "Mùng năm Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2013, 2, 15)?, "Mùng sáu Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2013, 4, 29)?, "Ngày nghỉ (thay cho ngày 04/05/2013)"),
    ],
    &mut map,
    Country::VN,
    "Vietnam",
  );

  build_year(
    years,
    2014,
    vec![

      (NaiveDate::from_ymd_res(2014, 1, 1)?, "Tết Dương lịch"),
      (NaiveDate::from_ymd_res(2014, 1, 30)?, "Giao thừa Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2014, 1, 31)?, "Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2014, 2, 1)?, "Mùng hai Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2014, 2, 2)?, "Mùng ba Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2014, 2, 3)?, "Mùng bốn Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2014, 4, 9)?, "Ngày Giỗ Tổ Hùng Vương"),
      (NaiveDate::from_ymd_res(2014, 4, 30)?, "Ngày Chiến thắng"),
      (NaiveDate::from_ymd_res(2014, 5, 1)?, "Ngày Quốc tế Lao động"),
      (NaiveDate::from_ymd_res(2014, 9, 2)?, "Quốc khánh"),
      (NaiveDate::from_ymd_res(2014, 1, 29)?, "29 Tết"),
      (NaiveDate::from_ymd_res(2014, 2, 4)?, "Mùng năm Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2014, 5, 2)?, "Ngày nghỉ (thay cho ngày 26/04/2014)"),
      (NaiveDate::from_ymd_res(2014, 9, 1)?, "Ngày nghỉ (thay cho ngày 06/09/2014)"),
    ],
    &mut map,
    Country::VN,
    "Vietnam",
  );

  build_year(
    years,
    2015,
    vec![

      (NaiveDate::from_ymd_res(2015, 1, 1)?, "Tết Dương lịch"),
      (NaiveDate::from_ymd_res(2015, 2, 18)?, "Giao thừa Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2015, 2, 19)?, "Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2015, 2, 20)?, "Mùng hai Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2015, 2, 21)?, "Mùng ba Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2015, 2, 22)?, "Mùng bốn Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2015, 4, 28)?, "Ngày Giỗ Tổ Hùng Vương"),
      (NaiveDate::from_ymd_res(2015, 4, 30)?, "Ngày Chiến thắng"),
      (NaiveDate::from_ymd_res(2015, 5, 1)?, "Ngày Quốc tế Lao động"),
      (NaiveDate::from_ymd_res(2015, 9, 2)?, "Quốc khánh"),
      (NaiveDate::from_ymd_res(2015, 2, 17)?, "29 Tết"),
      (NaiveDate::from_ymd_res(2015, 2, 23)?, "Mùng năm Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2015, 1, 2)?, "Ngày nghỉ (thay cho ngày 27/12/2014)"),
      (NaiveDate::from_ymd_res(2015, 2, 16)?, "Ngày nghỉ (thay cho ngày 14/02/2015)"),
      (NaiveDate::from_ymd_res(2015, 4, 29)?, "Ngày nghỉ (thay cho ngày 25/04/2015)"),
    ],
    &mut map,
    Country::VN,
    "Vietnam",
  );

  build_year(
    years,
    2016,
    vec![

      (NaiveDate::from_ymd_res(2016, 1, 1)?, "Tết Dương lịch"),
      (NaiveDate::from_ymd_res(2016, 2, 7)?, "Giao thừa Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2016, 2, 8)?, "Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2016, 2, 9)?, "Mùng hai Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2016, 2, 10)?, "Mùng ba Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2016, 2, 11)?, "Mùng bốn Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2016, 4, 16)?, "Ngày Giỗ Tổ Hùng Vương"),
      (NaiveDate::from_ymd_res(2016, 4, 30)?, "Ngày Chiến thắng"),
      (NaiveDate::from_ymd_res(2016, 5, 1)?, "Ngày Quốc tế Lao động"),
      (NaiveDate::from_ymd_res(2016, 9, 2)?, "Quốc khánh"),
      (NaiveDate::from_ymd_res(2016, 2, 12)?, "Mùng năm Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2016, 4, 18)?, "Ngày Giỗ Tổ Hùng Vương (nghỉ bù)"),
      (NaiveDate::from_ymd_res(2016, 5, 2)?, "Ngày Chiến thắng (nghỉ bù)"),
      (NaiveDate::from_ymd_res(2016, 5, 3)?, "Ngày Quốc tế Lao động (nghỉ bù)"),
    ],
    &mut map,
    Country::VN,
    "Vietnam",
  );

  build_year(
    years,
    2017,
    vec![

      (NaiveDate::from_ymd_res(2017, 1, 1)?, "Tết Dương lịch"),
      (NaiveDate::from_ymd_res(2017, 1, 27)?, "Giao thừa Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2017, 1, 28)?, "Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2017, 1, 29)?, "Mùng hai Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2017, 1, 30)?, "Mùng ba Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2017, 1, 31)?, "Mùng bốn Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2017, 4, 6)?, "Ngày Giỗ Tổ Hùng Vương"),
      (NaiveDate::from_ymd_res(2017, 4, 30)?, "Ngày Chiến thắng"),
      (NaiveDate::from_ymd_res(2017, 5, 1)?, "Ngày Quốc tế Lao động"),
      (NaiveDate::from_ymd_res(2017, 9, 2)?, "Quốc khánh"),
      (NaiveDate::from_ymd_res(2017, 1, 26)?, "29 Tết"),
      (NaiveDate::from_ymd_res(2017, 2, 1)?, "Mùng năm Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2017, 1, 2)?, "Tết Dương lịch (nghỉ bù)"),
      (NaiveDate::from_ymd_res(2017, 5, 2)?, "Ngày Chiến thắng (nghỉ bù)"),
      (NaiveDate::from_ymd_res(2017, 9, 4)?, "Quốc khánh (nghỉ bù)"),
    ],
    &mut map,
    Country::VN,
    "Vietnam",
  );

  build_year(
    years,
    2018,
    vec![

      (NaiveDate::from_ymd_res(2018, 1, 1)?, "Tết Dương lịch"),
      (NaiveDate::from_ymd_res(2018, 2, 15)?, "Giao thừa Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2018, 2, 16)?, "Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2018, 2, 17)?, "Mùng hai Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2018, 2, 18)?, "Mùng ba Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2018, 2, 19)?, "Mùng bốn Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2018, 4, 25)?, "Ngày Giỗ Tổ Hùng Vương"),
      (NaiveDate::from_ymd_res(2018, 4, 30)?, "Ngày Chiến thắng"),
      (NaiveDate::from_ymd_res(2018, 5, 1)?, "Ngày Quốc tế Lao động"),
      (NaiveDate::from_ymd_res(2018, 9, 2)?, "Quốc khánh"),
      (NaiveDate::from_ymd_res(2018, 2, 14)?, "29 Tết"),
      (NaiveDate::from_ymd_res(2018, 2, 20)?, "Mùng năm Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2018, 9, 3)?, "Quốc khánh (nghỉ bù)"),
      (NaiveDate::from_ymd_res(2018, 12, 31)?, "Ngày nghỉ (thay cho ngày 05/01/2019)"),
    ],
    &mut map,
    Country::VN,
    "Vietnam",
  );

  build_year(
    years,
    2019,
    vec![

      (NaiveDate::from_ymd_res(2019, 1, 1)?, "Tết Dương lịch"),
      (NaiveDate::from_ymd_res(2019, 2, 4)?, "Giao thừa Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2019, 2, 5)?, "Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2019, 2, 6)?, "Mùng hai Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2019, 2, 7)?, "Mùng ba Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2019, 2, 8)?, "Mùng bốn Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2019, 4, 14)?, "Ngày Giỗ Tổ Hùng Vương"),
      (NaiveDate::from_ymd_res(2019, 4, 30)?, "Ngày Chiến thắng"),
      (NaiveDate::from_ymd_res(2019, 5, 1)?, "Ngày Quốc tế Lao động"),
      (NaiveDate::from_ymd_res(2019, 9, 2)?, "Quốc khánh"),
      (NaiveDate::from_ymd_res(2019, 4, 15)?, "Ngày Giỗ Tổ Hùng Vương (nghỉ bù)"),
      (NaiveDate::from_ymd_res(2019, 4, 29)?, "Ngày nghỉ (thay cho ngày 04/05/2019)"),
    ],
    &mut map,
    Country::VN,
    "Vietnam",
  );

  build_year(
    years,
    2020,
    vec![

      (NaiveDate::from_ymd_res(2020, 1, 1)?, "Tết Dương lịch"),
      (NaiveDate::from_ymd_res(2020, 1, 24)?, "Giao thừa Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2020, 1, 25)?, "Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2020, 1, 26)?, "Mùng hai Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2020, 1, 27)?, "Mùng ba Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2020, 1, 28)?, "Mùng bốn Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2020, 4, 2)?, "Ngày Giỗ Tổ Hùng Vương"),
      (NaiveDate::from_ymd_res(2020, 4, 30)?, "Ngày Chiến thắng"),
      (NaiveDate::from_ymd_res(2020, 5, 1)?, "Ngày Quốc tế Lao động"),
      (NaiveDate::from_ymd_res(2020, 9, 2)?, "Quốc khánh"),
      (NaiveDate::from_ymd_res(2020, 1, 23)?, "29 Tết"),
      (NaiveDate::from_ymd_res(2020, 1, 29)?, "Mùng năm Tết Nguyên Đán"),
    ],
    &mut map,
    Country::VN,
    "Vietnam",
  );

  build_year(
    years,
    2021,
    vec![

      (NaiveDate::from_ymd_res(2021, 1, 1)?, "Tết Dương lịch"),
      (NaiveDate::from_ymd_res(2021, 2, 11)?, "Giao thừa Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2021, 2, 12)?, "Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2021, 2, 13)?, "Mùng hai Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2021, 2, 14)?, "Mùng ba Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2021, 2, 15)?, "Mùng bốn Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2021, 4, 21)?, "Ngày Giỗ Tổ Hùng Vương"),
      (NaiveDate::from_ymd_res(2021, 4, 30)?, "Ngày Chiến thắng"),
      (NaiveDate::from_ymd_res(2021, 5, 1)?, "Ngày Quốc tế Lao động"),
      (NaiveDate::from_ymd_res(2021, 9, 2)?, "Quốc khánh"),
      (NaiveDate::from_ymd_res(2021, 9, 3)?, "Quốc khánh"),
      (NaiveDate::from_ymd_res(2021, 2, 10)?, "29 Tết"),
      (NaiveDate::from_ymd_res(2021, 2, 16)?, "Mùng năm Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2021, 5, 3)?, "Ngày Quốc tế Lao động (nghỉ bù)"),
    ],
    &mut map,
    Country::VN,
    "Vietnam",
  );

  build_year(
    years,
    2022,
    vec![

      (NaiveDate::from_ymd_res(2022, 1, 1)?, "Tết Dương lịch"),
      (NaiveDate::from_ymd_res(2022, 1, 31)?, "Giao thừa Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2022, 2, 1)?, "Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2022, 2, 2)?, "Mùng hai Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2022, 2, 3)?, "Mùng ba Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2022, 2, 4)?, "Mùng bốn Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2022, 4, 10)?, "Ngày Giỗ Tổ Hùng Vương"),
      (NaiveDate::from_ymd_res(2022, 4, 30)?, "Ngày Chiến thắng"),
      (NaiveDate::from_ymd_res(2022, 5, 1)?, "Ngày Quốc tế Lao động"),
      (NaiveDate::from_ymd_res(2022, 9, 2)?, "Quốc khánh"),
      (NaiveDate::from_ymd_res(2022, 9, 1)?, "Quốc khánh"),
      (NaiveDate::from_ymd_res(2022, 1, 3)?, "Tết Dương lịch (nghỉ bù)"),
      (NaiveDate::from_ymd_res(2022, 4, 11)?, "Ngày Giỗ Tổ Hùng Vương (nghỉ bù)"),
      (NaiveDate::from_ymd_res(2022, 5, 2)?, "Ngày Chiến thắng (nghỉ bù)"),
      (NaiveDate::from_ymd_res(2022, 5, 3)?, "Ngày Quốc tế Lao động (nghỉ bù)"),
    ],
    &mut map,
    Country::VN,
    "Vietnam",
  );

  build_year(
    years,
    2023,
    vec![

      (NaiveDate::from_ymd_res(2023, 1, 1)?, "Tết Dương lịch"),
      (NaiveDate::from_ymd_res(2023, 1, 21)?, "Giao thừa Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2023, 1, 22)?, "Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2023, 1, 23)?, "Mùng hai Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2023, 1, 24)?, "Mùng ba Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2023, 1, 25)?, "Mùng bốn Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2023, 4, 29)?, "Ngày Giỗ Tổ Hùng Vương"),
      (NaiveDate::from_ymd_res(2023, 4, 30)?, "Ngày Chiến thắng"),
      (NaiveDate::from_ymd_res(2023, 5, 1)?, "Ngày Quốc tế Lao động"),
      (NaiveDate::from_ymd_res(2023, 9, 2)?, "Quốc khánh"),
      (NaiveDate::from_ymd_res(2023, 9, 1)?, "Quốc khánh"),
      (NaiveDate::from_ymd_res(2023, 1, 20)?, "29 Tết"),
      (NaiveDate::from_ymd_res(2023, 1, 26)?, "Mùng năm Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2023, 1, 2)?, "Tết Dương lịch (nghỉ bù)"),
      (NaiveDate::from_ymd_res(2023, 5, 2)?, "Ngày Giỗ Tổ Hùng Vương (nghỉ bù)"),
      (NaiveDate::from_ymd_res(2023, 5, 3)?, "Ngày Chiến thắng (nghỉ bù)"),
      (NaiveDate::from_ymd_res(2023, 9, 4)?, "Quốc khánh (nghỉ bù)"),
    ],
    &mut map,
    Country::VN,
    "Vietnam",
  );

  build_year(
    years,
    2024,
    vec![

      (NaiveDate::from_ymd_res(2024, 1, 1)?, "Tết Dương lịch"),
      (NaiveDate::from_ymd_res(2024, 2, 9)?, "Giao thừa Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2024, 2, 10)?, "Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2024, 2, 11)?, "Mùng hai Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2024, 2, 12)?, "Mùng ba Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2024, 2, 13)?, "Mùng bốn Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2024, 4, 18)?, "Ngày Giỗ Tổ Hùng Vương"),
      (NaiveDate::from_ymd_res(2024, 4, 30)?, "Ngày Chiến thắng"),
      (NaiveDate::from_ymd_res(2024, 5, 1)?, "Ngày Quốc tế Lao động"),
      (NaiveDate::from_ymd_res(2024, 9, 2)?, "Quốc khánh"),
      (NaiveDate::from_ymd_res(2024, 9, 3)?, "Quốc khánh"),
      (NaiveDate::from_ymd_res(2024, 2, 8)?, "29 Tết"),
      (NaiveDate::from_ymd_res(2024, 2, 14)?, "Mùng năm Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2024, 4, 29)?, "Ngày nghỉ (thay cho ngày 04/05/2024)"),
    ],
    &mut map,
    Country::VN,
    "Vietnam",
  );

  build_year(
    years,
    2025,
    vec![

      (NaiveDate::from_ymd_res(2025, 1, 1)?, "Tết Dương lịch"),
      (NaiveDate::from_ymd_res(2025, 1, 28)?, "Giao thừa Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2025, 1, 29)?, "Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2025, 1, 30)?, "Mùng hai Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2025, 1, 31)?, "Mùng ba Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2025, 2, 1)?, "Mùng bốn Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2025, 4, 7)?, "Ngày Giỗ Tổ Hùng Vương"),
      (NaiveDate::from_ymd_res(2025, 4, 30)?, "Ngày Chiến thắng"),
      (NaiveDate::from_ymd_res(2025, 5, 1)?, "Ngày Quốc tế Lao động"),
      (NaiveDate::from_ymd_res(2025, 9, 2)?, "Quốc khánh"),
      (NaiveDate::from_ymd_res(2025, 9, 1)?, "Quốc khánh"),
      (NaiveDate::from_ymd_res(2025, 1, 27)?, "29 Tết"),
      (NaiveDate::from_ymd_res(2025, 5, 2)?, "Ngày nghỉ (thay cho ngày 26/04/2025)"),
    ],
    &mut map,
    Country::VN,
    "Vietnam",
  );

  build_year(
    years,
    2026,
    vec![

      (NaiveDate::from_ymd_res(2026, 1, 1)?, "Tết Dương lịch"),
      (NaiveDate::from_ymd_res(2026, 2, 16)?, "Giao thừa Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2026, 2, 17)?, "Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2026, 2, 18)?, "Mùng hai Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2026, 2, 19)?, "Mùng ba Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2026, 2, 20)?, "Mùng bốn Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2026, 4, 26)?, "Ngày Giỗ Tổ Hùng Vương"),
      (NaiveDate::from_ymd_res(2026, 4, 30)?, "Ngày Chiến thắng"),
      (NaiveDate::from_ymd_res(2026, 5, 1)?, "Ngày Quốc tế Lao động"),
      (NaiveDate::from_ymd_res(2026, 9, 2)?, "Quốc khánh"),
      (NaiveDate::from_ymd_res(2026, 9, 1)?, "Quốc khánh"),
      (NaiveDate::from_ymd_res(2026, 11, 24)?, "Ngày Văn hóa Việt Nam"),
      (NaiveDate::from_ymd_res(2026, 4, 27)?, "Ngày Giỗ Tổ Hùng Vương (nghỉ bù)"),
      (NaiveDate::from_ymd_res(2026, 8, 31)?, "Ngày nghỉ (thay cho ngày 22/08/2026)"),
    ],
    &mut map,
    Country::VN,
    "Vietnam",
  );

  build_year(
    years,
    2027,
    vec![

      (NaiveDate::from_ymd_res(2027, 1, 1)?, "Tết Dương lịch"),
      (NaiveDate::from_ymd_res(2027, 2, 5)?, "Giao thừa Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2027, 2, 6)?, "Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2027, 2, 7)?, "Mùng hai Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2027, 2, 8)?, "Mùng ba Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2027, 2, 9)?, "Mùng bốn Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2027, 4, 16)?, "Ngày Giỗ Tổ Hùng Vương"),
      (NaiveDate::from_ymd_res(2027, 4, 30)?, "Ngày Chiến thắng"),
      (NaiveDate::from_ymd_res(2027, 5, 1)?, "Ngày Quốc tế Lao động"),
      (NaiveDate::from_ymd_res(2027, 9, 2)?, "Quốc khánh"),
      (NaiveDate::from_ymd_res(2027, 9, 3)?, "Quốc khánh"),
      (NaiveDate::from_ymd_res(2027, 11, 24)?, "Ngày Văn hóa Việt Nam"),
      (NaiveDate::from_ymd_res(2027, 2, 4)?, "29 Tết"),
      (NaiveDate::from_ymd_res(2027, 2, 10)?, "Mùng năm Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2027, 5, 3)?, "Ngày Quốc tế Lao động (nghỉ bù)"),
    ],
    &mut map,
    Country::VN,
    "Vietnam",
  );

  build_year(
    years,
    2028,
    vec![

      (NaiveDate::from_ymd_res(2028, 1, 1)?, "Tết Dương lịch"),
      (NaiveDate::from_ymd_res(2028, 1, 25)?, "Giao thừa Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2028, 1, 26)?, "Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2028, 1, 27)?, "Mùng hai Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2028, 1, 28)?, "Mùng ba Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2028, 1, 29)?, "Mùng bốn Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2028, 4, 4)?, "Ngày Giỗ Tổ Hùng Vương"),
      (NaiveDate::from_ymd_res(2028, 4, 30)?, "Ngày Chiến thắng"),
      (NaiveDate::from_ymd_res(2028, 5, 1)?, "Ngày Quốc tế Lao động"),
      (NaiveDate::from_ymd_res(2028, 9, 2)?, "Quốc khánh"),
      (NaiveDate::from_ymd_res(2028, 9, 1)?, "Quốc khánh"),
      (NaiveDate::from_ymd_res(2028, 11, 24)?, "Ngày Văn hóa Việt Nam"),
      (NaiveDate::from_ymd_res(2028, 1, 24)?, "29 Tết"),
      (NaiveDate::from_ymd_res(2028, 1, 3)?, "Tết Dương lịch (nghỉ bù)"),
      (NaiveDate::from_ymd_res(2028, 5, 2)?, "Ngày Chiến thắng (nghỉ bù)"),
      (NaiveDate::from_ymd_res(2028, 9, 4)?, "Quốc khánh (nghỉ bù)"),
    ],
    &mut map,
    Country::VN,
    "Vietnam",
  );

  build_year(
    years,
    2029,
    vec![

      (NaiveDate::from_ymd_res(2029, 1, 1)?, "Tết Dương lịch"),
      (NaiveDate::from_ymd_res(2029, 2, 12)?, "Giao thừa Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2029, 2, 13)?, "Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2029, 2, 14)?, "Mùng hai Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2029, 2, 15)?, "Mùng ba Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2029, 2, 16)?, "Mùng bốn Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2029, 4, 23)?, "Ngày Giỗ Tổ Hùng Vương"),
      (NaiveDate::from_ymd_res(2029, 4, 30)?, "Ngày Chiến thắng"),
      (NaiveDate::from_ymd_res(2029, 5, 1)?, "Ngày Quốc tế Lao động"),
      (NaiveDate::from_ymd_res(2029, 9, 2)?, "Quốc khánh"),
      (NaiveDate::from_ymd_res(2029, 9, 3)?, "Quốc khánh"),
      (NaiveDate::from_ymd_res(2029, 11, 24)?, "Ngày Văn hóa Việt Nam"),
      (NaiveDate::from_ymd_res(2029, 9, 4)?, "Quốc khánh (nghỉ bù)"),
      (NaiveDate::from_ymd_res(2029, 11, 26)?, "Ngày Văn hóa Việt Nam (nghỉ bù)"),
    ],
    &mut map,
    Country::VN,
    "Vietnam",
  );

  build_year(
    years,
    2030,
    vec![

      (NaiveDate::from_ymd_res(2030, 1, 1)?, "Tết Dương lịch"),
      (NaiveDate::from_ymd_res(2030, 2, 1)?, "Giao thừa Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2030, 2, 2)?, "Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2030, 2, 3)?, "Mùng hai Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2030, 2, 4)?, "Mùng ba Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2030, 2, 5)?, "Mùng bốn Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2030, 4, 12)?, "Ngày Giỗ Tổ Hùng Vương"),
      (NaiveDate::from_ymd_res(2030, 4, 30)?, "Ngày Chiến thắng"),
      (NaiveDate::from_ymd_res(2030, 5, 1)?, "Ngày Quốc tế Lao động"),
      (NaiveDate::from_ymd_res(2030, 9, 2)?, "Quốc khánh"),
      (NaiveDate::from_ymd_res(2030, 9, 3)?, "Quốc khánh"),
      (NaiveDate::from_ymd_res(2030, 11, 24)?, "Ngày Văn hóa Việt Nam"),
      (NaiveDate::from_ymd_res(2030, 1, 31)?, "29 Tết"),
      (NaiveDate::from_ymd_res(2030, 2, 6)?, "Mùng năm Tết Nguyên Đán"),
      (NaiveDate::from_ymd_res(2030, 11, 25)?, "Ngày Văn hóa Việt Nam (nghỉ bù)"),
    ],
    &mut map,
    Country::VN,
    "Vietnam",
  );

  Ok(map)
}