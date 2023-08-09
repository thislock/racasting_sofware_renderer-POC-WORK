
use sdl2::Sdl;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::sys::KeyCode;

use find_directional_line;

use Player;
use Map;
use PLAYER_VEL;
use player_turn_vel;

pub fn keyboard_events(sdl_context: &Sdl, running: &mut bool, local_player: &mut Player, map1: &Map) {

  for event in sdl_context.event_pump().unwrap().poll_iter() {    
    match event {
      Event::KeyDown { keycode: Some(keycode), .. } => {
    
        match keycode {
    
          Keycode::Right => {
            local_player.rot += player_turn_vel;
          },
          Keycode::Left => {
            local_player.rot -= player_turn_vel;
          },
          Keycode::W => {
            let step_val = find_directional_line(&local_player.pos, local_player.rot, PLAYER_VEL);
            if !map1.is_pos_out_of_map(step_val) {
              local_player.pos[0] = step_val[0];
              local_player.pos[1] = step_val[1];
            }
          },
          Keycode::S => {
            let step_val = find_directional_line(&local_player.pos, local_player.rot, -PLAYER_VEL);
            if !map1.is_pos_out_of_map(step_val) {
              local_player.pos[0] = step_val[0];
              local_player.pos[1] = step_val[1];
            }
          },
          Keycode::D => {
            let mut rotation = local_player.rot - 90.0;
            if rotation < 0.0 {
              rotation += 360.0
            }
            let step_val = find_directional_line(&local_player.pos, rotation, -PLAYER_VEL);
            if !map1.is_pos_out_of_map(step_val) {
              local_player.pos[0] = step_val[0];
              local_player.pos[1] = step_val[1];
            }
          }
          Keycode::A => {
            let mut rotation = local_player.rot + 90.0;
            if rotation > 360.0 {
              rotation -= 360.0
            }
            let step_val = find_directional_line(&local_player.pos, rotation, -PLAYER_VEL);
            if !map1.is_pos_out_of_map(step_val) {
              local_player.pos[0] = step_val[0];
              local_player.pos[1] = step_val[1];
            }
          }
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