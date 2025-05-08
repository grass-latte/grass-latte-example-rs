use rand::random_range;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    grass_latte::set_port_range((3030, 3030));
    grass_latte::serve_webpage();
    grass_latte::clear();

    while !grass_latte::poll_button(["Start"], "Start", false) {
        thread::sleep(Duration::from_millis(200));
    }
    grass_latte::delete_element(["Start"]);

    let counter = Arc::new(Mutex::new(0));
    let callback = move || {
        let val = counter.clone();
        let mut val = val.lock().unwrap();
        *val += 1;
        grass_latte::send_text(["Buttons", "Incrementer", "Value"], format!("{val}"), false);
    };
    grass_latte::send_button_with_callback(
        ["Buttons", "Incrementer"],
        "Click button to increment".to_string(),
        false,
        callback.clone(),
    );
    callback();

    let mut threads = Vec::new();

    for i in 0..10 {
        threads.push(thread::spawn(move || {
            let thread_name = format!("ID: {i}");
            let mut progress = 0.0;

            while progress < 1.0 {
                grass_latte::send_progress(
                    ["Threads".to_string(), thread_name.clone()],
                    None::<String>,
                    progress,
                    true,
                );
                grass_latte::send_text(
                    [
                        "Threads".to_string(),
                        thread_name.clone(),
                        "Info".to_string(),
                    ],
                    format!("Updated to {progress}"),
                    false,
                );
                progress += random_range(0.0..0.05);
                thread::sleep(Duration::from_millis(random_range(100..1000)));
            }
            grass_latte::send_progress(
                ["Threads".to_string(), thread_name.clone()],
                None::<String>,
                progress,
                true,
            );
            grass_latte::send_text(
                [
                    "Threads".to_string(),
                    thread_name.clone(),
                    "Info".to_string(),
                ],
                "Done",
                false,
            );
        }));
    }

    threads.into_iter().for_each(|t| {
        t.join().ok();
    });
}
