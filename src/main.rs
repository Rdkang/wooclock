#![allow(dead_code)]
#![allow(unused_variables)]

use clap::{Parser, Subcommand};
use colored::*;
use notify_rust::Notification;
use std::time::SystemTime;

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

fn main() {
    let args = Args::parse();
    match &args.stopwatch {
        Some(Commands::Status) => {
            println!("'myapp add' was used, name is: ")
        }
        None => {
            println!("Default subcommand");
        }
        _ => {
            print("wow".yellow());
        }
    }

    println!("Hello, world!");
    // notify("wow");
    let path = "/tmp/wooclock.txt";
    let time_now = SystemTime::now();
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
        .unwrap()
        .wait_for_action(|action| match action {
            "default" => println!("you clicked \"default\""),
            "clicked" => println!("that was correct"),
            // here "__closed" is a hard coded keyword
            "__closed" => println!("the notification was closed"),
            _ => (),
        });
}
