use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TerminalConfig {
    #[serde(rename = "$schema")]
    schema: Value,
    pub default_profile: String,
    pub profiles: Profiles,
    pub schemes: Vec<Scheme>,

    #[serde(skip_serializing_if = "Option::is_none")]
    always_on_top: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    always_show_tabs: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    center_on_launch: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_service_warning: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    copy_on_select: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    focus_follow_mouse: Option<Value>,
    #[serde(rename = "compatability.isolatedMode", skip_serializing_if = "Option::is_none")]
    compatibility_isolated_mode: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    copy_formatting: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trim_block_selection: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trim_paste: Option<Value>,
    #[serde(rename = "experimental.detectURLs", skip_serializing_if = "Option::is_none")]
    experimental_detect_urls: Option<Value>,
    #[serde(rename = "experimental.enableColorSelection", skip_serializing_if = "Option::is_none")]
    experimental_enable_color_selection: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_animations: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    large_paste_warning: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    multi_line_paste_warning: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    startup_actions: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled_profile_sources: Option<Value>,
    #[serde(rename = "experimental.rendering.forceFullRepaint", skip_serializing_if = "Option::is_none")]
    experimental_rendering_force_full_repaint: Option<Value>,
    #[serde(rename = "experimental.rendering.software", skip_serializing_if = "Option::is_none")]
    experimental_rendering_software: Option<Value>,
    #[serde(rename = "experimental.input.forceVT", skip_serializing_if = "Option::is_none")]
    experimental_input_force_vt: Option<Value>,
    #[serde(rename = "experimental.useBackgroundImageForWindow", skip_serializing_if = "Option::is_none")]
    experimental_use_background_image_for_window: Option<Value>,
    #[serde(rename = "compatibility.reloadEnvironmentVariables", skip_serializing_if = "Option::is_none")]
    compatibility_reload_environment_variables: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    initial_cols: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    initial_position: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    initial_rows: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_on_user_login: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    first_window_preference: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launch_mode: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rows_to_scroll: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minimize_to_notification_area: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    always_show_notification_icon: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    show_admin_shield: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_acrylic_in_tab_row: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    actions: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    keybindings: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    new_tab_menu: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    theme: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    themes: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    show_tabs_in_titlebar: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    show_terminal_title_in_titlebar: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    snap_to_grid_on_resize: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tab_width_mode: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    word_delimiters: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    confirm_close_all_tabs: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_tab_switcher: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tab_switcher_mode: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    windowing_behavior: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    new_tab_position: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_hide_window: Option<Value>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Profiles {
    pub defaults: Map<String, Value>,
    pub list: Vec<Map<String, Value>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Scheme {
    pub background: String,
    pub black: String,
    pub blue: String,
    pub bright_black: String,
    pub bright_blue: String,
    pub bright_cyan: String,
    pub bright_green: String,
    pub bright_purple: String,
    pub bright_red: String,
    pub bright_white: String,
    pub bright_yellow: String,
    pub cursor_color: String,
    pub cyan: String,
    pub foreground: String,
    pub green: String,
    pub name: String,
    pub purple: String,
    pub red: String,
    pub selection_background: String,
    pub white: String,
    pub yellow: String,
}
