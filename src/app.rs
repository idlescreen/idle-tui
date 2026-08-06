// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 IdleScreen

use idle_dbus::{TranceClient, daemon_available};
use std::process::Command;
use std::time::{Duration, Instant};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ActivePane {
    Dashboard,
    Settings,
    Screensavers,
}

pub struct App {
    pub client: Option<TranceClient>,
    pub daemon_running: bool,
    pub idle_enabled: bool,
    pub idle_timeout_mins: u32,
    pub render_scale: f32,
    pub show_fps_overlay: bool,
    pub active_saver: String,
    pub on_battery: bool,
    pub screensavers: Vec<String>,
    pub selected_saver_idx: usize,
    pub active_pane: ActivePane,
    pub selected_setting_idx: usize,
    pub cpu_usage_pct: f32,
    pub mem_used_pct: f32,
    pub mem_used_mb: u64,
    pub mem_total_mb: u64,
    pub inhibitors: Vec<(u32, String, String)>,
    pub tick_count: u64,
    pub cosmic_de_detected: bool,
    pub cosmic_applet_installed: bool,
    pub status_message: Option<String>,
    pub last_action: Option<Instant>,
}

impl App {
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

    pub fn new() -> Self {
        let mut app = Self {
            client: None,
            daemon_running: false,
            idle_enabled: true,
            idle_timeout_mins: 5,
            render_scale: 1.0,
            show_fps_overlay: false,
            active_saver: "Random".to_string(),
            on_battery: false,
            screensavers: Vec::new(),
            selected_saver_idx: 0,
            active_pane: ActivePane::Dashboard,
            selected_setting_idx: 0,
            cpu_usage_pct: 0.0,
            mem_used_pct: 0.0,
            mem_used_mb: 0,
            mem_total_mb: 0,
            inhibitors: Vec::new(),
            tick_count: 0,
            cosmic_de_detected: Self::is_cosmic_de(),
            cosmic_applet_installed: Self::is_cosmic_applet_installed(),
            status_message: None,
            last_action: None,
        };
        app.refresh_state();
        app
    }

    pub fn install_cosmic_applet(&mut self) {
        self.status_message = Some("Installing idle-cosmic package...".to_string());
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
            self.status_message = Some("Error: No supported package manager (dnf/apt)".to_string());
            return;
        };

        match status {
            Ok(s) if s.success() => {
                self.cosmic_applet_installed = Self::is_cosmic_applet_installed();
                self.status_message = Some("idle-cosmic installed successfully!".to_string());
            }
            Ok(s) => {
                self.status_message = Some(format!("Installation exited with status: {s}"));
            }
            Err(e) => {
                self.status_message = Some(format!("Failed to run installer: {e}"));
            }
        }
    }

    pub fn refresh_state(&mut self) {
        if let Some(action_time) = self.last_action {
            if action_time.elapsed() < Duration::from_millis(600) {
                let sys = idle_runner::toolkit::sys_info::get_system_info();
                self.cpu_usage_pct = sys.cpu_usage_pct;
                self.mem_used_pct = sys.mem_used_pct;
                self.mem_used_mb = sys.mem_used_mb;
                self.mem_total_mb = sys.mem_total_mb;
                if self.selected_saver_idx > self.screensavers.len() {
                    self.selected_saver_idx = self.screensavers.len();
                }
                return;
            } else {
                self.last_action = None;
            }
        }

        self.daemon_running = daemon_available();
        if self.daemon_running {
            if let Ok(client) = TranceClient::connect() {
                if let Ok(status) = client.get_status() {
                    self.idle_enabled = status.idle_enabled;
                    self.idle_timeout_mins = status.idle_timeout_mins;
                    self.active_saver = if status.active_saver.is_empty() {
                        "Random".to_string()
                    } else {
                        status.active_saver
                    };
                    self.show_fps_overlay = status.show_fps_overlay;
                    self.render_scale = status.render_scale.parse::<f32>().unwrap_or(1.0);
                    self.on_battery = status.inhibited;
                }
                if let Ok(savers) = client.list_savers() {
                    self.screensavers = savers;
                }
                if let Ok(inhibs) = client.list_inhibitors() {
                    self.inhibitors = inhibs;
                } else {
                    self.inhibitors = Vec::new();
                }
                self.client = Some(client);
            }
        } else {
            self.client = None;
            self.screensavers = idle_runner::discovery::detect_screensavers();
        }

        let sys = idle_runner::toolkit::sys_info::get_system_info();
        self.on_battery = sys.power_status.contains("Battery");
        self.cpu_usage_pct = sys.cpu_usage_pct;
        self.mem_used_pct = sys.mem_used_pct;
        self.mem_used_mb = sys.mem_used_mb;
        self.mem_total_mb = sys.mem_total_mb;

        if self.selected_saver_idx > self.screensavers.len() {
            self.selected_saver_idx = self.screensavers.len();
        }
    }

    pub fn toggle_daemon(&mut self) {
        if self.daemon_running {
            let _ = Command::new("systemctl")
                .args(["--user", "stop", "idle-daemon.service"])
                .status();
        } else {
            let sys_status = Command::new("systemctl")
                .args(["--user", "enable", "--now", "idle-daemon.service"])
                .status();
            let success = sys_status.map(|s| s.success()).unwrap_or(false);
            if !success {
                let _ = Command::new("idle-daemon").arg("daemon").spawn();
            }
        }
        std::thread::sleep(Duration::from_millis(350));
        self.refresh_state();
    }

    pub fn toggle_idle(&mut self) {
        if let Some(ref client) = self.client {
            if self.idle_enabled {
                let _ = client.disable();
            } else {
                let _ = client.enable();
            }
        }
        self.idle_enabled = !self.idle_enabled;
        self.last_action = Some(Instant::now());
    }

    pub fn adjust_timeout(&mut self, delta: i32) {
        let mut val = self.idle_timeout_mins as i32 + delta;
        val = val.clamp(1, 240);
        self.idle_timeout_mins = val as u32;
        if let Some(ref client) = self.client {
            let _ = client.set_timeout(self.idle_timeout_mins);
        }
        self.last_action = Some(Instant::now());
    }

    pub fn adjust_scale(&mut self, delta: f32) {
        let mut val = self.render_scale + delta;
        val = val.clamp(0.25, 1.0);
        self.render_scale = val;
        if let Some(ref client) = self.client {
            let _ = client.set_render_scale(self.render_scale);
        }
        self.last_action = Some(Instant::now());
    }

    pub fn toggle_fps(&mut self) {
        if let Some(ref client) = self.client {
            let _ = client.set_show_fps_overlay(!self.show_fps_overlay);
        }
        self.show_fps_overlay = !self.show_fps_overlay;
        self.last_action = Some(Instant::now());
    }

    pub fn select_saver(&mut self) {
        if let Some(ref client) = self.client {
            let name = if self.selected_saver_idx == 0 {
                ""
            } else {
                &self.screensavers[self.selected_saver_idx - 1]
            };
            let _ = client.set_saver(name);
            self.active_saver = if name.is_empty() { "Random".to_string() } else { name.to_string() };
        }
        self.last_action = Some(Instant::now());
    }

    pub fn preview_saver(&mut self) {
        let saver = if self.selected_saver_idx == 0 {
            if self.screensavers.is_empty() {
                "beams".to_string()
            } else {
                self.screensavers[0].clone()
            }
        } else {
            self.screensavers[self.selected_saver_idx - 1].clone()
        };

        if !self.daemon_running {
            self.toggle_daemon();
        }

        let mut started_via_dbus = false;
        if self.daemon_running {
            if self.client.is_none() {
                self.refresh_state();
            }
            if let Some(ref client) = self.client
                && client.preview(&saver).is_ok()
            {
                started_via_dbus = true;
            }
        }
        if !started_via_dbus {
            let _ = Command::new("idle-daemon")
                .args(["run-plugin", &saver])
                .status();
        }
    }
}

#[cfg(test)]
#[path = "app_tests.rs"]
mod tests;
