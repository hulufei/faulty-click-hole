use rdev::{grab, Button, Event, EventType};
use std::{sync::Mutex, time::SystemTime};

fn main() {
    let prev_click_time = Mutex::new(SystemTime::now());

    let callback = move |event: Event| -> Option<Event> {
        if let EventType::ButtonPress(Button::Left) = event.event_type {
            let curr_click_time = event.time;
            let mut prev_click_time = prev_click_time.lock().unwrap();

            if let Ok(duration) = curr_click_time.duration_since(*prev_click_time) {
                println!("Duration {:?}", duration);
                *prev_click_time = curr_click_time;
                if duration.as_millis() < 150 {
                    println!("Ignore Faulty Click");
                    return None;
                }
            }
        }
        Some(event)
    };

    if let Err(error) = grab(callback) {
        println!("Error: {:?}", error)
    }
}
