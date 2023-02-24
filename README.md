# Welcome to wooclock 👋
![Version](https://img.shields.io/badge/version-1.0.2-blue.svg?cacheSeconds=2592000)

> A command-line opinionated clock program written for my needs

## Installation

To build wooclock, you'll need to have Rust installed on your system. You can install Rust by following the instructions [here](https://www.rust-lang.org/tools/install).

Once Rust is installed, you can download and compile the wooclock program by cloning this repository and running the following command:

```bash
git clone https://github.com/Rdkang/wooclock
```

```bash
cargo build --release
```
This will compile the wooclock program and create an executable file in the `target/release` directory.

## Usage

To use wooclock, simply run the executable file with no arguments to get the possible options or with `--help`:

```shell
./wooclock
```

### Subcommands

Wooclock includes the following subcommands:
`stopwatch`

The stopwatch subcommand allows you to start and stop a stopwatch.
Flags
```
    new: Starts a new stopwatch
    status: Shows the current status of the stopwatch
    stop: Stops the stopwatch
    rofi: Displays the stopwatch in a rofi dialog
```

Example

To start a new stopwatch, run the following command:

```shell
wooclock stopwatch new
```

`timer`

The timer subcommand allows you to set a timer for a specific amount of time.
Flags

```
    new: will ask for timer length in rofi input and then start a new timer
    status: Shows the current status of the timer
    stop: Stops the timer
    rofi: Displays the timer options in a rofi dialog
```

## Contributing

If you'd like to contribute to wooclock, please fork the repository and submit a pull request. Contributions are always welcome!

## Author

👤 **rdkang**

* Github: [@rdkang](https://github.com/rdkang)

## Roadmap
- [ ] TODO: dry principle for when using intertwine the cli arguments, and should work with timer as well. with the name of ClockType as the prompt
- [ ] TODO: handle sigterm. And create a stop file
- [ ] TODO: make sure only one instance
- [ ] TODO: config file for the wallpapers path in open_image()
- [ ] TODO: shell completion
- [ ] TODO: short flag for the options
- [ ] TODO: split code to separate files for each clock type and general functions
- [ ] TODO: better notification formatting

## Show your support

Give a ⭐️ if this project helped you!


***
_This README was generated with ❤️ by [readme-md-generator](https://github.com/kefranabg/readme-md-generator)_
