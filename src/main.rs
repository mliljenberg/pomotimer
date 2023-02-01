use notify_rust::{Notification};
use colored::*;
use clap::*;
use std::thread::sleep;
use std::time::Duration;
use indicatif::{ProgressBar};
use std::{thread, io};
use std::io::{Read, Write};
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
    let mutex: Arc<Mutex<i8>> = Arc::new(Mutex::new(0));
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
        println!("\n\nTime to Work 💻, {} cycles left, press ENTER to pause/restart \n", format!("{}", cli.repeat - r).cyan());

        let deps: i32 = cli.timer * 60;
        let pb = ProgressBar::new(deps as u64);
        pb.inc(0);
        for _ in 0..deps {
            sleep(Duration::from_secs(1));
            user_pause(&mutex, &pb);
            pb.inc(1);
        }
        pb.finish_and_clear();

        println!("{}", "Time for a break 😴".red());
        Notification::new().summary("Break Time").body("Time to take a break now 😴").sound_name(STOP_SOUND).timeout(5000).show().unwrap();

        let deps: i32 = cli.stop * 60;
        let pb = ProgressBar::new(deps as u64);
        pb.inc(0);
        for _ in 0..deps {
            sleep(Duration::from_secs(1));
            user_pause(&mutex, &pb);
            pb.inc(1);
        }
        pb.finish_and_clear();
    }

    // Alert user
    println!("🎊 🎊 🎊You are done! Great work! 🎊 🎊 🎊");
    Notification::new().summary("Done!").body("🎊 🎊 🎊You are done! Great work! 🎊 🎊 🎊").sound_name(SOUND).timeout(5000).show().unwrap();
}

/*
    If user has pressed enter we wait until the user does so again
 */
fn user_pause(mutex: &Arc<Mutex<i8>>, pb: &ProgressBar) {
    while *mutex.lock().unwrap() == 1 {
        pb.suspend(|| {
            sleep(Duration::from_millis(200));
        });
        if *mutex.lock().unwrap() == 0 {
            print!("\x1b[2K\x1b[1A\x1b[2K\x1b[1A\x1b[2K"); // Deletes 2 lines;
            io::stdout().flush().unwrap();
        }
    }
}
