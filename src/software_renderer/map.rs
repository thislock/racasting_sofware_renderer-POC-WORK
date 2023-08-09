
pub type map_lay = Vec<u8>;

pub struct Map {
  pub map_layout: map_lay,
  pub map_width: u32,
  pub map_height: u32,
}

// nothing being 0, a thing being 1, and the starting point of the player being 2
pub fn new_map(map: map_lay, map_width: u32, map_height: u32) -> Map {
  Map { map_layout: map, map_width: map_width, map_height: map_height }
}

impl Map {

  pub fn get_map_item_from_pos(&self, found_pos: [i32;2]) -> u8 {

    if self.is_pos_out_of_map([found_pos[0] as f32, found_pos[1] as f32]) {
      return 0;
    }

    self.map_layout[self.generate_raw_from_map_pos(found_pos) as usize]

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
      if *i == 1 {
        walls.push(self.generate_map_pos_from_raw(raw_pos_counter))
      }
      raw_pos_counter += 1;
    });
  
    walls
  
  }
}
