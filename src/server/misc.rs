use chrono::{Timelike, Local};

use crate::server::state::AppState;

pub fn log(state: &AppState, level: u8, msg: &str) {
    println!("{}", msg);
    let t = Local::now();
    let formatted_msg = format!("{:02}:{:02}:{:02} | {}", 
        t.hour(), t.minute(), t.second(), msg);

    let mut data = state.write().unwrap();
    data.logs.push_back(formatted_msg);
    if data.logs.len() > 256 {
        data.logs.pop_front();
    }

    if level > 0 {
        data.toast_counter += 1;
        data.latest_toast = Some((level, msg.to_string())); 
    }
}

