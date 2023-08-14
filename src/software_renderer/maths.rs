
const PI: f32 = 3.141592653589793;

// converts an angle to radians, used for directional line drawing
const RADIAN_CALC_BASE: f32 = PI/180.0;
fn to_radians(angle: f32) -> f32 {
  RADIAN_CALC_BASE * angle
}

// a calculation that draws a line of x length, in d direction, and returns where the line ends
pub fn find_directional_line(starting_pos: &[f32;2], steping_angle: f32, steping_length: f32) -> [f32;2] {

  let angle_radians = to_radians(steping_angle);

  let delta_pos = [
    steping_length * angle_radians.cos(),
    steping_length * angle_radians.sin()  
  ];

  [
    starting_pos[0] + delta_pos[0],
    starting_pos[1] + delta_pos[1]
  ]

}