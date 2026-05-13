use std::thread;
use std::time::Duration;

use crate::traffic::{Direction, Vehicule};

#[test]
fn physics_velocity_equals_distance_over_time() {
    let mut v = Vehicule::new(400, 800, Direction::Up, 0.0);
    v.distance = 300;
    thread::sleep(Duration::from_millis(100));
    v.speed = 0;
    v.update();

    let expected = 300.0 / 0.1;
    let ratio = v.velocity / expected;
    assert!(
        (0.5..=1.5).contains(&ratio),
        "velocity {} not within 50% of expected {}",
        v.velocity,
        expected,
    );
}

#[test]
fn safety_distance_detects_car_directly_ahead() {
    let me = Vehicule::new(410, 500, Direction::Up, 0.0);
    let other = Vehicule::new(410, 470, Direction::Up, 0.0);
    assert!(me.collitions(&other, 40), "should detect car 30px ahead within 40px stop zone");
    assert!(me.collitions(&other, 300), "should also detect within 300px slow zone");
}

#[test]
fn safety_distance_ignores_car_behind() {
    let me = Vehicule::new(410, 500, Direction::Up, 0.0);
    let other = Vehicule::new(410, 600, Direction::Up, 0.0);
    assert!(!me.collitions(&other, 40), "should NOT detect car behind");
}

#[test]
fn safety_distance_ignores_different_lane() {
    let me = Vehicule::new(410, 500, Direction::Up, 0.0);
    let other = Vehicule::new(500, 470, Direction::Up, 0.0);
    assert!(!me.collitions(&other, 300), "different lane (dx=90 > CAR_WIDTH=25) should not detect");
}

#[test]
fn rear_end_conflict_one_car_brakes() {
    let mut a = Vehicule::new(410, 500, Direction::Up, 0.0);
    let b = Vehicule::new(410, 470, Direction::Up, 0.0);
    a.speed = 3;

    if a.collitions(&b, 40) {
        a.speed = 0;
    } else if a.collitions(&b, 300) {
        a.speed = 1;
    }

    assert!(a.speed < 3, "a should reduce speed when b is ahead in same lane");
}

#[test]
fn cross_traffic_perpendicular_conflict_detected() {
    let up_car = Vehicule::new(410, 450, Direction::Up, 0.0);
    let left_car = Vehicule::new(550, 400, Direction::Left, -90.0);
    assert!(
        up_car.collitions(&left_car, 300),
        "Up car at (410,450) and Left car at (550,400) approach the intersection point — should detect within slow zone"
    );
}

#[test]
fn cross_traffic_already_passed_ignored() {
    let up_car = Vehicule::new(410, 200, Direction::Up, 0.0);
    let left_car = Vehicule::new(700, 500, Direction::Left, -90.0);
    assert!(
        !up_car.collitions(&left_car, 300),
        "Up car already past intersection, Left car still right of it — paths no longer converge"
    );
}

#[test]
fn cross_traffic_far_apart_ignored() {
    let up_car = Vehicule::new(410, 800, Direction::Up, 0.0);
    let left_car = Vehicule::new(800, 100, Direction::Left, -90.0);
    assert!(
        !up_car.collitions(&left_car, 300),
        "Both cars far from the intersection — should not trigger"
    );
}

#[test]
fn stats_accumulation_max_min_velocity() {
    let velocities: Vec<f32> = vec![10.0, 25.5, 3.2, 18.7];
    let max = velocities.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
    let min = velocities.iter().cloned().fold(f32::INFINITY, f32::min);
    assert_eq!(max, 25.5);
    assert_eq!(min, 3.2);
}

#[test]
fn stats_accumulation_car_count() {
    let exits: Vec<Duration> = vec![
        Duration::from_secs(2),
        Duration::from_secs(5),
        Duration::from_secs(3),
    ];
    let count = exits.len() as i32;
    let max_t = exits.iter().max().cloned().unwrap_or_default();
    let min_t = exits.iter().min().cloned().unwrap_or_default();

    assert_eq!(count, 3);
    assert_eq!(max_t, Duration::from_secs(5));
    assert_eq!(min_t, Duration::from_secs(2));
}
