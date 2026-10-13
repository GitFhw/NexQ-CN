use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    AppHandle,
};

/// Build the idle-state tray menu.
pub fn build_idle_menu(app: &AppHandle, locale: &str) -> Result<Menu<tauri::Wry>, tauri::Error> {
    let zh = locale == "zh-CN";
    let start = MenuItem::with_id(app, "start_meeting", if zh { "开始会议" } else { "Start Meeting" }, true, None::<&str>)?;
    let sep1 = PredefinedMenuItem::separator(app)?;
    let copy_ai = MenuItem::with_id(app, "copy_ai_answer", if zh { "复制上次 AI 回答" } else { "Copy Last AI Answer" }, true, None::<&str>)?;
    let copy_actions = MenuItem::with_id(app, "copy_action_items", if zh { "复制行动项" } else { "Copy Action Items" }, true, None::<&str>)?;
    let copy_summary = MenuItem::with_id(app, "copy_summary", if zh { "复制摘要" } else { "Copy Summary" }, true, None::<&str>)?;
    let copy_transcript = MenuItem::with_id(app, "copy_transcript", if zh { "复制转写记录" } else { "Copy Transcript" }, true, None::<&str>)?;
    let sep2 = PredefinedMenuItem::separator(app)?;
    let settings = MenuItem::with_id(app, "settings", if zh { "设置" } else { "Settings" }, true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", if zh { "退出 NexQ" } else { "Quit NexQ" }, true, None::<&str>)?;

    Menu::with_items(app, &[
        &start, &sep1,
        &copy_ai, &copy_actions, &copy_summary, &copy_transcript, &sep2,
        &settings, &quit,
    ])
}

/// Build the meeting-active tray menu.
pub fn build_meeting_menu(app: &AppHandle, locale: &str) -> Result<Menu<tauri::Wry>, tauri::Error> {
    let zh = locale == "zh-CN";
    let stop = MenuItem::with_id(app, "stop_meeting", if zh { "结束会议" } else { "Stop Meeting" }, true, None::<&str>)?;
    let sep1 = PredefinedMenuItem::separator(app)?;
    let mute_mic = MenuItem::with_id(app, "toggle_mic", if zh { "静音麦克风" } else { "Mute Microphone" }, true, None::<&str>)?;
    let mute_sys = MenuItem::with_id(app, "toggle_system", if zh { "静音系统音频" } else { "Mute System Audio" }, true, None::<&str>)?;
    let sep2 = PredefinedMenuItem::separator(app)?;
    let stealth = MenuItem::with_id(app, "toggle_stealth", if zh { "隐身模式" } else { "Stealth Mode" }, true, None::<&str>)?;
    let show_overlay = MenuItem::with_id(app, "show_overlay", if zh { "显示悬浮窗" } else { "Show Overlay" }, true, None::<&str>)?;
    let sep3 = PredefinedMenuItem::separator(app)?;
    let copy_ai = MenuItem::with_id(app, "copy_ai_answer", if zh { "复制上次 AI 回答" } else { "Copy Last AI Answer" }, true, None::<&str>)?;
    let copy_actions = MenuItem::with_id(app, "copy_action_items", if zh { "复制行动项" } else { "Copy Action Items" }, true, None::<&str>)?;
    let copy_summary = MenuItem::with_id(app, "copy_summary", if zh { "复制摘要" } else { "Copy Summary" }, true, None::<&str>)?;
    let copy_transcript = MenuItem::with_id(app, "copy_transcript", if zh { "复制转写记录" } else { "Copy Transcript" }, true, None::<&str>)?;
    let sep4 = PredefinedMenuItem::separator(app)?;
    let settings = MenuItem::with_id(app, "settings", if zh { "设置" } else { "Settings" }, true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", if zh { "退出 NexQ" } else { "Quit NexQ" }, true, None::<&str>)?;

    Menu::with_items(app, &[
        &stop, &sep1,
        &mute_mic, &mute_sys, &sep2,
        &stealth, &show_overlay, &sep3,
        &copy_ai, &copy_actions, &copy_summary, &copy_transcript, &sep4,
        &settings, &quit,
    ])
}
