#![allow(dead_code)]
// #![allow(unused_variables)]

use clap::{Parser, Subcommand, ValueEnum};
use colored::*;
use notify_rust::Notification;
use opener::open;
use rand::seq::IteratorRandom;
use rofi::Rofi;
use std::io::ErrorKind;
use std::thread::sleep;
use std::time::{Duration, SystemTime};
use std::{fmt, fs};
extern crate alloc;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: ClockType,
}

#[derive(Subcommand)]
enum ClockType {
    #[command(arg_required_else_help = true)]
    Stopwatch {
        // #[arg(value_name = "Command")]
        option: Commands,
    },
    #[command(arg_required_else_help = true)]
    Timer {
        // #[arg(default_value = Commands::Rofi)]
        option: Commands,
    },
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
enum Commands {
    // Status { status: Option<bool> },
    /// starts a new timer
    New,
    //
    /// shows status
    Status,
    /// stops the current
    Stop,
    /// can choose all options in rofi
    Rofi,
}

enum Paths {
    Stopwatch,
    Timer,
    StopwatchStop,
    TimerStop,
    TimerLength,
}

impl fmt::Display for Paths {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // when call .to_string() on it will make it this path string
            Paths::Stopwatch => write!(f, "/tmp/wooclock-stopwatch.txt"),
            Paths::StopwatchStop => write!(f, "/tmp/wooclock-stopwatch-stop.txt"),
            Paths::Timer => write!(f, "/tmp/wooclock-timer.txt"),
            Paths::TimerStop => write!(f, "/tmp/wooclock-timer-stop.txt"),
            Paths::TimerLength => write!(f, "/tmp/wooclock-timer-length.txt"),
        }
    }
}

fn main() {
    let arguments = Cli::parse();
    let time_now = SystemTime::now();

    match arguments.command {
        ClockType::Stopwatch { option } => match option {
            Commands::New => new_stopwatch(time_now),
            Commands::Stop => stop_stopwatch(Paths::StopwatchStop.to_string(), Paths::Stopwatch.to_string()),
            Commands::Status => stopwatch_status(),
            Commands::Rofi => rofi_options(time_now),
        },
        ClockType::Timer { option } => match option {
            Commands::New => new_timer(),
            Commands::Stop => stop_timer(),
            Commands::Status => timer_status(),
            Commands::Rofi => rofi_options(time_now),
        },
    }
}

fn stop_stopwatch(stop_path: std::string::String, process_path: std::string::String) {
    match std::fs::File::create(stop_path) {
        Ok(_msg) => {
            let current_time: u64 = read_time(process_path);
            notify(&format!("stopwatch ran for {}", time_formatted(current_time)));
        }
        Err(error) => {
            eprintln!("problem in stop_process {}", error);
        }
    };
}

fn stop_timer() {
    match std::fs::File::create(Paths::TimerStop.to_string()) {
        Ok(_msg) => {
            let current_time: u64 = read_time(Paths::Timer.to_string());
            let timer_length: u64 = read_time(Paths::TimerLength.to_string());
            let elapsed_time: u64 = timer_length - current_time;
            notify(&format!("timer ran for {}", time_formatted(elapsed_time)));
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
    notify("stopwatch started");
    remove_stop_file(Paths::StopwatchStop.to_string());

    loop {
        // if stop file is found, will stop the loop.
        if std::path::Path::new(&Paths::StopwatchStop.to_string()).exists() {
            print("stop file exists, exiting".yellow());
            break;
        }
        // waits 1 second, gets the time and writes to the file
        sleep(Duration::new(1, 0));
        write_content(Paths::Stopwatch.to_string(), &get_time(now).to_string())
    }
}

fn write_content(path: std::string::String, content: &str) {
    let file = fs::write(path, content);
    match file {
        Ok(msg) => msg,
        Err(_e) => {
            notify("problem with writing to file");
            std::process::exit(3);
        }
    }
}

fn stopwatch_status() {
    let current_time: u64 = read_time(Paths::Stopwatch.to_string());
    if std::path::Path::new(&Paths::StopwatchStop.to_string()).exists() {
        notify(&format!("ended at {}", time_formatted(current_time)));
        std::process::exit(2);
    }
    notify(&format!("ongoing {}", &time_formatted(current_time)));
    std::process::exit(0);
}

fn read_time(path: std::string::String) -> u64 {
    fs::read_to_string(path)
        .unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                notify(&format!("file containing file not found: {}", error));
                std::process::exit(3)
            } else {
                notify(&format!("problem reading file {}", error));
                std::process::exit(3)
            }
        })
        .parse::<u64>()
        .unwrap()
}

fn get_time(now: std::time::SystemTime) -> u64 {
    match now.elapsed() {
        Ok(elapsed) => {
            print(time_formatted(elapsed.as_secs()).italic().cyan());
            elapsed.as_secs()
        }
        Err(error) => {
            notify(&format!("problem getting the time: {}", error));
            panic!("problem getting time");
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

fn notify_action(body: &str) {
    Notification::new()
        .summary("Wooclock")
        .appname("Wooclock")
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

fn notify(body: &str) {
    Notification::new()
        .summary("Wooclock")
        .appname("Wooclock")
        .body(body)
        .icon("org.gnome.clocks")
        .show()
        .unwrap();
}

// FIX: able to use for timer
fn rofi_options(now: std::time::SystemTime) {
    let entries: Vec<&str> = vec!["new", "show", "stop"];
    match Rofi::new(&entries).prompt("Wooclock").run() {
        Ok(choice) => {
            if choice == "new" {
                new_stopwatch(now)
            } else if choice == "show" {
                stopwatch_status()
            } else if choice == "stop" {
                stop_stopwatch(Paths::StopwatchStop.to_string(), Paths::Stopwatch.to_string());
            } else {
                std::process::exit(69);
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(69)
        }
    };
}

// timer
fn open_image() {
    let wallpapers_path = "/home/rdkang/Pictures/Wallpapers/samDoesArt";
    let mut rng = rand::thread_rng();
    let files = fs::read_dir(wallpapers_path).unwrap();
    let file = files.choose(&mut rng).unwrap().unwrap();
    print(format!("picture: {}", file.path().display()).cyan());
    open(file.path()).unwrap();
}

fn rofi_get_length() -> i32 {
    let entries: Vec<String> = vec!["enter timer length".to_string()];
    let user_choice = match Rofi::new(&entries).prompt("Wooclock Timer").run() {
        Ok(choice) => choice,
        Err(rofi::Error::Blank) => std::process::exit(1),
        Err(rofi::Error::Interrupted) => std::process::exit(1),
        Err(error) => {
            let message = format!("rofi had a problem getting your length: {}",error);
            notify(&message);
            print(message.red());
            std::process::exit(30)

        },
    };
    let timer_length = user_choice.parse::<i32>().unwrap();
    timer_length * 60
}

fn new_timer() {
    remove_stop_file(Paths::TimerStop.to_string());

    // interactively asks user for timer length using rofi
    let mut timer_length: i32 = rofi_get_length();

    write_content(Paths::TimerLength.to_string(), &timer_length.to_string());

    while timer_length != 0 {
        if std::path::Path::new(&Paths::TimerStop.to_string()).exists() {
            print("Stop file present, exiting loop".yellow());
            break;
        };

        print(timer_length.to_string().italic().cyan());
        write_content(Paths::Timer.to_string(), &timer_length.to_string());
        sleep(Duration::new(1, 0));
        timer_length -= 1;
    }

    notify(&format!(
        "{} timer finished",
        time_formatted(read_time(Paths::TimerLength.to_string()))
    ));
    open_image();
}

fn timer_status() {
    let timer_length: u64 = read_time(Paths::Timer.to_string());
    print(time_formatted(timer_length).green());
    if std::path::Path::new(&Paths::TimerStop.to_string()).exists() {
        notify(&format!(
            "ended at {}",
            time_formatted(read_time(Paths::TimerLength.to_string()))
        ));
    } else {
        notify(&format!("ongoing {} left", &time_formatted(timer_length)));
    }
}

/// tests
#[test]
fn test_from_sec() {
    assert_eq!(time_formatted(90), "1:30");
}
