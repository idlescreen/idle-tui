use std::process::Command;

pub fn is_cosmic_de() -> bool {
    if let Ok(desktop) = std::env::var("XDG_CURRENT_DESKTOP") {
        if desktop.to_lowercase().contains("cosmic") {
            return true;
        }
    }
    if let Ok(session) = std::env::var("DESKTOP_SESSION") {
        if session.to_lowercase().contains("cosmic") {
            return true;
        }
    }
    std::path::Path::new("/usr/bin/cosmic-panel").exists()
}

pub fn is_cosmic_applet_installed() -> bool {
    std::path::Path::new("/usr/bin/idlescreen-applet").exists()
        || std::path::Path::new("/usr/bin/trance-applet").exists()
}

pub fn install_cosmic_applet() -> Result<(), String> {
    let has_dnf = std::path::Path::new("/usr/bin/dnf").exists();
    let has_apt = std::path::Path::new("/usr/bin/apt").exists();
    let status = if has_dnf {
        Command::new("pkexec")
            .args(["dnf", "install", "-y", "idle-cosmic"])
            .status()
    } else if has_apt {
        Command::new("pkexec")
            .args(["apt", "install", "-y", "idle-cosmic"])
            .status()
    } else {
        return Err("Error: No supported package manager (dnf/apt)".to_string());
    };

    match status {
        Ok(s) if s.success() => Ok(()),
        Ok(s) => Err(format!("Installation exited with status: {}", s)),
        Err(e) => Err(format!("Failed to run installer: {}", e)),
    }
}
