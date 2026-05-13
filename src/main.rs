mod traffic;
mod utils;

use traffic::Vehicule;
use utils::{load_texture_from_path, render_frame, spawn_params, step_traffic};

use std::collections::VecDeque;
use std::time::Duration;

use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

const CAR_WIDTH: u32 = 35;
const CAR_HEIGHT: u32 = 30;
const DISTANCE: i32 = 40;
const SAFE_DISTANCE: i32 = 300;

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

    let mut rect: VecDeque<Vehicule> = VecDeque::new();
    let mut rng = rand::thread_rng();
    let mut can_add = false;
    let mut cooldown_time: i32 = 0;
    let mut close_calls: i32 = 0;

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
                        if let Some((x, y, dir, angle)) = spawn_params(key, ranger) {
                            rect.push_back(Vehicule::new(x, y, dir, angle));
                            can_add = true;
                        }
                    }
                }
                _ => {}
            }
        }

        if can_add {
            cooldown_time += 1;
            if cooldown_time >= 450 {
                can_add = false;
                cooldown_time = 0;
            }
        }

        step_traffic(&mut rect, &mut close_calls);

        render_frame(&mut canvas, &road_texture, &car_texture, &rect)?;

        std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
