use notify_rust::{Notification};
use colored::*;
use clap::*;
use std::thread::sleep;
use std::time::Duration;
use indicatif::{ProgressBar};
use std::{thread, io};
use std::io::Read;
use std::sync::{Arc, Mutex};

#[cfg(target_os = "macos")]
static SOUND: &'static str = "Submarine";

#[cfg(target_os = "macos")]
static STOP_SOUND: &'static str = "Blow";

#[cfg(all(unix, not(target_os = "macos")))]
static SOUND: &str = "message-new-instant";

#[cfg(target_os = "windows")]
static SOUND: &'static str = "Mail";


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    timer: i32,
    stop: i32,
    repeat: i32,
}

fn main() {
    let cli = Cli::parse();
    let mutex = Arc::new(Mutex::new(0));
    let pause = Arc::clone(&mutex);

    thread::spawn(move || {
        loop {
            let mut buffer = [0u8; 1];
            match io::stdin().read(&mut buffer) {
                Ok(_) => {
                    *pause.lock().unwrap() ^= 1;
                }
                Err(error) => println!("Error: {}", error),
            }
        }
    });

    for r in 0..cli.repeat {
        print!("\x1B[2J\x1B[1;1H");
        println!("Your 🍅 timer will start now! \nTimer is set for {} min, break for {} min and will repeat {} times",
                 format!("{} minutes", cli.timer).green(),
                 format!("{} minutes", cli.stop).red(),
                 format!("{} times", cli.repeat).cyan());
        Notification::new().summary("Work Time").body("Time to start working 💻").sound_name(SOUND).timeout(5000).show().unwrap();
        println!("\n\nTime to Work 💻, {} cycles left\n", format!("{}", cli.repeat - r).cyan());

        let deps: i32 = cli.timer * 60;
        let pb = ProgressBar::new(deps as u64);
        pb.inc(0);
        for _ in 0..deps {
            sleep(Duration::from_secs(1));
            pb.inc(1);
            while *mutex.lock().unwrap() == 1 {
                pb.suspend(|| {
                    sleep(Duration::from_millis(200));
                });
            }
        }
        pb.finish_and_clear();

        println!("{}", "Time for a break 😴".red());
        Notification::new().summary("Break Time").body("Time to take a break now 😴").sound_name(STOP_SOUND).timeout(5000).show().unwrap();

        let deps: i32 = cli.stop * 60;
        let pb = ProgressBar::new(deps as u64);
        pb.inc(0);
        for _ in 0..deps {
            sleep(Duration::from_secs(1));
            pb.inc(1);
        }
        pb.finish_and_clear();
    }

    // Alert user
    println!("🎊 🎊 🎊You are done! Great work! 🎊 🎊 🎊");
    Notification::new().summary("Done!").body("🎊 🎊 🎊You are done! Great work! 🎊 🎊 🎊").sound_name(SOUND).timeout(5000).show().unwrap();
}
