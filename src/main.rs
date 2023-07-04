use sysinfo::{System, SystemExt};
use std::env;

// Colors
const BLACK: &str = "\x1b[30m";
const WHITE: &str = "\x1b[37m";
const YELLOW: &str = "\x1b[33m";
const CYAN: &str = "\x1b[0;96m";
const PINK: &str = "\x1b[0;95m";
const GREEN: &str = "\x1b[0;92m";
const RED: &str = "\x1b[0;91m";
const LAVENDER: &str = "\x1b[0;94m";
const PEACH: &str = "\x1b[0;93m";
const DOT: &str = "";
const NC: &str = "\x1b[0m";

fn main() {
    println!();
    println!("{}   _~_   {}  {}os:{} {}", BLACK, WHITE, CYAN, NC, get_os_name());
    println!("{}  (o o)  {}  {}shell:{} {}", BLACK, WHITE, PINK, NC, get_shell_name());
    println!("{} /  V  \\ {}  {}wm:{} {}", YELLOW, WHITE, GREEN, NC, get_desktop_environment());
    println!("{}/(  _  )\\{}  {}ram:{} {}", YELLOW, WHITE, RED, NC, get_ram_usage());
    println!("{}  ^^ ^^  {}   {}{}{} {}{}{} {}{}{} {}{}{} {}{}{} {}{}{}", WHITE, WHITE, CYAN, DOT, NC, PINK, DOT, NC, GREEN, DOT, NC, RED, DOT, NC, LAVENDER, DOT, NC, PEACH, DOT, NC);
    println!();
}

fn get_ram_usage() -> String {
    let mut system = System::new_all();
    system.refresh_memory();

    let total_ram_mb = system.total_memory() / (1024 * 1024);
    let used_ram_mb = system.used_memory() / (1024 * 1024);

    format!("{} MB / {} MB", used_ram_mb, total_ram_mb)
}

fn get_os_name() -> String {
    let mut system = System::new_all();
    system.refresh_system();

    system.name().unwrap_or_else(|| "Not Found".to_string())
}

fn get_shell_name() -> String {
    env::var("SHELL")
        .ok()
        .and_then(|shell| std::path::Path::new(&shell).file_name().map(|file_name| file_name.to_owned()))
        .and_then(|file_name| file_name.to_str().map(|name| name.to_owned()))
        .unwrap_or_else(|| "bash".to_string())
}


fn get_desktop_environment() -> String {
    match std::env::var("XDG_CURRENT_DESKTOP").ok() {
        Some(de_name) => de_name,
        None => std::env::var("XDG_SESSION_DESKTOP").ok().unwrap_or_else(|| {
            if cfg!(target_os = "windows") {
                "Desktop Window Manager".to_string()
            } else if cfg!(target_os = "macos") {
                "Aqua".to_string()
            } else {
                "None".to_string()
            }
        }),
    }
}
