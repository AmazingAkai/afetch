
use sysinfo::{System, SystemExt};
use std::env;

fn main() {
    // Penguin Colours
    const BLACK: &str = "\x1b[30m";
    const WHITE: &str = "\x1b[37m";
    const YELLOW: &str = "\x1b[33m";

    // Text Colours
    const CYAN: &str = "\x1b[0;96m";
    const PINK: &str = "\x1b[0;95m";
    const GREEN: &str = "\x1b[0;92m";
    const RED: &str = "\x1b[0;91m";
    const LAVENDER: &str = "\x1b[0;94m";
    const PEACH: &str = "\x1b[0;93m";

    // Misc
    const DOT: &str = "";
    const NC: &str = "\x1b[0m";

    println!("");
    println!("{}   _~_   {}  {}os:{} {}", BLACK, WHITE, CYAN, NC, get_os_name());
    println!("{}  (o o)  {}  {}shell:{} {}", BLACK, WHITE, PINK, NC, get_shell_name());
    println!("{} /  V  \\ {}  {}wm:{} {}", YELLOW, WHITE, GREEN, NC, get_desktop_environment());
    println!(
        "{}/(  _  )\\{}  {}ram:{} {}",
        YELLOW, WHITE, RED, NC, get_ram_usage()
    );
    println!(
        "{}  ^^ ^^  {}   {}{}{} {}{}{} {}{}{} {}{}{} {}{}{} {}{}{}",
        WHITE, WHITE, CYAN, DOT, NC, PINK, DOT, NC, GREEN, DOT, NC, RED, DOT, NC, LAVENDER, DOT, NC, PEACH, DOT, NC
    );
    println!("");
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

    if let Some(name) = system.name() {
        name
    } else {
        "Not Found".to_string()
    }
}
fn get_shell_name() -> String {
    if let Ok(shell) = env::var("SHELL") {
        if let Some(file_name) = std::path::Path::new(&shell).file_name() {
            if let Some(name) = file_name.to_str() {
                return String::from(name);
            }
        }
    }

    String::from("bash")
}



fn get_desktop_environment() -> String {
    if cfg!(target_os = "linux") {
        if let Some(de_name) = get_de_name() {
            return de_name;
        } else if let Some(wm_name) = get_wm_name() {
            return wm_name;
        }
    } else if cfg!(target_os = "windows") {
        return "Desktop Window Manager".to_string();
    } else if cfg!(target_os = "macos") {
        return "Aqua".to_string();
    }

    String::from("None")
}

fn get_de_name() -> Option<String> {
    env::var("XDG_CURRENT_DESKTOP").ok()
}

fn get_wm_name() -> Option<String> {
    env::var("XDG_SESSION_DESKTOP").ok()
}