use std::{thread, time};

pub fn wait(seconds: u64) {
    thread::sleep(time::Duration::from_millis(seconds * 1000));
}