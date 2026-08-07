// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 IdleScreen

use std::time::Instant;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ActivePane {
    Dashboard,
    Settings,
    Screensavers,
}

pub struct App {
    pub client: Option<idle_dbus::TranceClient>,
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
            cosmic_de_detected: crate::cosmic::is_cosmic_de(),
            cosmic_applet_installed: crate::cosmic::is_cosmic_applet_installed(),
            status_message: None,
            last_action: None,
        };
        app.refresh_state();
        app
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
        // Ensure we have a live D-Bus client (start daemon if needed).
        if self.client.is_none() {
            if !self.daemon_running {
                self.toggle_daemon();
            } else {
                self.refresh_state();
            }
        }
        let name = if self.selected_saver_idx == 0 {
            // Empty string is the daemon wire value for random (see SetSaver).
            String::new()
        } else if self.selected_saver_idx - 1 < self.screensavers.len() {
            self.screensavers[self.selected_saver_idx - 1].clone()
        } else {
            return;
        };

        if let Some(ref client) = self.client {
            match client.set_saver(&name) {
                Ok(()) => {
                    self.active_saver = if name.is_empty() {
                        "Random".to_string()
                    } else {
                        name
                    };
                    self.status_message = Some(if self.active_saver == "Random" {
                        "Saver: Random (pick at idle)".into()
                    } else {
                        format!("Saver: {}", self.active_saver)
                    });
                }
                Err(e) => {
                    self.status_message = Some(format!("Failed to set saver: {e}"));
                }
            }
        } else {
            self.status_message =
                Some("Daemon not running — start it (Settings → Daemon) first".into());
        }
        self.last_action = Some(Instant::now());
    }
}

#[cfg(test)]
#[path = "app_tests.rs"]
mod tests;