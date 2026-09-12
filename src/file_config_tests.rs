// SPDX-License-Identifier: MIT

use super::*;

#[test]
fn merge_field_replaces_only_target_key() {
    let existing = "# note\ntheme: \"synthwave\"\nidle_timeout_mins: 5\nmy_key: keep\n";
    let out = merge_field(existing, "idle_timeout_mins", "15");
    assert!(out.contains("idle_timeout_mins: 15"));
    assert!(!out.contains("idle_timeout_mins: 5\n"));
    assert!(out.contains("theme: \"synthwave\""));
    assert!(out.contains("# note"));
    assert!(out.contains("my_key: keep"));
}

#[test]
fn merge_field_appends_missing_key() {
    let out = merge_field("idle_enabled: true\n", "active_saver", "\"storm\"");
    assert!(out.contains("idle_enabled: true"));
    assert!(out.contains("active_saver: \"storm\""));
}

#[test]
fn merge_field_ignores_key_inside_section() {
    let existing = "[saver]\nidle_timeout_mins: 99\n";
    let out = merge_field(existing, "idle_timeout_mins", "15");
    // section line untouched; new top-level key appended
    assert!(out.contains("[saver]\nidle_timeout_mins: 99\n"));
    assert!(out.trim_end().ends_with("idle_timeout_mins: 15"));
}

#[test]
fn merge_field_empty_file_gets_header() {
    let out = merge_field("", "idle_enabled", "false");
    assert!(out.contains("# IdleScreen themes and settings"));
    assert!(out.contains("idle_enabled: false"));
}

#[test]
fn merge_field_comment_prefixed_key_is_not_a_match() {
    let existing = "# idle_timeout_mins: 99\nidle_enabled: true\n";
    let out = merge_field(existing, "idle_timeout_mins", "15");
    assert!(out.contains("# idle_timeout_mins: 99"));
    assert!(out.contains("idle_timeout_mins: 15"));
}

static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

#[test]
fn load_parses_known_keys_and_ignores_sections() {
    let _g = ENV_LOCK.lock().unwrap();
    let yaml = "idle_timeout_mins: 12\nactive_saver: \"storm\"\nidle_enabled: false\nshow_fps_overlay: true\nrender_scale: 0.5\ntheme: \"synthwave\"\n[saver]\nidle_timeout_mins: 99\n";
    let dir = std::env::temp_dir().join(format!("idle-tui-test-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("config.yaml"), yaml).unwrap();
    unsafe { std::env::set_var("IDLE_TUI_CONFIG_DIR", &dir) };
    let s = load();
    unsafe { std::env::remove_var("IDLE_TUI_CONFIG_DIR") };
    assert_eq!(s.idle_timeout_mins, 12); // section line must not override
    assert_eq!(s.active_saver.as_deref(), Some("storm"));
    assert!(!s.idle_enabled);
    assert!(s.show_fps_overlay);
    assert_eq!(s.render_scale, Some(0.5));
}

#[test]
fn load_null_render_scale_is_none() {
    let _g = ENV_LOCK.lock().unwrap();
    let dir = std::env::temp_dir().join(format!("idle-tui-test2-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("config.yaml"), "render_scale: null\n").unwrap();
    unsafe { std::env::set_var("IDLE_TUI_CONFIG_DIR", &dir) };
    let s = load();
    unsafe { std::env::remove_var("IDLE_TUI_CONFIG_DIR") };
    assert!(s.render_scale.is_none());
}
