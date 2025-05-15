use rand::Rng;
use std::{thread, time::Duration};

pub fn read_frame() -> [i32; 1280] {
    let mut rng = rand::rng();
    let mut frame = [0i32; 1280];
    for x in frame.iter_mut() {
        *x = rng.random_range(0..256);
    }
    frame
}

pub fn detect_objects(_frame: &[i32; 1280]) -> [i32; 20] {
    thread::sleep(Duration::from_millis(15));
    let mut rng = rand::rng();
    let mut objects = [0i32; 20];
    for x in objects.iter_mut() {
        *x = rng.random_range(0..256);
    }
    objects
}

pub fn contours(frame: &[i32; 1280]) -> [i32; 100] {
    let mut rng = rand::rng();
    let mut conts = [0i32; 100];
    for x in conts.iter_mut() {
        *x = rng.random_range(0..256);
    }
    conts
}

pub fn position(objects: &[i32; 20], contours: &[i32; 100]) -> i64 {
    let mut rng = rand::rng();
    let mut sum: i64 = 0;
    for _ in 0..1000000 {
        let next: i64 = rng.random_range(-50..51);
        sum += next;
    }
    (objects[0] as i64) + (contours[0] as i64) + sum
}
