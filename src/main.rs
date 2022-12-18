#![allow(dead_code)]
// #![allow(unused_variables)]

use clap::{Parser, Subcommand};
use colored::*;
use notify_rust::Notification;
use std::thread::sleep;
use std::time::{Duration, SystemTime};
use std::{fmt, fs};
extern crate alloc;
/*
TODO
-
*/

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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Paths::Stopwatch => write!(f, "/tmp/wooclock-stopwatch.txt"),
            Paths::StopwatchStop => write!(f, "/tmp/wooclock-stopwatch-stop.txt"),
            Paths::Timer => write!(f, "/tmp/wooclock-timer.txt"),
            Paths::TimerStop => write!(f, "/tmp/wooclock-timer-stop.txt"),
        }
    }
}

fn main() {
    let args = Args::parse();
    let now = SystemTime::now();

    match &args.stopwatch {
        Some(Commands::Status) => {
            stopwatch_status(Paths::Stopwatch.to_string());
        }
        Some(Commands::New) => new_stopwatch(now),
        Some(Commands::Stop) => stop_process(
            Paths::StopwatchStop.to_string(),
            Paths::Stopwatch.to_string(),
        ),

        None => println!("no subcommands"),
        // _ => print("subcommand something else".yellow()),
    }
}

fn stop_process(stop_path: std::string::String, process_path: std::string::String) {
    match std::fs::File::create(stop_path) {
        Ok(_msg) => {
            let current_time = read_time(process_path);
            notify(&format!("ended at {}", &current_time));
        }
        Err(error) => {
            eprintln!("problem in stop_process {}", error);
        }
    };
}

fn remove_stop_file(path: std::string::String) {
    if std::path::Path::new(&path).exists() {
        std::fs::remove_file(&path).unwrap();
    }
}

fn new_stopwatch(now: std::time::SystemTime) {
    notify("started a new stopwatch");
    remove_stop_file(Paths::StopwatchStop.to_string());

    loop {
        if std::path::Path::new(&Paths::StopwatchStop.to_string()).exists() {
            print("stop file exists, exiting".yellow());
            break;
        }
        sleep(Duration::new(1, 0));
        let time = get_time(now);
        write_time(Paths::Stopwatch.to_string(), time)
    }
}

fn write_time(path: std::string::String, time: alloc::string::String) {
    let file = fs::write(path, time);
    match file {
        Ok(msg) => msg,
        Err(_e) => {
            notify("problem with writing to file");
            std::process::exit(3);
        }
    }
}

fn stopwatch_status(path: std::string::String) {
    let current_time = read_time(path);
    if std::path::Path::new(&Paths::StopwatchStop.to_string()).exists() {
        notify(&format!("ended at {}", &current_time));
        std::process::exit(2);
    }
    notify(&format!("ongoing {}", &current_time));
    std::process::exit(0);
}

fn read_time(path: std::string::String) -> String {
    let file = fs::read_to_string(path);
    match file {
        Ok(msg) => msg,
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
            output
        }
        Err(e) => {
            println!("Error: {e:?}");
            "Error in get_time".to_string()
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
    Notification::new()
        .summary("wooclock")
        .appname("wooclock")
        .body(body)
        .icon("org.gnome.clocks")
        .action("default", "default")
        .action("stop", "stop")
        .action("second", "second")
        .action("third", "third")
        .show()
        .unwrap()
        .wait_for_action(|action| match action {
            "default" => println!("clicked on notification"),
            "stop" => println!("stopeed the current"),
            "__closed" => println!("the notification was closed"),
            _ => print("other".blue()),
        });
}
