/*
# Emojis and characters for the status bar
# 💎 💻 💡 🔌 ⚡ 📁 🐧 ↑\|  🔋
*/

use std::{fs, thread, time};
use std::io::{stdout,Write};

fn print_status() {
    let now = chrono::offset::Local::now();
    let ts_s = now.format("%Y-%m-%dT%H:%M (%:z)");
    let data_s = fs::read_to_string("/proc/uptime").expect("nouptime");
    let uptimes: Vec<&str> = data_s.strip_suffix("\n").expect("uptimes").split(" ").collect();
    let data_s = fs::read_to_string("/proc/loadavg").expect("noload");
    let loads: Vec<&str> = data_s.strip_suffix("\n").expect("loads").split(" ").collect();

    let time_idle: f32 = uptimes[1].parse().unwrap();
    let time_up: f32 = uptimes[0].parse().unwrap();
    let time_up_w = (time_up / (60.0*60.0*24.0*7.0)) as i32;
    let time_up_w_rem = time_up % (60.0*60.0*24.0*7.0);
    let time_up_d = (time_up_w_rem / (60.0*60.0*24.0)) as i32;
    let time_up_d_rem = time_up_w_rem % (60.0*60.0*24.0);
    let time_up_h = (time_up_d_rem / (60.0*60.0)) as i32;
    let time_up_h_rem = time_up_d_rem % (60.0*60.0);
    let time_up_m = (time_up_h_rem / 60.0) as i32;

    let pct_idle = (time_idle / 8.0) / time_up;
    
    println!("{}w {}d {}h {}m ({:.1}%) | {} ({}) | <bat> | {}",
	     time_up_w,
	     time_up_d,
	     time_up_h,
	     time_up_m,
	     pct_idle*100.0,
	     loads[0],
	     loads[3],
	     ts_s );
    stdout().flush();
}

fn main() {
    loop {
	print_status();
	thread::sleep(time::Duration::from_secs(3));
    }
}

