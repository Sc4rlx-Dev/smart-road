use std::time::Instant;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy)]
pub struct Vehicule {
    pub x: i32,
    pub y: i32,
    pub direction: Direction,
    pub speed: i32,
    pub velocity: f32,
    pub distance: i32,
    pub timer: Instant,
    pub states: bool,
    pub frame_count: u32,
    pub angle: f64,
    pub turning: bool,
}

impl Vehicule {
    pub fn new(x: i32, y: i32, direction: Direction, angle: f64) -> Self {
        Vehicule {
            x,
            y,
            direction,
            angle,
            speed: 3,
            distance: 0,
            timer: Instant::now(),
            states: true,
            frame_count: 0,
            turning: false,
            velocity: 0.0,
        }
    }

    pub fn update(&mut self) {
        match self.direction {
            Direction::Up => self.y -= self.speed,
            Direction::Down => self.y += self.speed,
            Direction::Left => self.x -= self.speed,
            Direction::Right => self.x += self.speed,
        }
        self.distance += self.speed;
    }
}
