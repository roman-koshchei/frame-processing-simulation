use rand::Rng;
use std::thread;
use std::time::{Duration, Instant};
mod lib;

fn main() {
    let frame = read_frame();
    let handle = thread::spawn(move || detect_objects(&frame));
    let contours_init = contours(&frame);
    let objects_init = handle.join().unwrap();

    let mut current_position: i64 = 0;
    let mut cycle_times = Vec::with_capacity(1000);
    let mut position_times = Vec::with_capacity(1000);

    let mut prev_objects = objects_init;
    let mut prev_contours = contours_init;

    for _ in 0..10000 {
        let start_cycle = Instant::now();

        let next_frame = read_frame();
        let handle = thread::spawn(move || detect_objects(&next_frame));

        let start_position = Instant::now();
        current_position = position(&prev_objects, &prev_contours);
        position_times.push(start_position.elapsed().as_millis() as i64);

        let next_contours = contours(&next_frame);

        let next_objects = handle.join().unwrap();

        prev_objects = next_objects;
        prev_contours = next_contours;

        cycle_times.push(start_cycle.elapsed().as_millis() as i64);
    }

    for cycle in &cycle_times {
        println!("Loop cycle took: {}", cycle);
    }
    println!(
        "Loop cycle took on average: {}",
        cycle_times.iter().sum::<i64>() as f64 / cycle_times.len() as f64
    );
    println!("Current position: {}", current_position);
    println!();
    println!(
        "Position cycle took on average: {}",
        position_times.iter().sum::<i64>() as f64 / position_times.len() as f64
    );
}
