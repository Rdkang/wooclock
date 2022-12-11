#![allow(dead_code)]
#![allow(unused_variables)]

use clap::{Parser, Subcommand};
use colored::*;
use notify_rust::Notification;
use std::thread::sleep;
use std::time::{Duration, SystemTime};
use std::{fmt, fs};
extern crate alloc;

// #[derive(Parser, Debug)]
// #[clap(author,version,about,long_about=None)]
// struct Args {
//     /// using stopwatch
//     #[clap[short,long]]
//     stopwatch: bool,
//     /// using timer
//     #[clap[short,long]]
//     timer: bool,
// }

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Args {
    #[command(subcommand)]
    stopwatch: Option<Commands>,
    timer: Option<bool>,
}

#[derive(Subcommand)]
enum Commands {
    // Status { status: Option<bool> },
    /// starts a new timer
    New,
    //
    /// shows status
    Status,
    /// stops the current
    Stop,
}

enum Paths {
    Stopwatch,
    Timer,
    StopwatchStop,
    TimerStop,
}

impl fmt::Display for Paths {
    fn fmt(&self, f: &mut fmt::Formatter) -> &str {
        match self {
            Paths::Stopwatch => "/tmp/wooclock-stopwatch.txt",
            Paths::StopwatchStop => "/tmp/wooclock-stopwatch-stop.txt",
            Paths::Timer => "/tmp/wooclock-timer.txt",
            Paths::TimerStop => "/tmp/wooclock-timer-stop.txt",
        }
    }
}

fn main() {
    let args = Args::parse();
    let now = SystemTime::now();

    match &args.stopwatch {
        Some(Commands::Status) => {
            stopwatch_status(Paths::Stopwatch);
        }
        // Some(Commands::New) => new_stopwatch(stopwatch_path, now),
        None => {
            println!("Default subcommand");
        }
        _ => {
            print("subcommand something else".yellow());
        }
    }
}

fn stop_process(path: &str) {
    let file = match std::fs::File::create(path) {
        Ok(msg) => {
            let current_time = read_time(path);
            notify(&format!("ended at {} msg={:?}", &current_time, msg));
        }
        Err(error) => {
            eprintln!("problem in stop_process {}", error);
        }
    };
}

fn new_stopwatch(path: &str, now: std::time::SystemTime) {
    let stop_path = "/tmp/wooclock-stopwatch-stop.txt";
    // deletes the stop file if exists
    if std::path::Path::new(stop_path).exists() {
        std::fs::remove_file(stop_path).unwrap()
    }

    notify("started a new stopwatch");

    loop {
        if std::path::Path::new(stop_path).exists() {
            print("stop file exists, exiting".yellow());
            break;
        }
        sleep(Duration::new(1, 0));
        let time = get_time(now);
        write_time(path, time)
    }
}

fn write_time(path: &str, time: alloc::string::String) {
    let file = fs::write(path, time);
    match file {
        Ok(msg) => msg,
        Err(_e) => {
            notify("problem with writing to file");
            std::process::exit(3);
        }
    }
}

fn stopwatch_status(path: Paths) {
    let current_time = read_time(path);
    if std::path::Path::new("/tmp/wooclock-stopwatch.txt").exists() {
        notify(&format!("ended at {}", &current_time));
        std::process::exit(2);
    }
    notify(&format!("ongoing {}", &current_time));
}

fn read_time(path: &str) -> String {
    let file = fs::read_to_string(path);
    match file {
        Ok(msg) => return msg.to_string(),
        Err(e) => {
            notify(&format!("problem reading file {}", e));
            std::process::exit(4);
        }
    }
}

fn get_time(now: std::time::SystemTime) -> String {
    match now.elapsed() {
        Ok(elapsed) => {
            let time = elapsed.as_secs();
            let output = time_formatted(time);
            print(output.italic().cyan().bold());
            return output;
        }
        Err(e) => {
            println!("Error: {e:?}");
            return "Error in get_time".to_string();
        }
    }
}

fn time_formatted(secs: u64) -> String {
    let sec = (secs % 60) as u8;
    let min = ((secs / 60) % 60) as u8;
    let hrs = secs / 60 / 60;

    // 0>2 pads the number with 0s to the left if less than 2 digits wide
    if hrs > 0 {
        // If there are hours to show:
        format!("{hrs}:{min:0>2}:{sec:0>2}")
    } else if min > 0 {
        // Else if there are minutes to show:
        format!("{min}:{sec:0>2}")
    } else {
        // If there are only seconds to show:
        format!("{sec}s")
    }
}

fn print(text: ColoredString) {
    println!("{}", text)
}
fn _print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn notify(body: &str) {
    let body_final = format!("{}", &body.to_string());
    Notification::new()
        .summary("wooclock")
        .body(&body_final)
        .icon("org.gnome.clocks")
        .action("default", "default")
        .action("clicked", "click here")
        .action("clicked", "second")
        // .hint(Hint::Resident(true))
        .show()
        .unwrap();
    // .wait_for_action(|action| match action {
    //     "default" => println!("you clicked \"default\""),
    //     "clicked" => println!("that was correct"),
    //     // here "__closed" is a hard coded keyword
    //     "__closed" => println!("the notification was closed"),
    //     _ => (),
    // });
}
