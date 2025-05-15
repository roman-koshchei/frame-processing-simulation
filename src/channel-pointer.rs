use core_affinity::CoreId;
use rand::Rng;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

// Newtype to wrap the raw pointer
#[derive(Copy, Clone)]
struct FramePtr(*const [i32; 1280]);

// Safety: We assert that FramePtr is Send because:
// 1. The pointer points to a [i32; 1280] that remains alive and immutable
//    until the worker thread's detect_objects call completes.
// 2. The main thread does not modify or drop the frame until objects_receiver.recv()
//    confirms processing, as ensured by frame_storage.
// 3. [i32; 1280] itself is Send, and the pointer is only used immutably.
unsafe impl Send for FramePtr {}

fn read_frame() -> [i32; 1280] {
    let mut rng = rand::rng();
    let mut frame = [0i32; 1280];
    for x in frame.iter_mut() {
        *x = rng.random_range(0..256);
    }
    frame
}

fn detect_objects(_frame: &[i32; 1280]) -> [i32; 20] {
    thread::sleep(Duration::from_millis(15));
    let mut rng = rand::rng();
    let mut objects = [0i32; 20];
    for x in objects.iter_mut() {
        *x = rng.random_range(0..256);
    }
    objects
}

fn contours(_frame: &[i32; 1280]) -> [i32; 100] {
    let mut rng = rand::rng();
    let mut conts = [0i32; 100];
    for x in conts.iter_mut() {
        *x = rng.random_range(0..256);
    }
    conts
}

fn position(objects: &[i32; 20], contours: &[i32; 100]) -> i64 {
    let mut rng = rand::rng();
    let mut sum: i64 = 0;
    for _ in 0..1000000 {
        let next: i64 = rng.random_range(-50..51);
        sum += next;
    }
    (objects[0] as i64) + (contours[0] as i64) + sum
}

fn main() {
    core_affinity::set_for_current(CoreId { id: 0 });

    // Create channels for communication
    // Use FramePtr to send pointers safely
    let (frame_sender, frame_receiver) = mpsc::channel::<FramePtr>();
    let (objects_sender, objects_receiver) = mpsc::channel::<[i32; 20]>();

    // Spawn the dedicated worker thread
    let worker_handle = thread::spawn(move || {
        core_affinity::set_for_current(CoreId { id: 1 });

        while let Ok(FramePtr(frame_ptr)) = frame_receiver.recv() {
            // Safety: frame_ptr points to a valid, immutable [i32; 1280]
            // that remains alive until detect_objects completes, as guaranteed
            // by frame_storage and the recv() synchronization.
            let frame = unsafe { &*frame_ptr };
            let objects = detect_objects(frame);
            objects_sender.send(objects).unwrap();
        }
    });

    // Initial setup
    let frame = read_frame();
    let frame_box = Box::new(frame);
    frame_sender
        .send(FramePtr(&*frame_box as *const [i32; 1280]))
        .unwrap();
    let objects_init = objects_receiver.recv().unwrap();
    let contours_init = contours(&frame_box);

    let mut current_position: i64 = 0;
    let mut cycle_times = Vec::with_capacity(1000);
    let mut position_times = Vec::with_capacity(1000);

    let mut prev_objects = objects_init;
    let mut prev_contours = contours_init;

    // Store frames to ensure lifetime

    // Main loop
    for _ in 0..10000 {
        let start_cycle = Instant::now();

        // Read and box the frame
        let next_frame = Box::new(read_frame());
        // Send pointer wrapped in FramePtr
        frame_sender
            .send(FramePtr(&*next_frame as *const [i32; 1280]))
            .unwrap();

        let start_position = Instant::now();
        current_position = position(&prev_objects, &prev_contours);
        position_times.push(start_position.elapsed().as_millis() as i64);

        // Use the latest stored frame for contours
        let next_contours = contours(&next_frame);

        // Receive objects, confirming worker has processed the frame
        let next_objects = objects_receiver.recv().unwrap();
        // Drop older frames, keeping the latest for contours

        prev_objects = next_objects;
        prev_contours = next_contours;

        cycle_times.push(start_cycle.elapsed().as_millis() as i64);
    }

    // Signal the worker thread to exit
    drop(frame_sender);
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
