use crate::error::{Error, Result};

pub mod publisher;
pub mod search;

pub(crate) fn get_reg_type(game_version: &str) -> Result<String> {
  let segments = game_version.split(".").collect::<Vec<&str>>();
  if segments.len() != 4 {
    return Err(Error::InvalidVersionString(game_version.to_string()));
  }
  let major = segments[0]
    .parse::<i64>()
    .map_err(|_| Error::InvalidVersionString(game_version.to_string()))?;
  let major_offset = major - 1;
  let minor = segments[1]
    .parse::<i64>()
    .map_err(|_| Error::InvalidVersionString(game_version.to_string()))?;
  let num = format!("10{major_offset}{minor:02}")
    .parse::<i64>()
    .map_err(|_| Error::InvalidVersionString(game_version.to_string()))?;

  Ok(format!("_blizzard._udp,_w3xp{:x}", num))
}

#[test]
fn test_get_reg_type() {
  assert_eq!(
    get_reg_type("1.33.0.00000").unwrap(),
    "_blizzard._udp,_w3xp2731"
  );
  assert_eq!(
    get_reg_type("1.34.0.00000").unwrap(),
    "_blizzard._udp,_w3xp2732"
  );
  assert_eq!(
    get_reg_type("2.0.0.00000").unwrap(),
    "_blizzard._udp,_w3xp2774"
  );
}
