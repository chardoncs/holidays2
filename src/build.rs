
use std::collections::HashSet;

use crate::{prelude::*, HolidayMap, Result, Year};

/// Generate holiday map for the specified countries and years.
pub fn build(countries: Option<&HashSet<Country>>, years: Option<&std::ops::Range<Year>>) -> Result<HolidayMap> {
  let mut map = HolidayMap::new();

  #[cfg(feature = "country-AO")]
  if countries.is_none() || countries.unwrap().contains(&Country::AO) {
      map.insert(Country::AO, ao::build(&years)?);
  }

  #[cfg(feature = "country-AR")]
  if countries.is_none() || countries.unwrap().contains(&Country::AR) {
      map.insert(Country::AR, ar::build(&years)?);
  }

  #[cfg(feature = "country-AM")]
  if countries.is_none() || countries.unwrap().contains(&Country::AM) {
      map.insert(Country::AM, am::build(&years)?);
  }

  #[cfg(feature = "country-AW")]
  if countries.is_none() || countries.unwrap().contains(&Country::AW) {
      map.insert(Country::AW, aw::build(&years)?);
  }

  #[cfg(feature = "country-AU")]
  if countries.is_none() || countries.unwrap().contains(&Country::AU) {
      map.insert(Country::AU, au::build(&years)?);
  }

  #[cfg(feature = "country-AT")]
  if countries.is_none() || countries.unwrap().contains(&Country::AT) {
      map.insert(Country::AT, at::build(&years)?);
  }

  #[cfg(feature = "country-AZ")]
  if countries.is_none() || countries.unwrap().contains(&Country::AZ) {
      map.insert(Country::AZ, az::build(&years)?);
  }

  #[cfg(feature = "country-BD")]
  if countries.is_none() || countries.unwrap().contains(&Country::BD) {
      map.insert(Country::BD, bd::build(&years)?);
  }

  #[cfg(feature = "country-BY")]
  if countries.is_none() || countries.unwrap().contains(&Country::BY) {
      map.insert(Country::BY, by::build(&years)?);
  }

  #[cfg(feature = "country-BE")]
  if countries.is_none() || countries.unwrap().contains(&Country::BE) {
      map.insert(Country::BE, be::build(&years)?);
  }

  #[cfg(feature = "country-BO")]
  if countries.is_none() || countries.unwrap().contains(&Country::BO) {
      map.insert(Country::BO, bo::build(&years)?);
  }

  #[cfg(feature = "country-BA")]
  if countries.is_none() || countries.unwrap().contains(&Country::BA) {
      map.insert(Country::BA, ba::build(&years)?);
  }

  #[cfg(feature = "country-BW")]
  if countries.is_none() || countries.unwrap().contains(&Country::BW) {
      map.insert(Country::BW, bw::build(&years)?);
  }

  #[cfg(feature = "country-BR")]
  if countries.is_none() || countries.unwrap().contains(&Country::BR) {
      map.insert(Country::BR, br::build(&years)?);
  }

  #[cfg(feature = "country-BG")]
  if countries.is_none() || countries.unwrap().contains(&Country::BG) {
      map.insert(Country::BG, bg::build(&years)?);
  }

  #[cfg(feature = "country-BI")]
  if countries.is_none() || countries.unwrap().contains(&Country::BI) {
      map.insert(Country::BI, bi::build(&years)?);
  }

  #[cfg(feature = "country-CA")]
  if countries.is_none() || countries.unwrap().contains(&Country::CA) {
      map.insert(Country::CA, ca::build(&years)?);
  }

  #[cfg(feature = "country-CL")]
  if countries.is_none() || countries.unwrap().contains(&Country::CL) {
      map.insert(Country::CL, cl::build(&years)?);
  }

  #[cfg(feature = "country-CN")]
  if countries.is_none() || countries.unwrap().contains(&Country::CN) {
      map.insert(Country::CN, cn::build(&years)?);
  }

  #[cfg(feature = "country-CO")]
  if countries.is_none() || countries.unwrap().contains(&Country::CO) {
      map.insert(Country::CO, co::build(&years)?);
  }

  #[cfg(feature = "country-HR")]
  if countries.is_none() || countries.unwrap().contains(&Country::HR) {
      map.insert(Country::HR, hr::build(&years)?);
  }

  #[cfg(feature = "country-CU")]
  if countries.is_none() || countries.unwrap().contains(&Country::CU) {
      map.insert(Country::CU, cu::build(&years)?);
  }

  #[cfg(feature = "country-CW")]
  if countries.is_none() || countries.unwrap().contains(&Country::CW) {
      map.insert(Country::CW, cw::build(&years)?);
  }

  #[cfg(feature = "country-CY")]
  if countries.is_none() || countries.unwrap().contains(&Country::CY) {
      map.insert(Country::CY, cy::build(&years)?);
  }

  #[cfg(feature = "country-CZ")]
  if countries.is_none() || countries.unwrap().contains(&Country::CZ) {
      map.insert(Country::CZ, cz::build(&years)?);
  }

  #[cfg(feature = "country-DK")]
  if countries.is_none() || countries.unwrap().contains(&Country::DK) {
      map.insert(Country::DK, dk::build(&years)?);
  }

  #[cfg(feature = "country-DJ")]
  if countries.is_none() || countries.unwrap().contains(&Country::DJ) {
      map.insert(Country::DJ, dj::build(&years)?);
  }

  #[cfg(feature = "country-DO")]
  if countries.is_none() || countries.unwrap().contains(&Country::DO) {
      map.insert(Country::DO, r#do::build(&years)?);
  }

  #[cfg(feature = "country-EG")]
  if countries.is_none() || countries.unwrap().contains(&Country::EG) {
      map.insert(Country::EG, eg::build(&years)?);
  }

  #[cfg(feature = "country-EE")]
  if countries.is_none() || countries.unwrap().contains(&Country::EE) {
      map.insert(Country::EE, ee::build(&years)?);
  }

  #[cfg(feature = "country-ET")]
  if countries.is_none() || countries.unwrap().contains(&Country::ET) {
      map.insert(Country::ET, et::build(&years)?);
  }

  #[cfg(feature = "country-FI")]
  if countries.is_none() || countries.unwrap().contains(&Country::FI) {
      map.insert(Country::FI, fi::build(&years)?);
  }

  #[cfg(feature = "country-FR")]
  if countries.is_none() || countries.unwrap().contains(&Country::FR) {
      map.insert(Country::FR, fr::build(&years)?);
  }

  #[cfg(feature = "country-GE")]
  if countries.is_none() || countries.unwrap().contains(&Country::GE) {
      map.insert(Country::GE, ge::build(&years)?);
  }

  #[cfg(feature = "country-DE")]
  if countries.is_none() || countries.unwrap().contains(&Country::DE) {
      map.insert(Country::DE, de::build(&years)?);
  }

  #[cfg(feature = "country-GR")]
  if countries.is_none() || countries.unwrap().contains(&Country::GR) {
      map.insert(Country::GR, gr::build(&years)?);
  }

  #[cfg(feature = "country-HN")]
  if countries.is_none() || countries.unwrap().contains(&Country::HN) {
      map.insert(Country::HN, hn::build(&years)?);
  }

  #[cfg(feature = "country-HK")]
  if countries.is_none() || countries.unwrap().contains(&Country::HK) {
      map.insert(Country::HK, hk::build(&years)?);
  }

  #[cfg(feature = "country-HU")]
  if countries.is_none() || countries.unwrap().contains(&Country::HU) {
      map.insert(Country::HU, hu::build(&years)?);
  }

  #[cfg(feature = "country-IS")]
  if countries.is_none() || countries.unwrap().contains(&Country::IS) {
      map.insert(Country::IS, is::build(&years)?);
  }

  #[cfg(feature = "country-IN")]
  if countries.is_none() || countries.unwrap().contains(&Country::IN) {
      map.insert(Country::IN, r#in::build(&years)?);
  }

  #[cfg(feature = "country-ID")]
  if countries.is_none() || countries.unwrap().contains(&Country::ID) {
      map.insert(Country::ID, id::build(&years)?);
  }

  #[cfg(feature = "country-IE")]
  if countries.is_none() || countries.unwrap().contains(&Country::IE) {
      map.insert(Country::IE, ie::build(&years)?);
  }

  #[cfg(feature = "country-IM")]
  if countries.is_none() || countries.unwrap().contains(&Country::IM) {
      map.insert(Country::IM, im::build(&years)?);
  }

  #[cfg(feature = "country-IL")]
  if countries.is_none() || countries.unwrap().contains(&Country::IL) {
      map.insert(Country::IL, il::build(&years)?);
  }

  #[cfg(feature = "country-IT")]
  if countries.is_none() || countries.unwrap().contains(&Country::IT) {
      map.insert(Country::IT, it::build(&years)?);
  }

  #[cfg(feature = "country-JM")]
  if countries.is_none() || countries.unwrap().contains(&Country::JM) {
      map.insert(Country::JM, jm::build(&years)?);
  }

  #[cfg(feature = "country-JP")]
  if countries.is_none() || countries.unwrap().contains(&Country::JP) {
      map.insert(Country::JP, jp::build(&years)?);
  }

  #[cfg(feature = "country-KZ")]
  if countries.is_none() || countries.unwrap().contains(&Country::KZ) {
      map.insert(Country::KZ, kz::build(&years)?);
  }

  #[cfg(feature = "country-KE")]
  if countries.is_none() || countries.unwrap().contains(&Country::KE) {
      map.insert(Country::KE, ke::build(&years)?);
  }

  #[cfg(feature = "country-LV")]
  if countries.is_none() || countries.unwrap().contains(&Country::LV) {
      map.insert(Country::LV, lv::build(&years)?);
  }

  #[cfg(feature = "country-LS")]
  if countries.is_none() || countries.unwrap().contains(&Country::LS) {
      map.insert(Country::LS, ls::build(&years)?);
  }

  #[cfg(feature = "country-LI")]
  if countries.is_none() || countries.unwrap().contains(&Country::LI) {
      map.insert(Country::LI, li::build(&years)?);
  }

  #[cfg(feature = "country-LT")]
  if countries.is_none() || countries.unwrap().contains(&Country::LT) {
      map.insert(Country::LT, lt::build(&years)?);
  }

  #[cfg(feature = "country-LU")]
  if countries.is_none() || countries.unwrap().contains(&Country::LU) {
      map.insert(Country::LU, lu::build(&years)?);
  }

  #[cfg(feature = "country-MG")]
  if countries.is_none() || countries.unwrap().contains(&Country::MG) {
      map.insert(Country::MG, mg::build(&years)?);
  }

  #[cfg(feature = "country-MY")]
  if countries.is_none() || countries.unwrap().contains(&Country::MY) {
      map.insert(Country::MY, my::build(&years)?);
  }

  #[cfg(feature = "country-MW")]
  if countries.is_none() || countries.unwrap().contains(&Country::MW) {
      map.insert(Country::MW, mw::build(&years)?);
  }

  #[cfg(feature = "country-MT")]
  if countries.is_none() || countries.unwrap().contains(&Country::MT) {
      map.insert(Country::MT, mt::build(&years)?);
  }

  #[cfg(feature = "country-MX")]
  if countries.is_none() || countries.unwrap().contains(&Country::MX) {
      map.insert(Country::MX, mx::build(&years)?);
  }

  #[cfg(feature = "country-MD")]
  if countries.is_none() || countries.unwrap().contains(&Country::MD) {
      map.insert(Country::MD, md::build(&years)?);
  }

  #[cfg(feature = "country-MA")]
  if countries.is_none() || countries.unwrap().contains(&Country::MA) {
      map.insert(Country::MA, ma::build(&years)?);
  }

  #[cfg(feature = "country-MZ")]
  if countries.is_none() || countries.unwrap().contains(&Country::MZ) {
      map.insert(Country::MZ, mz::build(&years)?);
  }

  #[cfg(feature = "country-NL")]
  if countries.is_none() || countries.unwrap().contains(&Country::NL) {
      map.insert(Country::NL, nl::build(&years)?);
  }

  #[cfg(feature = "country-NA")]
  if countries.is_none() || countries.unwrap().contains(&Country::NA) {
      map.insert(Country::NA, na::build(&years)?);
  }

  #[cfg(feature = "country-NZ")]
  if countries.is_none() || countries.unwrap().contains(&Country::NZ) {
      map.insert(Country::NZ, nz::build(&years)?);
  }

  #[cfg(feature = "country-NI")]
  if countries.is_none() || countries.unwrap().contains(&Country::NI) {
      map.insert(Country::NI, ni::build(&years)?);
  }

  #[cfg(feature = "country-NG")]
  if countries.is_none() || countries.unwrap().contains(&Country::NG) {
      map.insert(Country::NG, ng::build(&years)?);
  }

  #[cfg(feature = "country-MK")]
  if countries.is_none() || countries.unwrap().contains(&Country::MK) {
      map.insert(Country::MK, mk::build(&years)?);
  }

  #[cfg(feature = "country-NO")]
  if countries.is_none() || countries.unwrap().contains(&Country::NO) {
      map.insert(Country::NO, no::build(&years)?);
  }

  #[cfg(feature = "country-PK")]
  if countries.is_none() || countries.unwrap().contains(&Country::PK) {
      map.insert(Country::PK, pk::build(&years)?);
  }

  #[cfg(feature = "country-PY")]
  if countries.is_none() || countries.unwrap().contains(&Country::PY) {
      map.insert(Country::PY, py::build(&years)?);
  }

  #[cfg(feature = "country-PE")]
  if countries.is_none() || countries.unwrap().contains(&Country::PE) {
      map.insert(Country::PE, pe::build(&years)?);
  }

  #[cfg(feature = "country-PL")]
  if countries.is_none() || countries.unwrap().contains(&Country::PL) {
      map.insert(Country::PL, pl::build(&years)?);
  }

  #[cfg(feature = "country-PT")]
  if countries.is_none() || countries.unwrap().contains(&Country::PT) {
      map.insert(Country::PT, pt::build(&years)?);
  }

  #[cfg(feature = "country-RO")]
  if countries.is_none() || countries.unwrap().contains(&Country::RO) {
      map.insert(Country::RO, ro::build(&years)?);
  }

  #[cfg(feature = "country-RU")]
  if countries.is_none() || countries.unwrap().contains(&Country::RU) {
      map.insert(Country::RU, ru::build(&years)?);
  }

  #[cfg(feature = "country-SA")]
  if countries.is_none() || countries.unwrap().contains(&Country::SA) {
      map.insert(Country::SA, sa::build(&years)?);
  }

  #[cfg(feature = "country-RS")]
  if countries.is_none() || countries.unwrap().contains(&Country::RS) {
      map.insert(Country::RS, rs::build(&years)?);
  }

  #[cfg(feature = "country-SG")]
  if countries.is_none() || countries.unwrap().contains(&Country::SG) {
      map.insert(Country::SG, sg::build(&years)?);
  }

  #[cfg(feature = "country-SK")]
  if countries.is_none() || countries.unwrap().contains(&Country::SK) {
      map.insert(Country::SK, sk::build(&years)?);
  }

  #[cfg(feature = "country-SI")]
  if countries.is_none() || countries.unwrap().contains(&Country::SI) {
      map.insert(Country::SI, si::build(&years)?);
  }

  #[cfg(feature = "country-ZA")]
  if countries.is_none() || countries.unwrap().contains(&Country::ZA) {
      map.insert(Country::ZA, za::build(&years)?);
  }

  #[cfg(feature = "country-KR")]
  if countries.is_none() || countries.unwrap().contains(&Country::KR) {
      map.insert(Country::KR, kr::build(&years)?);
  }

  #[cfg(feature = "country-ES")]
  if countries.is_none() || countries.unwrap().contains(&Country::ES) {
      map.insert(Country::ES, es::build(&years)?);
  }

  #[cfg(feature = "country-SZ")]
  if countries.is_none() || countries.unwrap().contains(&Country::SZ) {
      map.insert(Country::SZ, sz::build(&years)?);
  }

  #[cfg(feature = "country-SE")]
  if countries.is_none() || countries.unwrap().contains(&Country::SE) {
      map.insert(Country::SE, se::build(&years)?);
  }

  #[cfg(feature = "country-CH")]
  if countries.is_none() || countries.unwrap().contains(&Country::CH) {
      map.insert(Country::CH, ch::build(&years)?);
  }

  #[cfg(feature = "country-TW")]
  if countries.is_none() || countries.unwrap().contains(&Country::TW) {
      map.insert(Country::TW, tw::build(&years)?);
  }

  #[cfg(feature = "country-TR")]
  if countries.is_none() || countries.unwrap().contains(&Country::TR) {
      map.insert(Country::TR, tr::build(&years)?);
  }

  #[cfg(feature = "country-TN")]
  if countries.is_none() || countries.unwrap().contains(&Country::TN) {
      map.insert(Country::TN, tn::build(&years)?);
  }

  #[cfg(feature = "country-UA")]
  if countries.is_none() || countries.unwrap().contains(&Country::UA) {
      map.insert(Country::UA, ua::build(&years)?);
  }

  #[cfg(feature = "country-AE")]
  if countries.is_none() || countries.unwrap().contains(&Country::AE) {
      map.insert(Country::AE, ae::build(&years)?);
  }

  #[cfg(feature = "country-GB")]
  if countries.is_none() || countries.unwrap().contains(&Country::GB) {
      map.insert(Country::GB, gb::build(&years)?);
  }

  #[cfg(feature = "country-US")]
  if countries.is_none() || countries.unwrap().contains(&Country::US) {
      map.insert(Country::US, us::build(&years)?);
  }

  #[cfg(feature = "country-UY")]
  if countries.is_none() || countries.unwrap().contains(&Country::UY) {
      map.insert(Country::UY, uy::build(&years)?);
  }

  #[cfg(feature = "country-UZ")]
  if countries.is_none() || countries.unwrap().contains(&Country::UZ) {
      map.insert(Country::UZ, uz::build(&years)?);
  }

  #[cfg(feature = "country-VE")]
  if countries.is_none() || countries.unwrap().contains(&Country::VE) {
      map.insert(Country::VE, ve::build(&years)?);
  }

  #[cfg(feature = "country-VN")]
  if countries.is_none() || countries.unwrap().contains(&Country::VN) {
      map.insert(Country::VN, vn::build(&years)?);
  }

  #[cfg(feature = "country-ZM")]
  if countries.is_none() || countries.unwrap().contains(&Country::ZM) {
      map.insert(Country::ZM, zm::build(&years)?);
  }

  #[cfg(feature = "country-ZW")]
  if countries.is_none() || countries.unwrap().contains(&Country::ZW) {
      map.insert(Country::ZW, zw::build(&years)?);
  }

  Ok(map)
}
