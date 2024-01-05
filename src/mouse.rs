
use sdl2::{Sdl, video::Window, render::Canvas};
use sdl2::mouse::{MouseButton, self};

use crate::player::{self, Player};

pub enum MouseButtonPressed {
  LEFT,
  RIGHT,
  MIDDLE,
  NONE,
}

pub struct PlayerMouse {
  // where it was on the last frame
  last_curser_pos: [i32;2],
  // where it is now
  pub curser_pos: [i32;2],
  mouse_button: MouseButtonPressed,
  // positive number is upwards, negitive number is downwards
  scroll_wheel_travel: i32,
  // if the player's mouse is locked on the window 
  pub mouse_lock: bool,
}

impl PlayerMouse {

  pub fn new() -> Self {

    Self {
      last_curser_pos: [0,0],
      curser_pos: [0,0],
      mouse_button: MouseButtonPressed::NONE,
      scroll_wheel_travel: 0,
      mouse_lock: false,
    }

  }

  pub fn update_pos(&mut self, events: &sdl2::EventPump) {

    let mouse_state = events.mouse_state();

    self.last_curser_pos = self.curser_pos;

    self.curser_pos = [
      mouse_state.x(),
      mouse_state.y(),
    ];

  }

  pub fn get_mouse_travel(&mut self) -> [i32;2] {

    [
      self.curser_pos[0] - self.last_curser_pos[0],
      self.curser_pos[1] - self.last_curser_pos[1],
    ]

  }

}

// ice physics but for looking around, dont know if ill use it but whatever
const ICE_LOOKING: bool = false;

pub fn operate_mouse_lock(sdl_context: &Sdl, canvas: &mut Canvas<Window>, player_mouse: &mut PlayerMouse, player: &mut Player) {

  let window_size = canvas.window().size();

  let middle_of_window = [(window_size.0 / 2) as i32, (window_size.1 / 2) as i32];

  let central_deviation = [
    player_mouse.curser_pos[0] - middle_of_window[0],
    player_mouse.curser_pos[1] - middle_of_window[1],
  ];

  const YOUR_NORMAL: bool = true;
  
  if ICE_LOOKING {
    if player_mouse.mouse_lock {
      player.rot += ((player_mouse.curser_pos[0] - middle_of_window[0]) as f32) / 10.0;
      if (player_mouse.curser_pos[0] - middle_of_window[0]).abs() <= 10 {
        sdl_context.mouse().warp_mouse_in_window(canvas.window(), middle_of_window[0], middle_of_window[1]);
      } else {
        sdl_context.mouse().warp_mouse_in_window(canvas.window(), player_mouse.curser_pos[0] - (central_deviation[0]/5), player_mouse.curser_pos[1] - (central_deviation[1]/5));
      }
    }
  } else if YOUR_NORMAL && player_mouse.mouse_lock {
    player.rot += ((player_mouse.curser_pos[0] - middle_of_window[0]) as f32) / 10.0;
    sdl_context.mouse().warp_mouse_in_window(canvas.window(), middle_of_window[0], middle_of_window[1]);
  }

}