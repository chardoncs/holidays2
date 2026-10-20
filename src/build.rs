
use crate::{prelude::*, HolidayMap, Result, Year};

/// Generate the holiday map for every compiled-in country.
pub fn build(years: Option<&std::ops::Range<Year>>) -> Result<HolidayMap> {
  let mut map = HolidayMap::new();

  #[cfg(feature = "country-AO")]
  map.insert(Country::AO, ao::build(&years)?);

  #[cfg(feature = "country-AR")]
  map.insert(Country::AR, ar::build(&years)?);

  #[cfg(feature = "country-AM")]
  map.insert(Country::AM, am::build(&years)?);

  #[cfg(feature = "country-AW")]
  map.insert(Country::AW, aw::build(&years)?);

  #[cfg(feature = "country-AU")]
  map.insert(Country::AU, au::build(&years)?);

  #[cfg(feature = "country-AT")]
  map.insert(Country::AT, at::build(&years)?);

  #[cfg(feature = "country-AZ")]
  map.insert(Country::AZ, az::build(&years)?);

  #[cfg(feature = "country-BD")]
  map.insert(Country::BD, bd::build(&years)?);

  #[cfg(feature = "country-BY")]
  map.insert(Country::BY, by::build(&years)?);

  #[cfg(feature = "country-BE")]
  map.insert(Country::BE, be::build(&years)?);

  #[cfg(feature = "country-BO")]
  map.insert(Country::BO, bo::build(&years)?);

  #[cfg(feature = "country-BA")]
  map.insert(Country::BA, ba::build(&years)?);

  #[cfg(feature = "country-BW")]
  map.insert(Country::BW, bw::build(&years)?);

  #[cfg(feature = "country-BR")]
  map.insert(Country::BR, br::build(&years)?);

  #[cfg(feature = "country-BG")]
  map.insert(Country::BG, bg::build(&years)?);

  #[cfg(feature = "country-BI")]
  map.insert(Country::BI, bi::build(&years)?);

  #[cfg(feature = "country-CA")]
  map.insert(Country::CA, ca::build(&years)?);

  #[cfg(feature = "country-CL")]
  map.insert(Country::CL, cl::build(&years)?);

  #[cfg(feature = "country-CN")]
  map.insert(Country::CN, cn::build(&years)?);

  #[cfg(feature = "country-CO")]
  map.insert(Country::CO, co::build(&years)?);

  #[cfg(feature = "country-HR")]
  map.insert(Country::HR, hr::build(&years)?);

  #[cfg(feature = "country-CU")]
  map.insert(Country::CU, cu::build(&years)?);

  #[cfg(feature = "country-CW")]
  map.insert(Country::CW, cw::build(&years)?);

  #[cfg(feature = "country-CY")]
  map.insert(Country::CY, cy::build(&years)?);

  #[cfg(feature = "country-CZ")]
  map.insert(Country::CZ, cz::build(&years)?);

  #[cfg(feature = "country-DK")]
  map.insert(Country::DK, dk::build(&years)?);

  #[cfg(feature = "country-DJ")]
  map.insert(Country::DJ, dj::build(&years)?);

  #[cfg(feature = "country-DO")]
  map.insert(Country::DO, r#do::build(&years)?);

  #[cfg(feature = "country-EG")]
  map.insert(Country::EG, eg::build(&years)?);

  #[cfg(feature = "country-EE")]
  map.insert(Country::EE, ee::build(&years)?);

  #[cfg(feature = "country-ET")]
  map.insert(Country::ET, et::build(&years)?);

  #[cfg(feature = "country-FI")]
  map.insert(Country::FI, fi::build(&years)?);

  #[cfg(feature = "country-FR")]
  map.insert(Country::FR, fr::build(&years)?);

  #[cfg(feature = "country-GE")]
  map.insert(Country::GE, ge::build(&years)?);

  #[cfg(feature = "country-DE")]
  map.insert(Country::DE, de::build(&years)?);

  #[cfg(feature = "country-GR")]
  map.insert(Country::GR, gr::build(&years)?);

  #[cfg(feature = "country-HN")]
  map.insert(Country::HN, hn::build(&years)?);

  #[cfg(feature = "country-HK")]
  map.insert(Country::HK, hk::build(&years)?);

  #[cfg(feature = "country-HU")]
  map.insert(Country::HU, hu::build(&years)?);

  #[cfg(feature = "country-IS")]
  map.insert(Country::IS, is::build(&years)?);

  #[cfg(feature = "country-IN")]
  map.insert(Country::IN, r#in::build(&years)?);

  #[cfg(feature = "country-ID")]
  map.insert(Country::ID, id::build(&years)?);

  #[cfg(feature = "country-IE")]
  map.insert(Country::IE, ie::build(&years)?);

  #[cfg(feature = "country-IM")]
  map.insert(Country::IM, im::build(&years)?);

  #[cfg(feature = "country-IL")]
  map.insert(Country::IL, il::build(&years)?);

  #[cfg(feature = "country-IT")]
  map.insert(Country::IT, it::build(&years)?);

  #[cfg(feature = "country-JM")]
  map.insert(Country::JM, jm::build(&years)?);

  #[cfg(feature = "country-JP")]
  map.insert(Country::JP, jp::build(&years)?);

  #[cfg(feature = "country-KZ")]
  map.insert(Country::KZ, kz::build(&years)?);

  #[cfg(feature = "country-KE")]
  map.insert(Country::KE, ke::build(&years)?);

  #[cfg(feature = "country-LV")]
  map.insert(Country::LV, lv::build(&years)?);

  #[cfg(feature = "country-LS")]
  map.insert(Country::LS, ls::build(&years)?);

  #[cfg(feature = "country-LI")]
  map.insert(Country::LI, li::build(&years)?);

  #[cfg(feature = "country-LT")]
  map.insert(Country::LT, lt::build(&years)?);

  #[cfg(feature = "country-LU")]
  map.insert(Country::LU, lu::build(&years)?);

  #[cfg(feature = "country-MG")]
  map.insert(Country::MG, mg::build(&years)?);

  #[cfg(feature = "country-MY")]
  map.insert(Country::MY, my::build(&years)?);

  #[cfg(feature = "country-MW")]
  map.insert(Country::MW, mw::build(&years)?);

  #[cfg(feature = "country-MT")]
  map.insert(Country::MT, mt::build(&years)?);

  #[cfg(feature = "country-MX")]
  map.insert(Country::MX, mx::build(&years)?);

  #[cfg(feature = "country-MD")]
  map.insert(Country::MD, md::build(&years)?);

  #[cfg(feature = "country-MA")]
  map.insert(Country::MA, ma::build(&years)?);

  #[cfg(feature = "country-MZ")]
  map.insert(Country::MZ, mz::build(&years)?);

  #[cfg(feature = "country-NL")]
  map.insert(Country::NL, nl::build(&years)?);

  #[cfg(feature = "country-NA")]
  map.insert(Country::NA, na::build(&years)?);

  #[cfg(feature = "country-NZ")]
  map.insert(Country::NZ, nz::build(&years)?);

  #[cfg(feature = "country-NI")]
  map.insert(Country::NI, ni::build(&years)?);

  #[cfg(feature = "country-NG")]
  map.insert(Country::NG, ng::build(&years)?);

  #[cfg(feature = "country-MK")]
  map.insert(Country::MK, mk::build(&years)?);

  #[cfg(feature = "country-NO")]
  map.insert(Country::NO, no::build(&years)?);

  #[cfg(feature = "country-PK")]
  map.insert(Country::PK, pk::build(&years)?);

  #[cfg(feature = "country-PY")]
  map.insert(Country::PY, py::build(&years)?);

  #[cfg(feature = "country-PE")]
  map.insert(Country::PE, pe::build(&years)?);

  #[cfg(feature = "country-PL")]
  map.insert(Country::PL, pl::build(&years)?);

  #[cfg(feature = "country-PT")]
  map.insert(Country::PT, pt::build(&years)?);

  #[cfg(feature = "country-RO")]
  map.insert(Country::RO, ro::build(&years)?);

  #[cfg(feature = "country-RU")]
  map.insert(Country::RU, ru::build(&years)?);

  #[cfg(feature = "country-SA")]
  map.insert(Country::SA, sa::build(&years)?);

  #[cfg(feature = "country-RS")]
  map.insert(Country::RS, rs::build(&years)?);

  #[cfg(feature = "country-SG")]
  map.insert(Country::SG, sg::build(&years)?);

  #[cfg(feature = "country-SK")]
  map.insert(Country::SK, sk::build(&years)?);

  #[cfg(feature = "country-SI")]
  map.insert(Country::SI, si::build(&years)?);

  #[cfg(feature = "country-ZA")]
  map.insert(Country::ZA, za::build(&years)?);

  #[cfg(feature = "country-KR")]
  map.insert(Country::KR, kr::build(&years)?);

  #[cfg(feature = "country-ES")]
  map.insert(Country::ES, es::build(&years)?);

  #[cfg(feature = "country-SZ")]
  map.insert(Country::SZ, sz::build(&years)?);

  #[cfg(feature = "country-SE")]
  map.insert(Country::SE, se::build(&years)?);

  #[cfg(feature = "country-CH")]
  map.insert(Country::CH, ch::build(&years)?);

  #[cfg(feature = "country-TW")]
  map.insert(Country::TW, tw::build(&years)?);

  #[cfg(feature = "country-TR")]
  map.insert(Country::TR, tr::build(&years)?);

  #[cfg(feature = "country-TN")]
  map.insert(Country::TN, tn::build(&years)?);

  #[cfg(feature = "country-UA")]
  map.insert(Country::UA, ua::build(&years)?);

  #[cfg(feature = "country-AE")]
  map.insert(Country::AE, ae::build(&years)?);

  #[cfg(feature = "country-GB")]
  map.insert(Country::GB, gb::build(&years)?);

  #[cfg(feature = "country-US")]
  map.insert(Country::US, us::build(&years)?);

  #[cfg(feature = "country-UY")]
  map.insert(Country::UY, uy::build(&years)?);

  #[cfg(feature = "country-UZ")]
  map.insert(Country::UZ, uz::build(&years)?);

  #[cfg(feature = "country-VE")]
  map.insert(Country::VE, ve::build(&years)?);

  #[cfg(feature = "country-VN")]
  map.insert(Country::VN, vn::build(&years)?);

  #[cfg(feature = "country-ZM")]
  map.insert(Country::ZM, zm::build(&years)?);

  #[cfg(feature = "country-ZW")]
  map.insert(Country::ZW, zw::build(&years)?);

  Ok(map)
}
