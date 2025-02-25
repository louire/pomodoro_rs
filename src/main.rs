use clap::{Parser, Subcommand};
use std::io::{self, Write, BufReader};
use std::thread;
use std::time::Duration;
use std::process::Command;
use std::fs::File;
use chrono::Local;
use colored::*;
use rand::seq::SliceRandom;
use rand::prelude::*;
use dialoguer::{Confirm, theme::ColorfulTheme};
use indicatif::{ProgressBar, ProgressStyle};
use std::path::{Path, PathBuf};
use dirs::home_dir;
use rodio::{Decoder, OutputStream, Sink};

/// Available emojis for different timer states
#[derive(Clone)]
struct Emojis {
    work: Vec<&'static str>,
    break_short: Vec<&'static str>,
    break_long: Vec<&'static str>,
    success: Vec<&'static str>,
    rust: Vec<&'static str>,
}

/// Collection of motivational messages
struct Motivations {
    start_work: Vec<&'static str>,
    during_work: Vec<&'static str>,
    end_work: Vec<&'static str>,
    start_break: Vec<&'static str>,
    end_break: Vec<&'static str>,
}

/// CLI application for a friendly Pomodoro timer
#[derive(Parser)]
#[command(
    author, 
    version, 
    about = "🍅 A friendly Pomodoro timer with Rust 🦀", 
    long_about = None
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

/// Available commands for the Pomodoro timer
#[derive(Subcommand)]
enum Commands {
    /// Start a Pomodoro work interval (25 minutes by default)
    Start {
        /// Custom duration in minutes
        #[arg(short, long, default_value_t = 25)]
        duration: u64,
        
        /// Task description
        #[arg(short, long)]
        task: Option<String>,
    },
    
    /// Start a break (5 minutes by default)
    Break {
        /// Break duration in minutes
        #[arg(short, long, default_value_t = 5)]
        duration: u64,
        
        /// Whether this is a long break
        #[arg(short, long)]
        long: bool,
    },
    
    /// Schedule a sequence of pomodoros
    Schedule {
        /// Number of pomodoro sessions
        #[arg(short, long, default_value_t = 4)]
        sessions: u32,
        
        /// Work duration in minutes
        #[arg(short, long, default_value_t = 25)]
        work: u64,
        
        /// Short break duration in minutes
        #[arg(short = 'b', long, default_value_t = 5)]
        short_break: u64,
        
        /// Long break duration in minutes
        #[arg(short, long, default_value_t = 15)]
        long_break: u64,
        
        /// Task description
        #[arg(short, long)]
        task: Option<String>,
    },
    
    /// Install the binary to your PATH
    Install,
    
    /// Get a random productivity tip
    Tip,
}

/// Initialize emoji collections
fn init_emojis() -> Emojis {
    Emojis {
        work: vec!["🍅", "💻", "📝", "🔨", "⚙️", "🧠", "🦀", "🚀", "⏳", "🔍"],
        break_short: vec!["☕", "🍵", "🧘", "🌱", "🌞", "💆", "🦀", "🎵", "🍃", "🌈"],
        break_long: vec!["🌴", "🏖️", "🎮", "📚", "🍦", "🦀", "🎨", "🌿", "🧁", "🎬"],
        success: vec!["✅", "🎉", "🏆", "💯", "🌟", "🙌", "🦀", "🥳", "💪", "🌺"],
        rust: vec!["🦀"],
    }
}

/// Initialize motivational messages
fn init_motivations() -> Motivations {
    Motivations {
        start_work: vec![
            "Time to focus! You've got this!",
            "Let's make the most of these minutes!",
            "The Rust crab believes in you!",
            "Deep work mode: engaged!",
            "Your future self will thank you for focusing now.",
        ],
        during_work: vec![
            "Stay focused, you're doing great!",
            "One line of code at a time...",
            "The 🦀 is coding alongside you!",
            "Keep going, you're in the flow!",
            "Small steps lead to big accomplishments.",
        ],
        end_work: vec![
            "Great job! Take a well-deserved break.",
            "Pomodoro complete! The 🦀 is proud of you!",
            "You've earned your rest!",
            "Excellent focus session!",
            "Progress made! Time to recharge.",
        ],
        start_break: vec![
            "Break time! Rest your mind.",
            "The 🦀 says: time to relax!",
            "Refresh and recharge!",
            "Stretch, hydrate, breathe!",
            "Short breaks make long coding sessions possible.",
        ],
        end_break: vec![
            "Break's over! Ready to dive back in?",
            "Time to get back to it! The 🦀 is ready!",
            "Refreshed and ready to go!",
            "Back to making progress!",
            "Let's continue building amazing things!",
        ],
    }
}

/// Get a random element from a string vector
fn random_from<'a>(vec: &'a [&'static str]) -> &'a str {
    vec.choose(&mut thread_rng()).unwrap_or(&"")
}

fn main() {
    let cli = Cli::parse();
    
    // Initialize emojis and motivational messages
    let emojis = init_emojis();
    let motivations = init_motivations();
    
    // Set up Ctrl+C handler for clean termination
    let success_emojis = emojis.success.clone();
    let rust_emojis = emojis.rust.clone();
    
    ctrlc::set_handler(move || {
        println!("\n\n{} Pomodoro timer stopped. Take care! {}\n", 
                 random_from(&rust_emojis), 
                 random_from(&success_emojis));
        std::process::exit(0);
    }).expect("Error setting Ctrl+C handler");

    // Display welcome message on first run
    print_welcome_message(&emojis);
    
    // If no command is provided, run the default loop
    match &cli.command {
        Some(command) => match command {
            Commands::Start { duration, task } => {
                let task_desc = task.clone().unwrap_or_else(|| "no description".to_string());
                run_work_session(*duration, &task_desc, &emojis, &motivations);
            },
            Commands::Break { duration, long } => {
                run_break(*duration, *long, &emojis, &motivations);
            },
            Commands::Schedule { sessions, work, short_break, long_break, task } => {
                let task_desc = task.clone().unwrap_or_else(|| "no description".to_string());
                run_schedule(*sessions, *work, *short_break, *long_break, &task_desc, &emojis, &motivations);
            },
            Commands::Install => {
                install_to_path();
            },
            Commands::Tip => {
                show_random_tip(&emojis);
            },
        },
        None => {
            // Default loop - repeat 25/5 pattern until user exits
            println!("{} Starting default Pomodoro cycle (25min work, 5min break) {}\n", 
                     random_from(&emojis.work), 
                     random_from(&emojis.rust));
            
            println!("{}", "Press Ctrl+C at any time to exit.".yellow());
                
            loop {
                // Ask for task description
                let task = dialoguer::Input::<String>::new()
                    .with_prompt("What are you working on? (optional)")
                    .allow_empty(true)
                    .interact_text()
                    .unwrap_or_else(|_| "".to_string());
                
                let task_desc = if task.is_empty() { "Focused work".to_string() } else { task };
                
                // Run work session
                run_work_session(25, &task_desc, &emojis, &motivations);
                
                // Run break
                run_break(5, false, &emojis, &motivations);
                
                // Ask if user wants to continue
                if !Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt("Start another Pomodoro cycle?")
                    .default(true)
                    .interact()
                    .unwrap_or(false) {
                    
                    println!("\n{} Thanks for using Pomodoro_rs! Have a productive day! {}\n",
                             random_from(&emojis.rust),
                             random_from(&emojis.success));
                    break;
                }
            }
        }
    }
}

/// Display a welcome message with ASCII art
fn print_welcome_message(_emojis: &Emojis) {
    println!("{}", r#"
    ╔═══════════════════════════════════════════╗
    ║                                           ║
    ║        🍅 Welcome to Pomodoro_rs 🦀       ║
    ║                                           ║
    ║   Your friendly Rust-powered Pomodoro     ║
    ║        timer with cute emojis and         ║
    ║          encouraging messages!            ║
    ║                                           ║
    ║           Made by: Loui Recio 🦀          ║
    ║            github.com/louire              ║
    ║                                           ║
    ╚═══════════════════════════════════════════╝
    "#.bright_red());
}

/// Run a work session with timer and motivational messages
fn run_work_session(minutes: u64, task_desc: &str, emojis: &Emojis, motivations: &Motivations) {
    let work_emoji = random_from(&emojis.work);
    let rust_emoji = random_from(&emojis.rust);
    
    println!("\n{} {} {}", work_emoji, random_from(&motivations.start_work).bright_green(), rust_emoji);
    println!("{} Starting {} minute Pomodoro for: {}\n", 
             work_emoji, 
             minutes.to_string().bright_yellow(), 
             task_desc.bright_cyan());
    
    run_fancy_timer(minutes, "Pomodoro", task_desc, &emojis.work, &motivations.during_work);
    
    println!("\n{} {} {}", 
             random_from(&emojis.success), 
             random_from(&motivations.end_work).bright_green(),
             rust_emoji);
             
    // This will play the alert sound
    notify("Pomodoro completed!", 
           &format!("{} You completed a {} minute pomodoro for: {}", 
                   random_from(&emojis.success), 
                   minutes, 
                   task_desc));
}

/// Run a break session with timer and motivational messages
fn run_break(minutes: u64, is_long: bool, emojis: &Emojis, motivations: &Motivations) {
    let break_type = if is_long { "long" } else { "short" };
    let break_emojis = if is_long { &emojis.break_long } else { &emojis.break_short };
    let break_emoji = random_from(break_emojis);
    let rust_emoji = random_from(&emojis.rust);
    
    println!("\n{} {} {}", break_emoji, random_from(&motivations.start_break).bright_blue(), rust_emoji);
    println!("{} Starting {} minute {} break\n", 
             break_emoji, 
             minutes.to_string().bright_yellow(), 
             break_type.bright_magenta());
    
    run_fancy_timer(minutes, &format!("{} Break", if is_long { "Long" } else { "Short" }), 
                  "Time to relax", break_emojis, &motivations.start_break);
    
    println!("\n{} {} {}", 
             random_from(&emojis.success), 
             random_from(&motivations.end_break).bright_green(),
             rust_emoji);
             
    notify("Break ended!", 
           &format!("{} Your {} minute break has ended", 
                   random_from(&emojis.success), 
                   minutes));
}

/// Run a schedule of pomodoro sessions with breaks
fn run_schedule(sessions: u32, work: u64, short_break: u64, long_break: u64, 
               task_desc: &str, emojis: &Emojis, motivations: &Motivations) {
    let rust_emoji = random_from(&emojis.rust);
    
    println!("{} Scheduling {} work sessions ({} min) with short breaks ({} min) and a long break ({} min) {}",
             random_from(&emojis.work),
             sessions.to_string().bright_yellow(), 
             work.to_string().bright_green(),
             short_break.to_string().bright_blue(),
             long_break.to_string().bright_magenta(),
             rust_emoji);
    
    for i in 1..=sessions {
        println!("\n{} {} === Session {}/{} === {} {}", 
                 random_from(&emojis.work),
                 "🔄".bright_yellow(),
                 i.to_string().bright_yellow(), 
                 sessions.to_string().bright_yellow(),
                 "🔄".bright_yellow(),
                 random_from(&emojis.rust));
        
        // Work period
        run_work_session(work, task_desc, emojis, motivations);
        
        // Determine break type
        if i < sessions {
            run_break(short_break, false, emojis, motivations);
        } else {
            println!("\n{} All sessions completed! Time for a well-deserved long break! {}", 
                     random_from(&emojis.success),
                     rust_emoji);
            run_break(long_break, true, emojis, motivations);
            
            println!("\n{} Great job completing all {} Pomodoros! {}",
                     random_from(&emojis.success),
                     sessions.to_string().bright_yellow(),
                     rust_emoji);
        }
    }
}

/// Run a fancy timer with progress bar and motivational messages
fn run_fancy_timer(minutes: u64, timer_type: &str, description: &str, 
                 emoji_set: &[&'static str], motivation_set: &[&'static str]) {
    let total_seconds = minutes * 60;
    let start_time = Local::now();
    
    // Create a progress bar
    let pb = ProgressBar::new(total_seconds);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
        .expect("Failed to set progress bar template")
        .progress_chars("#>-"));
    
    for remaining in (0..total_seconds).rev() {
        // Update progress bar
        pb.set_position(total_seconds - remaining);
        
        // Calculate remaining minutes and seconds
        let mins = remaining / 60;
        let secs = remaining % 60;
        
        // Every minute (or at specific intervals), show a motivational message
        if remaining % 60 == 0 && remaining > 0 && remaining < total_seconds {
            println!("\n{} {}", 
                     random_from(emoji_set),
                     random_from(motivation_set).bright_green());
        }
        
        // Calculate elapsed time and estimated end time
        let elapsed = Local::now().signed_duration_since(start_time);
        let elapsed_secs = elapsed.num_seconds() as u64;
        let end_time = Local::now() + chrono::Duration::seconds(remaining as i64);
        
        // Print current status
        print!("\r{} {}: {:02}:{:02} remaining | {:02}:{:02} elapsed | End: {} | {}  ",
               random_from(emoji_set),
               timer_type.bright_yellow(), 
               mins, secs, 
               elapsed_secs / 60, elapsed_secs % 60,
               end_time.format("%H:%M:%S").to_string().bright_cyan(),
               description.bright_green());
        io::stdout().flush().unwrap();
        
        // Wait one second
        thread::sleep(Duration::from_secs(1));
    }
    
    pb.finish_with_message(format!("{} completed!", timer_type));
    println!("\n{} {} completed! {} {}", 
             random_from(emoji_set),
             timer_type.bright_yellow(), 
             description.bright_green(),
             random_from(&["Great job!", "Well done!", "Excellent!", "Fantastic!", "Amazing!"]));
}

/// Display a desktop notification and play alert sound
fn notify(title: &str, message: &str) {
    // Show desktop notification
    match notify_rust::Notification::new()
        .summary(title)
        .body(message)
        .show() {
            Ok(_) => (),
            Err(_) => println!("\n{}: {}", title.bright_yellow(), message.bright_green()), // Fallback if notifications fail
        }
    
    // Play alert sound
    play_alert_sound();
}

/// Play the alert sound when a timer completes
fn play_alert_sound() {
    thread::spawn(|| {
        // Try to get the sound file from different possible locations
        let sound_paths = vec![
            // Check in src/assets directory
            Path::new("src/assets/alert.wav").to_path_buf(),
            // Check in current directory assets
            Path::new("assets/alert.wav").to_path_buf(),
            // Check in executable directory
            std::env::current_exe()
                .ok()
                .and_then(|path| path.parent().map(|p| p.join("assets/alert.wav")))
                .unwrap_or_else(|| Path::new("alert.wav").to_path_buf()),
            // Fallback to just the filename
            Path::new("alert.wav").to_path_buf(),
        ];
        
        // Try each path until we find the sound file
        for sound_path in sound_paths {
            if sound_path.exists() {
                match play_sound(&sound_path) {
                    Ok(_) => break,
                    Err(e) => {
                        eprintln!("Could not play sound from {:?}: {}", sound_path, e);
                        continue;
                    }
                }
            }
        }
    });
}

/// Play sound from file path
fn play_sound(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;
    
    // Load the sound file
    let file = BufReader::new(File::open(path)?);
    let source = Decoder::new(file)?;
    
    // Play the sound
    sink.append(source);
    
    // The sound plays in a separate thread. We need to keep the sink alive
    // until the sound finishes playing.
    sink.sleep_until_end();
    
    Ok(())
}

/// Install the binary to user's PATH
fn install_to_path() {
    println!("🦀 Let's install pomodoro_rs to your PATH!");
    
    // First build the release version
    println!("Building release version...");
    let build_result = Command::new("cargo")
        .args(["build", "--release"])
        .status();
    
    if let Err(e) = build_result {
        println!("❌ Failed to build: {}", e);
        return;
    }
    
    // Create assets directory in the target
    println!("Setting up assets directory...");
    let target_assets_dir = PathBuf::from("target/release/assets");
    if !target_assets_dir.exists() {
        if let Err(e) = std::fs::create_dir_all(&target_assets_dir) {
            println!("⚠️ Warning: Failed to create assets directory: {}", e);
        }
    }
    
    // Copy sound file to target assets directory
    let sound_paths = vec![
        Path::new("src/assets/alert.wav").to_path_buf(),
        Path::new("assets/alert.wav").to_path_buf(),
    ];
    
    for sound_path in sound_paths {
        if sound_path.exists() {
            println!("Found sound file at: {:?}", sound_path);
            let dest_path = target_assets_dir.join("alert.wav");
            
            match std::fs::copy(&sound_path, &dest_path) {
                Ok(_) => {
                    println!("✅ Successfully copied sound file to: {:?}", dest_path);
                    break;
                },
                Err(e) => {
                    println!("⚠️ Warning: Failed to copy sound file: {}", e);
                }
            }
        }
    }
    
    // Determine target directory
    let home = match home_dir() {
        Some(path) => path,
        None => {
            println!("❌ Could not determine your home directory");
            return;
        }
    };
    
    let target_dir = PathBuf::from(&home).join(".local").join("bin");
    
    // Create target directory if it doesn't exist
    if !target_dir.exists() {
        println!("Creating directory: {:?}", target_dir);
        if let Err(e) = std::fs::create_dir_all(&target_dir) {
            println!("❌ Failed to create directory: {}", e);
            return;
        }
    }
    
    // Copy the binary
    let binary_path = std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join("target")
        .join("release")
        .join("pomodoro_rs");
    
    let dest_path = target_dir.join("pomodoro_rs");
    
    println!("Copying from {:?} to {:?}", binary_path, dest_path);
    
    if let Err(e) = std::fs::copy(&binary_path, &dest_path) {
        println!("❌ Failed to copy binary: {}", e);
        return;
    }
    
    // Create assets directory in the destination
    let dest_assets_dir = target_dir.join("assets");
    if !dest_assets_dir.exists() {
        println!("Creating assets directory at: {:?}", dest_assets_dir);
        if let Err(e) = std::fs::create_dir_all(&dest_assets_dir) {
            println!("⚠️ Warning: Failed to create assets directory: {}", e);
        }
    }
    
    // Copy sound file to destination assets directory
    let source_sound = std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join("target")
        .join("release")
        .join("assets")
        .join("alert.wav");
        
    if source_sound.exists() {
        let dest_sound = dest_assets_dir.join("alert.wav");
        println!("Copying sound file to: {:?}", dest_sound);
        
        if let Err(e) = std::fs::copy(&source_sound, &dest_sound) {
            println!("⚠️ Warning: Failed to copy sound file: {}", e);
        } else {
            println!("✅ Sound file copied successfully");
        }
    } else {
        println!("⚠️ Warning: Sound file not found at {:?}", source_sound);
    }
    
    // Make it executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let metadata = std::fs::metadata(&dest_path).unwrap();
        let mut perms = metadata.permissions();
        perms.set_mode(0o755);
        if let Err(e) = std::fs::set_permissions(&dest_path, perms) {
            println!("❌ Failed to set permissions: {}", e);
            return;
        }
    }
    
    println!("\n✅ Installation successful! 🦀");
    println!("Binary installed to: {:?}", dest_path);
    
    // Check if the installation directory is already in PATH
    let path_env = match std::env::var("PATH") {
        Ok(val) => val,
        Err(_) => {
            println!("\nMake sure {:?} is in your PATH.", target_dir);
            println!("You might need to add this to your shell profile:");
            println!("  export PATH=\"$HOME/.local/bin:$PATH\"");
            println!("\nNow you can run the command 'pomodoro_rs' from anywhere!");
            return;
        }
    };
    
    let path_entries: Vec<&str> = path_env.split(':').collect();
    let target_dir_str = target_dir.to_string_lossy();
    
    if path_entries.contains(&target_dir_str.as_ref()) {
        println!("\nGood news! {:?} is already in your PATH.", target_dir);
        println!("You can run the command 'pomodoro_rs' from anywhere!");
        return;
    }
    
    // Ask if the user wants to add it to their PATH
    if !Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Would you like to add the installation directory to your PATH?")
        .default(true)
        .interact()
        .unwrap_or(false) {
        
        println!("\nYou'll need to manually add {:?} to your PATH.", target_dir);
        println!("Add this line to your shell profile:");
        println!("  export PATH=\"$HOME/.local/bin:$PATH\"");
        return;
    }
    
    // Try to detect which shell is being used
    let shell = std::env::var("SHELL").unwrap_or_else(|_| String::from("unknown"));
    let shell_basename = Path::new(&shell).file_name()
        .map(|name| name.to_string_lossy().to_string())
        .unwrap_or_else(|| String::from("unknown"));
    
    // Determine shell profile file
    let profile_file = match shell_basename.as_str() {
        "bash" => home.join(".bashrc"),
        "zsh" => home.join(".zshrc"),
        "fish" => home.join(".config").join("fish").join("config.fish"),
        _ => {
            println!("\nCould not detect your shell profile file.");
            println!("Please manually add {:?} to your PATH.", target_dir);
            println!("Add this line to your shell profile:");
            println!("  export PATH=\"$HOME/.local/bin:$PATH\"");
            return;
        }
    };
    
    // Ask for confirmation since we're modifying a config file
    println!("\nDetected shell: {}", shell_basename);
    println!("Will add PATH entry to: {:?}", profile_file);
    
    if !Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(&format!("Proceed to modify {:?}?", profile_file))
        .default(true)
        .interact()
        .unwrap_or(false) {
        
        println!("\nNo changes made to your shell profile.");
        println!("You'll need to manually add {:?} to your PATH.", target_dir);
        return;
    }
    
    // Add the directory to PATH in the appropriate file
    let path_line = if shell_basename == "fish" {
        format!("set -x PATH $HOME/.local/bin $PATH\n")
    } else {
        format!("export PATH=\"$HOME/.local/bin:$PATH\"\n")
    };
    
    let result = if profile_file.exists() {
        // Append to existing file
        std::fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(&profile_file)
            .and_then(|mut file| {
                file.write_all(b"\n# Added by pomodoro_rs installer\n")?;
                file.write_all(path_line.as_bytes())
            })
    } else {
        // Create new file
        std::fs::write(&profile_file, format!("# Added by pomodoro_rs installer\n{}", path_line))
    };
    
    match result {
        Ok(_) => {
            println!("\n✅ Successfully updated your shell profile!");
            println!("To apply the changes immediately, run:");
            match shell_basename.as_str() {
                "fish" => println!("  source {:?}", profile_file),
                _ => println!("  source {:?}", profile_file),
            }
            println!("\nOr simply restart your terminal.");
            println!("After that, you can run the command 'pomodoro_rs' from anywhere!");
        },
        Err(e) => {
            println!("\n❌ Failed to update your shell profile: {}", e);
            println!("Please manually add the following line to {:?}:", profile_file);
            println!("  {}", path_line.trim());
        }
    }
}

/// Show a random productivity tip
fn show_random_tip(emojis: &Emojis) {
    let tips = vec![
        "The Pomodoro Technique works best when you fully commit to the task during work periods.",
        "Keep a list of small tasks to tackle during short breaks to maintain productivity momentum.",
        "Physical activity during breaks (like stretching) can boost your energy for the next Pomodoro.",
        "Try different Pomodoro lengths to find what works best for you - not everyone is optimal at 25 minutes.",
        "Use Pomodoros to estimate task completion times by tracking how many you need for similar tasks.",
        "The 'rule of three' suggests focusing on completing just three main tasks per day.",
        "Consider using noise-cancelling headphones or white noise during Pomodoros to improve focus.",
        "Hydration improves cognitive function - keep water nearby during your work sessions.",
        "For creative tasks, sometimes a longer Pomodoro (40-60 minutes) works better than the standard 25.",
        "Track your completed Pomodoros to visualize your productivity trends over time.",
        "The Rust crab says: sometimes your most productive Pomodoro isn't the one where you write the most code!",
    ];
    
    println!("\n{} {} {}", 
             random_from(&emojis.work),
             "Productivity Tip:".bright_yellow(),
             random_from(&emojis.rust));
    
    println!("{} {}\n", 
             "💡",
             random_from(&tips).bright_green());
}