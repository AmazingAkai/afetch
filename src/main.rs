use std::process::Command;

fn get_distribution() -> String {
    if std::path::Path::new("/etc/os-release").exists() {
        let output = Command::new("sh")
            .arg("-c")
            .arg("source /etc/os-release && echo $PRETTY_NAME")
            .output()
            .expect("Failed to execute command");

        return String::from_utf8_lossy(&output.stdout).trim().to_string();
    }

    String::from(std::env::consts::OS).trim().to_string()
}

fn get_kernel_version() -> String {
    let output = Command::new("uname")
        .arg("-r")
        .output()
        .expect("Failed to execute command");

    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

fn get_package_count(package_manager: &str) -> String {
    if package_manager == "N/A" {
        return String::from("N/A");
    }

    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("{} -qa | wc -l", package_manager))
        .output()
        .expect("Failed to execute command");

    let count = String::from_utf8_lossy(&output.stdout).trim().to_string();
    format!("{} ({})", count, package_manager)
}

fn get_window_manager() -> String {
    std::env::var("XDG_CURRENT_DESKTOP")
        .or_else(|_| std::env::var("DESKTOP_SESSION"))
        .unwrap_or_else(|_| String::from("N/A"))
}

fn print_system_info() {
    let cyan: &str = "\x1b[0;96m";
    let pink: &str = "\x1b[0;95m";
    let green: &str = "\x1b[0;92m";
    let red: &str = "\x1b[0;91m";
    let lavender: &str = "\x1b[0;94m";
    let peach: &str = "\x1b[0;93m";
    let nc: &str = "\x1b[0m";
    let dot: &str = "󰝥";

    let distribution = get_distribution();
    let kernel = get_kernel_version();

    let package_manager: &str = match distribution.as_str() {
        s if s.contains("Ubuntu") || s.contains("Debian") => "dpkg",
        s if s.contains("Fedora") || s.contains("Red Hat") => "rpm",
        _ => "N/A",
    };


    let symbol: &str = match distribution.as_str() {
        s if s.contains("Debian") => "",
        s if s.contains("Ubuntu")=> "",
        s if s.contains("Fedora")=> "",
        s if s.contains("Red Hat")=> "󱄛",
        s if s.contains("Arch")=> "󰣇",
        _ => "N/A",
    };

    let package_count = get_package_count(package_manager);
    let window_manager = get_window_manager();

    println!(
        r#"
         {cyan}{symbol}{nc} {distribution}
 /\_/\   {pink}{nc} {kernel}
(=^ω^=)  {green}󰏔{nc} {package_count}
(\_/)    {red}󱡶{nc} {window_manager}
         {green}{dot}{nc} {pink}{dot}{nc} {red}{dot}{nc} {cyan}{dot}{nc} {lavender}{dot}{nc} {peach}{dot}{nc}
"#);
}

fn main() {
    print_system_info();
}
