use lib::{contours, detect_objects, position, read_frame};
use std::thread;
use std::time::Instant;
mod lib;

fn main() {
    let frame = read_frame();

    let mut current_position: i64 = 0;
    let mut cycle_times = Vec::with_capacity(1000);
    let mut position_times = Vec::with_capacity(1000);

    let mut prev_objects = detect_objects(&frame);
    let mut prev_contours = contours(&frame);

    for _ in 0..1000000 {
        let start_cycle = Instant::now();

        let next_frame = read_frame();

        thread::scope(|s| {
            let handle = s.spawn(|| detect_objects(&next_frame));

            let start_position = Instant::now();
            current_position = position(&prev_objects, &prev_contours);
            position_times.push(start_position.elapsed().as_millis() as i64);

            prev_contours = contours(&next_frame);
            prev_objects = handle.join().expect("Subthread errored");
        });

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
