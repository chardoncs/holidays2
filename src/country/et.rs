//! Ethiopia
use super::*;

/// Generate holiday map for Ethiopia.
#[allow(unused_mut, unused_variables)]
pub fn build(
    years: &Option<&std::ops::Range<Year>>,
) -> Result<HashMap<Year, BTreeMap<NaiveDate, Holiday>>> {
    let mut map = HashMap::new();

    build_year(
        years,
        2001,
        vec![
            (NaiveDate::from_ymd_res(2001, 1, 7)?, "የገና ወይም የልደት በዓል"),
            (NaiveDate::from_ymd_res(2001, 1, 19)?, "የጥምቀት በዓል"),
            (NaiveDate::from_ymd_res(2001, 3, 2)?, "የአድዋ ድል በዓል"),
            (NaiveDate::from_ymd_res(2001, 4, 13)?, "የስቅለት በዓል"),
            (NaiveDate::from_ymd_res(2001, 4, 15)?, "የትንሳኤ(ፋሲካ) በዓል"),
            (
                NaiveDate::from_ymd_res(2001, 5, 1)?,
                "የዓለም የሠራተኞች (የላብአደሮች) ቀን",
            ),
            (NaiveDate::from_ymd_res(2001, 5, 5)?, "የአርበኞች (የድል) ቀን በዓል"),
            (NaiveDate::from_ymd_res(2001, 5, 28)?, "ደርግ የወደቀበት ቀን"),
            (
                NaiveDate::from_ymd_res(2001, 9, 11)?,
                "የዘመን መለወጫ (እንቁጣጣሽ) በዓል",
            ),
            (NaiveDate::from_ymd_res(2001, 9, 27)?, "የመስቀል በዓል"),
            (NaiveDate::from_ymd_res(2001, 12, 16)?, "የኢድ አልፈጥር (ግምት)"),
            (
                NaiveDate::from_ymd_res(2001, 3, 5)?,
                "የኢድ አልአድሃ (አረፋ) (ግምት)",
            ),
            (NaiveDate::from_ymd_res(2001, 6, 4)?, "የመውሊድ በዓል (ግምት)"),
        ],
        &mut map,
        Country::ET,
        "Ethiopia",
    );

    build_year(
        years,
        2002,
        vec![
            (NaiveDate::from_ymd_res(2002, 1, 7)?, "የገና ወይም የልደት በዓል"),
            (NaiveDate::from_ymd_res(2002, 1, 19)?, "የጥምቀት በዓል"),
            (NaiveDate::from_ymd_res(2002, 3, 2)?, "የአድዋ ድል በዓል"),
            (NaiveDate::from_ymd_res(2002, 5, 3)?, "የስቅለት በዓል"),
            (
                NaiveDate::from_ymd_res(2002, 5, 5)?,
                "የትንሳኤ(ፋሲካ) በዓል; የአርበኞች (የድል) ቀን በዓል",
            ),
            (
                NaiveDate::from_ymd_res(2002, 5, 1)?,
                "የዓለም የሠራተኞች (የላብአደሮች) ቀን",
            ),
            (NaiveDate::from_ymd_res(2002, 5, 28)?, "ደርግ የወደቀበት ቀን"),
            (
                NaiveDate::from_ymd_res(2002, 9, 11)?,
                "የዘመን መለወጫ (እንቁጣጣሽ) በዓል",
            ),
            (NaiveDate::from_ymd_res(2002, 9, 27)?, "የመስቀል በዓል"),
            (NaiveDate::from_ymd_res(2002, 12, 5)?, "የኢድ አልፈጥር (ግምት)"),
            (
                NaiveDate::from_ymd_res(2002, 2, 22)?,
                "የኢድ አልአድሃ (አረፋ) (ግምት)",
            ),
            (NaiveDate::from_ymd_res(2002, 5, 24)?, "የመውሊድ በዓል (ግምት)"),
        ],
        &mut map,
        Country::ET,
        "Ethiopia",
    );

    build_year(
        years,
        2003,
        vec![
            (NaiveDate::from_ymd_res(2003, 1, 7)?, "የገና ወይም የልደት በዓል"),
            (NaiveDate::from_ymd_res(2003, 1, 19)?, "የጥምቀት በዓል"),
            (NaiveDate::from_ymd_res(2003, 3, 2)?, "የአድዋ ድል በዓል"),
            (NaiveDate::from_ymd_res(2003, 4, 25)?, "የስቅለት በዓል"),
            (NaiveDate::from_ymd_res(2003, 4, 27)?, "የትንሳኤ(ፋሲካ) በዓል"),
            (
                NaiveDate::from_ymd_res(2003, 5, 1)?,
                "የዓለም የሠራተኞች (የላብአደሮች) ቀን",
            ),
            (NaiveDate::from_ymd_res(2003, 5, 5)?, "የአርበኞች (የድል) ቀን በዓል"),
            (NaiveDate::from_ymd_res(2003, 5, 28)?, "ደርግ የወደቀበት ቀን"),
            (
                NaiveDate::from_ymd_res(2003, 9, 12)?,
                "የዘመን መለወጫ (እንቁጣጣሽ) በዓል",
            ),
            (NaiveDate::from_ymd_res(2003, 9, 28)?, "የመስቀል በዓል"),
            (NaiveDate::from_ymd_res(2003, 11, 25)?, "የኢድ አልፈጥር (ግምት)"),
            (
                NaiveDate::from_ymd_res(2003, 2, 11)?,
                "የኢድ አልአድሃ (አረፋ) (ግምት)",
            ),
            (NaiveDate::from_ymd_res(2003, 5, 13)?, "የመውሊድ በዓል (ግምት)"),
        ],
        &mut map,
        Country::ET,
        "Ethiopia",
    );

    build_year(
        years,
        2004,
        vec![
            (NaiveDate::from_ymd_res(2004, 1, 7)?, "የገና ወይም የልደት በዓል"),
            (NaiveDate::from_ymd_res(2004, 1, 20)?, "የጥምቀት በዓል"),
            (NaiveDate::from_ymd_res(2004, 3, 2)?, "የአድዋ ድል በዓል"),
            (NaiveDate::from_ymd_res(2004, 4, 9)?, "የስቅለት በዓል"),
            (NaiveDate::from_ymd_res(2004, 4, 11)?, "የትንሳኤ(ፋሲካ) በዓል"),
            (
                NaiveDate::from_ymd_res(2004, 5, 1)?,
                "የመውሊድ በዓል (ግምት); የዓለም የሠራተኞች (የላብአደሮች) ቀን",
            ),
            (NaiveDate::from_ymd_res(2004, 5, 5)?, "የአርበኞች (የድል) ቀን በዓል"),
            (NaiveDate::from_ymd_res(2004, 5, 28)?, "ደርግ የወደቀበት ቀን"),
            (
                NaiveDate::from_ymd_res(2004, 9, 11)?,
                "የዘመን መለወጫ (እንቁጣጣሽ) በዓል",
            ),
            (NaiveDate::from_ymd_res(2004, 9, 27)?, "የመስቀል በዓል"),
            (NaiveDate::from_ymd_res(2004, 11, 14)?, "የኢድ አልፈጥር (ግምት)"),
            (
                NaiveDate::from_ymd_res(2004, 2, 1)?,
                "የኢድ አልአድሃ (አረፋ) (ግምት)",
            ),
        ],
        &mut map,
        Country::ET,
        "Ethiopia",
    );

    build_year(
        years,
        2005,
        vec![
            (NaiveDate::from_ymd_res(2005, 1, 7)?, "የገና ወይም የልደት በዓል"),
            (NaiveDate::from_ymd_res(2005, 1, 19)?, "የጥምቀት በዓል"),
            (NaiveDate::from_ymd_res(2005, 3, 2)?, "የአድዋ ድል በዓል"),
            (NaiveDate::from_ymd_res(2005, 4, 29)?, "የስቅለት በዓል"),
            (
                NaiveDate::from_ymd_res(2005, 5, 1)?,
                "የትንሳኤ(ፋሲካ) በዓል; የዓለም የሠራተኞች (የላብአደሮች) ቀን",
            ),
            (NaiveDate::from_ymd_res(2005, 5, 5)?, "የአርበኞች (የድል) ቀን በዓል"),
            (NaiveDate::from_ymd_res(2005, 5, 28)?, "ደርግ የወደቀበት ቀን"),
            (
                NaiveDate::from_ymd_res(2005, 9, 11)?,
                "የዘመን መለወጫ (እንቁጣጣሽ) በዓል",
            ),
            (NaiveDate::from_ymd_res(2005, 9, 27)?, "የመስቀል በዓል"),
            (NaiveDate::from_ymd_res(2005, 11, 3)?, "የኢድ አልፈጥር (ግምት)"),
            (
                NaiveDate::from_ymd_res(2005, 1, 21)?,
                "የኢድ አልአድሃ (አረፋ) (ግምት)",
            ),
            (NaiveDate::from_ymd_res(2005, 4, 21)?, "የመውሊድ በዓል (ግምት)"),
        ],
        &mut map,
        Country::ET,
        "Ethiopia",
    );

    build_year(
        years,
        2006,
        vec![
            (NaiveDate::from_ymd_res(2006, 1, 7)?, "የገና ወይም የልደት በዓል"),
            (NaiveDate::from_ymd_res(2006, 1, 19)?, "የጥምቀት በዓል"),
            (NaiveDate::from_ymd_res(2006, 3, 2)?, "የአድዋ ድል በዓል"),
            (NaiveDate::from_ymd_res(2006, 4, 21)?, "የስቅለት በዓል"),
            (NaiveDate::from_ymd_res(2006, 4, 23)?, "የትንሳኤ(ፋሲካ) በዓል"),
            (
                NaiveDate::from_ymd_res(2006, 5, 1)?,
                "የዓለም የሠራተኞች (የላብአደሮች) ቀን",
            ),
            (NaiveDate::from_ymd_res(2006, 5, 5)?, "የአርበኞች (የድል) ቀን በዓል"),
            (NaiveDate::from_ymd_res(2006, 5, 28)?, "ደርግ የወደቀበት ቀን"),
            (
                NaiveDate::from_ymd_res(2006, 9, 11)?,
                "የዘመን መለወጫ (እንቁጣጣሽ) በዓል",
            ),
            (NaiveDate::from_ymd_res(2006, 9, 27)?, "የመስቀል በዓል"),
            (NaiveDate::from_ymd_res(2006, 10, 23)?, "የኢድ አልፈጥር (ግምት)"),
            (
                NaiveDate::from_ymd_res(2006, 1, 10)?,
                "የኢድ አልአድሃ (አረፋ) (ግምት)",
            ),
            (
                NaiveDate::from_ymd_res(2006, 12, 31)?,
                "የኢድ አልአድሃ (አረፋ) (ግምት)",
            ),
            (NaiveDate::from_ymd_res(2006, 4, 10)?, "የመውሊድ በዓል (ግምት)"),
        ],
        &mut map,
        Country::ET,
        "Ethiopia",
    );

    build_year(
        years,
        2007,
        vec![
            (NaiveDate::from_ymd_res(2007, 1, 7)?, "የገና ወይም የልደት በዓል"),
            (NaiveDate::from_ymd_res(2007, 1, 19)?, "የጥምቀት በዓል"),
            (NaiveDate::from_ymd_res(2007, 3, 2)?, "የአድዋ ድል በዓል"),
            (NaiveDate::from_ymd_res(2007, 4, 6)?, "የስቅለት በዓል"),
            (NaiveDate::from_ymd_res(2007, 4, 8)?, "የትንሳኤ(ፋሲካ) በዓል"),
            (
                NaiveDate::from_ymd_res(2007, 5, 1)?,
                "የዓለም የሠራተኞች (የላብአደሮች) ቀን",
            ),
            (NaiveDate::from_ymd_res(2007, 5, 5)?, "የአርበኞች (የድል) ቀን በዓል"),
            (NaiveDate::from_ymd_res(2007, 5, 28)?, "ደርግ የወደቀበት ቀን"),
            (
                NaiveDate::from_ymd_res(2007, 9, 12)?,
                "የዘመን መለወጫ (እንቁጣጣሽ) በዓል",
            ),
            (NaiveDate::from_ymd_res(2007, 9, 28)?, "የመስቀል በዓል"),
            (NaiveDate::from_ymd_res(2007, 10, 13)?, "የኢድ አልፈጥር (ግምት)"),
            (
                NaiveDate::from_ymd_res(2007, 12, 20)?,
                "የኢድ አልአድሃ (አረፋ) (ግምት)",
            ),
            (NaiveDate::from_ymd_res(2007, 3, 31)?, "የመውሊድ በዓል (ግምት)"),
        ],
        &mut map,
        Country::ET,
        "Ethiopia",
    );

    build_year(
        years,
        2008,
        vec![
            (NaiveDate::from_ymd_res(2008, 1, 7)?, "የገና ወይም የልደት በዓል"),
            (NaiveDate::from_ymd_res(2008, 1, 20)?, "የጥምቀት በዓል"),
            (NaiveDate::from_ymd_res(2008, 3, 2)?, "የአድዋ ድል በዓል"),
            (NaiveDate::from_ymd_res(2008, 4, 25)?, "የስቅለት በዓል"),
            (NaiveDate::from_ymd_res(2008, 4, 27)?, "የትንሳኤ(ፋሲካ) በዓል"),
            (
                NaiveDate::from_ymd_res(2008, 5, 1)?,
                "የዓለም የሠራተኞች (የላብአደሮች) ቀን",
            ),
            (NaiveDate::from_ymd_res(2008, 5, 5)?, "የአርበኞች (የድል) ቀን በዓል"),
            (NaiveDate::from_ymd_res(2008, 5, 28)?, "ደርግ የወደቀበት ቀን"),
            (
                NaiveDate::from_ymd_res(2008, 9, 11)?,
                "የዘመን መለወጫ (እንቁጣጣሽ) በዓል",
            ),
            (NaiveDate::from_ymd_res(2008, 9, 27)?, "የመስቀል በዓል"),
            (NaiveDate::from_ymd_res(2008, 10, 1)?, "የኢድ አልፈጥር (ግምት)"),
            (
                NaiveDate::from_ymd_res(2008, 12, 8)?,
                "የኢድ አልአድሃ (አረፋ) (ግምት)",
            ),
            (NaiveDate::from_ymd_res(2008, 3, 20)?, "የመውሊድ በዓል (ግምት)"),
        ],
        &mut map,
        Country::ET,
        "Ethiopia",
    );

    build_year(
        years,
        2009,
        vec![
            (NaiveDate::from_ymd_res(2009, 1, 7)?, "የገና ወይም የልደት በዓል"),
            (NaiveDate::from_ymd_res(2009, 1, 19)?, "የጥምቀት በዓል"),
            (NaiveDate::from_ymd_res(2009, 3, 2)?, "የአድዋ ድል በዓል"),
            (NaiveDate::from_ymd_res(2009, 4, 17)?, "የስቅለት በዓል"),
            (NaiveDate::from_ymd_res(2009, 4, 19)?, "የትንሳኤ(ፋሲካ) በዓል"),
            (
                NaiveDate::from_ymd_res(2009, 5, 1)?,
                "የዓለም የሠራተኞች (የላብአደሮች) ቀን",
            ),
            (NaiveDate::from_ymd_res(2009, 5, 5)?, "የአርበኞች (የድል) ቀን በዓል"),
            (NaiveDate::from_ymd_res(2009, 5, 28)?, "ደርግ የወደቀበት ቀን"),
            (
                NaiveDate::from_ymd_res(2009, 9, 11)?,
                "የዘመን መለወጫ (እንቁጣጣሽ) በዓል",
            ),
            (NaiveDate::from_ymd_res(2009, 9, 27)?, "የመስቀል በዓል"),
            (NaiveDate::from_ymd_res(2009, 9, 20)?, "የኢድ አልፈጥር (ግምት)"),
            (
                NaiveDate::from_ymd_res(2009, 11, 27)?,
                "የኢድ አልአድሃ (አረፋ) (ግምት)",
            ),
            (NaiveDate::from_ymd_res(2009, 3, 9)?, "የመውሊድ በዓል (ግምት)"),
        ],
        &mut map,
        Country::ET,
        "Ethiopia",
    );

    build_year(
        years,
        2010,
        vec![
            (NaiveDate::from_ymd_res(2010, 1, 7)?, "የገና ወይም የልደት በዓል"),
            (NaiveDate::from_ymd_res(2010, 1, 19)?, "የጥምቀት በዓል"),
            (NaiveDate::from_ymd_res(2010, 3, 2)?, "የአድዋ ድል በዓል"),
            (NaiveDate::from_ymd_res(2010, 4, 2)?, "የስቅለት በዓል"),
            (NaiveDate::from_ymd_res(2010, 4, 4)?, "የትንሳኤ(ፋሲካ) በዓል"),
            (
                NaiveDate::from_ymd_res(2010, 5, 1)?,
                "የዓለም የሠራተኞች (የላብአደሮች) ቀን",
            ),
            (NaiveDate::from_ymd_res(2010, 5, 5)?, "የአርበኞች (የድል) ቀን በዓል"),
            (NaiveDate::from_ymd_res(2010, 5, 28)?, "ደርግ የወደቀበት ቀን"),
            (
                NaiveDate::from_ymd_res(2010, 9, 11)?,
                "የዘመን መለወጫ (እንቁጣጣሽ) በዓል",
            ),
            (NaiveDate::from_ymd_res(2010, 9, 27)?, "የመስቀል በዓል"),
            (NaiveDate::from_ymd_res(2010, 9, 10)?, "የኢድ አልፈጥር (ግምት)"),
            (
                NaiveDate::from_ymd_res(2010, 11, 16)?,
                "የኢድ አልአድሃ (አረፋ) (ግምት)",
            ),
            (NaiveDate::from_ymd_res(2010, 2, 26)?, "የመውሊድ በዓል (ግምት)"),
        ],
        &mut map,
        Country::ET,
        "Ethiopia",
    );

    build_year(
        years,
        2011,
        vec![
            (NaiveDate::from_ymd_res(2011, 1, 7)?, "የገና ወይም የልደት በዓል"),
            (NaiveDate::from_ymd_res(2011, 1, 19)?, "የጥምቀት በዓል"),
            (NaiveDate::from_ymd_res(2011, 3, 2)?, "የአድዋ ድል በዓል"),
            (NaiveDate::from_ymd_res(2011, 4, 22)?, "የስቅለት በዓል"),
            (NaiveDate::from_ymd_res(2011, 4, 24)?, "የትንሳኤ(ፋሲካ) በዓል"),
            (
                NaiveDate::from_ymd_res(2011, 5, 1)?,
                "የዓለም የሠራተኞች (የላብአደሮች) ቀን",
            ),
            (NaiveDate::from_ymd_res(2011, 5, 5)?, "የአርበኞች (የድል) ቀን በዓል"),
            (NaiveDate::from_ymd_res(2011, 5, 28)?, "ደርግ የወደቀበት ቀን"),
            (
                NaiveDate::from_ymd_res(2011, 9, 12)?,
                "የዘመን መለወጫ (እንቁጣጣሽ) በዓል",
            ),
            (NaiveDate::from_ymd_res(2011, 9, 28)?, "የመስቀል በዓል"),
            (NaiveDate::from_ymd_res(2011, 8, 30)?, "የኢድ አልፈጥር (ግምት)"),
            (
                NaiveDate::from_ymd_res(2011, 11, 6)?,
                "የኢድ አልአድሃ (አረፋ) (ግምት)",
            ),
            (NaiveDate::from_ymd_res(2011, 2, 15)?, "የመውሊድ በዓል (ግምት)"),
        ],
        &mut map,
        Country::ET,
        "Ethiopia",
    );

    build_year(
        years,
        2012,
        vec![
            (NaiveDate::from_ymd_res(2012, 1, 7)?, "የገና ወይም የልደት በዓል"),
            (NaiveDate::from_ymd_res(2012, 1, 20)?, "የጥምቀት በዓል"),
            (NaiveDate::from_ymd_res(2012, 3, 2)?, "የአድዋ ድል በዓል"),
            (NaiveDate::from_ymd_res(2012, 4, 13)?, "የስቅለት በዓል"),
            (NaiveDate::from_ymd_res(2012, 4, 15)?, "የትንሳኤ(ፋሲካ) በዓል"),
            (
                NaiveDate::from_ymd_res(2012, 5, 1)?,
                "የዓለም የሠራተኞች (የላብአደሮች) ቀን",
            ),
            (NaiveDate::from_ymd_res(2012, 5, 5)?, "የአርበኞች (የድል) ቀን በዓል"),
            (NaiveDate::from_ymd_res(2012, 5, 28)?, "ደርግ የወደቀበት ቀን"),
            (
                NaiveDate::from_ymd_res(2012, 9, 11)?,
                "የዘመን መለወጫ (እንቁጣጣሽ) በዓል",
            ),
            (NaiveDate::from_ymd_res(2012, 9, 27)?, "የመስቀል በዓል"),
            (NaiveDate::from_ymd_res(2012, 8, 19)?, "የኢድ አልፈጥር (ግምት)"),
            (
                NaiveDate::from_ymd_res(2012, 10, 26)?,
                "የኢድ አልአድሃ (አረፋ) (ግምት)",
            ),
            (NaiveDate::from_ymd_res(2012, 2, 4)?, "የመውሊድ በዓል (ግምት)"),
        ],
        &mut map,
        Country::ET,
        "Ethiopia",
    );

    build_year(
        years,
        2013,
        vec![
            (NaiveDate::from_ymd_res(2013, 1, 7)?, "የገና ወይም የልደት በዓል"),
            (NaiveDate::from_ymd_res(2013, 1, 19)?, "የጥምቀት በዓል"),
            (NaiveDate::from_ymd_res(2013, 3, 2)?, "የአድዋ ድል በዓል"),
            (NaiveDate::from_ymd_res(2013, 5, 3)?, "የስቅለት በዓል"),
            (
                NaiveDate::from_ymd_res(2013, 5, 5)?,
                "የትንሳኤ(ፋሲካ) በዓል; የአርበኞች (የድል) ቀን በዓል",
            ),
            (
                NaiveDate::from_ymd_res(2013, 5, 1)?,
                "የዓለም የሠራተኞች (የላብአደሮች) ቀን",
            ),
            (NaiveDate::from_ymd_res(2013, 5, 28)?, "ደርግ የወደቀበት ቀን"),
            (
                NaiveDate::from_ymd_res(2013, 9, 11)?,
                "የዘመን መለወጫ (እንቁጣጣሽ) በዓል",
            ),
            (NaiveDate::from_ymd_res(2013, 9, 27)?, "የመስቀል በዓል"),
            (NaiveDate::from_ymd_res(2013, 8, 8)?, "የኢድ አልፈጥር (ግምት)"),
            (
                NaiveDate::from_ymd_res(2013, 10, 15)?,
                "የኢድ አልአድሃ (አረፋ) (ግምት)",
            ),
            (NaiveDate::from_ymd_res(2013, 1, 24)?, "የመውሊድ በዓል (ግምት)"),
        ],
        &mut map,
        Country::ET,
        "Ethiopia",
    );

    build_year(
        years,
        2014,
        vec![
            (NaiveDate::from_ymd_res(2014, 1, 7)?, "የገና ወይም የልደት በዓል"),
            (NaiveDate::from_ymd_res(2014, 1, 19)?, "የጥምቀት በዓል"),
            (NaiveDate::from_ymd_res(2014, 3, 2)?, "የአድዋ ድል በዓል"),
            (NaiveDate::from_ymd_res(2014, 4, 18)?, "የስቅለት በዓል"),
            (NaiveDate::from_ymd_res(2014, 4, 20)?, "የትንሳኤ(ፋሲካ) በዓል"),
            (
                NaiveDate::from_ymd_res(2014, 5, 1)?,
                "የዓለም የሠራተኞች (የላብአደሮች) ቀን",
            ),
            (NaiveDate::from_ymd_res(2014, 5, 5)?, "የአርበኞች (የድል) ቀን በዓል"),
            (NaiveDate::from_ymd_res(2014, 5, 28)?, "ደርግ የወደቀበት ቀን"),
            (
                NaiveDate::from_ymd_res(2014, 9, 11)?,
                "የዘመን መለወጫ (እንቁጣጣሽ) በዓል",
            ),
            (NaiveDate::from_ymd_res(2014, 9, 27)?, "የመስቀል በዓል"),
            (NaiveDate::from_ymd_res(2014, 7, 28)?, "የኢድ አልፈጥር (ግምት)"),
            (
                NaiveDate::from_ymd_res(2014, 10, 4)?,
                "የኢድ አልአድሃ (አረፋ) (ግምት)",
            ),
            (NaiveDate::from_ymd_res(2014, 1, 13)?, "የመውሊድ በዓል (ግምት)"),
        ],
        &mut map,
        Country::ET,
        "Ethiopia",
    );

    build_year(
        years,
        2015,
        vec![
            (NaiveDate::from_ymd_res(2015, 1, 7)?, "የገና ወይም የልደት በዓል"),
            (NaiveDate::from_ymd_res(2015, 1, 19)?, "የጥምቀት በዓል"),
            (NaiveDate::from_ymd_res(2015, 3, 2)?, "የአድዋ ድል በዓል"),
            (NaiveDate::from_ymd_res(2015, 4, 10)?, "የስቅለት በዓል"),
            (NaiveDate::from_ymd_res(2015, 4, 12)?, "የትንሳኤ(ፋሲካ) በዓል"),
            (
                NaiveDate::from_ymd_res(2015, 5, 1)?,
                "የዓለም የሠራተኞች (የላብአደሮች) ቀን",
            ),
            (NaiveDate::from_ymd_res(2015, 5, 5)?, "የአርበኞች (የድል) ቀን በዓል"),
            (NaiveDate::from_ymd_res(2015, 5, 28)?, "ደርግ የወደቀበት ቀን"),
            (
                NaiveDate::from_ymd_res(2015, 9, 12)?,
                "የዘመን መለወጫ (እንቁጣጣሽ) በዓል",
            ),
            (NaiveDate::from_ymd_res(2015, 9, 28)?, "የመስቀል በዓል"),
            (NaiveDate::from_ymd_res(2015, 7, 17)?, "የኢድ አልፈጥር (ግምት)"),
            (
                NaiveDate::from_ymd_res(2015, 9, 23)?,
                "የኢድ አልአድሃ (አረፋ) (ግምት)",
            ),
            (NaiveDate::from_ymd_res(2015, 1, 3)?, "የመውሊድ በዓል (ግምት)"),
            (NaiveDate::from_ymd_res(2015, 12, 23)?, "የመውሊድ በዓል (ግምት)"),
        ],
        &mut map,
        Country::ET,
        "Ethiopia",
    );

    build_year(
        years,
        2016,
        vec![
            (NaiveDate::from_ymd_res(2016, 1, 7)?, "የገና ወይም የልደት በዓል"),
            (NaiveDate::from_ymd_res(2016, 1, 20)?, "የጥምቀት በዓል"),
            (NaiveDate::from_ymd_res(2016, 3, 2)?, "የአድዋ ድል በዓል"),
            (NaiveDate::from_ymd_res(2016, 4, 29)?, "የስቅለት በዓል"),
            (
                NaiveDate::from_ymd_res(2016, 5, 1)?,
                "የትንሳኤ(ፋሲካ) በዓል; የዓለም የሠራተኞች (የላብአደሮች) ቀን",
            ),
            (NaiveDate::from_ymd_res(2016, 5, 5)?, "የአርበኞች (የድል) ቀን በዓል"),
            (NaiveDate::from_ymd_res(2016, 5, 28)?, "ደርግ የወደቀበት ቀን"),
            (
                NaiveDate::from_ymd_res(2016, 9, 11)?,
                "የኢድ አልአድሃ (አረፋ) (ግምት); የዘመን መለወጫ (እንቁጣጣሽ) በዓል",
            ),
            (NaiveDate::from_ymd_res(2016, 9, 27)?, "የመስቀል በዓል"),
            (NaiveDate::from_ymd_res(2016, 7, 6)?, "የኢድ አልፈጥር (ግምት)"),
            (NaiveDate::from_ymd_res(2016, 12, 11)?, "የመውሊድ በዓል (ግምት)"),
        ],
        &mut map,
        Country::ET,
        "Ethiopia",
    );

    build_year(
        years,
        2017,
        vec![
            (NaiveDate::from_ymd_res(2017, 1, 7)?, "የገና ወይም የልደት በዓል"),
            (NaiveDate::from_ymd_res(2017, 1, 19)?, "የጥምቀት በዓል"),
            (NaiveDate::from_ymd_res(2017, 3, 2)?, "የአድዋ ድል በዓል"),
            (NaiveDate::from_ymd_res(2017, 4, 14)?, "የስቅለት በዓል"),
            (NaiveDate::from_ymd_res(2017, 4, 16)?, "የትንሳኤ(ፋሲካ) በዓል"),
            (
                NaiveDate::from_ymd_res(2017, 5, 1)?,
                "የዓለም የሠራተኞች (የላብአደሮች) ቀን",
            ),
            (NaiveDate::from_ymd_res(2017, 5, 5)?, "የአርበኞች (የድል) ቀን በዓል"),
            (NaiveDate::from_ymd_res(2017, 5, 28)?, "ደርግ የወደቀበት ቀን"),
            (
                NaiveDate::from_ymd_res(2017, 9, 11)?,
                "የዘመን መለወጫ (እንቁጣጣሽ) በዓል",
            ),
            (NaiveDate::from_ymd_res(2017, 9, 27)?, "የመስቀል በዓል"),
            (NaiveDate::from_ymd_res(2017, 6, 25)?, "የኢድ አልፈጥር (ግምት)"),
            (
                NaiveDate::from_ymd_res(2017, 9, 1)?,
                "የኢድ አልአድሃ (አረፋ) (ግምት)",
            ),
            (NaiveDate::from_ymd_res(2017, 11, 30)?, "የመውሊድ በዓል (ግምት)"),
        ],
        &mut map,
        Country::ET,
        "Ethiopia",
    );

    build_year(
        years,
        2018,
        vec![
            (NaiveDate::from_ymd_res(2018, 1, 7)?, "የገና ወይም የልደት በዓል"),
            (NaiveDate::from_ymd_res(2018, 1, 19)?, "የጥምቀት በዓል"),
            (NaiveDate::from_ymd_res(2018, 3, 2)?, "የአድዋ ድል በዓል"),
            (NaiveDate::from_ymd_res(2018, 4, 6)?, "የስቅለት በዓል"),
            (NaiveDate::from_ymd_res(2018, 4, 8)?, "የትንሳኤ(ፋሲካ) በዓል"),
            (
                NaiveDate::from_ymd_res(2018, 5, 1)?,
                "የዓለም የሠራተኞች (የላብአደሮች) ቀን",
            ),
            (NaiveDate::from_ymd_res(2018, 5, 5)?, "የአርበኞች (የድል) ቀን በዓል"),
            (NaiveDate::from_ymd_res(2018, 5, 28)?, "ደርግ የወደቀበት ቀን"),
            (
                NaiveDate::from_ymd_res(2018, 9, 11)?,
                "የዘመን መለወጫ (እንቁጣጣሽ) በዓል",
            ),
            (NaiveDate::from_ymd_res(2018, 9, 27)?, "የመስቀል በዓል"),
            (NaiveDate::from_ymd_res(2018, 6, 15)?, "የኢድ አልፈጥር"),
            (NaiveDate::from_ymd_res(2018, 8, 22)?, "የኢድ አልአድሃ (አረፋ)"),
            (NaiveDate::from_ymd_res(2018, 11, 21)?, "የመውሊድ በዓል"),
        ],
        &mut map,
        Country::ET,
        "Ethiopia",
    );

    build_year(
        years,
        2019,
        vec![
            (NaiveDate::from_ymd_res(2019, 1, 7)?, "የገና ወይም የልደት በዓል"),
            (NaiveDate::from_ymd_res(2019, 1, 19)?, "የጥምቀት በዓል"),
            (NaiveDate::from_ymd_res(2019, 3, 2)?, "የአድዋ ድል በዓል"),
            (NaiveDate::from_ymd_res(2019, 4, 26)?, "የስቅለት በዓል"),
            (NaiveDate::from_ymd_res(2019, 4, 28)?, "የትንሳኤ(ፋሲካ) በዓል"),
            (
                NaiveDate::from_ymd_res(2019, 5, 1)?,
                "የዓለም የሠራተኞች (የላብአደሮች) ቀን",
            ),
            (NaiveDate::from_ymd_res(2019, 5, 5)?, "የአርበኞች (የድል) ቀን በዓል"),
            (NaiveDate::from_ymd_res(2019, 5, 28)?, "ደርግ የወደቀበት ቀን"),
            (
                NaiveDate::from_ymd_res(2019, 9, 12)?,
                "የዘመን መለወጫ (እንቁጣጣሽ) በዓል",
            ),
            (NaiveDate::from_ymd_res(2019, 9, 28)?, "የመስቀል በዓል"),
            (NaiveDate::from_ymd_res(2019, 6, 4)?, "የኢድ አልፈጥር"),
            (NaiveDate::from_ymd_res(2019, 8, 11)?, "የኢድ አልአድሃ (አረፋ)"),
            (NaiveDate::from_ymd_res(2019, 11, 10)?, "የመውሊድ በዓል"),
        ],
        &mut map,
        Country::ET,
        "Ethiopia",
    );

    build_year(
        years,
        2020,
        vec![
            (NaiveDate::from_ymd_res(2020, 1, 7)?, "የገና ወይም የልደት በዓል"),
            (NaiveDate::from_ymd_res(2020, 1, 20)?, "የጥምቀት በዓል"),
            (NaiveDate::from_ymd_res(2020, 3, 2)?, "የአድዋ ድል በዓል"),
            (NaiveDate::from_ymd_res(2020, 4, 17)?, "የስቅለት በዓል"),
            (NaiveDate::from_ymd_res(2020, 4, 19)?, "የትንሳኤ(ፋሲካ) በዓል"),
            (
                NaiveDate::from_ymd_res(2020, 5, 1)?,
                "የዓለም የሠራተኞች (የላብአደሮች) ቀን",
            ),
            (NaiveDate::from_ymd_res(2020, 5, 5)?, "የአርበኞች (የድል) ቀን በዓል"),
            (NaiveDate::from_ymd_res(2020, 5, 28)?, "ደርግ የወደቀበት ቀን"),
            (
                NaiveDate::from_ymd_res(2020, 9, 11)?,
                "የዘመን መለወጫ (እንቁጣጣሽ) በዓል",
            ),
            (NaiveDate::from_ymd_res(2020, 9, 27)?, "የመስቀል በዓል"),
            (NaiveDate::from_ymd_res(2020, 5, 24)?, "የኢድ አልፈጥር"),
            (NaiveDate::from_ymd_res(2020, 7, 31)?, "የኢድ አልአድሃ (አረፋ)"),
            (NaiveDate::from_ymd_res(2020, 10, 29)?, "የመውሊድ በዓል"),
        ],
        &mut map,
        Country::ET,
        "Ethiopia",
    );

    build_year(
        years,
        2021,
        vec![
            (NaiveDate::from_ymd_res(2021, 1, 7)?, "የገና ወይም የልደት በዓል"),
            (NaiveDate::from_ymd_res(2021, 1, 19)?, "የጥምቀት በዓል"),
            (NaiveDate::from_ymd_res(2021, 3, 2)?, "የአድዋ ድል በዓል"),
            (NaiveDate::from_ymd_res(2021, 4, 30)?, "የስቅለት በዓል"),
            (NaiveDate::from_ymd_res(2021, 5, 2)?, "የትንሳኤ(ፋሲካ) በዓል"),
            (
                NaiveDate::from_ymd_res(2021, 5, 1)?,
                "የዓለም የሠራተኞች (የላብአደሮች) ቀን",
            ),
            (NaiveDate::from_ymd_res(2021, 5, 5)?, "የአርበኞች (የድል) ቀን በዓል"),
            (NaiveDate::from_ymd_res(2021, 5, 28)?, "ደርግ የወደቀበት ቀን"),
            (
                NaiveDate::from_ymd_res(2021, 9, 11)?,
                "የዘመን መለወጫ (እንቁጣጣሽ) በዓል",
            ),
            (NaiveDate::from_ymd_res(2021, 9, 27)?, "የመስቀል በዓል"),
            (NaiveDate::from_ymd_res(2021, 5, 13)?, "የኢድ አልፈጥር"),
            (NaiveDate::from_ymd_res(2021, 7, 20)?, "የኢድ አልአድሃ (አረፋ)"),
            (NaiveDate::from_ymd_res(2021, 10, 18)?, "የመውሊድ በዓል"),
        ],
        &mut map,
        Country::ET,
        "Ethiopia",
    );

    build_year(
        years,
        2022,
        vec![
            (NaiveDate::from_ymd_res(2022, 1, 7)?, "የገና ወይም የልደት በዓል"),
            (NaiveDate::from_ymd_res(2022, 1, 19)?, "የጥምቀት በዓል"),
            (NaiveDate::from_ymd_res(2022, 3, 2)?, "የአድዋ ድል በዓል"),
            (NaiveDate::from_ymd_res(2022, 4, 22)?, "የስቅለት በዓል"),
            (NaiveDate::from_ymd_res(2022, 4, 24)?, "የትንሳኤ(ፋሲካ) በዓል"),
            (
                NaiveDate::from_ymd_res(2022, 5, 1)?,
                "የዓለም የሠራተኞች (የላብአደሮች) ቀን",
            ),
            (NaiveDate::from_ymd_res(2022, 5, 5)?, "የአርበኞች (የድል) ቀን በዓል"),
            (NaiveDate::from_ymd_res(2022, 5, 28)?, "ደርግ የወደቀበት ቀን"),
            (
                NaiveDate::from_ymd_res(2022, 9, 11)?,
                "የዘመን መለወጫ (እንቁጣጣሽ) በዓል",
            ),
            (NaiveDate::from_ymd_res(2022, 9, 27)?, "የመስቀል በዓል"),
            (NaiveDate::from_ymd_res(2022, 5, 2)?, "የኢድ አልፈጥር"),
            (NaiveDate::from_ymd_res(2022, 7, 9)?, "የኢድ አልአድሃ (አረፋ)"),
            (NaiveDate::from_ymd_res(2022, 10, 8)?, "የመውሊድ በዓል"),
        ],
        &mut map,
        Country::ET,
        "Ethiopia",
    );

    build_year(
        years,
        2023,
        vec![
            (NaiveDate::from_ymd_res(2023, 1, 7)?, "የገና ወይም የልደት በዓል"),
            (NaiveDate::from_ymd_res(2023, 1, 19)?, "የጥምቀት በዓል"),
            (NaiveDate::from_ymd_res(2023, 3, 2)?, "የአድዋ ድል በዓል"),
            (NaiveDate::from_ymd_res(2023, 4, 14)?, "የስቅለት በዓል"),
            (NaiveDate::from_ymd_res(2023, 4, 16)?, "የትንሳኤ(ፋሲካ) በዓል"),
            (
                NaiveDate::from_ymd_res(2023, 5, 1)?,
                "የዓለም የሠራተኞች (የላብአደሮች) ቀን",
            ),
            (NaiveDate::from_ymd_res(2023, 5, 5)?, "የአርበኞች (የድል) ቀን በዓል"),
            (NaiveDate::from_ymd_res(2023, 5, 28)?, "ደርግ የወደቀበት ቀን"),
            (
                NaiveDate::from_ymd_res(2023, 9, 12)?,
                "የዘመን መለወጫ (እንቁጣጣሽ) በዓል",
            ),
            (NaiveDate::from_ymd_res(2023, 9, 28)?, "የመስቀል በዓል"),
            (NaiveDate::from_ymd_res(2023, 4, 21)?, "የኢድ አልፈጥር"),
            (NaiveDate::from_ymd_res(2023, 6, 28)?, "የኢድ አልአድሃ (አረፋ)"),
            (NaiveDate::from_ymd_res(2023, 9, 27)?, "የመውሊድ በዓል"),
        ],
        &mut map,
        Country::ET,
        "Ethiopia",
    );

    build_year(
        years,
        2024,
        vec![
            (NaiveDate::from_ymd_res(2024, 1, 7)?, "የገና ወይም የልደት በዓል"),
            (NaiveDate::from_ymd_res(2024, 1, 20)?, "የጥምቀት በዓል"),
            (NaiveDate::from_ymd_res(2024, 3, 2)?, "የአድዋ ድል በዓል"),
            (NaiveDate::from_ymd_res(2024, 5, 3)?, "የስቅለት በዓል"),
            (
                NaiveDate::from_ymd_res(2024, 5, 5)?,
                "የትንሳኤ(ፋሲካ) በዓል; የአርበኞች (የድል) ቀን በዓል",
            ),
            (
                NaiveDate::from_ymd_res(2024, 5, 1)?,
                "የዓለም የሠራተኞች (የላብአደሮች) ቀን",
            ),
            (NaiveDate::from_ymd_res(2024, 5, 28)?, "ደርግ የወደቀበት ቀን"),
            (
                NaiveDate::from_ymd_res(2024, 9, 11)?,
                "የዘመን መለወጫ (እንቁጣጣሽ) በዓል",
            ),
            (NaiveDate::from_ymd_res(2024, 9, 27)?, "የመስቀል በዓል"),
            (NaiveDate::from_ymd_res(2024, 4, 10)?, "የኢድ አልፈጥር"),
            (NaiveDate::from_ymd_res(2024, 6, 16)?, "የኢድ አልአድሃ (አረፋ)"),
            (NaiveDate::from_ymd_res(2024, 9, 15)?, "የመውሊድ በዓል"),
        ],
        &mut map,
        Country::ET,
        "Ethiopia",
    );

    build_year(
        years,
        2025,
        vec![
            (NaiveDate::from_ymd_res(2025, 1, 7)?, "የገና ወይም የልደት በዓል"),
            (NaiveDate::from_ymd_res(2025, 1, 19)?, "የጥምቀት በዓል"),
            (NaiveDate::from_ymd_res(2025, 3, 2)?, "የአድዋ ድል በዓል"),
            (NaiveDate::from_ymd_res(2025, 4, 18)?, "የስቅለት በዓል"),
            (NaiveDate::from_ymd_res(2025, 4, 20)?, "የትንሳኤ(ፋሲካ) በዓል"),
            (
                NaiveDate::from_ymd_res(2025, 5, 1)?,
                "የዓለም የሠራተኞች (የላብአደሮች) ቀን",
            ),
            (NaiveDate::from_ymd_res(2025, 5, 5)?, "የአርበኞች (የድል) ቀን በዓል"),
            (
                NaiveDate::from_ymd_res(2025, 9, 11)?,
                "የዘመን መለወጫ (እንቁጣጣሽ) በዓል",
            ),
            (NaiveDate::from_ymd_res(2025, 9, 27)?, "የመስቀል በዓል"),
            (NaiveDate::from_ymd_res(2025, 3, 30)?, "የኢድ አልፈጥር"),
            (NaiveDate::from_ymd_res(2025, 6, 6)?, "የኢድ አልአድሃ (አረፋ)"),
            (NaiveDate::from_ymd_res(2025, 9, 5)?, "የመውሊድ በዓል"),
        ],
        &mut map,
        Country::ET,
        "Ethiopia",
    );

    build_year(
        years,
        2026,
        vec![
            (NaiveDate::from_ymd_res(2026, 1, 7)?, "የገና ወይም የልደት በዓል"),
            (NaiveDate::from_ymd_res(2026, 1, 19)?, "የጥምቀት በዓል"),
            (NaiveDate::from_ymd_res(2026, 3, 2)?, "የአድዋ ድል በዓል"),
            (NaiveDate::from_ymd_res(2026, 4, 10)?, "የስቅለት በዓል"),
            (NaiveDate::from_ymd_res(2026, 4, 12)?, "የትንሳኤ(ፋሲካ) በዓል"),
            (
                NaiveDate::from_ymd_res(2026, 5, 1)?,
                "የዓለም የሠራተኞች (የላብአደሮች) ቀን",
            ),
            (NaiveDate::from_ymd_res(2026, 5, 5)?, "የአርበኞች (የድል) ቀን በዓል"),
            (
                NaiveDate::from_ymd_res(2026, 9, 11)?,
                "የዘመን መለወጫ (እንቁጣጣሽ) በዓል",
            ),
            (NaiveDate::from_ymd_res(2026, 9, 27)?, "የመስቀል በዓል"),
            (NaiveDate::from_ymd_res(2026, 3, 20)?, "የኢድ አልፈጥር (ግምት)"),
            (
                NaiveDate::from_ymd_res(2026, 5, 27)?,
                "የኢድ አልአድሃ (አረፋ) (ግምት)",
            ),
            (NaiveDate::from_ymd_res(2026, 8, 25)?, "የመውሊድ በዓል (ግምት)"),
        ],
        &mut map,
        Country::ET,
        "Ethiopia",
    );

    build_year(
        years,
        2027,
        vec![
            (NaiveDate::from_ymd_res(2027, 1, 7)?, "የገና ወይም የልደት በዓል"),
            (NaiveDate::from_ymd_res(2027, 1, 19)?, "የጥምቀት በዓል"),
            (NaiveDate::from_ymd_res(2027, 3, 2)?, "የአድዋ ድል በዓል"),
            (NaiveDate::from_ymd_res(2027, 4, 30)?, "የስቅለት በዓል"),
            (NaiveDate::from_ymd_res(2027, 5, 2)?, "የትንሳኤ(ፋሲካ) በዓል"),
            (
                NaiveDate::from_ymd_res(2027, 5, 1)?,
                "የዓለም የሠራተኞች (የላብአደሮች) ቀን",
            ),
            (NaiveDate::from_ymd_res(2027, 5, 5)?, "የአርበኞች (የድል) ቀን በዓል"),
            (
                NaiveDate::from_ymd_res(2027, 9, 12)?,
                "የዘመን መለወጫ (እንቁጣጣሽ) በዓል",
            ),
            (NaiveDate::from_ymd_res(2027, 9, 28)?, "የመስቀል በዓል"),
            (NaiveDate::from_ymd_res(2027, 3, 9)?, "የኢድ አልፈጥር (ግምት)"),
            (
                NaiveDate::from_ymd_res(2027, 5, 16)?,
                "የኢድ አልአድሃ (አረፋ) (ግምት)",
            ),
            (NaiveDate::from_ymd_res(2027, 8, 14)?, "የመውሊድ በዓል (ግምት)"),
        ],
        &mut map,
        Country::ET,
        "Ethiopia",
    );

    build_year(
        years,
        2028,
        vec![
            (NaiveDate::from_ymd_res(2028, 1, 7)?, "የገና ወይም የልደት በዓል"),
            (NaiveDate::from_ymd_res(2028, 1, 20)?, "የጥምቀት በዓል"),
            (NaiveDate::from_ymd_res(2028, 3, 2)?, "የአድዋ ድል በዓል"),
            (NaiveDate::from_ymd_res(2028, 4, 14)?, "የስቅለት በዓል"),
            (NaiveDate::from_ymd_res(2028, 4, 16)?, "የትንሳኤ(ፋሲካ) በዓል"),
            (
                NaiveDate::from_ymd_res(2028, 5, 1)?,
                "የዓለም የሠራተኞች (የላብአደሮች) ቀን",
            ),
            (
                NaiveDate::from_ymd_res(2028, 5, 5)?,
                "የአርበኞች (የድል) ቀን በዓል; የኢድ አልአድሃ (አረፋ) (ግምት)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 9, 11)?,
                "የዘመን መለወጫ (እንቁጣጣሽ) በዓል",
            ),
            (NaiveDate::from_ymd_res(2028, 9, 27)?, "የመስቀል በዓል"),
            (NaiveDate::from_ymd_res(2028, 2, 26)?, "የኢድ አልፈጥር (ግምት)"),
            (NaiveDate::from_ymd_res(2028, 8, 3)?, "የመውሊድ በዓል (ግምት)"),
        ],
        &mut map,
        Country::ET,
        "Ethiopia",
    );

    build_year(
        years,
        2029,
        vec![
            (NaiveDate::from_ymd_res(2029, 1, 7)?, "የገና ወይም የልደት በዓል"),
            (NaiveDate::from_ymd_res(2029, 1, 19)?, "የጥምቀት በዓል"),
            (NaiveDate::from_ymd_res(2029, 3, 2)?, "የአድዋ ድል በዓል"),
            (NaiveDate::from_ymd_res(2029, 4, 6)?, "የስቅለት በዓል"),
            (NaiveDate::from_ymd_res(2029, 4, 8)?, "የትንሳኤ(ፋሲካ) በዓል"),
            (
                NaiveDate::from_ymd_res(2029, 5, 1)?,
                "የዓለም የሠራተኞች (የላብአደሮች) ቀን",
            ),
            (NaiveDate::from_ymd_res(2029, 5, 5)?, "የአርበኞች (የድል) ቀን በዓል"),
            (
                NaiveDate::from_ymd_res(2029, 9, 11)?,
                "የዘመን መለወጫ (እንቁጣጣሽ) በዓል",
            ),
            (NaiveDate::from_ymd_res(2029, 9, 27)?, "የመስቀል በዓል"),
            (NaiveDate::from_ymd_res(2029, 2, 14)?, "የኢድ አልፈጥር (ግምት)"),
            (
                NaiveDate::from_ymd_res(2029, 4, 24)?,
                "የኢድ አልአድሃ (አረፋ) (ግምት)",
            ),
            (NaiveDate::from_ymd_res(2029, 7, 24)?, "የመውሊድ በዓል (ግምት)"),
        ],
        &mut map,
        Country::ET,
        "Ethiopia",
    );

    build_year(
        years,
        2030,
        vec![
            (NaiveDate::from_ymd_res(2030, 1, 7)?, "የገና ወይም የልደት በዓል"),
            (NaiveDate::from_ymd_res(2030, 1, 19)?, "የጥምቀት በዓል"),
            (NaiveDate::from_ymd_res(2030, 3, 2)?, "የአድዋ ድል በዓል"),
            (NaiveDate::from_ymd_res(2030, 4, 26)?, "የስቅለት በዓል"),
            (NaiveDate::from_ymd_res(2030, 4, 28)?, "የትንሳኤ(ፋሲካ) በዓል"),
            (
                NaiveDate::from_ymd_res(2030, 5, 1)?,
                "የዓለም የሠራተኞች (የላብአደሮች) ቀን",
            ),
            (NaiveDate::from_ymd_res(2030, 5, 5)?, "የአርበኞች (የድል) ቀን በዓል"),
            (
                NaiveDate::from_ymd_res(2030, 9, 11)?,
                "የዘመን መለወጫ (እንቁጣጣሽ) በዓል",
            ),
            (NaiveDate::from_ymd_res(2030, 9, 27)?, "የመስቀል በዓል"),
            (NaiveDate::from_ymd_res(2030, 2, 4)?, "የኢድ አልፈጥር (ግምት)"),
            (
                NaiveDate::from_ymd_res(2030, 4, 13)?,
                "የኢድ አልአድሃ (አረፋ) (ግምት)",
            ),
            (NaiveDate::from_ymd_res(2030, 7, 13)?, "የመውሊድ በዓል (ግምት)"),
        ],
        &mut map,
        Country::ET,
        "Ethiopia",
    );

    Ok(map)
}
