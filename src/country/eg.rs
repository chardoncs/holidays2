
//! Egypt
use super::*;

/// Generate holiday map for Egypt.
#[allow(unused_mut, unused_variables)]
pub fn build(years: &Option<&std::ops::Range<Year>>) -> Result<HashMap<Year, BTreeMap<NaiveDate, Holiday>>> {
  let mut map = HashMap::new();

  build_year(
    years,
    2001,
    vec![

      (NaiveDate::from_ymd_res(2001, 4, 25)?, "عيد تحرير سيناء"),
      (NaiveDate::from_ymd_res(2001, 4, 16)?, "عيد شم النسيم"),
      (NaiveDate::from_ymd_res(2001, 5, 1)?, "عيد العمال"),
      (NaiveDate::from_ymd_res(2001, 7, 23)?, "عيد ثورة ٢٣ يوليو"),
      (NaiveDate::from_ymd_res(2001, 10, 6)?, "عيد القوات المسلحة"),
      (NaiveDate::from_ymd_res(2001, 3, 26)?, "رأس السنة الهجرية (تقديري)"),
      (NaiveDate::from_ymd_res(2001, 6, 4)?, "المولد النبوي الشريف (تقديري)"),
      (NaiveDate::from_ymd_res(2001, 12, 16)?, "عيد الفطر المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2001, 12, 17)?, "عيد الفطر المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2001, 3, 4)?, "وقفة عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2001, 3, 5)?, "عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2001, 3, 6)?, "عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2001, 3, 7)?, "عيد الأضحى المبارك (تقديري)"),
    ],
    &mut map,
    Country::EG,
    "Egypt",
  );

  build_year(
    years,
    2002,
    vec![

      (NaiveDate::from_ymd_res(2002, 1, 7)?, "عيد الميلاد المجيد"),
      (NaiveDate::from_ymd_res(2002, 4, 25)?, "عيد تحرير سيناء"),
      (NaiveDate::from_ymd_res(2002, 5, 6)?, "عيد شم النسيم"),
      (NaiveDate::from_ymd_res(2002, 5, 1)?, "عيد العمال"),
      (NaiveDate::from_ymd_res(2002, 7, 23)?, "عيد ثورة ٢٣ يوليو"),
      (NaiveDate::from_ymd_res(2002, 10, 6)?, "عيد القوات المسلحة"),
      (NaiveDate::from_ymd_res(2002, 3, 15)?, "رأس السنة الهجرية (تقديري)"),
      (NaiveDate::from_ymd_res(2002, 5, 24)?, "المولد النبوي الشريف (تقديري)"),
      (NaiveDate::from_ymd_res(2002, 12, 5)?, "عيد الفطر المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2002, 12, 6)?, "عيد الفطر المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2002, 2, 21)?, "وقفة عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2002, 2, 22)?, "عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2002, 2, 23)?, "عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2002, 2, 24)?, "عيد الأضحى المبارك (تقديري)"),
    ],
    &mut map,
    Country::EG,
    "Egypt",
  );

  build_year(
    years,
    2003,
    vec![

      (NaiveDate::from_ymd_res(2003, 1, 7)?, "عيد الميلاد المجيد"),
      (NaiveDate::from_ymd_res(2003, 4, 25)?, "عيد تحرير سيناء"),
      (NaiveDate::from_ymd_res(2003, 4, 28)?, "عيد شم النسيم"),
      (NaiveDate::from_ymd_res(2003, 5, 1)?, "عيد العمال"),
      (NaiveDate::from_ymd_res(2003, 7, 23)?, "عيد ثورة ٢٣ يوليو"),
      (NaiveDate::from_ymd_res(2003, 10, 6)?, "عيد القوات المسلحة"),
      (NaiveDate::from_ymd_res(2003, 3, 4)?, "رأس السنة الهجرية (تقديري)"),
      (NaiveDate::from_ymd_res(2003, 5, 13)?, "المولد النبوي الشريف (تقديري)"),
      (NaiveDate::from_ymd_res(2003, 11, 25)?, "عيد الفطر المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2003, 11, 26)?, "عيد الفطر المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2003, 2, 10)?, "وقفة عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2003, 2, 11)?, "عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2003, 2, 12)?, "عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2003, 2, 13)?, "عيد الأضحى المبارك (تقديري)"),
    ],
    &mut map,
    Country::EG,
    "Egypt",
  );

  build_year(
    years,
    2004,
    vec![

      (NaiveDate::from_ymd_res(2004, 1, 7)?, "عيد الميلاد المجيد"),
      (NaiveDate::from_ymd_res(2004, 4, 25)?, "عيد تحرير سيناء"),
      (NaiveDate::from_ymd_res(2004, 4, 12)?, "عيد شم النسيم"),
      (NaiveDate::from_ymd_res(2004, 5, 1)?, "المولد النبوي الشريف (تقديري); عيد العمال"),
      (NaiveDate::from_ymd_res(2004, 7, 23)?, "عيد ثورة ٢٣ يوليو"),
      (NaiveDate::from_ymd_res(2004, 10, 6)?, "عيد القوات المسلحة"),
      (NaiveDate::from_ymd_res(2004, 2, 21)?, "رأس السنة الهجرية (تقديري)"),
      (NaiveDate::from_ymd_res(2004, 11, 14)?, "عيد الفطر المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2004, 11, 15)?, "عيد الفطر المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2004, 1, 31)?, "وقفة عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2004, 2, 1)?, "عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2004, 2, 2)?, "عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2004, 2, 3)?, "عيد الأضحى المبارك (تقديري)"),
    ],
    &mut map,
    Country::EG,
    "Egypt",
  );

  build_year(
    years,
    2005,
    vec![

      (NaiveDate::from_ymd_res(2005, 1, 7)?, "عيد الميلاد المجيد"),
      (NaiveDate::from_ymd_res(2005, 4, 25)?, "عيد تحرير سيناء"),
      (NaiveDate::from_ymd_res(2005, 5, 2)?, "عيد شم النسيم"),
      (NaiveDate::from_ymd_res(2005, 5, 1)?, "عيد العمال"),
      (NaiveDate::from_ymd_res(2005, 7, 23)?, "عيد ثورة ٢٣ يوليو"),
      (NaiveDate::from_ymd_res(2005, 10, 6)?, "عيد القوات المسلحة"),
      (NaiveDate::from_ymd_res(2005, 2, 10)?, "رأس السنة الهجرية (تقديري)"),
      (NaiveDate::from_ymd_res(2005, 4, 21)?, "المولد النبوي الشريف (تقديري)"),
      (NaiveDate::from_ymd_res(2005, 11, 3)?, "عيد الفطر المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2005, 11, 4)?, "عيد الفطر المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2005, 1, 20)?, "وقفة عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2005, 1, 21)?, "عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2005, 1, 22)?, "عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2005, 1, 23)?, "عيد الأضحى المبارك (تقديري)"),
    ],
    &mut map,
    Country::EG,
    "Egypt",
  );

  build_year(
    years,
    2006,
    vec![

      (NaiveDate::from_ymd_res(2006, 1, 7)?, "عيد الميلاد المجيد"),
      (NaiveDate::from_ymd_res(2006, 4, 25)?, "عيد تحرير سيناء"),
      (NaiveDate::from_ymd_res(2006, 4, 24)?, "عيد شم النسيم"),
      (NaiveDate::from_ymd_res(2006, 5, 1)?, "عيد العمال"),
      (NaiveDate::from_ymd_res(2006, 7, 23)?, "عيد ثورة ٢٣ يوليو"),
      (NaiveDate::from_ymd_res(2006, 10, 6)?, "عيد القوات المسلحة"),
      (NaiveDate::from_ymd_res(2006, 1, 31)?, "رأس السنة الهجرية (تقديري)"),
      (NaiveDate::from_ymd_res(2006, 4, 10)?, "المولد النبوي الشريف (تقديري)"),
      (NaiveDate::from_ymd_res(2006, 10, 23)?, "عيد الفطر المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2006, 10, 24)?, "عيد الفطر المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2006, 1, 9)?, "وقفة عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2006, 12, 30)?, "وقفة عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2006, 1, 10)?, "عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2006, 12, 31)?, "عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2006, 1, 11)?, "عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2006, 1, 12)?, "عيد الأضحى المبارك (تقديري)"),
    ],
    &mut map,
    Country::EG,
    "Egypt",
  );

  build_year(
    years,
    2007,
    vec![

      (NaiveDate::from_ymd_res(2007, 1, 7)?, "عيد الميلاد المجيد"),
      (NaiveDate::from_ymd_res(2007, 4, 25)?, "عيد تحرير سيناء"),
      (NaiveDate::from_ymd_res(2007, 4, 9)?, "عيد شم النسيم"),
      (NaiveDate::from_ymd_res(2007, 5, 1)?, "عيد العمال"),
      (NaiveDate::from_ymd_res(2007, 7, 23)?, "عيد ثورة ٢٣ يوليو"),
      (NaiveDate::from_ymd_res(2007, 10, 6)?, "عيد القوات المسلحة"),
      (NaiveDate::from_ymd_res(2007, 1, 20)?, "رأس السنة الهجرية (تقديري)"),
      (NaiveDate::from_ymd_res(2007, 3, 31)?, "المولد النبوي الشريف (تقديري)"),
      (NaiveDate::from_ymd_res(2007, 10, 13)?, "عيد الفطر المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2007, 10, 14)?, "عيد الفطر المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2007, 12, 19)?, "وقفة عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2007, 12, 20)?, "عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2007, 1, 1)?, "عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2007, 12, 21)?, "عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2007, 1, 2)?, "عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2007, 12, 22)?, "عيد الأضحى المبارك (تقديري)"),
    ],
    &mut map,
    Country::EG,
    "Egypt",
  );

  build_year(
    years,
    2008,
    vec![

      (NaiveDate::from_ymd_res(2008, 1, 7)?, "عيد الميلاد المجيد"),
      (NaiveDate::from_ymd_res(2008, 4, 25)?, "عيد تحرير سيناء"),
      (NaiveDate::from_ymd_res(2008, 4, 28)?, "عيد شم النسيم"),
      (NaiveDate::from_ymd_res(2008, 5, 1)?, "عيد العمال"),
      (NaiveDate::from_ymd_res(2008, 7, 23)?, "عيد ثورة ٢٣ يوليو"),
      (NaiveDate::from_ymd_res(2008, 10, 6)?, "عيد القوات المسلحة"),
      (NaiveDate::from_ymd_res(2008, 1, 10)?, "رأس السنة الهجرية (تقديري)"),
      (NaiveDate::from_ymd_res(2008, 12, 29)?, "رأس السنة الهجرية (تقديري)"),
      (NaiveDate::from_ymd_res(2008, 3, 20)?, "المولد النبوي الشريف (تقديري)"),
      (NaiveDate::from_ymd_res(2008, 10, 1)?, "عيد الفطر المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2008, 10, 2)?, "عيد الفطر المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2008, 12, 7)?, "وقفة عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2008, 12, 8)?, "عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2008, 12, 9)?, "عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2008, 12, 10)?, "عيد الأضحى المبارك (تقديري)"),
    ],
    &mut map,
    Country::EG,
    "Egypt",
  );

  build_year(
    years,
    2009,
    vec![

      (NaiveDate::from_ymd_res(2009, 1, 7)?, "عيد الميلاد المجيد"),
      (NaiveDate::from_ymd_res(2009, 1, 25)?, "عيد الشرطة"),
      (NaiveDate::from_ymd_res(2009, 4, 25)?, "عيد تحرير سيناء"),
      (NaiveDate::from_ymd_res(2009, 4, 20)?, "عيد شم النسيم"),
      (NaiveDate::from_ymd_res(2009, 5, 1)?, "عيد العمال"),
      (NaiveDate::from_ymd_res(2009, 7, 23)?, "عيد ثورة ٢٣ يوليو"),
      (NaiveDate::from_ymd_res(2009, 10, 6)?, "عيد القوات المسلحة"),
      (NaiveDate::from_ymd_res(2009, 12, 18)?, "رأس السنة الهجرية (تقديري)"),
      (NaiveDate::from_ymd_res(2009, 3, 9)?, "المولد النبوي الشريف (تقديري)"),
      (NaiveDate::from_ymd_res(2009, 9, 20)?, "عيد الفطر المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2009, 9, 21)?, "عيد الفطر المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2009, 11, 26)?, "وقفة عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2009, 11, 27)?, "عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2009, 11, 28)?, "عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2009, 11, 29)?, "عيد الأضحى المبارك (تقديري)"),
    ],
    &mut map,
    Country::EG,
    "Egypt",
  );

  build_year(
    years,
    2010,
    vec![

      (NaiveDate::from_ymd_res(2010, 1, 7)?, "عيد الميلاد المجيد"),
      (NaiveDate::from_ymd_res(2010, 1, 25)?, "عيد الشرطة"),
      (NaiveDate::from_ymd_res(2010, 4, 25)?, "عيد تحرير سيناء"),
      (NaiveDate::from_ymd_res(2010, 4, 5)?, "عيد شم النسيم"),
      (NaiveDate::from_ymd_res(2010, 5, 1)?, "عيد العمال"),
      (NaiveDate::from_ymd_res(2010, 7, 23)?, "عيد ثورة ٢٣ يوليو"),
      (NaiveDate::from_ymd_res(2010, 10, 6)?, "عيد القوات المسلحة"),
      (NaiveDate::from_ymd_res(2010, 12, 7)?, "رأس السنة الهجرية (تقديري)"),
      (NaiveDate::from_ymd_res(2010, 2, 26)?, "المولد النبوي الشريف (تقديري)"),
      (NaiveDate::from_ymd_res(2010, 9, 10)?, "عيد الفطر المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2010, 9, 11)?, "عيد الفطر المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2010, 11, 15)?, "وقفة عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2010, 11, 16)?, "عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2010, 11, 17)?, "عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2010, 11, 18)?, "عيد الأضحى المبارك (تقديري)"),
    ],
    &mut map,
    Country::EG,
    "Egypt",
  );

  build_year(
    years,
    2011,
    vec![

      (NaiveDate::from_ymd_res(2011, 1, 7)?, "عيد الميلاد المجيد"),
      (NaiveDate::from_ymd_res(2011, 1, 25)?, "عيد الشرطة"),
      (NaiveDate::from_ymd_res(2011, 4, 25)?, "عيد تحرير سيناء; عيد شم النسيم"),
      (NaiveDate::from_ymd_res(2011, 5, 1)?, "عيد العمال"),
      (NaiveDate::from_ymd_res(2011, 7, 23)?, "عيد ثورة ٢٣ يوليو"),
      (NaiveDate::from_ymd_res(2011, 10, 6)?, "عيد القوات المسلحة"),
      (NaiveDate::from_ymd_res(2011, 11, 26)?, "رأس السنة الهجرية (تقديري)"),
      (NaiveDate::from_ymd_res(2011, 2, 15)?, "المولد النبوي الشريف (تقديري)"),
      (NaiveDate::from_ymd_res(2011, 8, 30)?, "عيد الفطر المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2011, 8, 31)?, "عيد الفطر المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2011, 11, 5)?, "وقفة عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2011, 11, 6)?, "عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2011, 11, 7)?, "عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2011, 11, 8)?, "عيد الأضحى المبارك (تقديري)"),
    ],
    &mut map,
    Country::EG,
    "Egypt",
  );

  build_year(
    years,
    2012,
    vec![

      (NaiveDate::from_ymd_res(2012, 1, 7)?, "عيد الميلاد المجيد"),
      (NaiveDate::from_ymd_res(2012, 1, 25)?, "ثورة ٢٥ يناير وعيد الشرطة"),
      (NaiveDate::from_ymd_res(2012, 4, 25)?, "عيد تحرير سيناء"),
      (NaiveDate::from_ymd_res(2012, 4, 16)?, "عيد شم النسيم"),
      (NaiveDate::from_ymd_res(2012, 5, 1)?, "عيد العمال"),
      (NaiveDate::from_ymd_res(2012, 7, 23)?, "عيد ثورة ٢٣ يوليو"),
      (NaiveDate::from_ymd_res(2012, 10, 6)?, "عيد القوات المسلحة"),
      (NaiveDate::from_ymd_res(2012, 11, 15)?, "رأس السنة الهجرية (تقديري)"),
      (NaiveDate::from_ymd_res(2012, 2, 4)?, "المولد النبوي الشريف (تقديري)"),
      (NaiveDate::from_ymd_res(2012, 8, 19)?, "عيد الفطر المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2012, 8, 20)?, "عيد الفطر المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2012, 10, 25)?, "وقفة عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2012, 10, 26)?, "عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2012, 10, 27)?, "عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2012, 10, 28)?, "عيد الأضحى المبارك (تقديري)"),
    ],
    &mut map,
    Country::EG,
    "Egypt",
  );

  build_year(
    years,
    2013,
    vec![

      (NaiveDate::from_ymd_res(2013, 1, 7)?, "عيد الميلاد المجيد"),
      (NaiveDate::from_ymd_res(2013, 1, 25)?, "ثورة ٢٥ يناير وعيد الشرطة"),
      (NaiveDate::from_ymd_res(2013, 4, 25)?, "عيد تحرير سيناء"),
      (NaiveDate::from_ymd_res(2013, 5, 6)?, "عيد شم النسيم"),
      (NaiveDate::from_ymd_res(2013, 5, 1)?, "عيد العمال"),
      (NaiveDate::from_ymd_res(2013, 7, 23)?, "عيد ثورة ٢٣ يوليو"),
      (NaiveDate::from_ymd_res(2013, 10, 6)?, "عيد القوات المسلحة"),
      (NaiveDate::from_ymd_res(2013, 11, 4)?, "رأس السنة الهجرية (تقديري)"),
      (NaiveDate::from_ymd_res(2013, 1, 24)?, "المولد النبوي الشريف (تقديري)"),
      (NaiveDate::from_ymd_res(2013, 8, 8)?, "عيد الفطر المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2013, 8, 9)?, "عيد الفطر المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2013, 10, 14)?, "وقفة عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2013, 10, 15)?, "عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2013, 10, 16)?, "عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2013, 10, 17)?, "عيد الأضحى المبارك (تقديري)"),
    ],
    &mut map,
    Country::EG,
    "Egypt",
  );

  build_year(
    years,
    2014,
    vec![

      (NaiveDate::from_ymd_res(2014, 1, 7)?, "عيد الميلاد المجيد"),
      (NaiveDate::from_ymd_res(2014, 1, 25)?, "ثورة ٢٥ يناير وعيد الشرطة"),
      (NaiveDate::from_ymd_res(2014, 4, 25)?, "عيد تحرير سيناء"),
      (NaiveDate::from_ymd_res(2014, 4, 21)?, "عيد شم النسيم"),
      (NaiveDate::from_ymd_res(2014, 5, 1)?, "عيد العمال"),
      (NaiveDate::from_ymd_res(2014, 6, 30)?, "عيد ثورة ٣٠ يونيو"),
      (NaiveDate::from_ymd_res(2014, 7, 23)?, "عيد ثورة ٢٣ يوليو"),
      (NaiveDate::from_ymd_res(2014, 10, 6)?, "عيد الأضحى المبارك (تقديري); عيد القوات المسلحة"),
      (NaiveDate::from_ymd_res(2014, 10, 25)?, "رأس السنة الهجرية (تقديري)"),
      (NaiveDate::from_ymd_res(2014, 1, 13)?, "المولد النبوي الشريف (تقديري)"),
      (NaiveDate::from_ymd_res(2014, 7, 28)?, "عيد الفطر المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2014, 7, 29)?, "عيد الفطر المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2014, 10, 3)?, "وقفة عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2014, 10, 4)?, "عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2014, 10, 5)?, "عيد الأضحى المبارك (تقديري)"),
    ],
    &mut map,
    Country::EG,
    "Egypt",
  );

  build_year(
    years,
    2015,
    vec![

      (NaiveDate::from_ymd_res(2015, 1, 7)?, "عيد الميلاد المجيد"),
      (NaiveDate::from_ymd_res(2015, 1, 25)?, "ثورة ٢٥ يناير وعيد الشرطة"),
      (NaiveDate::from_ymd_res(2015, 4, 25)?, "عيد تحرير سيناء"),
      (NaiveDate::from_ymd_res(2015, 4, 13)?, "عيد شم النسيم"),
      (NaiveDate::from_ymd_res(2015, 5, 1)?, "عيد العمال"),
      (NaiveDate::from_ymd_res(2015, 6, 30)?, "عيد ثورة ٣٠ يونيو"),
      (NaiveDate::from_ymd_res(2015, 7, 23)?, "عيد ثورة ٢٣ يوليو"),
      (NaiveDate::from_ymd_res(2015, 10, 6)?, "عيد القوات المسلحة"),
      (NaiveDate::from_ymd_res(2015, 10, 14)?, "رأس السنة الهجرية (تقديري)"),
      (NaiveDate::from_ymd_res(2015, 1, 3)?, "المولد النبوي الشريف (تقديري)"),
      (NaiveDate::from_ymd_res(2015, 12, 23)?, "المولد النبوي الشريف (تقديري)"),
      (NaiveDate::from_ymd_res(2015, 7, 17)?, "عيد الفطر المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2015, 7, 18)?, "عيد الفطر المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2015, 9, 22)?, "وقفة عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2015, 9, 23)?, "عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2015, 9, 24)?, "عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2015, 9, 25)?, "عيد الأضحى المبارك (تقديري)"),
    ],
    &mut map,
    Country::EG,
    "Egypt",
  );

  build_year(
    years,
    2016,
    vec![

      (NaiveDate::from_ymd_res(2016, 1, 7)?, "عيد الميلاد المجيد"),
      (NaiveDate::from_ymd_res(2016, 1, 25)?, "ثورة ٢٥ يناير وعيد الشرطة"),
      (NaiveDate::from_ymd_res(2016, 4, 25)?, "عيد تحرير سيناء"),
      (NaiveDate::from_ymd_res(2016, 5, 2)?, "عيد شم النسيم"),
      (NaiveDate::from_ymd_res(2016, 5, 1)?, "عيد العمال"),
      (NaiveDate::from_ymd_res(2016, 6, 30)?, "عيد ثورة ٣٠ يونيو"),
      (NaiveDate::from_ymd_res(2016, 7, 23)?, "عيد ثورة ٢٣ يوليو"),
      (NaiveDate::from_ymd_res(2016, 10, 6)?, "عيد القوات المسلحة"),
      (NaiveDate::from_ymd_res(2016, 10, 2)?, "رأس السنة الهجرية (تقديري)"),
      (NaiveDate::from_ymd_res(2016, 12, 11)?, "المولد النبوي الشريف (تقديري)"),
      (NaiveDate::from_ymd_res(2016, 7, 6)?, "عيد الفطر المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2016, 7, 7)?, "عيد الفطر المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2016, 9, 10)?, "وقفة عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2016, 9, 11)?, "عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2016, 9, 12)?, "عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2016, 9, 13)?, "عيد الأضحى المبارك (تقديري)"),
    ],
    &mut map,
    Country::EG,
    "Egypt",
  );

  build_year(
    years,
    2017,
    vec![

      (NaiveDate::from_ymd_res(2017, 1, 7)?, "عيد الميلاد المجيد"),
      (NaiveDate::from_ymd_res(2017, 1, 25)?, "ثورة ٢٥ يناير وعيد الشرطة"),
      (NaiveDate::from_ymd_res(2017, 4, 25)?, "عيد تحرير سيناء"),
      (NaiveDate::from_ymd_res(2017, 4, 17)?, "عيد شم النسيم"),
      (NaiveDate::from_ymd_res(2017, 5, 1)?, "عيد العمال"),
      (NaiveDate::from_ymd_res(2017, 6, 30)?, "عيد ثورة ٣٠ يونيو"),
      (NaiveDate::from_ymd_res(2017, 7, 23)?, "عيد ثورة ٢٣ يوليو"),
      (NaiveDate::from_ymd_res(2017, 10, 6)?, "عيد القوات المسلحة"),
      (NaiveDate::from_ymd_res(2017, 9, 21)?, "رأس السنة الهجرية (تقديري)"),
      (NaiveDate::from_ymd_res(2017, 11, 30)?, "المولد النبوي الشريف (تقديري)"),
      (NaiveDate::from_ymd_res(2017, 6, 25)?, "عيد الفطر المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2017, 6, 26)?, "عيد الفطر المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2017, 8, 31)?, "وقفة عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2017, 9, 1)?, "عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2017, 9, 2)?, "عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2017, 9, 3)?, "عيد الأضحى المبارك (تقديري)"),
    ],
    &mut map,
    Country::EG,
    "Egypt",
  );

  build_year(
    years,
    2018,
    vec![

      (NaiveDate::from_ymd_res(2018, 1, 7)?, "عيد الميلاد المجيد"),
      (NaiveDate::from_ymd_res(2018, 1, 25)?, "ثورة ٢٥ يناير وعيد الشرطة"),
      (NaiveDate::from_ymd_res(2018, 4, 25)?, "عيد تحرير سيناء"),
      (NaiveDate::from_ymd_res(2018, 4, 9)?, "عيد شم النسيم"),
      (NaiveDate::from_ymd_res(2018, 5, 1)?, "عيد العمال"),
      (NaiveDate::from_ymd_res(2018, 6, 30)?, "عيد ثورة ٣٠ يونيو"),
      (NaiveDate::from_ymd_res(2018, 7, 23)?, "عيد ثورة ٢٣ يوليو"),
      (NaiveDate::from_ymd_res(2018, 10, 6)?, "عيد القوات المسلحة"),
      (NaiveDate::from_ymd_res(2018, 9, 11)?, "رأس السنة الهجرية (تقديري)"),
      (NaiveDate::from_ymd_res(2018, 11, 20)?, "المولد النبوي الشريف (تقديري)"),
      (NaiveDate::from_ymd_res(2018, 6, 15)?, "عيد الفطر المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2018, 6, 16)?, "عيد الفطر المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2018, 8, 20)?, "وقفة عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2018, 8, 21)?, "عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2018, 8, 22)?, "عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2018, 8, 23)?, "عيد الأضحى المبارك (تقديري)"),
    ],
    &mut map,
    Country::EG,
    "Egypt",
  );

  build_year(
    years,
    2019,
    vec![

      (NaiveDate::from_ymd_res(2019, 1, 7)?, "عيد الميلاد المجيد"),
      (NaiveDate::from_ymd_res(2019, 1, 25)?, "ثورة ٢٥ يناير وعيد الشرطة"),
      (NaiveDate::from_ymd_res(2019, 4, 25)?, "عيد تحرير سيناء"),
      (NaiveDate::from_ymd_res(2019, 4, 29)?, "عيد شم النسيم"),
      (NaiveDate::from_ymd_res(2019, 5, 1)?, "عيد العمال"),
      (NaiveDate::from_ymd_res(2019, 6, 30)?, "عيد ثورة ٣٠ يونيو"),
      (NaiveDate::from_ymd_res(2019, 7, 23)?, "عيد ثورة ٢٣ يوليو"),
      (NaiveDate::from_ymd_res(2019, 10, 6)?, "عيد القوات المسلحة"),
      (NaiveDate::from_ymd_res(2019, 8, 31)?, "رأس السنة الهجرية (تقديري)"),
      (NaiveDate::from_ymd_res(2019, 11, 9)?, "المولد النبوي الشريف (تقديري)"),
      (NaiveDate::from_ymd_res(2019, 6, 4)?, "عيد الفطر المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2019, 6, 5)?, "عيد الفطر المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2019, 8, 10)?, "وقفة عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2019, 8, 11)?, "عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2019, 8, 12)?, "عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2019, 8, 13)?, "عيد الأضحى المبارك (تقديري)"),
    ],
    &mut map,
    Country::EG,
    "Egypt",
  );

  build_year(
    years,
    2020,
    vec![

      (NaiveDate::from_ymd_res(2020, 1, 7)?, "عيد الميلاد المجيد"),
      (NaiveDate::from_ymd_res(2020, 1, 25)?, "ثورة ٢٥ يناير وعيد الشرطة"),
      (NaiveDate::from_ymd_res(2020, 4, 25)?, "عيد تحرير سيناء"),
      (NaiveDate::from_ymd_res(2020, 4, 20)?, "عيد شم النسيم"),
      (NaiveDate::from_ymd_res(2020, 5, 1)?, "عيد العمال"),
      (NaiveDate::from_ymd_res(2020, 7, 2)?, "عيد ثورة ٣٠ يونيو (يوم تعويضي)"),
      (NaiveDate::from_ymd_res(2020, 7, 23)?, "عيد ثورة ٢٣ يوليو"),
      (NaiveDate::from_ymd_res(2020, 10, 8)?, "عيد القوات المسلحة (يوم تعويضي)"),
      (NaiveDate::from_ymd_res(2020, 8, 20)?, "رأس السنة الهجرية"),
      (NaiveDate::from_ymd_res(2020, 10, 29)?, "المولد النبوي الشريف"),
      (NaiveDate::from_ymd_res(2020, 5, 24)?, "عيد الفطر المبارك"),
      (NaiveDate::from_ymd_res(2020, 5, 25)?, "عيد الفطر المبارك"),
      (NaiveDate::from_ymd_res(2020, 7, 30)?, "وقفة عيد الأضحى المبارك"),
      (NaiveDate::from_ymd_res(2020, 7, 31)?, "عيد الأضحى المبارك"),
      (NaiveDate::from_ymd_res(2020, 8, 1)?, "عيد الأضحى المبارك"),
      (NaiveDate::from_ymd_res(2020, 8, 2)?, "عيد الأضحى المبارك"),
    ],
    &mut map,
    Country::EG,
    "Egypt",
  );

  build_year(
    years,
    2021,
    vec![

      (NaiveDate::from_ymd_res(2021, 1, 7)?, "عيد الميلاد المجيد"),
      (NaiveDate::from_ymd_res(2021, 1, 28)?, "ثورة ٢٥ يناير وعيد الشرطة (يوم تعويضي)"),
      (NaiveDate::from_ymd_res(2021, 4, 29)?, "عيد تحرير سيناء (يوم تعويضي)"),
      (NaiveDate::from_ymd_res(2021, 5, 3)?, "عيد شم النسيم"),
      (NaiveDate::from_ymd_res(2021, 5, 1)?, "عيد العمال"),
      (NaiveDate::from_ymd_res(2021, 7, 1)?, "عيد ثورة ٣٠ يونيو (يوم تعويضي)"),
      (NaiveDate::from_ymd_res(2021, 7, 23)?, "عيد ثورة ٢٣ يوليو"),
      (NaiveDate::from_ymd_res(2021, 10, 7)?, "عيد القوات المسلحة (يوم تعويضي)"),
      (NaiveDate::from_ymd_res(2021, 8, 12)?, "رأس السنة الهجرية (يوم تعويضي)"),
      (NaiveDate::from_ymd_res(2021, 10, 18)?, "المولد النبوي الشريف"),
      (NaiveDate::from_ymd_res(2021, 5, 13)?, "عيد الفطر المبارك"),
      (NaiveDate::from_ymd_res(2021, 5, 14)?, "عيد الفطر المبارك"),
      (NaiveDate::from_ymd_res(2021, 7, 19)?, "وقفة عيد الأضحى المبارك"),
      (NaiveDate::from_ymd_res(2021, 7, 20)?, "عيد الأضحى المبارك"),
      (NaiveDate::from_ymd_res(2021, 7, 21)?, "عيد الأضحى المبارك"),
      (NaiveDate::from_ymd_res(2021, 7, 22)?, "عيد الأضحى المبارك"),
      (NaiveDate::from_ymd_res(2021, 7, 17)?, "عيد الأضحى المبارك"),
      (NaiveDate::from_ymd_res(2021, 7, 18)?, "عيد الأضحى المبارك"),
    ],
    &mut map,
    Country::EG,
    "Egypt",
  );

  build_year(
    years,
    2022,
    vec![

      (NaiveDate::from_ymd_res(2022, 1, 27)?, "ثورة ٢٥ يناير وعيد الشرطة (يوم تعويضي)"),
      (NaiveDate::from_ymd_res(2022, 4, 25)?, "عيد تحرير سيناء; عيد شم النسيم"),
      (NaiveDate::from_ymd_res(2022, 5, 5)?, "عيد العمال (يوم تعويضي)"),
      (NaiveDate::from_ymd_res(2022, 6, 30)?, "عيد ثورة ٣٠ يونيو"),
      (NaiveDate::from_ymd_res(2022, 7, 23)?, "عيد ثورة ٢٣ يوليو"),
      (NaiveDate::from_ymd_res(2022, 10, 6)?, "عيد القوات المسلحة"),
      (NaiveDate::from_ymd_res(2022, 7, 30)?, "رأس السنة الهجرية"),
      (NaiveDate::from_ymd_res(2022, 10, 8)?, "المولد النبوي الشريف"),
      (NaiveDate::from_ymd_res(2022, 5, 1)?, "عيد الفطر المبارك"),
      (NaiveDate::from_ymd_res(2022, 5, 2)?, "عيد الفطر المبارك"),
      (NaiveDate::from_ymd_res(2022, 7, 9)?, "وقفة عيد الأضحى المبارك"),
      (NaiveDate::from_ymd_res(2022, 7, 10)?, "عيد الأضحى المبارك"),
      (NaiveDate::from_ymd_res(2022, 7, 11)?, "عيد الأضحى المبارك"),
      (NaiveDate::from_ymd_res(2022, 7, 12)?, "عيد الأضحى المبارك"),
      (NaiveDate::from_ymd_res(2022, 7, 13)?, "عيد الأضحى المبارك"),
      (NaiveDate::from_ymd_res(2022, 7, 14)?, "عيد الأضحى المبارك"),
      (NaiveDate::from_ymd_res(2022, 1, 6)?, "عيد الميلاد المجيد (يوم تعويضي)"),
      (NaiveDate::from_ymd_res(2022, 5, 4)?, "عيد الفطر المبارك (يوم تعويضي)"),
    ],
    &mut map,
    Country::EG,
    "Egypt",
  );

  build_year(
    years,
    2023,
    vec![

      (NaiveDate::from_ymd_res(2023, 1, 26)?, "ثورة ٢٥ يناير وعيد الشرطة (يوم تعويضي)"),
      (NaiveDate::from_ymd_res(2023, 4, 27)?, "عيد تحرير سيناء (يوم تعويضي)"),
      (NaiveDate::from_ymd_res(2023, 4, 17)?, "عيد شم النسيم"),
      (NaiveDate::from_ymd_res(2023, 5, 4)?, "عيد العمال (يوم تعويضي)"),
      (NaiveDate::from_ymd_res(2023, 6, 30)?, "عيد الأضحى المبارك; عيد ثورة ٣٠ يونيو"),
      (NaiveDate::from_ymd_res(2023, 7, 23)?, "عيد ثورة ٢٣ يوليو"),
      (NaiveDate::from_ymd_res(2023, 10, 6)?, "عيد القوات المسلحة"),
      (NaiveDate::from_ymd_res(2023, 7, 20)?, "رأس السنة الهجرية (يوم تعويضي)"),
      (NaiveDate::from_ymd_res(2023, 9, 28)?, "المولد النبوي الشريف"),
      (NaiveDate::from_ymd_res(2023, 4, 21)?, "عيد الفطر المبارك"),
      (NaiveDate::from_ymd_res(2023, 4, 22)?, "عيد الفطر المبارك"),
      (NaiveDate::from_ymd_res(2023, 6, 27)?, "وقفة عيد الأضحى المبارك"),
      (NaiveDate::from_ymd_res(2023, 6, 28)?, "عيد الأضحى المبارك"),
      (NaiveDate::from_ymd_res(2023, 6, 29)?, "عيد الأضحى المبارك"),
      (NaiveDate::from_ymd_res(2023, 1, 8)?, "عيد الميلاد المجيد (يوم تعويضي)"),
      (NaiveDate::from_ymd_res(2023, 4, 24)?, "عيد الفطر المبارك (يوم تعويضي)"),
      (NaiveDate::from_ymd_res(2023, 7, 2)?, "عيد ثورة ٣٠ يونيو (يوم تعويضي)"),
      (NaiveDate::from_ymd_res(2023, 7, 3)?, "عيد الأضحى المبارك (يوم تعويضي)"),
    ],
    &mut map,
    Country::EG,
    "Egypt",
  );

  build_year(
    years,
    2024,
    vec![

      (NaiveDate::from_ymd_res(2024, 1, 7)?, "عيد الميلاد المجيد"),
      (NaiveDate::from_ymd_res(2024, 1, 25)?, "ثورة ٢٥ يناير وعيد الشرطة"),
      (NaiveDate::from_ymd_res(2024, 4, 25)?, "عيد تحرير سيناء"),
      (NaiveDate::from_ymd_res(2024, 5, 6)?, "عيد شم النسيم"),
      (NaiveDate::from_ymd_res(2024, 6, 30)?, "عيد ثورة ٣٠ يونيو"),
      (NaiveDate::from_ymd_res(2024, 7, 25)?, "عيد ثورة ٢٣ يوليو (يوم تعويضي)"),
      (NaiveDate::from_ymd_res(2024, 10, 6)?, "عيد القوات المسلحة"),
      (NaiveDate::from_ymd_res(2024, 7, 11)?, "رأس السنة الهجرية (يوم تعويضي)"),
      (NaiveDate::from_ymd_res(2024, 9, 15)?, "المولد النبوي الشريف"),
      (NaiveDate::from_ymd_res(2024, 4, 10)?, "عيد الفطر المبارك"),
      (NaiveDate::from_ymd_res(2024, 4, 11)?, "عيد الفطر المبارك"),
      (NaiveDate::from_ymd_res(2024, 6, 15)?, "وقفة عيد الأضحى المبارك"),
      (NaiveDate::from_ymd_res(2024, 6, 16)?, "عيد الأضحى المبارك"),
      (NaiveDate::from_ymd_res(2024, 6, 17)?, "عيد الأضحى المبارك"),
      (NaiveDate::from_ymd_res(2024, 6, 18)?, "عيد الأضحى المبارك"),
      (NaiveDate::from_ymd_res(2024, 5, 5)?, "عيد العمال (يوم تعويضي)"),
    ],
    &mut map,
    Country::EG,
    "Egypt",
  );

  build_year(
    years,
    2025,
    vec![

      (NaiveDate::from_ymd_res(2025, 1, 7)?, "عيد الميلاد المجيد"),
      (NaiveDate::from_ymd_res(2025, 1, 25)?, "ثورة ٢٥ يناير وعيد الشرطة"),
      (NaiveDate::from_ymd_res(2025, 4, 25)?, "عيد تحرير سيناء"),
      (NaiveDate::from_ymd_res(2025, 4, 21)?, "عيد شم النسيم"),
      (NaiveDate::from_ymd_res(2025, 5, 1)?, "عيد العمال"),
      (NaiveDate::from_ymd_res(2025, 7, 3)?, "عيد ثورة ٣٠ يونيو (يوم تعويضي)"),
      (NaiveDate::from_ymd_res(2025, 7, 24)?, "عيد ثورة ٢٣ يوليو (يوم تعويضي)"),
      (NaiveDate::from_ymd_res(2025, 10, 9)?, "عيد القوات المسلحة (يوم تعويضي)"),
      (NaiveDate::from_ymd_res(2025, 6, 26)?, "رأس السنة الهجرية"),
      (NaiveDate::from_ymd_res(2025, 9, 4)?, "المولد النبوي الشريف"),
      (NaiveDate::from_ymd_res(2025, 3, 30)?, "عيد الفطر المبارك"),
      (NaiveDate::from_ymd_res(2025, 3, 31)?, "عيد الفطر المبارك"),
      (NaiveDate::from_ymd_res(2025, 6, 5)?, "وقفة عيد الأضحى المبارك"),
      (NaiveDate::from_ymd_res(2025, 6, 6)?, "عيد الأضحى المبارك"),
      (NaiveDate::from_ymd_res(2025, 6, 7)?, "عيد الأضحى المبارك"),
      (NaiveDate::from_ymd_res(2025, 6, 8)?, "عيد الأضحى المبارك"),
    ],
    &mut map,
    Country::EG,
    "Egypt",
  );

  build_year(
    years,
    2026,
    vec![

      (NaiveDate::from_ymd_res(2026, 1, 7)?, "عيد الميلاد المجيد"),
      (NaiveDate::from_ymd_res(2026, 1, 29)?, "ثورة ٢٥ يناير وعيد الشرطة (يوم تعويضي)"),
      (NaiveDate::from_ymd_res(2026, 4, 25)?, "عيد تحرير سيناء"),
      (NaiveDate::from_ymd_res(2026, 4, 13)?, "عيد شم النسيم"),
      (NaiveDate::from_ymd_res(2026, 5, 1)?, "عيد العمال"),
      (NaiveDate::from_ymd_res(2026, 7, 2)?, "عيد ثورة ٣٠ يونيو (يوم تعويضي)"),
      (NaiveDate::from_ymd_res(2026, 7, 23)?, "عيد ثورة ٢٣ يوليو"),
      (NaiveDate::from_ymd_res(2026, 10, 8)?, "عيد القوات المسلحة (يوم تعويضي)"),
      (NaiveDate::from_ymd_res(2026, 6, 18)?, "رأس السنة الهجرية (يوم تعويضي تقديري)"),
      (NaiveDate::from_ymd_res(2026, 8, 25)?, "المولد النبوي الشريف (تقديري)"),
      (NaiveDate::from_ymd_res(2026, 3, 20)?, "عيد الفطر المبارك"),
      (NaiveDate::from_ymd_res(2026, 3, 21)?, "عيد الفطر المبارك"),
      (NaiveDate::from_ymd_res(2026, 5, 26)?, "وقفة عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2026, 5, 27)?, "عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2026, 5, 28)?, "عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2026, 5, 29)?, "عيد الأضحى المبارك (تقديري)"),
    ],
    &mut map,
    Country::EG,
    "Egypt",
  );

  build_year(
    years,
    2027,
    vec![

      (NaiveDate::from_ymd_res(2027, 1, 7)?, "عيد الميلاد المجيد"),
      (NaiveDate::from_ymd_res(2027, 1, 28)?, "ثورة ٢٥ يناير وعيد الشرطة (يوم تعويضي)"),
      (NaiveDate::from_ymd_res(2027, 4, 29)?, "عيد تحرير سيناء (يوم تعويضي)"),
      (NaiveDate::from_ymd_res(2027, 5, 3)?, "عيد شم النسيم"),
      (NaiveDate::from_ymd_res(2027, 5, 1)?, "عيد العمال"),
      (NaiveDate::from_ymd_res(2027, 7, 1)?, "عيد ثورة ٣٠ يونيو (يوم تعويضي)"),
      (NaiveDate::from_ymd_res(2027, 7, 23)?, "عيد ثورة ٢٣ يوليو"),
      (NaiveDate::from_ymd_res(2027, 10, 7)?, "عيد القوات المسلحة (يوم تعويضي)"),
      (NaiveDate::from_ymd_res(2027, 6, 10)?, "رأس السنة الهجرية (يوم تعويضي تقديري)"),
      (NaiveDate::from_ymd_res(2027, 8, 14)?, "المولد النبوي الشريف (تقديري)"),
      (NaiveDate::from_ymd_res(2027, 3, 9)?, "عيد الفطر المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2027, 3, 10)?, "عيد الفطر المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2027, 5, 15)?, "وقفة عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2027, 5, 16)?, "عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2027, 5, 17)?, "عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2027, 5, 18)?, "عيد الأضحى المبارك (تقديري)"),
    ],
    &mut map,
    Country::EG,
    "Egypt",
  );

  build_year(
    years,
    2028,
    vec![

      (NaiveDate::from_ymd_res(2028, 1, 7)?, "عيد الميلاد المجيد"),
      (NaiveDate::from_ymd_res(2028, 1, 27)?, "ثورة ٢٥ يناير وعيد الشرطة (يوم تعويضي)"),
      (NaiveDate::from_ymd_res(2028, 4, 27)?, "عيد تحرير سيناء (يوم تعويضي)"),
      (NaiveDate::from_ymd_res(2028, 4, 17)?, "عيد شم النسيم"),
      (NaiveDate::from_ymd_res(2028, 5, 4)?, "عيد العمال (يوم تعويضي); وقفة عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2028, 6, 30)?, "عيد ثورة ٣٠ يونيو"),
      (NaiveDate::from_ymd_res(2028, 7, 27)?, "عيد ثورة ٢٣ يوليو (يوم تعويضي)"),
      (NaiveDate::from_ymd_res(2028, 10, 6)?, "عيد القوات المسلحة"),
      (NaiveDate::from_ymd_res(2028, 5, 25)?, "رأس السنة الهجرية (تقديري)"),
      (NaiveDate::from_ymd_res(2028, 8, 3)?, "المولد النبوي الشريف (تقديري)"),
      (NaiveDate::from_ymd_res(2028, 2, 26)?, "عيد الفطر المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2028, 2, 27)?, "عيد الفطر المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2028, 5, 5)?, "عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2028, 5, 6)?, "عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2028, 5, 7)?, "عيد الأضحى المبارك (تقديري)"),
    ],
    &mut map,
    Country::EG,
    "Egypt",
  );

  build_year(
    years,
    2029,
    vec![

      (NaiveDate::from_ymd_res(2029, 1, 7)?, "عيد الميلاد المجيد"),
      (NaiveDate::from_ymd_res(2029, 1, 25)?, "ثورة ٢٥ يناير وعيد الشرطة"),
      (NaiveDate::from_ymd_res(2029, 4, 26)?, "عيد الأضحى المبارك (تقديري); عيد تحرير سيناء (يوم تعويضي)"),
      (NaiveDate::from_ymd_res(2029, 4, 9)?, "عيد شم النسيم"),
      (NaiveDate::from_ymd_res(2029, 5, 3)?, "عيد العمال (يوم تعويضي)"),
      (NaiveDate::from_ymd_res(2029, 6, 30)?, "عيد ثورة ٣٠ يونيو"),
      (NaiveDate::from_ymd_res(2029, 7, 26)?, "عيد ثورة ٢٣ يوليو (يوم تعويضي)"),
      (NaiveDate::from_ymd_res(2029, 10, 6)?, "عيد القوات المسلحة"),
      (NaiveDate::from_ymd_res(2029, 5, 17)?, "رأس السنة الهجرية (يوم تعويضي تقديري)"),
      (NaiveDate::from_ymd_res(2029, 7, 24)?, "المولد النبوي الشريف (تقديري)"),
      (NaiveDate::from_ymd_res(2029, 2, 14)?, "عيد الفطر المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2029, 2, 15)?, "عيد الفطر المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2029, 4, 23)?, "وقفة عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2029, 4, 24)?, "عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2029, 4, 25)?, "عيد الأضحى المبارك (تقديري)"),
    ],
    &mut map,
    Country::EG,
    "Egypt",
  );

  build_year(
    years,
    2030,
    vec![

      (NaiveDate::from_ymd_res(2030, 1, 7)?, "عيد الميلاد المجيد"),
      (NaiveDate::from_ymd_res(2030, 1, 25)?, "ثورة ٢٥ يناير وعيد الشرطة"),
      (NaiveDate::from_ymd_res(2030, 4, 25)?, "عيد تحرير سيناء"),
      (NaiveDate::from_ymd_res(2030, 4, 29)?, "عيد شم النسيم"),
      (NaiveDate::from_ymd_res(2030, 5, 2)?, "عيد العمال (يوم تعويضي)"),
      (NaiveDate::from_ymd_res(2030, 7, 4)?, "عيد ثورة ٣٠ يونيو (يوم تعويضي)"),
      (NaiveDate::from_ymd_res(2030, 7, 25)?, "عيد ثورة ٢٣ يوليو (يوم تعويضي)"),
      (NaiveDate::from_ymd_res(2030, 10, 10)?, "عيد القوات المسلحة (يوم تعويضي)"),
      (NaiveDate::from_ymd_res(2030, 5, 3)?, "رأس السنة الهجرية (تقديري)"),
      (NaiveDate::from_ymd_res(2030, 7, 13)?, "المولد النبوي الشريف (تقديري)"),
      (NaiveDate::from_ymd_res(2030, 2, 4)?, "عيد الفطر المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2030, 2, 5)?, "عيد الفطر المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2030, 4, 12)?, "وقفة عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2030, 4, 13)?, "عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2030, 4, 14)?, "عيد الأضحى المبارك (تقديري)"),
      (NaiveDate::from_ymd_res(2030, 4, 15)?, "عيد الأضحى المبارك (تقديري)"),
    ],
    &mut map,
    Country::EG,
    "Egypt",
  );

  Ok(map)
}