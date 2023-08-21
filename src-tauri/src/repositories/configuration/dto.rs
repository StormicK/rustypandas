use serde::{Deserialize, Serialize};
use serde_json::{Value, Map};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TerminalConfig {
    #[serde(rename = "$schema")]
    schema: Value,
    default_profile: Value,
    pub profiles: Profiles,
    pub schemes: Vec<Scheme>,

    always_on_top: Option<Value>,
    always_show_tabs: Option<Value>,
    center_on_launch: Option<Value>,
    input_service_warning: Option<Value>,
    copy_on_select: Option<Value>,
    focus_follow_mouse: Option<Value>,
    #[serde(rename = "compatability.isolatedMode")]
    compatibility_isolated_mode: Option<Value>,
    copy_formatting: Option<Value>,
    trim_block_selection: Option<Value>,
    trim_paste: Option<Value>,
    #[serde(rename = "experimental.detectURLs")]
    experimental_detect_urls: Option<Value>,
    #[serde(rename = "experimental.enableColorSelection")]
    experimental_enable_color_selection: Option<Value>,
    disable_animations: Option<Value>,
    large_paste_warning: Option<Value>,
    multi_line_paste_warning: Option<Value>,
    startup_actions: Option<Value>,
    disabled_profile_sources: Option<Value>,
    #[serde(rename = "experimental.rendering.forceFullRepaint")]
    experimental_rendering_force_full_repaint: Option<Value>,
    #[serde(rename = "experimental.rendering.software")]
    experimental_rendering_software: Option<Value>,
    #[serde(rename = "experimental.input.forceVT")]
    experimental_input_force_vt: Option<Value>,
    #[serde(rename = "experimental.useBackgroundImageForWindow")]
    experimental_use_background_image_for_window: Option<Value>,
    #[serde(rename = "compatibility.reloadEnvironmentVariables")]
    compatibility_reload_environment_variables: Option<Value>,
    initial_cols: Option<Value>,
    initial_position: Option<Value>,
    initial_rows: Option<Value>,
    start_on_user_login: Option<Value>,
    first_window_preference: Option<Value>,
    launch_mode: Option<Value>,
    rows_to_scroll: Option<Value>,
    minimize_to_notification_area: Option<Value>,
    always_show_notification_icon: Option<Value>,
    show_admin_shield: Option<Value>,
    use_acrylic_in_tab_row: Option<Value>,
    actions: Option<Value>,
    keybindings: Option<Value>,
    new_tab_menu: Option<Value>,
    language: Option<Value>,
    theme: Option<Value>,
    themes: Option<Value>,
    show_tabs_in_titlebar: Option<Value>,
    show_terminal_title_in_titlebar: Option<Value>,
    snap_to_grid_on_resize: Option<Value>,
    tab_width_mode: Option<Value>,
    word_delimiters: Option<Value>,
    confirm_close_all_tabs: Option<Value>,
    use_tab_switcher: Option<Value>,
    tab_switcher_mode: Option<Value>,
    windowing_behavior: Option<Value>,
    new_tab_position: Option<Value>,
    auto_hide_window: Option<Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Profiles {
    pub defaults: Map<String, Value>,
    pub list: Vec<Profile>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    acrylic_opacity: Option<f64>,
    antialiasing_mode: Option<String>,
    background: Option<String>,
    unfocused_appearance: Option<Value>,
    font: Option<Value>,
    pub background_image: Option<String>,
    pub background_image_alignment: Option<String>,
    pub background_image_opacity: Option<f64>,
    pub background_image_stretch_mode: Option<String>,
    bell_style: Option<Value>,
    bell_sound: Option<Value>,
    close_on_exit: Option<Value>,
    pub color_scheme: Option<String>,
    commandline: Option<String>,
    cursor_color: Option<Value>,
    cursor_height: Option<i32>,
    cursor_shape: Option<String>,
    elevate: Option<bool>,
    environment: Option<Value>,
    #[serde(rename = "experimental.autoMarkPrompts")]
    experimental_auto_mark_prompts: Option<bool>,
    #[serde(rename = "experimental.connectionPassthroughMode")]
    experimental_connection_passthrough_mode: Option<bool>,
    #[serde(rename = "experimental.retroTerminalEffect")]
    experimental_retro_terminal_effect: Option<bool>,
    #[serde(rename = "experimental.showMarksOnScrollbar")]
    experimental_show_marks_on_scrollbar: Option<bool>,
    #[serde(rename = "experimental.pixelShaderPath")]
    experimental_pixel_shader_path: Option<String>,
    use_atlas_engine: Option<bool>,
    font_face: Option<String>,
    font_size: Option<f64>,
    font_weight: Option<Value>,
    intense_text_style: Option<String>,
    foreground: Option<Value>,
    guid: String,
    hidden: bool,
    history_size: Option<i32>,
    icon: Option<Value>,
    name: String,
    opacity: Option<f64>,
    padding: Option<Value>,
    adjust_indistinguishable_colors: Option<String>,
    scrollbar_state: Option<String>,
    selection_background: Option<Value>,
    snap_on_input: Option<bool>,
    alt_gr_aliasing: Option<bool>,
    source: Option<String>,
    starting_directory: Option<String>,
    suppress_application_title: Option<bool>,
    tab_color: Option<Value>,
    tab_title: Option<String>,
    use_acrylic: Option<bool>,
}


#[derive(Debug, Deserialize, Serialize)]
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