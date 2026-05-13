use std::collections::VecDeque;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{Texture, TextureCreator};
use sdl2::surface::Surface;
use sdl2::video::WindowContext;
use crate::traffic::{Direction, Vehicule};
use crate::{DISTANCE, SAFE_DISTANCE};







pub fn load_texture_from_path<'a>(
    texture_creator: &'a TextureCreator<WindowContext>,
    path: &str,
) -> Result<Texture<'a>, String> {
    let img = image::open(path).map_err(|e| e.to_string())?.to_rgba8();
    let (width, height) = img.dimensions();

    let mut surface = Surface::new(width, height, PixelFormatEnum::RGBA32)
        .map_err(|e| e.to_string())?;

    surface.with_lock_mut(|buffer: &mut [u8]| {
        buffer.copy_from_slice(&img);
    });

    texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())
}

pub fn is_off_screen(v: &Vehicule) -> bool {
    match v.direction {
        Direction::Up => v.y < -10,
        Direction::Down => v.y > 810,
        Direction::Left => v.x < -10,
        Direction::Right => v.x > 810,
    }
}

pub fn spawn_params(key: Keycode, ranger: i32) -> Option<(i32, i32, Direction, f64)> {
    match key {
        Keycode::Up => Some((410 + ranger, 800, Direction::Up, 0.0)),
        Keycode::Down => Some((275 + ranger, 0, Direction::Down, 180.0)),
        Keycode::Left => Some((800, 270 + ranger, Direction::Left, -90.0)),
        Keycode::Right => Some((0, 400 + ranger, Direction::Right, 90.0)),
        _ => None,
    }
}

pub fn step_traffic(rect: &mut VecDeque<Vehicule>, close_calls: &mut i32) {
    let mut new_cars: VecDeque<Vehicule> = VecDeque::new();
    let current_state = rect.clone();

    for (i, v) in rect.iter_mut().enumerate() {
        let mut can_update_car = true;
        let mut spedd_bolean = true;

        for (j, other) in current_state.iter().enumerate() {
            if i != j {
                if v.collitions(other, SAFE_DISTANCE) {
                    spedd_bolean = false;
                }
                if v.collitions(other, DISTANCE) {
                    can_update_car = false;
                    if v.states {
                        *close_calls += 1;
                    }
                    v.states = false;
                    break;
                }
            }
        }

        if can_update_car {
            if v.frame_count >= 10 {
                v.speed = if spedd_bolean { 3 } else { 1 };
                v.update();
                v.frame_count = 0;
            } else {
                v.frame_count += 1;
            }
        }

        if !is_off_screen(v) {
            new_cars.push_back(*v);
        }
    }

    *rect = new_cars;
}
