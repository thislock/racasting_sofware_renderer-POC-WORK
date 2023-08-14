
use Map;

const default_player_fov: f32 = 100.0;

pub const player_turn_vel: f32 = 2.5;

pub const PLAYER_VEL: f32 = 0.05;
pub struct Player {
  pub pos: [f32;2],
  pub rot: f32,
  pub fov: f32,
}

pub fn init_Player() -> Player {
  Player {
    pos: [0.0, 0.0],
    rot: 0.0,
    fov: default_player_fov,
  }
}


pub fn set_player_start_position(player: &mut Player, map: &Map) {

  let player_default = map.map_layout.iter().position(|&i| i == 1).unwrap();

  let set_val = map.generate_map_pos_from_raw( player_default as i32);

  let converted_set_val = [set_val[0] as f32, set_val[1] as f32];

  player.pos = converted_set_val;

}
