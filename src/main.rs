
extern crate sdl2;

#[path="./software_renderer/pixel_buffer.rs"]
mod pixel_buffer;
use pixel_buffer::*;

#[path="./software_renderer/raycasting.rs"]
mod raycasting;
use raycasting::*;

#[path="./software_renderer/map.rs"]
mod map;
use map::*;

#[path="./player.rs"]
mod player;
use player::*;

#[path="./keys.rs"]
mod keys;
use keys::*;

#[path="software_renderer/maths.rs"]
mod maths;
use maths::*;

#[path="software_renderer/texture.rs"]
mod texture;
use texture::*;

use sdl2::pixels::PixelFormat;

use std::ops::{Sub, Div};
use std::thread;
use std::time::{Duration, Instant};

pub const WIDTH: u32 = 300;
pub const HEIGHT: u32 = 200;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 640;

const FPS: u32 = 30;
const TITLE: &str = "Lochlans raycasting software renderer";

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window(TITLE, WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(sdl2::pixels::PixelFormatEnum::RGB24, WIDTH, HEIGHT)
        .unwrap();

    let mut buffer: PixelBuffer = vec![200; (WIDTH * HEIGHT * 3) as usize];

    // fps limiter, and frame culling
    let frame_time = Duration::from_secs(1).div(FPS);
    
    let mut frame_start = Instant::now();

    let mut fps_count = 0;
    let mut sec_counter = Instant::now();

    // gameplay and graphics and such
    
    let map1 = new_map(
        vec![
            0, 0, 3, 2, 3, 2, 3, 0,
            0, 2, 0, 0, 0, 0, 0, 2,
            0, 3, 1, 0, 0, 0, 0, 3,
            0, 0, 2, 3, 2, 0, 0, 2,
            0, 3, 0, 0, 0, 0, 0, 3,
            0, 0, 2, 3, 2, 3, 2, 0
        ],8,6
    );

    let mut local_player = init_Player();

    set_player_start_position(&mut local_player, &map1);

    // Main loop
    let mut running = true;
    while running {

        frame_start = Instant::now();

        keyboard_events(&sdl_context, &mut running, &mut local_player, &map1);
        
        // You can perform any rendering updates here by modifying the buffer.
        
        master_rendertask(&mut buffer, &map1, &mut local_player);

        //println!("position: {:?}, rotation: {}", local_player.pos, local_player.rot);

        // Update the texture with the buffer and draw it to the screen
        texture.update(None, &buffer, (WIDTH * 3) as usize).unwrap();
        canvas.clear();
        canvas.copy(&texture, None, None).unwrap();
        canvas.present();
        
        // some frame culling for a consistent FPS
        fps_count += 1;
        if Duration::as_secs(&Instant::duration_since(&Instant::now(), sec_counter)) >= 1 {
            println!("fps: {}", fps_count);
            sec_counter = Instant::now();
            fps_count = 0;
        }
        
        let rend_time_safe = (Instant::now() - frame_start).as_secs_f32();
        let frame_time_safe = frame_time.as_secs_f32();
        
        let time_to_render = Instant::now() - frame_start;
        
        if frame_time_safe - rend_time_safe >= frame_time.as_secs_f32() / (FPS as f32) {
            thread::sleep(frame_time.sub(time_to_render));
        }
        
    }
}
