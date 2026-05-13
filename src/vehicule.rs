use std::time::Instant;

const CAR_WIDTH: u32 = 25;
const CAR_HEIGHT: u32 = 30;

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

    pub fn collitions(&self, other: &Vehicule, safe_distance: i32) -> bool {
        let dx = (self.x - other.x).abs();
        let dy = (self.y - other.y).abs();

        match self.direction {
            Direction::Down => {
                other.y >= self.y && dy <= safe_distance && dx < CAR_WIDTH as i32
            }
            Direction::Up => {
                self.y >= other.y && dy <= safe_distance && dx < CAR_WIDTH as i32
            }
            Direction::Left => {
                self.x >= other.x && dx <= safe_distance && dy < CAR_HEIGHT as i32
            }
            Direction::Right => {
                other.x >= self.x && dx <= safe_distance && dy < CAR_HEIGHT as i32
            }
        }
    }
}
