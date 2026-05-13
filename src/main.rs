mod vehicule;

use vehicule::{Direction, Vehicule};

use std::collections::VecDeque;
use std::time::Duration;

use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator};
use sdl2::surface::Surface;
use sdl2::video::WindowContext;

const CAR_WIDTH: u32 = 35;
const CAR_HEIGHT: u32 = 30;
const DISTANCE: i32 = 40;
const SAFE_DISTANCE: i32 = 300;

fn load_texture_from_path<'a>(
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

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Smart_Road", 800, 800)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .build()
        .map_err(|e| e.to_string())?;

    let texture_creator = canvas.texture_creator();
    let road_texture = load_texture_from_path(&texture_creator, "src/img/road.jpg")?;
    let car_texture = load_texture_from_path(&texture_creator, "src/img/car.png")?;
    let background_rect = Rect::new(0, 0, 800, 800);

    let mut rect: VecDeque<Vehicule> = VecDeque::new();
    let mut rng = rand::thread_rng();
    let mut can_add = false;
    let mut cooldown_time: i32 = 0;

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown { keycode: Some(k), .. } => {
                    if !can_add {
                        let key = if k == Keycode::R {
                            let dirs = [Keycode::Up, Keycode::Down, Keycode::Left, Keycode::Right];
                            dirs[rng.gen_range(0..dirs.len())]
                        } else {
                            k
                        };

                        let ranger = rng.gen_range(0..3) * 45;
                        let (x, y, dir, angle) = match key {
                            Keycode::Up => (410 + ranger, 800, Direction::Up, 0.0),
                            Keycode::Down => (275 + ranger, 0, Direction::Down, 180.0),
                            Keycode::Left => (800, 270 + ranger, Direction::Left, -90.0),
                            Keycode::Right => (0, 400 + ranger, Direction::Right, 90.0),
                            _ => continue,
                        };

                        let v = Vehicule::new(x, y, dir, angle);
                        rect.push_back(v);
                        can_add = true;
                    }
                }
                _ => {}
            }
        }

        if can_add {
            cooldown_time += 1;
            if cooldown_time >= 350 {
                can_add = false;
                cooldown_time = 0;
            }
        }

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

            let out = match v.direction {
                Direction::Up => v.y < -10,
                Direction::Down => v.y > 810,
                Direction::Left => v.x < -10,
                Direction::Right => v.x > 810,
            };

            if !out {
                new_cars.push_back(*v);
            }
        }

        rect = new_cars;

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.copy(&road_texture, None, Some(background_rect))?;

        for v in &rect {
            let target = Rect::new(v.x, v.y, CAR_WIDTH, CAR_HEIGHT);
            canvas.copy_ex(&car_texture, None, Some(target), v.angle, None, false, false)?;
        }

        canvas.present();

        std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
