use std::collections::HashMap;

use crate::pixel_buffer::Color;


pub type map_lay = Vec<u8>;

pub struct Map {
  pub map_layout: map_lay,
  pub map_width: u32,
  pub map_height: u32,
  pub wall_format: HashMap<char, Color>,
}

// nothing being 0, a thing being 1, and the starting point of the player being 2
pub fn new_map(map: map_lay, map_width: u32, map_height: u32, wall_format: HashMap<char, Color>) -> Map {
  Map { map_layout: map, map_width: map_width, map_height: map_height, wall_format }
}

/*
define what charecters are what colors, in this case it's +
spaces will always represent, well nothing
+++++++++++
+++    ++++
+++
+++
+++++++++++
*/
pub fn map_from_txt(defined_walls: HashMap<char, Color>, map: &str) -> Map {

  let mut wall_collection: Vec<Vec<char>> = Vec::new();

  let mut current_wall_sector = vec![
    // space fixes weird problem with maps getting mangled without space in the front of the map
    ' '
  ];

  // so it doesnt read janky doube newlines, creating gaps in the map
  let mut was_newline = false;

  map.chars().for_each(|letter| {

    if letter == '\n' && !was_newline {
      was_newline = true;
      wall_collection.push(current_wall_sector.clone());
      current_wall_sector.clear();
      current_wall_sector.push(' ');
    } else if letter != '\n' {
      was_newline = false;
      current_wall_sector.push(letter);
    }

  });

  let biggest_sector = {
    let mut a = 0;
    wall_collection.iter().for_each(|wall_sec| {
      if wall_sec.len() > a {
        a = wall_sec.len();
      }
    });
    a
  };

  // builds the map from the given text formated
  
  
  let mut map_data: Vec<u8> = vec![];
  
  wall_collection.iter_mut().for_each(|wall| {
    while wall.len() < biggest_sector {
      wall.push(' ');
    }
    wall.iter().for_each(|data| {
      map_data.push(*data as u8);
    });
    
    println!("{:?}", wall);
  });

  let map_width = wall_collection[0].len();
  let map_height = wall_collection.len();
  
  println!("map width: {}, map height: {}", map_width, map_height);
  
  return new_map(map_data, map_width as u32, map_height as u32, defined_walls);

}

impl Map {

  pub fn get_map_item_from_pos(&self, found_pos: [i32;2]) -> u8 {

    if self.is_pos_out_of_map([found_pos[0] as f32, found_pos[1] as f32]) {
      return 0;
    }

    self.map_layout[self.generate_raw_from_map_pos(found_pos) as usize]

  }

  pub fn is_pos_wall(&self, pos: &[f32;2]) -> bool {
    
    let map_pos = [pos[0] as i32, pos[1] as i32];

    if self.get_map_item_from_pos([map_pos[0], map_pos[1]]) != ' ' as u8 {
      return true;
    }

    false

  }

  pub fn is_pos_out_of_map(&self, pos_checked: [f32;2]) -> bool {

    if (pos_checked[0] as i32) < 1 
    || (pos_checked[1] as i32) < 1
    
    || (pos_checked[0] as i32) > self.map_width as i32
    || (pos_checked[1] as i32) > self.map_height as i32 {
      return true;
    }

    if self.generate_raw_from_map_pos(
      [pos_checked[0] as i32, pos_checked[1] as i32]) > 
      ((self.map_width * self.map_height) - 1) as i32 
      {
      return true;
    }

    false

  }

  pub fn generate_raw_from_map_pos(&self, map_pos: [i32;2]) -> i32 {
    ((map_pos[1]-1) * self.map_width as i32) + map_pos[0]
  }

  pub fn generate_map_pos_from_raw(&self, raw_pos: i32) -> [i32;2] {

    [
      ((raw_pos as u32) % self.map_width) as i32+1, // x
      ((raw_pos as u32) / self.map_width) as i32+1  // y
    ]
  
  }
  
  // generates a list of all the wall objects infront of the player
  pub fn get_map_walls(&self) -> Vec<[i32;2]> {
    
    let mut walls: Vec<[i32;2]> = vec![];
  
    let mut raw_pos_counter = 0;
  
    self.map_layout.iter().for_each(|i| {
      if *i == ' ' as u8 {
        walls.push(self.generate_map_pos_from_raw(raw_pos_counter))
      }
      raw_pos_counter += 1;
    });
  
    walls
  
  }
}
