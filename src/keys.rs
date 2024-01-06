
use sdl2::{Sdl, event};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;

use find_directional_line;

use Player;
use Map;
use player_turn_vel;
use sdl2::video::Window;

use crate::mouse::PlayerMouse;
use crate::player::{PLAYER_TERMINAL_VEL, PLAYER_ACCELERATION};

// for keyboard events that start when the key is pressed, and stoppes when lifted
pub struct KeySwitch {
  key: Keycode,
  pub pressed: bool,
}

impl KeySwitch {

  pub fn new(key: Keycode) -> Self {
    Self {
      key,
      pressed: false,
    }
  }

  pub fn read_key(&mut self, event: &Event) {

    match *event {
      Event::KeyDown { keycode: Some(keycode), .. } => {
        if keycode == self.key {
          self.pressed = true;
        }
      }
      Event::KeyUp { keycode: Some(keycode), .. } => {
        if keycode == self.key {
          self.pressed = false;
        }
      }

      _ => {}
    }
  }

  pub fn comp_key(&self, key_compaired: Keycode) -> bool {
    if self.key == key_compaired {
      return true;
    } else {
      return false;
    }
  }

}

fn player_move_in_direction(player: &mut Player, map: &Map, angle: f32) {
  player.player_stopped_moving = false;
  if player.velocity < PLAYER_TERMINAL_VEL {
    player.velocity += PLAYER_ACCELERATION;
  }
  let move_dest = find_directional_line(&player.pos, angle, player.velocity);
  if !map.is_pos_out_of_map(move_dest) && !map.is_pos_wall(&move_dest) {
    player.pos = move_dest;
  }
}

pub fn build_keylist() -> Vec<KeySwitch> {
  
  vec![
    KeySwitch::new(Keycode::Right),
    KeySwitch::new(Keycode::Left),
    KeySwitch::new(Keycode::W),
    KeySwitch::new(Keycode::A),
    KeySwitch::new(Keycode::S),
    KeySwitch::new(Keycode::D),
    KeySwitch::new(Keycode::Escape),
    KeySwitch::new(Keycode::F),
  ]

}

fn invert_rotation(a: f32) -> f32 {

  let b = a - 180.0;

  return b.abs()

}

pub fn keyboard_events(sdl_context: &Sdl, running: &mut bool, player: &mut Player, map: &Map, player_mouse: &mut PlayerMouse, key_list: &mut Vec<KeySwitch>) {

  let mut event_pump = sdl_context.event_pump().unwrap();

  player_mouse.update_pos(&event_pump);

  for event in event_pump.poll_iter() {

    key_list.iter_mut().for_each(|key| {
      key.read_key(&event);
    });

    match event {

      Event::Quit { .. } |
      Event::KeyDown {
        keycode: Some(Keycode::F4), ..
      } => *running = false,
      _ => {}
      
    }

  }

  // sets it to this by default, if they do move it's switched off after this set
  player.player_stopped_moving = true;

  key_list.iter_mut().for_each(|key| {
    if key.pressed {
      match key.key {

        Keycode::Right => player.rot += player_turn_vel,
        Keycode::Left => player.rot -= player_turn_vel,
          
        Keycode::W => player_move_in_direction(player, map, player.rot),
        Keycode::S => player_move_in_direction(player, map, invert_rotation(player.rot)),

        Keycode::D => player_move_in_direction(player, map, player.rot + 90.0),
        Keycode::A => player_move_in_direction(player, map, player.rot - 90.0),

        Keycode::Escape => {
          player_mouse.mouse_lock = false;
          sdl_context.mouse().show_cursor(true);
        },
          
        Keycode::F => {
          player_mouse.mouse_lock = true;
          sdl_context.mouse().show_cursor(false);
        },

        _ => {}
      }
    }
  });

  player.operate_player_acceleration();

}