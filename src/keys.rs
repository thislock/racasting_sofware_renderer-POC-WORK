
use sdl2::Sdl;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::sys::KeyCode;

use find_directional_line;

use Player;
use Map;
use PLAYER_VEL;
use player_turn_vel;

fn player_move_in_direction(player: &mut Player, map: &Map, dst: f32, angle: f32) {
  let move_dest = find_directional_line(&player.pos, angle, dst);
  if !map.is_pos_out_of_map(move_dest) && !map.is_pos_wall(&move_dest) {
    player.pos = move_dest;
  }
}

pub fn keyboard_events(sdl_context: &Sdl, running: &mut bool, player: &mut Player, map: &Map) {

  for event in sdl_context.event_pump().unwrap().poll_iter() {    
    match event {
      Event::KeyDown { keycode: Some(keycode), .. } => {
    
        match keycode {
          Keycode::Right => player.rot += player_turn_vel,
          Keycode::Left => player.rot -= player_turn_vel,
          
          Keycode::W => player_move_in_direction(player, map, PLAYER_VEL, player.rot),
          Keycode::S => player_move_in_direction(player, map, -PLAYER_VEL, player.rot),

          Keycode::D => player_move_in_direction(player, map, PLAYER_VEL, player.rot + 90.0),
          Keycode::A => player_move_in_direction(player, map, PLAYER_VEL, player.rot - 90.0),
          _ => {}
        }
      } 
      Event::Quit { .. } |
      Event::KeyDown {
        keycode: Some(Keycode::Escape), ..
      } => *running = false,
      _ => {}
    }

  }
}