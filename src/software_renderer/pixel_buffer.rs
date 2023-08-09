
#[path="./raycasting.rs"]
mod raycasting;
use raycasting::*;

#[path="../player.rs"]
mod player;
use player::*;

use HEIGHT;
use WIDTH;

use std::collections::HashMap;

pub type PixelBuffer = Vec<u8>;

pub type Color = [u8;3];

pub type RenderChunks = Vec<(f32, ([i32;2], [u32;2]))>;

// Set the color of a pixel in the buffer
pub fn set_pixel(buffer: &mut PixelBuffer, x: u32, y: u32, color: Color) {

  let offset = ((y * WIDTH + x) * 3) as usize;

  buffer[offset] = color[0];
  buffer[offset + 1] = color[1];
  buffer[offset + 2] = color[2];

}

use Map;

pub fn master_rendertask( buffer: &mut PixelBuffer, map: &Map, player: &mut Player ) {

  if player.rot > 360.0 { player.rot = 0.0   }
  if player.rot < 0.0   { player.rot = 360.0 }

  raycast_walls(buffer, map, player); 
  
}