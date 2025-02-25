# 🍅 Pomodoro_rs 🦀

A friendly Pomodoro timer CLI application built with Rust, featuring cute emojis, encouraging messages, sound alerts, and a powerful yet simple interface.

![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)
![Rust Version](https://img.shields.io/badge/rust-stable-blue.svg)

<p align="center">
  <img src="./assets/pomodoro_rs.png" alt="Pomodoro_rs Screenshot" width="300">
</p>

## ✨ Features

- 🍅 Customizable work intervals and breaks
- 🦀 Cute Rust-themed emojis and encouraging messages
- 📊 Real-time countdown with fancy progress bars
- 💪 Motivational messages to keep you going
- 🔄 Default mode with automatic 25/5 minute cycles
- 🔔 Desktop notifications with sound alerts
- 🔊 Audio alerts when timers complete
- 📝 Task description support
- 🛑 Clean interruption with Ctrl+C
- 🚀 Easy installation to your PATH
- 💡 Random productivity tips

## 🎯 The Pomodoro Technique

The Pomodoro Technique is a time management method developed by Francesco Cirillo in the late 1980s. It uses a timer to break work into intervals, traditionally 25 minutes in length, separated by short breaks. Each interval is known as a "pomodoro", from the Italian word for tomato, after the tomato-shaped kitchen timer that Cirillo used as a university student.

## 📦 Installation

### Prerequisites

- **Rust and Cargo**: Make sure you have Rust and Cargo installed (minimum version 1.85.0). If not, you can install them from [rustup.rs](https://rustup.rs/).
- **Note**: This project is designed for Rust 2024 edition. If you're using an older version of Rust, you may need to adjust the `Cargo.toml` to use the 2021 edition instead.

### Building and Installing

```bash
# Clone the repository
git clone https://github.com/louire/pomodoro_rs.git
cd pomodoro_rs

# Build the project
cargo build --release

# Install to your PATH (interactive)
cargo run -- install
```

The interactive installer will:
1. Build the release version
2. Copy the binary to `~/.local/bin/`
3. Copy the sound file to the appropriate location
4. **Detect if the installation directory is already in your PATH**
5. **Ask if you want to add it to your PATH automatically**
6. **Detect your shell (bash, zsh, fish) and modify the appropriate profile file**
7. Provide instructions on how to apply the changes

This makes it easy to install and start using `pomodoro_rs` immediately without manual configuration.

### Setting Up Sound Alerts

The application looks for a sound file named `alert.wav` to play when timers complete. Place this file in any of these locations:

```
src/assets/alert.wav  (preferred location during development)
assets/alert.wav      (alternative location)
```

When you run `pomodoro_rs install`, the sound file will be automatically copied to the correct location. The installer handles all necessary file copying and PATH configuration.

## 🚀 Usage

### Quick Start (Default Mode)

Simply run the command without arguments to start the default 25/5 minute cycle loop:

```bash
pomodoro_rs
```

This will:
1. Ask what you're working on
2. Run a 25-minute work session with encouraging messages
3. Play a sound alert and show a notification when time is up
4. Give you a 5-minute break
5. Ask if you want to continue the cycle

<p align="center">
  <img src="./assets/pomodoro_rs.gif" alt="Pomodoro_rs Gif" width="600">
</p>

### Basic Commands

The application has these main commands:

#### Start a Pomodoro

```bash
# Start a default 25-minute pomodoro
pomodoro_rs start

# Start a 30-minute pomodoro with a task description
pomodoro_rs start -d 30 -t "Write documentation"
```

#### Take a Break

```bash
# Take a default 5-minute short break
pomodoro_rs break

# Take a 15-minute long break
pomodoro_rs break -d 15 -l
```

#### Schedule a Sequence

```bash
# Schedule 4 pomodoros with default settings
pomodoro_rs schedule

# Custom schedule with 3 pomodoros, 30-minute work intervals,
# 8-minute short breaks, and a 20-minute long break
pomodoro_rs schedule -s 3 -w 30 -b 8 -l 20 -t "Important project"
```

#### Other Commands

```bash
# Install to your PATH
pomodoro_rs install

# Get a random productivity tip
pomodoro_rs tip
```

### Command-Line Options

#### Start Command
- `-d, --duration <MINUTES>`: Set the duration of the pomodoro (default: 25)
- `-t, --task <DESCRIPTION>`: Add a task description

#### Break Command
- `-d, --duration <MINUTES>`: Set the duration of the break (default: 5)
- `-l, --long`: Flag to indicate a long break

#### Schedule Command
- `-s, --sessions <NUMBER>`: Number of pomodoro sessions (default: 4)
- `-w, --work <MINUTES>`: Duration of work intervals (default: 25)
- `-b, --short-break <MINUTES>`: Duration of short breaks (default: 5)
- `-l, --long-break <MINUTES>`: Duration of the final long break (default: 15)
- `-t, --task <DESCRIPTION>`: Add a task description for all pomodoros

## 🎨 Features in Detail

### Friendly Interface

The CLI features a colorful, emoji-filled interface that makes time management fun:

- Work sessions feature tomatoes 🍅, crabs 🦀, and other productivity emojis
- Break times show relaxing emojis like ☕ and 🌱
- Success is celebrated with 🎉 and 🏆
- Progress bars show your advancement through each timer
- Sound alerts play when timers finish (using `alert.wav`)

### Audio Notifications

When a timer completes, the application:
1. Shows a desktop notification
2. Plays the `alert.wav` sound file
3. Displays motivational messages and emojis in the terminal

The sound system:
- Works on all major platforms (Windows, macOS, Linux)
- Automatically finds the sound file in various locations
- Can be easily customized by replacing the alert.wav file

### Motivational Messages

Get encouragement throughout your work sessions:

- "The Ferris believes in you! 🦀"
- "Keep going, you're in the flow!"
- "Small steps lead to big accomplishments."

### Easy Default Mode

With no arguments, pomodoro_rs runs in an interactive loop:
- Asks what you're working on
- Runs 25-minute work sessions
- Takes 5-minute breaks
- Asks if you want to continue after each cycle

## Project Structure

```
pomodoro_rs/
├── src/
│   ├── main.rs          # Main application code
│   └── assets/
│       └── alert.wav    # Sound alert file
├── Cargo.toml           # Project configuration
├── LICENSE              # MIT License
└── README.md            # This file
```

## 🧩 Customization

Feel free to modify the code to add your own emojis and motivational messages! Look for the `init_emojis()` and `init_motivations()` functions in the code.

To use a different sound, simply replace the `alert.wav` file with your preferred sound (must be in WAV format).

### Adjusting for Different Rust Editions

If you're using an older version of Rust and need to adjust the edition:

1. Open `Cargo.toml`
2. Change the edition from `2024` to `2021`:
```toml
[package]
name = "pomodoro_rs"
version = "0.1.0"
edition = "2021"  # Change from 2024 to 2021
```

This will make the project compatible with older Rust versions, though some features might require minor adjustments.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🙏 Acknowledgments

- The Pomodoro Technique® and Pomodoro™ are registered and filed trademarks owned by Francesco Cirillo
- Thanks to the Rust community for the amazing ecosystem and ferris 🦀
- Sound powered by the `rodio` crate
- Inspired by productivity enthusiasts everywhere who love both focus and fun