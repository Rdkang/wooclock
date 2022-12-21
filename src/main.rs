#![allow(dead_code)]
#![allow(unused_variables)]

use clap::{Parser, Subcommand, ValueEnum};
use colored::*;
use notify_rust::Notification;
use rand::seq::IteratorRandom;
use rofi::Rofi;
use std::io::ErrorKind;
use std::thread::sleep;
use std::time::{Duration, SystemTime};
use std::{fmt, fs};
extern crate alloc;

/*
TODO - handle sigterm. and create a stop file
TODO - implement timer and timerstop
TODO - able to specify on cli if timer or stopwatch subcommands
TODO - make sure only one instance
TODO - config file for the wallpapers path in open_image()
*/

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
    let args = Cli::parse();
    let now = SystemTime::now();

    match args.command {
        ClockType::Stopwatch { option: function } => match function {
            Commands::New => new_stopwatch(now),
            Commands::Stop => stop_process(Paths::StopwatchStop.to_string(), Paths::Stopwatch.to_string()),
            Commands::Status => stopwatch_status(Paths::Stopwatch.to_string()),
            Commands::Rofi => rofi_options(now),
        },
        ClockType::Timer { option: function } => {
            print("test".green());
        }
    }

    /* match &args.stopwatch {
        Some(Commands::Status) => {
            stopwatch_status(Paths::Stopwatch.to_string());
        }
        Some(Commands::New) => new_stopwatch(now),
        Some(Commands::Stop) => {
            stop_process(Paths::StopwatchStop.to_string(), Paths::Stopwatch.to_string());
            std::process::exit(0);
        }
        Some(Commands::Rofi) => rofi_options(now),
        None => {
            println!("no subcommands");
        }
    } */
}

fn stop_process(stop_path: std::string::String, process_path: std::string::String) {
    match std::fs::File::create(stop_path) {
        Ok(_msg) => {
            let current_time = read_time(process_path);
            notify(&format!("stopwatch ran for {}", &current_time));
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
        write_content(Paths::Stopwatch.to_string(), get_time(now).as_str())
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
    fs::read_to_string(path).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            notify(&format!("file contaning file not found: {}", error));
            std::process::exit(3)
        } else {
            notify(&format!("problem reading file {}", error));
            std::process::exit(3)
        }
    })
}

fn get_time(now: std::time::SystemTime) -> String {
    match now.elapsed() {
        Ok(elapsed) => {
            let output = time_formatted(elapsed.as_secs());
            print(output.italic().cyan());
            output
        }
        Err(error) => {
            notify(&format!("problem getting the time: {}", error));
            // std::process::exit(8)
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

fn rofi_options(now: std::time::SystemTime) {
    // let entries: Vec<String> = vec!["new".to_string(), "show".to_string(), "stop".to_string()];
    let entries: Vec<&str> = vec!["new", "show", "stop"];
    match Rofi::new(&entries).prompt("stopwatchrs").run() {
        Ok(choice) => {
            println!("Choice: {}", choice);
            if choice == "new" {
                new_stopwatch(now)
            } else if choice == "show" {
                stopwatch_status(Paths::Stopwatch.to_string())
            } else if choice == "stop" {
                stop_process(Paths::StopwatchStop.to_string(), Paths::Stopwatch.to_string());
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
    print(format!("picture: {}", file.path().display()).cyan())
}

fn rofi_get_length() -> i32 {
    let entries: Vec<String> = vec!["enter timer length".to_string()];
    let user_choice = match Rofi::new(&entries).prompt("Wooclock Timer").run() {
        Ok(choice) => choice,
        // TODO improve error handling
        Err(_error) => 10.to_string(),
    };
    let timer_length = user_choice.parse::<i32>().unwrap();
    timer_length * 60
}

fn new_timer() {
    remove_stop_file(Paths::TimerStop.to_string());
    let mut timer_length: i32 = rofi_get_length();

    while timer_length != 0 {
        if std::path::Path::new(&Paths::TimerStop.to_string()).exists() {
            print("Stop file present, exiting loop".yellow());
        };

        print(timer_length.to_string().italic().cyan());
        write_content(Paths::Timer.to_string(), &timer_length.to_string());
        sleep(Duration::new(1, 0));
        timer_length -= 1;
    }

    notify(&format!("{}m timer finished", timer_length / 60));
    open_image();
}

/// tests
#[test]
fn test_from_sec() {
    assert_eq!(time_formatted(90), "1:30");
}
