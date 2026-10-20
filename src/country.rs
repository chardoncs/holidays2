use crate::{Error, Holiday, NaiveDateExt, Result, Year};

use chrono::NaiveDate;
use std::collections::{BTreeMap, HashMap};

use helper::build_year;

mod helper;

/// Two-letter country codes defined in ISO 3166-1 alpha-2 .
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum Country {
    #[cfg(feature = "country-AO")]
    /// Angola
    AO,
    #[cfg(feature = "country-AR")]
    /// Argentina
    AR,
    #[cfg(feature = "country-AM")]
    /// Armenia
    AM,
    #[cfg(feature = "country-AW")]
    /// Aruba
    AW,
    #[cfg(feature = "country-AU")]
    /// Australia
    AU,
    #[cfg(feature = "country-AT")]
    /// Austria
    AT,
    #[cfg(feature = "country-AZ")]
    /// Azerbaijan
    AZ,
    #[cfg(feature = "country-BD")]
    /// Bangladesh
    BD,
    #[cfg(feature = "country-BY")]
    /// Belarus
    BY,
    #[cfg(feature = "country-BE")]
    /// Belgium
    BE,
    #[cfg(feature = "country-BO")]
    /// Bolivia
    BO,
    #[cfg(feature = "country-BA")]
    /// Bosnia and Herzegovina
    BA,
    #[cfg(feature = "country-BW")]
    /// Botswana
    BW,
    #[cfg(feature = "country-BR")]
    /// Brazil
    BR,
    #[cfg(feature = "country-BG")]
    /// Bulgaria
    BG,
    #[cfg(feature = "country-BI")]
    /// Burundi
    BI,
    #[cfg(feature = "country-CA")]
    /// Canada
    CA,
    #[cfg(feature = "country-CL")]
    /// Chile
    CL,
    #[cfg(feature = "country-CN")]
    /// China
    CN,
    #[cfg(feature = "country-CO")]
    /// Colombia
    CO,
    #[cfg(feature = "country-HR")]
    /// Croatia
    HR,
    #[cfg(feature = "country-CU")]
    /// Cuba
    CU,
    #[cfg(feature = "country-CW")]
    /// Curaçao
    CW,
    #[cfg(feature = "country-CY")]
    /// Cyprus
    CY,
    #[cfg(feature = "country-CZ")]
    /// Czechia
    CZ,
    #[cfg(feature = "country-DK")]
    /// Denmark
    DK,
    #[cfg(feature = "country-DJ")]
    /// Djibouti
    DJ,
    #[cfg(feature = "country-DO")]
    /// Dominican Republic
    DO,
    #[cfg(feature = "country-EG")]
    /// Egypt
    EG,
    #[cfg(feature = "country-EE")]
    /// Estonia
    EE,
    #[cfg(feature = "country-ET")]
    /// Ethiopia
    ET,
    #[cfg(feature = "country-FI")]
    /// Finland
    FI,
    #[cfg(feature = "country-FR")]
    /// France
    FR,
    #[cfg(feature = "country-GE")]
    /// Georgia
    GE,
    #[cfg(feature = "country-DE")]
    /// Germany
    DE,
    #[cfg(feature = "country-GR")]
    /// Greece
    GR,
    #[cfg(feature = "country-HN")]
    /// Honduras
    HN,
    #[cfg(feature = "country-HK")]
    /// Hong Kong
    HK,
    #[cfg(feature = "country-HU")]
    /// Hungary
    HU,
    #[cfg(feature = "country-IS")]
    /// Iceland
    IS,
    #[cfg(feature = "country-IN")]
    /// India
    IN,
    #[cfg(feature = "country-ID")]
    /// Indonesia
    ID,
    #[cfg(feature = "country-IE")]
    /// Ireland
    IE,
    #[cfg(feature = "country-IM")]
    /// Isle of Man
    IM,
    #[cfg(feature = "country-IL")]
    /// Israel
    IL,
    #[cfg(feature = "country-IT")]
    /// Italy
    IT,
    #[cfg(feature = "country-JM")]
    /// Jamaica
    JM,
    #[cfg(feature = "country-JP")]
    /// Japan
    JP,
    #[cfg(feature = "country-KZ")]
    /// Kazakhstan
    KZ,
    #[cfg(feature = "country-KE")]
    /// Kenya
    KE,
    #[cfg(feature = "country-LV")]
    /// Latvia
    LV,
    #[cfg(feature = "country-LS")]
    /// Lesotho
    LS,
    #[cfg(feature = "country-LI")]
    /// Liechtenstein
    LI,
    #[cfg(feature = "country-LT")]
    /// Lithuania
    LT,
    #[cfg(feature = "country-LU")]
    /// Luxembourg
    LU,
    #[cfg(feature = "country-MG")]
    /// Madagascar
    MG,
    #[cfg(feature = "country-MY")]
    /// Malaysia
    MY,
    #[cfg(feature = "country-MW")]
    /// Malawi
    MW,
    #[cfg(feature = "country-MT")]
    /// Malta
    MT,
    #[cfg(feature = "country-MX")]
    /// Mexico
    MX,
    #[cfg(feature = "country-MD")]
    /// Moldova
    MD,
    #[cfg(feature = "country-MA")]
    /// Morocco
    MA,
    #[cfg(feature = "country-MZ")]
    /// Mozambique
    MZ,
    #[cfg(feature = "country-NL")]
    /// Netherlands
    NL,
    #[cfg(feature = "country-NA")]
    /// Namibia
    NA,
    #[cfg(feature = "country-NZ")]
    /// New Zealand
    NZ,
    #[cfg(feature = "country-NI")]
    /// Nicaragua
    NI,
    #[cfg(feature = "country-NG")]
    /// Nigeria
    NG,
    #[cfg(feature = "country-MK")]
    /// North Macedonia
    MK,
    #[cfg(feature = "country-NO")]
    /// Norway
    NO,
    #[cfg(feature = "country-PK")]
    /// Pakistan
    PK,
    #[cfg(feature = "country-PY")]
    /// Paraguay
    PY,
    #[cfg(feature = "country-PE")]
    /// Peru
    PE,
    #[cfg(feature = "country-PL")]
    /// Poland
    PL,
    #[cfg(feature = "country-PT")]
    /// Portugal
    PT,
    #[cfg(feature = "country-RO")]
    /// Romania
    RO,
    #[cfg(feature = "country-RU")]
    /// Russia
    RU,
    #[cfg(feature = "country-SA")]
    /// Saudi Arabia
    SA,
    #[cfg(feature = "country-RS")]
    /// Serbia
    RS,
    #[cfg(feature = "country-SG")]
    /// Singapore
    SG,
    #[cfg(feature = "country-SK")]
    /// Slovakia
    SK,
    #[cfg(feature = "country-SI")]
    /// Slovenia
    SI,
    #[cfg(feature = "country-ZA")]
    /// South Africa
    ZA,
    #[cfg(feature = "country-KR")]
    /// South Korea
    KR,
    #[cfg(feature = "country-ES")]
    /// Spain
    ES,
    #[cfg(feature = "country-SZ")]
    /// Swaziland
    SZ,
    #[cfg(feature = "country-SE")]
    /// Sweden
    SE,
    #[cfg(feature = "country-CH")]
    /// Switzerland
    CH,
    #[cfg(feature = "country-TW")]
    /// Taiwan
    TW,
    #[cfg(feature = "country-TR")]
    /// Turkey
    TR,
    #[cfg(feature = "country-TN")]
    /// Tunisia
    TN,
    #[cfg(feature = "country-UA")]
    /// Ukraine
    UA,
    #[cfg(feature = "country-AE")]
    /// United Arab Emirates
    AE,
    #[cfg(feature = "country-GB")]
    /// United Kingdom
    GB,
    #[cfg(feature = "country-US")]
    /// United States
    US,
    #[cfg(feature = "country-UY")]
    /// Uruguay
    UY,
    #[cfg(feature = "country-UZ")]
    /// Uzbekistan
    UZ,
    #[cfg(feature = "country-VE")]
    /// Venezuela
    VE,
    #[cfg(feature = "country-VN")]
    /// Vietnam
    VN,
    #[cfg(feature = "country-ZM")]
    /// Zambia
    ZM,
    #[cfg(feature = "country-ZW")]
    /// Zimbabwe
    ZW,
}

impl std::fmt::Display for Country {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl AsRef<str> for Country {
    fn as_ref(&self) -> &str {
        match self {
            #[cfg(feature = "country-AO")]
            Country::AO => "AO",
            #[cfg(feature = "country-AR")]
            Country::AR => "AR",
            #[cfg(feature = "country-AM")]
            Country::AM => "AM",
            #[cfg(feature = "country-AW")]
            Country::AW => "AW",
            #[cfg(feature = "country-AU")]
            Country::AU => "AU",
            #[cfg(feature = "country-AT")]
            Country::AT => "AT",
            #[cfg(feature = "country-AZ")]
            Country::AZ => "AZ",
            #[cfg(feature = "country-BD")]
            Country::BD => "BD",
            #[cfg(feature = "country-BY")]
            Country::BY => "BY",
            #[cfg(feature = "country-BE")]
            Country::BE => "BE",
            #[cfg(feature = "country-BO")]
            Country::BO => "BO",
            #[cfg(feature = "country-BA")]
            Country::BA => "BA",
            #[cfg(feature = "country-BW")]
            Country::BW => "BW",
            #[cfg(feature = "country-BR")]
            Country::BR => "BR",
            #[cfg(feature = "country-BG")]
            Country::BG => "BG",
            #[cfg(feature = "country-BI")]
            Country::BI => "BI",
            #[cfg(feature = "country-CA")]
            Country::CA => "CA",
            #[cfg(feature = "country-CL")]
            Country::CL => "CL",
            #[cfg(feature = "country-CN")]
            Country::CN => "CN",
            #[cfg(feature = "country-CO")]
            Country::CO => "CO",
            #[cfg(feature = "country-HR")]
            Country::HR => "HR",
            #[cfg(feature = "country-CU")]
            Country::CU => "CU",
            #[cfg(feature = "country-CW")]
            Country::CW => "CW",
            #[cfg(feature = "country-CY")]
            Country::CY => "CY",
            #[cfg(feature = "country-CZ")]
            Country::CZ => "CZ",
            #[cfg(feature = "country-DK")]
            Country::DK => "DK",
            #[cfg(feature = "country-DJ")]
            Country::DJ => "DJ",
            #[cfg(feature = "country-DO")]
            Country::DO => "DO",
            #[cfg(feature = "country-EG")]
            Country::EG => "EG",
            #[cfg(feature = "country-EE")]
            Country::EE => "EE",
            #[cfg(feature = "country-ET")]
            Country::ET => "ET",
            #[cfg(feature = "country-FI")]
            Country::FI => "FI",
            #[cfg(feature = "country-FR")]
            Country::FR => "FR",
            #[cfg(feature = "country-GE")]
            Country::GE => "GE",
            #[cfg(feature = "country-DE")]
            Country::DE => "DE",
            #[cfg(feature = "country-GR")]
            Country::GR => "GR",
            #[cfg(feature = "country-HN")]
            Country::HN => "HN",
            #[cfg(feature = "country-HK")]
            Country::HK => "HK",
            #[cfg(feature = "country-HU")]
            Country::HU => "HU",
            #[cfg(feature = "country-IS")]
            Country::IS => "IS",
            #[cfg(feature = "country-IN")]
            Country::IN => "IN",
            #[cfg(feature = "country-ID")]
            Country::ID => "ID",
            #[cfg(feature = "country-IE")]
            Country::IE => "IE",
            #[cfg(feature = "country-IM")]
            Country::IM => "IM",
            #[cfg(feature = "country-IL")]
            Country::IL => "IL",
            #[cfg(feature = "country-IT")]
            Country::IT => "IT",
            #[cfg(feature = "country-JM")]
            Country::JM => "JM",
            #[cfg(feature = "country-JP")]
            Country::JP => "JP",
            #[cfg(feature = "country-KZ")]
            Country::KZ => "KZ",
            #[cfg(feature = "country-KE")]
            Country::KE => "KE",
            #[cfg(feature = "country-LV")]
            Country::LV => "LV",
            #[cfg(feature = "country-LS")]
            Country::LS => "LS",
            #[cfg(feature = "country-LI")]
            Country::LI => "LI",
            #[cfg(feature = "country-LT")]
            Country::LT => "LT",
            #[cfg(feature = "country-LU")]
            Country::LU => "LU",
            #[cfg(feature = "country-MG")]
            Country::MG => "MG",
            #[cfg(feature = "country-MY")]
            Country::MY => "MY",
            #[cfg(feature = "country-MW")]
            Country::MW => "MW",
            #[cfg(feature = "country-MT")]
            Country::MT => "MT",
            #[cfg(feature = "country-MX")]
            Country::MX => "MX",
            #[cfg(feature = "country-MD")]
            Country::MD => "MD",
            #[cfg(feature = "country-MA")]
            Country::MA => "MA",
            #[cfg(feature = "country-MZ")]
            Country::MZ => "MZ",
            #[cfg(feature = "country-NL")]
            Country::NL => "NL",
            #[cfg(feature = "country-NA")]
            Country::NA => "NA",
            #[cfg(feature = "country-NZ")]
            Country::NZ => "NZ",
            #[cfg(feature = "country-NI")]
            Country::NI => "NI",
            #[cfg(feature = "country-NG")]
            Country::NG => "NG",
            #[cfg(feature = "country-MK")]
            Country::MK => "MK",
            #[cfg(feature = "country-NO")]
            Country::NO => "NO",
            #[cfg(feature = "country-PK")]
            Country::PK => "PK",
            #[cfg(feature = "country-PY")]
            Country::PY => "PY",
            #[cfg(feature = "country-PE")]
            Country::PE => "PE",
            #[cfg(feature = "country-PL")]
            Country::PL => "PL",
            #[cfg(feature = "country-PT")]
            Country::PT => "PT",
            #[cfg(feature = "country-RO")]
            Country::RO => "RO",
            #[cfg(feature = "country-RU")]
            Country::RU => "RU",
            #[cfg(feature = "country-SA")]
            Country::SA => "SA",
            #[cfg(feature = "country-RS")]
            Country::RS => "RS",
            #[cfg(feature = "country-SG")]
            Country::SG => "SG",
            #[cfg(feature = "country-SK")]
            Country::SK => "SK",
            #[cfg(feature = "country-SI")]
            Country::SI => "SI",
            #[cfg(feature = "country-ZA")]
            Country::ZA => "ZA",
            #[cfg(feature = "country-KR")]
            Country::KR => "KR",
            #[cfg(feature = "country-ES")]
            Country::ES => "ES",
            #[cfg(feature = "country-SZ")]
            Country::SZ => "SZ",
            #[cfg(feature = "country-SE")]
            Country::SE => "SE",
            #[cfg(feature = "country-CH")]
            Country::CH => "CH",
            #[cfg(feature = "country-TW")]
            Country::TW => "TW",
            #[cfg(feature = "country-TR")]
            Country::TR => "TR",
            #[cfg(feature = "country-TN")]
            Country::TN => "TN",
            #[cfg(feature = "country-UA")]
            Country::UA => "UA",
            #[cfg(feature = "country-AE")]
            Country::AE => "AE",
            #[cfg(feature = "country-GB")]
            Country::GB => "GB",
            #[cfg(feature = "country-US")]
            Country::US => "US",
            #[cfg(feature = "country-UY")]
            Country::UY => "UY",
            #[cfg(feature = "country-UZ")]
            Country::UZ => "UZ",
            #[cfg(feature = "country-VE")]
            Country::VE => "VE",
            #[cfg(feature = "country-VN")]
            Country::VN => "VN",
            #[cfg(feature = "country-ZM")]
            Country::ZM => "ZM",
            #[cfg(feature = "country-ZW")]
            Country::ZW => "ZW",
        }
    }
}

impl std::str::FromStr for Country {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(match s {
            #[cfg(feature = "country-AO")]
            "AO" => Country::AO,
            #[cfg(feature = "country-AR")]
            "AR" => Country::AR,
            #[cfg(feature = "country-AM")]
            "AM" => Country::AM,
            #[cfg(feature = "country-AW")]
            "AW" => Country::AW,
            #[cfg(feature = "country-AU")]
            "AU" => Country::AU,
            #[cfg(feature = "country-AT")]
            "AT" => Country::AT,
            #[cfg(feature = "country-AZ")]
            "AZ" => Country::AZ,
            #[cfg(feature = "country-BD")]
            "BD" => Country::BD,
            #[cfg(feature = "country-BY")]
            "BY" => Country::BY,
            #[cfg(feature = "country-BE")]
            "BE" => Country::BE,
            #[cfg(feature = "country-BO")]
            "BO" => Country::BO,
            #[cfg(feature = "country-BA")]
            "BA" => Country::BA,
            #[cfg(feature = "country-BW")]
            "BW" => Country::BW,
            #[cfg(feature = "country-BR")]
            "BR" => Country::BR,
            #[cfg(feature = "country-BG")]
            "BG" => Country::BG,
            #[cfg(feature = "country-BI")]
            "BI" => Country::BI,
            #[cfg(feature = "country-CA")]
            "CA" => Country::CA,
            #[cfg(feature = "country-CL")]
            "CL" => Country::CL,
            #[cfg(feature = "country-CN")]
            "CN" => Country::CN,
            #[cfg(feature = "country-CO")]
            "CO" => Country::CO,
            #[cfg(feature = "country-HR")]
            "HR" => Country::HR,
            #[cfg(feature = "country-CU")]
            "CU" => Country::CU,
            #[cfg(feature = "country-CW")]
            "CW" => Country::CW,
            #[cfg(feature = "country-CY")]
            "CY" => Country::CY,
            #[cfg(feature = "country-CZ")]
            "CZ" => Country::CZ,
            #[cfg(feature = "country-DK")]
            "DK" => Country::DK,
            #[cfg(feature = "country-DJ")]
            "DJ" => Country::DJ,
            #[cfg(feature = "country-DO")]
            "DO" => Country::DO,
            #[cfg(feature = "country-EG")]
            "EG" => Country::EG,
            #[cfg(feature = "country-EE")]
            "EE" => Country::EE,
            #[cfg(feature = "country-ET")]
            "ET" => Country::ET,
            #[cfg(feature = "country-FI")]
            "FI" => Country::FI,
            #[cfg(feature = "country-FR")]
            "FR" => Country::FR,
            #[cfg(feature = "country-GE")]
            "GE" => Country::GE,
            #[cfg(feature = "country-DE")]
            "DE" => Country::DE,
            #[cfg(feature = "country-GR")]
            "GR" => Country::GR,
            #[cfg(feature = "country-HN")]
            "HN" => Country::HN,
            #[cfg(feature = "country-HK")]
            "HK" => Country::HK,
            #[cfg(feature = "country-HU")]
            "HU" => Country::HU,
            #[cfg(feature = "country-IS")]
            "IS" => Country::IS,
            #[cfg(feature = "country-IN")]
            "IN" => Country::IN,
            #[cfg(feature = "country-ID")]
            "ID" => Country::ID,
            #[cfg(feature = "country-IE")]
            "IE" => Country::IE,
            #[cfg(feature = "country-IM")]
            "IM" => Country::IM,
            #[cfg(feature = "country-IL")]
            "IL" => Country::IL,
            #[cfg(feature = "country-IT")]
            "IT" => Country::IT,
            #[cfg(feature = "country-JM")]
            "JM" => Country::JM,
            #[cfg(feature = "country-JP")]
            "JP" => Country::JP,
            #[cfg(feature = "country-KZ")]
            "KZ" => Country::KZ,
            #[cfg(feature = "country-KE")]
            "KE" => Country::KE,
            #[cfg(feature = "country-LV")]
            "LV" => Country::LV,
            #[cfg(feature = "country-LS")]
            "LS" => Country::LS,
            #[cfg(feature = "country-LI")]
            "LI" => Country::LI,
            #[cfg(feature = "country-LT")]
            "LT" => Country::LT,
            #[cfg(feature = "country-LU")]
            "LU" => Country::LU,
            #[cfg(feature = "country-MG")]
            "MG" => Country::MG,
            #[cfg(feature = "country-MY")]
            "MY" => Country::MY,
            #[cfg(feature = "country-MW")]
            "MW" => Country::MW,
            #[cfg(feature = "country-MT")]
            "MT" => Country::MT,
            #[cfg(feature = "country-MX")]
            "MX" => Country::MX,
            #[cfg(feature = "country-MD")]
            "MD" => Country::MD,
            #[cfg(feature = "country-MA")]
            "MA" => Country::MA,
            #[cfg(feature = "country-MZ")]
            "MZ" => Country::MZ,
            #[cfg(feature = "country-NL")]
            "NL" => Country::NL,
            #[cfg(feature = "country-NA")]
            "NA" => Country::NA,
            #[cfg(feature = "country-NZ")]
            "NZ" => Country::NZ,
            #[cfg(feature = "country-NI")]
            "NI" => Country::NI,
            #[cfg(feature = "country-NG")]
            "NG" => Country::NG,
            #[cfg(feature = "country-MK")]
            "MK" => Country::MK,
            #[cfg(feature = "country-NO")]
            "NO" => Country::NO,
            #[cfg(feature = "country-PK")]
            "PK" => Country::PK,
            #[cfg(feature = "country-PY")]
            "PY" => Country::PY,
            #[cfg(feature = "country-PE")]
            "PE" => Country::PE,
            #[cfg(feature = "country-PL")]
            "PL" => Country::PL,
            #[cfg(feature = "country-PT")]
            "PT" => Country::PT,
            #[cfg(feature = "country-RO")]
            "RO" => Country::RO,
            #[cfg(feature = "country-RU")]
            "RU" => Country::RU,
            #[cfg(feature = "country-SA")]
            "SA" => Country::SA,
            #[cfg(feature = "country-RS")]
            "RS" => Country::RS,
            #[cfg(feature = "country-SG")]
            "SG" => Country::SG,
            #[cfg(feature = "country-SK")]
            "SK" => Country::SK,
            #[cfg(feature = "country-SI")]
            "SI" => Country::SI,
            #[cfg(feature = "country-ZA")]
            "ZA" => Country::ZA,
            #[cfg(feature = "country-KR")]
            "KR" => Country::KR,
            #[cfg(feature = "country-ES")]
            "ES" => Country::ES,
            #[cfg(feature = "country-SZ")]
            "SZ" => Country::SZ,
            #[cfg(feature = "country-SE")]
            "SE" => Country::SE,
            #[cfg(feature = "country-CH")]
            "CH" => Country::CH,
            #[cfg(feature = "country-TW")]
            "TW" => Country::TW,
            #[cfg(feature = "country-TR")]
            "TR" => Country::TR,
            #[cfg(feature = "country-TN")]
            "TN" => Country::TN,
            #[cfg(feature = "country-UA")]
            "UA" => Country::UA,
            #[cfg(feature = "country-AE")]
            "AE" => Country::AE,
            #[cfg(feature = "country-GB")]
            "GB" => Country::GB,
            #[cfg(feature = "country-US")]
            "US" => Country::US,
            #[cfg(feature = "country-UY")]
            "UY" => Country::UY,
            #[cfg(feature = "country-UZ")]
            "UZ" => Country::UZ,
            #[cfg(feature = "country-VE")]
            "VE" => Country::VE,
            #[cfg(feature = "country-VN")]
            "VN" => Country::VN,
            #[cfg(feature = "country-ZM")]
            "ZM" => Country::ZM,
            #[cfg(feature = "country-ZW")]
            "ZW" => Country::ZW,
            _ => return Err(Error::CountryNotAvailable),
        })
    }
}

#[cfg(feature = "country-AO")]
pub mod ao;

#[cfg(feature = "country-AR")]
pub mod ar;

#[cfg(feature = "country-AM")]
pub mod am;

#[cfg(feature = "country-AW")]
pub mod aw;

#[cfg(feature = "country-AU")]
pub mod au;

#[cfg(feature = "country-AT")]
pub mod at;

#[cfg(feature = "country-AZ")]
pub mod az;

#[cfg(feature = "country-BD")]
pub mod bd;

#[cfg(feature = "country-BY")]
pub mod by;

#[cfg(feature = "country-BE")]
pub mod be;

#[cfg(feature = "country-BO")]
pub mod bo;

#[cfg(feature = "country-BA")]
pub mod ba;

#[cfg(feature = "country-BW")]
pub mod bw;

#[cfg(feature = "country-BR")]
pub mod br;

#[cfg(feature = "country-BG")]
pub mod bg;

#[cfg(feature = "country-BI")]
pub mod bi;

#[cfg(feature = "country-CA")]
pub mod ca;

#[cfg(feature = "country-CL")]
pub mod cl;

#[cfg(feature = "country-CN")]
pub mod cn;

#[cfg(feature = "country-CO")]
pub mod co;

#[cfg(feature = "country-HR")]
pub mod hr;

#[cfg(feature = "country-CU")]
pub mod cu;

#[cfg(feature = "country-CW")]
pub mod cw;

#[cfg(feature = "country-CY")]
pub mod cy;

#[cfg(feature = "country-CZ")]
pub mod cz;

#[cfg(feature = "country-DK")]
pub mod dk;

#[cfg(feature = "country-DJ")]
pub mod dj;

#[cfg(feature = "country-DO")]
pub mod r#do;

#[cfg(feature = "country-EG")]
pub mod eg;

#[cfg(feature = "country-EE")]
pub mod ee;

#[cfg(feature = "country-ET")]
pub mod et;

#[cfg(feature = "country-FI")]
pub mod fi;

#[cfg(feature = "country-FR")]
pub mod fr;

#[cfg(feature = "country-GE")]
pub mod ge;

#[cfg(feature = "country-DE")]
pub mod de;

#[cfg(feature = "country-GR")]
pub mod gr;

#[cfg(feature = "country-HN")]
pub mod hn;

#[cfg(feature = "country-HK")]
pub mod hk;

#[cfg(feature = "country-HU")]
pub mod hu;

#[cfg(feature = "country-IS")]
pub mod is;

#[cfg(feature = "country-IN")]
pub mod r#in;

#[cfg(feature = "country-ID")]
pub mod id;

#[cfg(feature = "country-IE")]
pub mod ie;

#[cfg(feature = "country-IM")]
pub mod im;

#[cfg(feature = "country-IL")]
pub mod il;

#[cfg(feature = "country-IT")]
pub mod it;

#[cfg(feature = "country-JM")]
pub mod jm;

#[cfg(feature = "country-JP")]
pub mod jp;

#[cfg(feature = "country-KZ")]
pub mod kz;

#[cfg(feature = "country-KE")]
pub mod ke;

#[cfg(feature = "country-LV")]
pub mod lv;

#[cfg(feature = "country-LS")]
pub mod ls;

#[cfg(feature = "country-LI")]
pub mod li;

#[cfg(feature = "country-LT")]
pub mod lt;

#[cfg(feature = "country-LU")]
pub mod lu;

#[cfg(feature = "country-MG")]
pub mod mg;

#[cfg(feature = "country-MY")]
pub mod my;

#[cfg(feature = "country-MW")]
pub mod mw;

#[cfg(feature = "country-MT")]
pub mod mt;

#[cfg(feature = "country-MX")]
pub mod mx;

#[cfg(feature = "country-MD")]
pub mod md;

#[cfg(feature = "country-MA")]
pub mod ma;

#[cfg(feature = "country-MZ")]
pub mod mz;

#[cfg(feature = "country-NL")]
pub mod nl;

#[cfg(feature = "country-NA")]
pub mod na;

#[cfg(feature = "country-NZ")]
pub mod nz;

#[cfg(feature = "country-NI")]
pub mod ni;

#[cfg(feature = "country-NG")]
pub mod ng;

#[cfg(feature = "country-MK")]
pub mod mk;

#[cfg(feature = "country-NO")]
pub mod no;

#[cfg(feature = "country-PK")]
pub mod pk;

#[cfg(feature = "country-PY")]
pub mod py;

#[cfg(feature = "country-PE")]
pub mod pe;

#[cfg(feature = "country-PL")]
pub mod pl;

#[cfg(feature = "country-PT")]
pub mod pt;

#[cfg(feature = "country-RO")]
pub mod ro;

#[cfg(feature = "country-RU")]
pub mod ru;

#[cfg(feature = "country-SA")]
pub mod sa;

#[cfg(feature = "country-RS")]
pub mod rs;

#[cfg(feature = "country-SG")]
pub mod sg;

#[cfg(feature = "country-SK")]
pub mod sk;

#[cfg(feature = "country-SI")]
pub mod si;

#[cfg(feature = "country-ZA")]
pub mod za;

#[cfg(feature = "country-KR")]
pub mod kr;

#[cfg(feature = "country-ES")]
pub mod es;

#[cfg(feature = "country-SZ")]
pub mod sz;

#[cfg(feature = "country-SE")]
pub mod se;

#[cfg(feature = "country-CH")]
pub mod ch;

#[cfg(feature = "country-TW")]
pub mod tw;

#[cfg(feature = "country-TR")]
pub mod tr;

#[cfg(feature = "country-TN")]
pub mod tn;

#[cfg(feature = "country-UA")]
pub mod ua;

#[cfg(feature = "country-AE")]
pub mod ae;

#[cfg(feature = "country-GB")]
pub mod gb;

#[cfg(feature = "country-US")]
pub mod us;

#[cfg(feature = "country-UY")]
pub mod uy;

#[cfg(feature = "country-UZ")]
pub mod uz;

#[cfg(feature = "country-VE")]
pub mod ve;

#[cfg(feature = "country-VN")]
pub mod vn;

#[cfg(feature = "country-ZM")]
pub mod zm;

#[cfg(feature = "country-ZW")]
pub mod zw;
