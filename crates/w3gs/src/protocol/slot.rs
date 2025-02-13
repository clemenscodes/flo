use flo_util::binary::*;
use flo_util::{BinDecode, BinEncode};

use crate::protocol::constants::PacketTypeId;
pub use crate::protocol::constants::{RacePref, SlotLayout, SlotStatus, AI};
use crate::protocol::packet::PacketPayload;

#[derive(Debug, BinDecode, BinEncode, PartialEq, Clone)]
pub struct SlotInfo {
  // 7 + sizeof(SlotData) x num_slots
  pub(crate) _length_of_slot_data: u16,
  pub(crate) _num_slots: u8,
  #[bin(repeat = "_num_slots")]
  pub(crate) slots: Vec<SlotData>,
  pub random_seed: u32,
  pub slot_layout: SlotLayout,
  pub num_players: u8,
}

impl Default for SlotInfo {
  fn default() -> Self {
    SlotInfo {
      _length_of_slot_data: (7 + (SlotData::MIN_SIZE * 24)) as u16,
      _num_slots: 24,
      slots: {
        let mut slots = Vec::with_capacity(24);
        slots.resize_with(24, SlotData::default);
        slots
      },
      random_seed: 0,
      slot_layout: SlotLayout::Melee,
      num_players: 0,
    }
  }
}

impl SlotInfo {
  pub fn build() -> SlotInfoBuilder {
    SlotInfoBuilder {
      inner: Default::default(),
    }
  }

  // pub fn new(num_slots: u8, num_players: u8) -> Self {
  //   let slots = (0..(num_slots as usize))
  //     .map(|i| SlotData::new(i))
  //     .collect();
  //   Self {
  //     _length_of_slot_data: 7 + (SlotData::MIN_SIZE as u16) * (num_slots as u16),
  //     _num_slots: num_slots,
  //     slots,
  //     random_seed: rand::random(),
  //     slot_layout: SlotLayout::Melee,
  //     num_players,
  //   }
  // }

  pub fn slots(&self) -> &[SlotData] {
    &self.slots
  }

  pub fn slot_mut(&mut self, index: usize) -> Option<&mut SlotData> {
    self.slots.get_mut(index)
  }

  pub fn find_active_player_slot_mut(&mut self, player_id: u8) -> Option<&mut SlotData> {
    if player_id == 0 || player_id > self.slots.len() as u8 {
      return None;
    }
    let slot = self.slots.get_mut((player_id - 1) as usize)?;
    if slot.slot_status != SlotStatus::Occupied {
      return None;
    }
    Some(slot)
  }

  // TODO: handle teams, forces
  // pub fn join(&mut self) -> Option<&mut SlotData> {
  //   let (i, slot) = self
  //     .slots
  //     .iter_mut()
  //     .enumerate()
  //     .find(|(_, s)| s.slot_status == SlotStatus::Open)?;
  //   slot.player_id = (i + 1) as u8;
  //   slot.slot_status = SlotStatus::Occupied;
  //   let race_selectable = RacePref::SELECTABLE;
  //   slot.race = RacePref::RANDOM | race_selectable;
  //   slot.computer = false;
  //   slot.computer_type = AI::ComputerNormal;
  //   slot.handicap = 100;
  //   Some(slot)
  // }
}

#[derive(Debug)]
pub struct SlotInfoBuilder {
  inner: SlotInfo,
}
impl SlotInfoBuilder {
  pub fn num_slots(&mut self, value: usize) -> &mut Self {
    self.inner.slots.resize_with(value, || SlotData::default());
    self.inner._length_of_slot_data = (7 + (SlotData::MIN_SIZE * value)) as u16;
    self.inner._num_slots = value as u8;
    self
  }

  pub fn num_players(&mut self, value: usize) -> &mut Self {
    self.inner.num_players = value as u8;
    self
  }

  pub fn random_seed(&mut self, value: i32) -> &mut Self {
    let bytes = value.to_le_bytes();
    self.inner.random_seed = u32::from_le_bytes(bytes);
    self
  }

  pub fn slot_layout(&mut self, value: SlotLayout) -> &mut Self {
    self.inner.slot_layout = value;
    self
  }

  pub fn build(&mut self) -> SlotInfo {
    std::mem::replace(&mut self.inner, SlotInfo::default())
  }
}

impl PacketPayload for SlotInfo {
  const PACKET_TYPE_ID: PacketTypeId = PacketTypeId::SlotInfo;
}

#[derive(Debug, BinDecode, BinEncode, PartialEq, Clone)]
pub struct SlotData {
  pub player_id: u8,
  pub download_status: u8,
  pub slot_status: SlotStatus,
  pub computer: bool,
  pub team: u8,
  pub color: u8,
  #[bin(bitflags(u8))]
  pub race: RacePref,
  pub computer_type: AI,
  pub handicap: u8,
}

impl Default for SlotData {
  fn default() -> Self {
    Self {
      player_id: 0,
      download_status: 0xFF,
      slot_status: SlotStatus::Open,
      computer: false,
      team: 0,
      color: 0,
      race: RacePref::RANDOM | RacePref::SELECTABLE,
      computer_type: AI::ComputerNormal,
      handicap: 100,
    }
  }
}

#[test]
fn test_slot_info() {
  crate::packet::test_simple_payload_type(
    "slot_info_1.bin",
    &SlotInfo {
      _length_of_slot_data: 25,
      _num_slots: 2,
      slots: vec![
        SlotData {
          player_id: 1,
          download_status: 100,
          slot_status: SlotStatus::Occupied,
          computer: false,
          team: 0,
          color: 0,
          race: RacePref::RANDOM | RacePref::SELECTABLE,
          computer_type: AI::ComputerNormal,
          handicap: 100,
        },
        SlotData {
          player_id: 2,
          download_status: 255,
          slot_status: SlotStatus::Occupied,
          computer: false,
          team: 1,
          color: 1,
          race: RacePref::RANDOM | RacePref::SELECTABLE,
          computer_type: AI::ComputerNormal,
          handicap: 100,
        },
      ],
      random_seed: 22717299,
      slot_layout: SlotLayout::Melee,
      num_players: 2,
    },
  );
}

#[test]
fn test_slot_info_2() {
  crate::packet::test_simple_payload_type(
    "slot_info_2.bin",
    &SlotInfo {
      _length_of_slot_data: 25,
      _num_slots: 2,
      slots: vec![
        SlotData {
          player_id: 1,
          download_status: 100,
          slot_status: SlotStatus::Occupied,
          computer: false,
          team: 0,
          color: 0,
          race: RacePref::RANDOM | RacePref::SELECTABLE,
          computer_type: AI::ComputerNormal,
          handicap: 100,
        },
        SlotData {
          player_id: 2,
          download_status: 100,
          slot_status: SlotStatus::Occupied,
          computer: false,
          team: 1,
          color: 1,
          race: RacePref::RANDOM | RacePref::SELECTABLE,
          computer_type: AI::ComputerNormal,
          handicap: 100,
        },
      ],
      random_seed: 22717310,
      slot_layout: SlotLayout::Melee,
      num_players: 2,
    },
  );
}

#[test]
fn test_slot_info_3() {
  crate::packet::test_simple_payload_type(
    "slot_info_3.bin",
    &SlotInfo {
      _length_of_slot_data: 25,
      _num_slots: 2,
      slots: vec![
        SlotData {
          player_id: 1,
          download_status: 100,
          slot_status: SlotStatus::Occupied,
          computer: false,
          team: 0,
          color: 0,
          race: RacePref::RANDOM | RacePref::SELECTABLE,
          computer_type: AI::ComputerNormal,
          handicap: 100,
        },
        SlotData {
          player_id: 2,
          download_status: 100,
          slot_status: SlotStatus::Occupied,
          computer: false,
          team: 1,
          color: 1,
          race: RacePref::HUMAN | RacePref::SELECTABLE,
          computer_type: AI::ComputerNormal,
          handicap: 100,
        },
      ],
      random_seed: 22741640,
      slot_layout: SlotLayout::Melee,
      num_players: 2,
    },
  );
}
