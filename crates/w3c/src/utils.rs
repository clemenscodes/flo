use crate::types::w3c::*;

use anyhow;
use ureq;

pub fn get_race_flo(r : u32) -> String {
  String::from(
    match r { 0 => "H"
            , 1 => "O"
            , 2 => "NE"
            , 3 => "UD"
            , _ => "RND"})
}

pub fn flo_to_w3c_race(r: u32) -> u32 {
  match r { 0 => 1
          , 1 => 2
          , 2 => 4
          , 3 => 8
          , _ => 0 }
}

pub fn get_league(l: u32) -> String {
  String::from(match l { 0 => "GrandMaster"
                       , 1 => "Master"
                       , 2 => "Diamond"
                       , 3 => "Platinum"
                       , 4 => "Gold"
                       , 5 => "Silver"
                       , 6 => "Bronze"
                       , _ => "" })
}

pub fn get_player(target: &str, season: u16) -> anyhow::Result<Option<String>> {
  if target.contains('#') {
    Ok(Some(target.to_string()))
  }
  else {
    let search_uri =
      format!("https://statistic-service.w3champions.com/api/ladder/search?gateWay=20&searchFor={}&season={}"
             , target, season);
    let search: Vec<Search> = ureq::get(&search_uri).call()?.into_json::<Vec<Search>>()?;
    if !search.is_empty() {
      // search for ToD will give toy Toddy at first, so we search for exact match
      for s in &search {
        for id in &s.player.playerIds {
          if target == id.name {
            return Ok(Some(id.battleTag.clone()));
          }
        }
      }
      // if there is no exact match return first search result
      Ok(Some(search[0].player.playerIds[0].battleTag.clone()))
    } else {
      Ok(None)
    }
  }
}
