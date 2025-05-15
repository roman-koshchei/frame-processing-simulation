use rand::Rng;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

fn read_frame() -> [i32; 1280] {
    let mut rng = rand::thread_rng();
    let mut frame = [0i32; 1280];
    for x in frame.iter_mut() {
        *x = rng.gen_range(0..256);
    }
    frame
}

fn detect_objects(frame: &[i32; 1280]) -> [i32; 20] {
    thread::sleep(Duration::from_millis(15));
    let mut rng = rand::thread_rng();
    let mut objects = [0i32; 20];
    for x in objects.iter_mut() {
        *x = rng.gen_range(0..256);
    }
    objects
}

fn contours(frame: &[i32; 1280]) -> [i32; 100] {
    let mut rng = rand::thread_rng();
    let mut conts = [0i32; 100];
    for x in conts.iter_mut() {
        *x = rng.gen_range(0..256);
    }
    conts
}

fn position(objects: &[i32; 20], contours: &[i32; 100]) -> i64 {
    let mut rng = rand::thread_rng();
    let mut sum: i64 = 0;
    for _ in 0..1000000 {
        let next: i64 = rng.gen_range(-50..51);
        sum += next;
    }
    (objects[0] as i64) + (contours[0] as i64) + sum
}

fn main() {
    // Create channels for communication
    let (frame_sender, frame_receiver) = mpsc::channel::<[i32; 1280]>();
    let (objects_sender, objects_receiver) = mpsc::channel::<[i32; 20]>();

    // Spawn the dedicated worker thread
    let worker_handle = thread::spawn(move || {
        while let Ok(frame) = frame_receiver.recv() {
            let objects = detect_objects(&frame);
            objects_sender.send(objects).unwrap();
        }
    });

    // Initial setup
    let frame = read_frame();
    frame_sender.send(frame).unwrap();
    let objects_init = objects_receiver.recv().unwrap();
    let contours_init = contours(&frame);

    let mut current_position: i64 = 0;
    let mut cycle_times = Vec::with_capacity(1000);
    let mut position_times = Vec::with_capacity(1000);

    let mut prev_objects = objects_init;
    let mut prev_contours = contours_init;

    for _ in 0..10000 {
        let start_cycle = Instant::now();

        let next_frame = read_frame();
        frame_sender.send(next_frame.clone()).unwrap();

        let start_position = Instant::now();
        current_position = position(&prev_objects, &prev_contours);
        position_times.push(start_position.elapsed().as_millis() as i64);

        let next_contours = contours(&next_frame);

        let next_objects = objects_receiver.recv().unwrap();

        prev_objects = next_objects;
        prev_contours = next_contours;

        cycle_times.push(start_cycle.elapsed().as_millis() as i64);
    }

    // Signal the worker thread to exit by dropping the senders
    drop(frame_sender);
    // drop(objects_sender);
    worker_handle.join().unwrap();

    // Print results
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
