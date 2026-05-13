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

impl Direction {
    pub fn index(&self) -> usize {
        match self {
            Direction::Up => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Right => 3,
        }
    }
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
            speed: 4,
            distance: 0,
            timer: Instant::now(),
            states: true,
            turning: false,
            velocity: 0.0,
        }
    }

    pub fn update(&mut self) {
        let elapsed = self.timer.elapsed().as_secs_f32();
        if elapsed > 0.0 {
            self.velocity = self.distance as f32 / elapsed;
        }

        if let Some(new_dir) = self.should_turning() {
            self.direction = new_dir;
            self.angle = match new_dir {
                Direction::Up => 0.0,
                Direction::Down => 180.0,
                Direction::Left => -90.0,
                Direction::Right => 90.0,
            };
            self.turning = false;
            self.speed = 0;
        }

        match self.direction {
            Direction::Up => self.y -= self.speed,
            Direction::Down => self.y += self.speed,
            Direction::Left => self.x -= self.speed,
            Direction::Right => self.x += self.speed,
        }
        self.distance += self.speed;
    }

    pub fn should_turning(&self) -> Option<Direction> {
        if !self.turning {
            return None;
        }
        match self.direction {
            Direction::Up if self.x == 410 && self.y <= 355 => Some(Direction::Left),
            Direction::Up if self.x == 500 && self.y <= 490 => Some(Direction::Right),
            Direction::Down if self.x == 365 && self.y >= 405 => Some(Direction::Right),
            Direction::Down if self.x == 275 && self.y >= 270 => Some(Direction::Left),
            Direction::Left if self.y == 270 && self.x <= 500 => Some(Direction::Up),
            Direction::Left if self.y == 360 && self.x <= 365 => Some(Direction::Down),
            Direction::Right if self.y == 400 && self.x >= 405 => Some(Direction::Up),
            Direction::Right if self.y == 490 && self.x >= 275 => Some(Direction::Down),
            _ => None,
        }
    }

    pub fn collitions(&self, other: &Vehicule, safe_distance: i32) -> bool {
        let dx = (self.x - other.x).abs();
        let dy = (self.y - other.y).abs();

        let ahead = match self.direction {
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
        };

        if ahead {
            return true;
        }

        let perpendicular = matches!(
            (self.direction, other.direction),
            (Direction::Up | Direction::Down, Direction::Left | Direction::Right)
                | (Direction::Left | Direction::Right, Direction::Up | Direction::Down)
        );

        if !perpendicular {
            return false;
        }

        let self_remaining = match self.direction {
            Direction::Up => self.y - other.y,
            Direction::Down => other.y - self.y,
            Direction::Left => self.x - other.x,
            Direction::Right => other.x - self.x,
        };
        let other_remaining = match other.direction {
            Direction::Up => other.y - self.y,
            Direction::Down => self.y - other.y,
            Direction::Left => other.x - self.x,
            Direction::Right => self.x - other.x,
        };

        let in_range = self_remaining >= 0
            && self_remaining <= safe_distance
            && other_remaining >= 0
            && other_remaining <= safe_distance;
        if !in_range {
            return false;
        }

        self_remaining > other_remaining
            || (self_remaining == other_remaining
                && self.direction.index() > other.direction.index())
    }
}

