use bitflags::bitflags;

bitflags! {
  pub struct MapFlags: u32 {
    const HIDE_MINIMAP = 0x0001;
    const MODIFY_ALLY_PRIORITIES = 0x0002;
    const MELEE = 0x0004;
    const REVEAL_TERRAIN = 0x0010;
    const FIXED_PLAYER_SETTINGS = 0x0020;
    const CUSTOM_FORCES = 0x0040;
    const CUSTOM_TECH_TREE = 0x0080;
    const CUSTOM_ABILITIES = 0x0100;
    const CUSTOM_UPGRADES = 0x0200;
    const WATER_WAVES_ON_CLIFF_SHORES = 0x0800;
    const WATER_WAVES_ON_SLOPE_SHORES = 0x1000;
    const HAS_TERRAIN_FOG = 0x2000;
    const REQUIRES_EXPANSION = 0x4000;
    const ITEM_CLASSIFICATION = 0x8000;
    const WATER_TINTING = 0x10000;
    const ACCURATE_RANDOM = 0x20000;
    const ABILITY_SKINS = 0x40000;
  }
}
