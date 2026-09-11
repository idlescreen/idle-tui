use std::process::Command;

pub fn is_cosmic_de() -> bool {
    for var in ["XDG_CURRENT_DESKTOP", "DESKTOP_SESSION"] {
        if std::env::var(var)
            .map(|v| v.to_lowercase().contains("cosmic"))
            .unwrap_or(false)
        {
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
    // Per RULES §1.4 default-deny, a privileged effect must be accompanied
    // by an explicit signature gate. The applet install path was identified
    // in Sprint 08/09 (S08-H2) as bypassing the install.sh signature gate
    // because it does not set `IDLE_REQUIRE_MANIFEST_SIGNATURE=1` before
    // `pkexec`. Setting it here means the `dnf`/`apt` transaction will
    // surface a missing-key error rather than silently installing an
    // unsigned package.
    let status = if has_dnf {
        Command::new("pkexec")
            .args([
                "env",
                "IDLE_REQUIRE_MANIFEST_SIGNATURE=1",
                "dnf",
                "install",
                "-y",
                "idle-cosmic",
            ])
            .status()
    } else if has_apt {
        Command::new("pkexec")
            .args([
                "env",
                "IDLE_REQUIRE_MANIFEST_SIGNATURE=1",
                "apt",
                "install",
                "-y",
                "idle-cosmic",
            ])
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
