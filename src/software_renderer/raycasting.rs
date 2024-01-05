
use std::ops::Rem;

use PixelBuffer;

use Map;

#[path="../player.rs"]
mod player;
use player::*;

use find_directional_line;


fn subtract_f32_array(sub1: &[f32;2], sub2: &[f32;2]) -> [f32;2] {
  [
    sub1[0] - sub2[0],
    sub1[1] - sub2[1]
  ]
}

const raycast_check_dst: f32 = 0.03;
/* casts a ray from the given position to the ending of the map, 
 * or an object, and returns the distance, 
 * and point to draw as an x pos of the fov, and gets the wall type */
pub fn cast_ray(map: &Map, cast_from: [f32;2], angle: f32) -> (f32, u8) {
  
  let mut raycast_pos: [f32;2] = cast_from;

  let add_direction: [f32;2] = subtract_f32_array(
    &find_directional_line(&raycast_pos, angle, raycast_check_dst), &raycast_pos
  );

  let mut distance_away = 0.0;

  // continues the calculations unlil it his a wall, or the end of the map
  while !map.is_pos_wall(&raycast_pos) {
    
    distance_away += raycast_check_dst;

    raycast_pos = [
      raycast_pos[0] + add_direction[0],
      raycast_pos[1] + add_direction[1]
    ];

    if map.is_pos_out_of_map(raycast_pos) {
      break;
    }

  }

  // turns the distance to negitive if it never hit a wall
  if map.is_pos_out_of_map(raycast_pos) {
    distance_away = -1.0;
  }

  (distance_away, map.get_map_item_from_pos([raycast_pos[0] as i32, raycast_pos[1] as i32]))

}

use WIDTH;
use HEIGHT;

use set_pixel;

use RenderChunks;

use Color;

const BACKROUND_COLOR: Color = [200, 200, 200];

// casts a number if rays infront of the player, determining how small or large to draw walls
pub fn raycast_walls(buffer: &mut PixelBuffer, map: &Map, player: &Player) {

  // first val being distance, and the second being order, the third being the wall type
  let mut distance_list: Vec<(f32, i32, u8)> = vec![];

  let mut current_dst = (0.0, 0);

  // loops through each sector of pixels that the player fov owns
  for i in 0..player.fov as i32 {
    current_dst = cast_ray(map, player.pos, (player.rot + i as f32 - (player.fov/2.0) as f32));
    if current_dst.0 != -1.0 {
      distance_list.push((current_dst.0, i, current_dst.1));
    }
  }

  draw_raycasted_walls(buffer, &mut distance_list, player);

}

fn draw_raycasted_walls(buffer: &mut PixelBuffer, distance_list: &mut Vec<(f32, i32, u8)>, player: &Player) {

  // the amount of pixels each ray casted can take up
  let draw_sector = ((WIDTH as f32) / player.fov as f32) as i32;

  let mut draw_range = [0;2];

  let mut color = [0, 0, 0];

  const SCREEN_MIDDLE: u32 = HEIGHT/2;

  // loops through each recorded raycast, and draws it out
  distance_list.iter().for_each(|i| {

    draw_range = [
      i.1 * draw_sector,
      (i.1 * draw_sector) + draw_sector
    ];

    match i.2 {
      2 => color = [255, 255, 255],
      3 => color = [255, 0, 0],

      _ => color = [0, 0, 0]
    }

    // does some meth to make the vertical part of the collum smaller on the screen the longer the raycast lasted
    let mut collum_vertical_size = ((SCREEN_MIDDLE as f32) / i.0).round() as u32;
    if collum_vertical_size > SCREEN_MIDDLE {
      collum_vertical_size = SCREEN_MIDDLE
    }

    for x in draw_range[0] as u32..draw_range[1] as u32 {
      
      // draw the vertical collums
      for y in SCREEN_MIDDLE..SCREEN_MIDDLE+collum_vertical_size {
        // bottom half
        set_pixel(buffer, x, y, &color);
        // just inverts the bottom half
        set_pixel(buffer, x, y - collum_vertical_size, &color);
      }
      
      // draw the backround
      for y in SCREEN_MIDDLE+collum_vertical_size..HEIGHT {
        // bottom half
        set_pixel(buffer, x, y, &BACKROUND_COLOR);
        // just inverts the bottom half
        set_pixel(buffer, x, y - collum_vertical_size - SCREEN_MIDDLE, &BACKROUND_COLOR);
      }
  
    }

  });
}