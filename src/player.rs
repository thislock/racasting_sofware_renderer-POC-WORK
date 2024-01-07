
use Map;

const default_player_fov: f32 = 70.0;

pub const player_turn_vel: f32 = 2.5;

pub const PLAYER_TERMINAL_VEL: f32 = 0.06;
pub const PLAYER_ACCELERATION: f32 = 0.002;

pub struct Player {
  pub pos: [f32;2],
  pub velocity: f32,
  pub rot: f32,
  pub fov: f32,

  pub player_stopped_moving: bool,
}

impl Player {

  pub fn operate_player_acceleration(&mut self) {

    let slow_down_speed = PLAYER_ACCELERATION*5.0;

    if self.player_stopped_moving && self.velocity >= 0.0 {
      if self.velocity - slow_down_speed > 0.0 {
        self.velocity -= slow_down_speed;
      } else {
        self.velocity = 0.0;
      }
    }

  }

}

pub fn init_player() -> Player {
  Player {
    pos: [0.0, 0.0],
    velocity: 0.0,
    rot: 0.0,
    fov: default_player_fov,
    player_stopped_moving: false,
  }
}


pub fn set_player_start_position(player: &mut Player, map: &Map) {

  let player_default = map.map_layout.iter().position(|&i| i == '^' as u8).expect("no player marker placed in given map, please put the ^ symbol where the player spawns");
  
  let set_val = map.generate_map_pos_from_raw( player_default as i32);

  let converted_set_val = [set_val[0] as f32, set_val[1] as f32];

  player.pos = converted_set_val;

}
