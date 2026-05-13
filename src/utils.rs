use std::collections::VecDeque;
use std::time::Duration;

use rand::rngs::ThreadRng;
use rand::Rng;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::surface::Surface;
use sdl2::video::{Window, WindowContext};
use crate::traffic::{Direction, Vehicule};
use crate::{CAR_HEIGHT, CAR_WIDTH, COOLDOWN_FRAMES, DISTANCE, SAFE_DISTANCE};







pub struct Stats {
    pub nbr_of_cars: i32,
    pub close_calls: i32,
    pub vec_timer: Vec<Duration>,
    pub velocities: Vec<f32>,
}

impl Stats {
    pub fn new() -> Self {
        Stats {
            nbr_of_cars: 0,
            close_calls: 0,
            vec_timer: Vec::new(),
            velocities: Vec::new(),
        }
    }
}

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

pub fn try_spawn(
    rect: &mut VecDeque<Vehicule>,
    rng: &mut ThreadRng,
    cooldowns: &mut [i32; 4],
    key: Keycode,
) -> bool {
    let ranger = rng.gen_range(0..3) * 45;
    if let Some((x, y, dir, angle)) = spawn_params(key, ranger) {
        if cooldowns[dir.index()] == 0 {
            let mut v = Vehicule::new(x, y, dir, angle);
            if ranger == 0 || ranger == 90 {
                v.turning = true;
            }
            rect.push_back(v);
            cooldowns[dir.index()] = COOLDOWN_FRAMES;
            return true;
        }
    }
    false
}

pub fn random_direction_keycode(rng: &mut ThreadRng) -> Keycode {
    let dirs = [Keycode::Up, Keycode::Down, Keycode::Left, Keycode::Right];
    dirs[rng.gen_range(0..dirs.len())]
}

pub fn step_traffic(rect: &mut VecDeque<Vehicule>, stats: &mut Stats) {
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
                        stats.close_calls += 1;
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

        if is_off_screen(v) {
            stats.nbr_of_cars += 1;
            stats.vec_timer.push(v.timer.elapsed());
            stats.velocities.push(v.velocity);
        } else {
            new_cars.push_back(*v);
        }
    }

    *rect = new_cars;
}

pub fn render_frame(
    canvas: &mut Canvas<Window>,
    road_texture: &Texture,
    car_texture: &Texture,
    rect: &VecDeque<Vehicule>,
) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.copy(road_texture, None, Some(Rect::new(0, 0, 800, 800)))?;

    for v in rect {
        let target = Rect::new(v.x, v.y, CAR_WIDTH, CAR_HEIGHT);
        canvas.copy_ex(car_texture, None, Some(target), v.angle, None, false, false)?;
    }

    canvas.present();
    Ok(())
}
