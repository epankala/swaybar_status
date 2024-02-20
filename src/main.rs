/*
# Emojis and characters for the status bar
# 💎 💻 💡 🔌 ⚡ 📁 🐧 ↑\|  🔋
*/

use std::path::Path;
use std::{fs, thread, time};
use std::io::{stdout,Write};
use std::process::Command;
//use std::any::type_name;

static mut PREV_CAP: i32 = 0;

fn _print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn uptime_as_string_parts(time_up: f32) -> (i32, i32, i32, i32) {
	const SECONDS_IN_MINUTE: f32 = 60.0;
	const MINUTES_IN_HOUR: f32 = 60.0;
	const HOURS_IN_DAY: f32 = 24.0;
	const DAYS_IN_WEEK: f32 = 7.0;

    let time_up_w = (time_up / (SECONDS_IN_MINUTE * MINUTES_IN_HOUR * HOURS_IN_DAY * DAYS_IN_WEEK)) as i32;
    let time_up_w_rem = time_up % (SECONDS_IN_MINUTE * MINUTES_IN_HOUR * HOURS_IN_DAY * DAYS_IN_WEEK);
    let time_up_d = (time_up_w_rem / (SECONDS_IN_MINUTE * MINUTES_IN_HOUR * HOURS_IN_DAY)) as i32;
    let time_up_d_rem = time_up_w_rem % (SECONDS_IN_MINUTE * MINUTES_IN_HOUR * HOURS_IN_DAY);
    let time_up_h = (time_up_d_rem / (SECONDS_IN_MINUTE * MINUTES_IN_HOUR)) as i32;
    let time_up_h_rem = time_up_d_rem % (SECONDS_IN_MINUTE * MINUTES_IN_HOUR);
    let time_up_m = (time_up_h_rem / SECONDS_IN_MINUTE) as i32;

    (time_up_w, time_up_d, time_up_h, time_up_m)
}


fn print_status() {
    let now = chrono::offset::Local::now();
    let ts_s = now.format("%Y-%m-%dT%H:%M (%:z)");
    let data_s = fs::read_to_string("/proc/uptime").expect("nouptime");
    let uptimes: Vec<&str> = data_s.strip_suffix("\n").expect("uptimes").split(" ").collect();
    let data_s = fs::read_to_string("/proc/loadavg").expect("noload");
    let loads: Vec<&str> = data_s.strip_suffix("\n").expect("loads").split(" ").collect();

    let time_idle: f32 = uptimes[1].parse().unwrap();
    let time_up: f32 = uptimes[0].parse().unwrap();
	let (time_up_w, time_up_d, time_up_h, time_up_m) = uptime_as_string_parts(time_up);

    let pct_idle = (time_idle / 8.0) / time_up;

    let mut cap_i: i32 = 0;
    let mut bat_mode = "🔌".to_string();
    if Path::new("/sys/class/power_supply/BAT0").exists() {
		cap_i = fs::read_to_string("/sys/class/power_supply/BAT0/capacity").expect("no capacity").trim().parse().expect("Input not an integer");
		let stat_s = fs::read_to_string("/sys/class/power_supply/BAT0/status").expect("unknown");

		// 💎 💻 💡 🔌 ⚡ 📁 🐧 ↑\|  🔋

		match stat_s.trim() {
			"Not Charging" => {bat_mode="💎".to_string(); }
			"Charging" => {bat_mode="⚡".to_string(); }
			"Full" => { bat_mode="🔌".to_string(); }
			"Discharging" => {
				bat_mode="🔋".to_string();
			}
			_ => {
				bat_mode="🔌".to_string();
			}
		}

		unsafe {
			if cap_i < PREV_CAP {
				let mut notify = Command::new("/usr/bin/notify-send");
				notify
					.arg("-t")
					.arg("10000")
					.arg(format!("Battery at {}", cap_i))
					.output()
					.expect("Failed to send notification");
			}
			PREV_CAP = cap_i;
		};
    }


    println!("{}w {}d {}h {}m ({:.1}%) | {} ({}) | {} {} | {}",
	     time_up_w,
	     time_up_d,
	     time_up_h,
	     time_up_m,
	     pct_idle*100.0,
	     loads[0],
	     loads[3],
	     cap_i,
	     bat_mode,
	     ts_s );
    stdout().flush().unwrap();
}

fn main() {
    loop {
		print_status();
		thread::sleep(time::Duration::from_secs(3));
    }
}

