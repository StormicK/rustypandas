use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WindowsTerminalConfiguration {
    AnythingArray(Vec<Option<serde_json::Value>>),

    Bool(bool),

    Double(f64),

    String(String),

    WindowsTerminalConfigurationClass(WindowsTerminalConfigurationClass),
}

/// Properties that affect the entire window, regardless of the profile settings.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowsTerminalConfigurationClass {
    /// When set to true, tabs are always displayed. When set to false and showTabsInTitlebar is
    /// set to false, tabs only appear after opening a new tab.
    pub always_show_tabs: Option<bool>,

    /// When set to `true` closing a window with multiple tabs open WILL require confirmation.
    /// When set to `false` closing a window with multiple tabs open WILL NOT require
    /// confirmation.
    pub confirm_close_all_tabs: Option<bool>,

    /// When set to true, a selection is immediately copied to your clipboard upon creation. When
    /// set to false, the selection persists and awaits further action.
    pub copy_on_select: Option<bool>,

    /// Sets the default profile. Opens by clicking the '+' icon or typing the key binding
    /// assigned to 'newTab'. The guid of the desired default profile is used as the value.
    pub default_profile: Option<String>,

    /// The number of columns displayed in the window upon first load.
    pub initial_cols: Option<i64>,

    /// The position of the top left corner of the window upon first load. On a system with
    /// multiple displays, these coordinates are relative to the top left of the primary display.
    /// If launchMode is set to maximized, the window will be maximized on the monitor specified
    /// by those coordinates.
    pub initial_position: Option<String>,

    /// The number of rows displayed in the window upon first load.
    pub initial_rows: Option<i64>,

    /// Properties are specific to each custom key binding.
    pub keybindings: Option<Vec<Keybinding>>,

    /// Defines whether the Terminal will launch as maximized or not.
    pub launch_mode: Option<LaunchMode>,

    /// Sets the theme of the application.
    pub requested_theme: Option<RequestedTheme>,

    /// The number of rows to scroll at a time with the mouse wheel. This will override the
    /// system setting if the value is not zero or 'system'.
    pub rows_to_scroll: Option<RowsToScroll>,

    /// When set to true, the tabs are moved into the titlebar and the titlebar disappears. When
    /// set to false, the titlebar sits above the tabs.
    pub show_tabs_in_titlebar: Option<bool>,

    /// When set to true, titlebar displays the title of the selected tab. When set to false,
    /// titlebar displays 'Windows Terminal'.
    pub show_terminal_title_in_titlebar: Option<bool>,

    /// When set to true, the window will snap to the nearest character boundary on resize. When
    /// false, the window will resize 'smoothly'
    pub snap_to_grid_on_resize: Option<bool>,

    /// Sets the width of the tabs.
    pub tab_width_mode: Option<TabWidthMode>,

    /// Determines the delimiters used in a double click selection.
    pub word_delimiters: Option<String>,

    pub profiles: Profiles,

    pub schemes: Vec<SchemeList>,

    pub globals: Option<Globals>,
}

/// Properties that affect the entire window, regardless of the profile settings.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Globals {
    /// When set to true, tabs are always displayed. When set to false and showTabsInTitlebar is
    /// set to false, tabs only appear after opening a new tab.
    pub always_show_tabs: Option<bool>,

    /// When set to `true` closing a window with multiple tabs open WILL require confirmation.
    /// When set to `false` closing a window with multiple tabs open WILL NOT require
    /// confirmation.
    pub confirm_close_all_tabs: Option<bool>,

    /// When set to true, a selection is immediately copied to your clipboard upon creation. When
    /// set to false, the selection persists and awaits further action.
    pub copy_on_select: Option<bool>,

    /// Sets the default profile. Opens by clicking the '+' icon or typing the key binding
    /// assigned to 'newTab'. The guid of the desired default profile is used as the value.
    pub default_profile: String,

    /// The number of columns displayed in the window upon first load.
    pub initial_cols: Option<i64>,

    /// The position of the top left corner of the window upon first load. On a system with
    /// multiple displays, these coordinates are relative to the top left of the primary display.
    /// If launchMode is set to maximized, the window will be maximized on the monitor specified
    /// by those coordinates.
    pub initial_position: Option<String>,

    /// The number of rows displayed in the window upon first load.
    pub initial_rows: Option<i64>,

    /// Properties are specific to each custom key binding.
    pub keybindings: Option<Vec<Keybinding>>,

    /// Defines whether the Terminal will launch as maximized or not.
    pub launch_mode: Option<LaunchMode>,

    /// Sets the theme of the application.
    pub requested_theme: Option<RequestedTheme>,

    /// The number of rows to scroll at a time with the mouse wheel. This will override the
    /// system setting if the value is not zero or 'system'.
    pub rows_to_scroll: Option<RowsToScroll>,

    /// When set to true, the tabs are moved into the titlebar and the titlebar disappears. When
    /// set to false, the titlebar sits above the tabs.
    pub show_tabs_in_titlebar: Option<bool>,

    /// When set to true, titlebar displays the title of the selected tab. When set to false,
    /// titlebar displays 'Windows Terminal'.
    pub show_terminal_title_in_titlebar: Option<bool>,

    /// When set to true, the window will snap to the nearest character boundary on resize. When
    /// false, the window will resize 'smoothly'
    pub snap_to_grid_on_resize: Option<bool>,

    /// Sets the width of the tabs.
    pub tab_width_mode: Option<TabWidthMode>,

    /// Determines the delimiters used in a double click selection.
    pub word_delimiters: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Keybinding {
    /// The action executed when the associated key bindings are pressed.
    pub command: Command,

    /// Defines the key combinations used to call the command.
    pub keys: Keys,
}

/// The action executed when the associated key bindings are pressed.
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Command {
    Action(Action),

    Enum(CommandEnum),
}

/// Arguments corresponding to a Copy Text Action
///
/// Arguments corresponding to a New Tab Action
///
/// Arguments corresponding to a Switch To Tab Action
///
/// Arguments corresponding to a Move Focus Action
///
/// Arguments corresponding to a Resize Pane Action
///
/// Arguments corresponding to a Split Pane Action
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Action {
    /// The action to execute
    pub action: ActionEnum,

    /// If true, whitespace is removed and newlines are maintained. If false, newlines are
    /// removed and whitespace is maintained.
    pub trim_whitespace: Option<bool>,

    /// A commandline to use instead of the profile's
    pub commandline: Option<String>,

    /// The index of the profile in the new tab dropdown to open
    ///
    /// Which tab to switch to, with the first being 0
    pub index: Option<i64>,

    /// Either the GUID or name of a profile to use, instead of launching the default
    pub profile: Option<String>,

    /// A startingDirectory to use instead of the profile's
    pub starting_directory: Option<String>,

    /// An initial tabTitle to use instead of the profile's
    pub tab_title: Option<String>,

    /// The direction to move focus in, between panes
    ///
    /// The direction to move the pane separator in
    pub direction: Option<Direction>,

    /// The orientation to split the pane in, either vertical (think [|]), horizontal (think
    /// [-]), or auto (splits pane based on remaining space)
    pub split: Option<SplitState>,
}

/// The action to execute
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ActionEnum {
    #[serde(rename = "closePane")]
    ClosePane,

    #[serde(rename = "closeTab")]
    CloseTab,

    #[serde(rename = "closeWindow")]
    CloseWindow,

    Copy,

    #[serde(rename = "copyTextWithoutNewlines")]
    CopyTextWithoutNewlines,

    #[serde(rename = "decreaseFontSize")]
    DecreaseFontSize,

    #[serde(rename = "duplicateTab")]
    DuplicateTab,

    Find,

    #[serde(rename = "increaseFontSize")]
    IncreaseFontSize,

    #[serde(rename = "moveFocus")]
    MoveFocus,

    #[serde(rename = "moveFocusDown")]
    MoveFocusDown,

    #[serde(rename = "moveFocusLeft")]
    MoveFocusLeft,

    #[serde(rename = "moveFocusRight")]
    MoveFocusRight,

    #[serde(rename = "moveFocusUp")]
    MoveFocusUp,

    #[serde(rename = "newTab")]
    NewTab,

    #[serde(rename = "newTabProfile0")]
    NewTabProfile0,

    #[serde(rename = "newTabProfile1")]
    NewTabProfile1,

    #[serde(rename = "newTabProfile2")]
    NewTabProfile2,

    #[serde(rename = "newTabProfile3")]
    NewTabProfile3,

    #[serde(rename = "newTabProfile4")]
    NewTabProfile4,

    #[serde(rename = "newTabProfile5")]
    NewTabProfile5,

    #[serde(rename = "newTabProfile6")]
    NewTabProfile6,

    #[serde(rename = "newTabProfile7")]
    NewTabProfile7,

    #[serde(rename = "newTabProfile8")]
    NewTabProfile8,

    #[serde(rename = "nextTab")]
    NextTab,

    #[serde(rename = "openNewTabDropdown")]
    OpenNewTabDropdown,

    #[serde(rename = "openSettings")]
    OpenSettings,

    Paste,

    #[serde(rename = "prevTab")]
    PrevTab,

    #[serde(rename = "resetFontSize")]
    ResetFontSize,

    #[serde(rename = "resizePane")]
    ResizePane,

    #[serde(rename = "resizePaneDown")]
    ResizePaneDown,

    #[serde(rename = "resizePaneLeft")]
    ResizePaneLeft,

    #[serde(rename = "resizePaneRight")]
    ResizePaneRight,

    #[serde(rename = "resizePaneUp")]
    ResizePaneUp,

    #[serde(rename = "scrollDown")]
    ScrollDown,

    #[serde(rename = "scrollDownPage")]
    ScrollDownPage,

    #[serde(rename = "scrollUp")]
    ScrollUp,

    #[serde(rename = "scrollUpPage")]
    ScrollUpPage,

    #[serde(rename = "splitHorizontal")]
    SplitHorizontal,

    #[serde(rename = "splitPane")]
    SplitPane,

    #[serde(rename = "splitVertical")]
    SplitVertical,

    #[serde(rename = "switchToTab")]
    SwitchToTab,

    #[serde(rename = "switchToTab0")]
    SwitchToTab0,

    #[serde(rename = "switchToTab1")]
    SwitchToTab1,

    #[serde(rename = "switchToTab2")]
    SwitchToTab2,

    #[serde(rename = "switchToTab3")]
    SwitchToTab3,

    #[serde(rename = "switchToTab4")]
    SwitchToTab4,

    #[serde(rename = "switchToTab5")]
    SwitchToTab5,

    #[serde(rename = "switchToTab6")]
    SwitchToTab6,

    #[serde(rename = "switchToTab7")]
    SwitchToTab7,

    #[serde(rename = "switchToTab8")]
    SwitchToTab8,

    #[serde(rename = "toggleFullscreen")]
    ToggleFullscreen,
}

/// The direction to move focus in, between panes
///
/// The direction to move the pane separator in
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Direction {
    Down,

    Left,

    Right,

    Up,
}

/// The orientation to split the pane in, either vertical (think [|]), horizontal (think
/// [-]), or auto (splits pane based on remaining space)
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SplitState {
    Auto,

    Horizontal,

    Vertical,
}

/// The action to execute
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CommandEnum {
    #[serde(rename = "closePane")]
    ClosePane,

    #[serde(rename = "closeTab")]
    CloseTab,

    #[serde(rename = "closeWindow")]
    CloseWindow,

    Copy,

    #[serde(rename = "copyTextWithoutNewlines")]
    CopyTextWithoutNewlines,

    #[serde(rename = "decreaseFontSize")]
    DecreaseFontSize,

    #[serde(rename = "duplicateTab")]
    DuplicateTab,

    Find,

    #[serde(rename = "increaseFontSize")]
    IncreaseFontSize,

    #[serde(rename = "moveFocus")]
    MoveFocus,

    #[serde(rename = "moveFocusDown")]
    MoveFocusDown,

    #[serde(rename = "moveFocusLeft")]
    MoveFocusLeft,

    #[serde(rename = "moveFocusRight")]
    MoveFocusRight,

    #[serde(rename = "moveFocusUp")]
    MoveFocusUp,

    #[serde(rename = "newTab")]
    NewTab,

    #[serde(rename = "newTabProfile0")]
    NewTabProfile0,

    #[serde(rename = "newTabProfile1")]
    NewTabProfile1,

    #[serde(rename = "newTabProfile2")]
    NewTabProfile2,

    #[serde(rename = "newTabProfile3")]
    NewTabProfile3,

    #[serde(rename = "newTabProfile4")]
    NewTabProfile4,

    #[serde(rename = "newTabProfile5")]
    NewTabProfile5,

    #[serde(rename = "newTabProfile6")]
    NewTabProfile6,

    #[serde(rename = "newTabProfile7")]
    NewTabProfile7,

    #[serde(rename = "newTabProfile8")]
    NewTabProfile8,

    #[serde(rename = "nextTab")]
    NextTab,

    #[serde(rename = "openNewTabDropdown")]
    OpenNewTabDropdown,

    #[serde(rename = "openSettings")]
    OpenSettings,

    Paste,

    #[serde(rename = "prevTab")]
    PrevTab,

    #[serde(rename = "resetFontSize")]
    ResetFontSize,

    #[serde(rename = "resizePane")]
    ResizePane,

    #[serde(rename = "resizePaneDown")]
    ResizePaneDown,

    #[serde(rename = "resizePaneLeft")]
    ResizePaneLeft,

    #[serde(rename = "resizePaneRight")]
    ResizePaneRight,

    #[serde(rename = "resizePaneUp")]
    ResizePaneUp,

    #[serde(rename = "scrollDown")]
    ScrollDown,

    #[serde(rename = "scrollDownPage")]
    ScrollDownPage,

    #[serde(rename = "scrollUp")]
    ScrollUp,

    #[serde(rename = "scrollUpPage")]
    ScrollUpPage,

    #[serde(rename = "splitHorizontal")]
    SplitHorizontal,

    #[serde(rename = "splitPane")]
    SplitPane,

    #[serde(rename = "splitVertical")]
    SplitVertical,

    #[serde(rename = "switchToTab")]
    SwitchToTab,

    #[serde(rename = "switchToTab0")]
    SwitchToTab0,

    #[serde(rename = "switchToTab1")]
    SwitchToTab1,

    #[serde(rename = "switchToTab2")]
    SwitchToTab2,

    #[serde(rename = "switchToTab3")]
    SwitchToTab3,

    #[serde(rename = "switchToTab4")]
    SwitchToTab4,

    #[serde(rename = "switchToTab5")]
    SwitchToTab5,

    #[serde(rename = "switchToTab6")]
    SwitchToTab6,

    #[serde(rename = "switchToTab7")]
    SwitchToTab7,

    #[serde(rename = "switchToTab8")]
    SwitchToTab8,

    #[serde(rename = "toggleFullscreen")]
    ToggleFullscreen,
}

/// Defines the key combinations used to call the command.
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Keys {
    String(String),

    StringArray(Vec<String>),
}

/// Defines whether the Terminal will launch as maximized or not.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LaunchMode {
    Default,

    Maximized,
}

/// Sets the theme of the application.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequestedTheme {
    Dark,

    Light,

    System,
}

/// The number of rows to scroll at a time with the mouse wheel. This will override the
/// system setting if the value is not zero or 'system'.
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RowsToScroll {
    Integer(i64),

    String(String),
}

/// Sets the width of the tabs.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TabWidthMode {
    Equal,

    #[serde(rename = "titleLength")]
    TitleLength,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Profiles {
    ProfileListArray(Vec<ProfileList>),

    ProfilesObject(ProfilesObject),
}

/// A list of profiles and the properties specific to each.
///
/// Properties specific to a unique profile.
///
/// The default settings that apply to every profile.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileList {
    /// Unique identifier of the profile. Written in registry format:
    /// "{00000000-0000-0000-0000-000000000000}".
    pub guid: String,

    /// Name of the profile. Displays in the dropdown menu.
    pub name: String,

    /// When useAcrylic is set to true, it sets the transparency of the window for the profile.
    /// Accepts floating point values from 0-1 (default 0.5).
    pub acrylic_opacity: Option<f64>,

    /// Controls how text is antialiased in the renderer. Possible values are "grayscale",
    /// "cleartype" and "aliased". Note that changing this setting will require starting a new
    /// terminal instance.
    pub antialiasing_mode: Option<AntialiasingMode>,

    /// Sets the background color of the profile. Overrides background set in color scheme if
    /// colorscheme is set. Uses hex color format: "#rrggbb".
    pub background: Option<String>,

    /// Sets the file location of the Image to draw over the window background.
    pub background_image: Option<String>,

    pub background_image_alignment: Option<BackgroundImageAlignment>,

    /// (Not in SettingsSchema.md)
    pub background_image_opacity: Option<f64>,

    /// Sets how the background image is resized to fill the window.
    pub background_image_stretch_mode: Option<BackgroundImageStretchMode>,

    /// Sets how the profile reacts to termination or failure to launch. Possible values:
    /// "graceful" (close when exit is typed or the process exits normally), "always" (always
    /// close) and "never" (never close). true and false are accepted as synonyms for "graceful"
    /// and "never" respectively.
    pub close_on_exit: Option<CloseOnExitUnion>,

    /// Name of the terminal color scheme to use. Color schemes are defined under "schemes".
    pub color_scheme: Option<String>,

    /// Array of colors used in the profile if colorscheme is not set. Colors use hex color
    /// format: "#rrggbb". Ordering is as follows: [black, red, green, yellow, blue, magenta,
    /// cyan, white, bright black, bright red, bright green, bright yellow, bright blue, bright
    /// magenta, bright cyan, bright white]
    pub color_table: Option<Vec<ColorTable>>,

    /// Executable used in the profile.
    pub commandline: Option<String>,

    /// A GUID reference to a connection type. Currently undocumented as of 0.3, this is used for
    /// Azure Cloud Shell
    pub connection_type: Option<String>,

    /// Sets the cursor color for the profile. Uses hex color format: "#rrggbb".
    pub cursor_color: Option<String>,

    /// Sets the percentage height of the cursor starting from the bottom. Only works when
    /// cursorShape is set to "vintage". Accepts values from 25-100.
    pub cursor_height: Option<i64>,

    /// Sets the cursor shape for the profile. Possible values: "vintage" ( ▃ ), "bar" ( ┃,
    /// default ), "underscore" ( ▁ ), "filledBox" ( █ ), "emptyBox" ( ▯ )
    pub cursor_shape: Option<CursorShape>,

    /// When set to true, enable retro terminal effects. This is an experimental feature, and its
    /// continued existence is not guaranteed.
    #[serde(rename = "experimental.retroTerminalEffect")]
    pub experimental_retro_terminal_effect: Option<bool>,

    /// Name of the font face used in the profile.
    pub font_face: Option<String>,

    /// Sets the font size.
    pub font_size: Option<i64>,

    /// Sets the foreground color of the profile. Overrides foreground set in color scheme if
    /// colorscheme is set. Uses hex color format: "#rrggbb".
    pub foreground: Option<String>,

    /// If set to true, the profile will not appear in the list of profiles. This can be used to
    /// hide default profiles and dynamically generated profiles, while leaving them in your
    /// settings file.
    pub hidden: Option<bool>,

    /// The number of lines above the ones displayed in the window you can scroll back to.
    pub history_size: Option<i64>,

    /// Image file location of the icon used in the profile. Displays within the tab and the
    /// dropdown menu.
    pub icon: Option<String>,

    /// Sets the padding around the text within the window. Can have three different formats: "#"
    /// sets the same padding for all sides, "#, #" sets the same padding for left-right and
    /// top-bottom, and "#, #, #, #" sets the padding individually for left, top, right, and
    /// bottom.
    pub padding: Option<String>,

    /// Defines the visibility of the scrollbar.
    pub scrollbar_state: Option<ScrollbarState>,

    /// Sets the selection background color of the profile. Overrides selection background set in
    /// color scheme if colorscheme is set. Uses hex color format: "#rrggbb".
    pub selection_background: Option<String>,

    /// When set to true, the window will scroll to the command input line when typing. When set
    /// to false, the window will not scroll when you start typing.
    pub snap_on_input: Option<bool>,

    /// Stores the name of the profile generator that originated this profile.
    pub source: Option<String>,

    /// The directory the shell starts in when it is loaded.
    pub starting_directory: Option<String>,

    /// When set to true, tabTitle overrides the default title of the tab and any title change
    /// messages from the application will be suppressed. When set to false, tabTitle behaves as
    /// normal.
    pub suppress_application_title: Option<bool>,

    /// If set, will replace the name as the title to pass to the shell on startup. Some shells
    /// (like bash) may choose to ignore this initial value, while others (cmd, powershell) may
    /// use this value over the lifetime of the application.
    pub tab_title: Option<String>,

    /// When set to true, the window will have an acrylic background. When set to false, the
    /// window will have a plain, untextured background.
    pub use_acrylic: Option<bool>,
}

/// Controls how text is antialiased in the renderer. Possible values are "grayscale",
/// "cleartype" and "aliased". Note that changing this setting will require starting a new
/// terminal instance.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AntialiasingMode {
    Aliased,

    Cleartype,

    Grayscale,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BackgroundImageAlignment {
    Bottom,

    #[serde(rename = "bottomLeft")]
    BottomLeft,

    #[serde(rename = "bottomRight")]
    BottomRight,

    Center,

    Left,

    Right,

    Top,

    #[serde(rename = "topLeft")]
    TopLeft,

    #[serde(rename = "topRight")]
    TopRight,
}

/// Sets how the background image is resized to fill the window.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BackgroundImageStretchMode {
    Fill,

    None,

    Uniform,

    #[serde(rename = "uniformToFill")]
    UniformToFill,
}

/// Sets how the profile reacts to termination or failure to launch. Possible values:
/// "graceful" (close when exit is typed or the process exits normally), "always" (always
/// close) and "never" (never close). true and false are accepted as synonyms for "graceful"
/// and "never" respectively.
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CloseOnExitUnion {
    Bool(bool),

    Enum(CloseOnExitEnum),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CloseOnExitEnum {
    Always,

    Graceful,

    Never,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorTable {
    /// Sets the background color of the color table.
    pub background: Option<String>,

    /// Sets the color used as ANSI black.
    pub black: Option<String>,

    /// Sets the color used as ANSI blue.
    pub blue: Option<String>,

    /// Sets the color used as ANSI bright black.
    pub bright_black: Option<String>,

    /// Sets the color used as ANSI bright blue.
    pub bright_blue: Option<String>,

    /// Sets the color used as ANSI bright cyan.
    pub bright_cyan: Option<String>,

    /// Sets the color used as ANSI bright green.
    pub bright_green: Option<String>,

    /// Sets the color used as ANSI bright purple.
    pub bright_purple: Option<String>,

    /// Sets the color used as ANSI bright red.
    pub bright_red: Option<String>,

    /// Sets the color used as ANSI bright white.
    pub bright_white: Option<String>,

    /// Sets the color used as ANSI bright yellow.
    pub bright_yellow: Option<String>,

    /// Sets the color used as ANSI cyan.
    pub cyan: Option<String>,

    /// Sets the foreground color of the color table.
    pub foreground: Option<String>,

    /// Sets the color used as ANSI green.
    pub green: Option<String>,

    /// Sets the color used as ANSI purple.
    pub purple: Option<String>,

    /// Sets the color used as ANSI red.
    pub red: Option<String>,

    /// Sets the color used as ANSI white.
    pub white: Option<String>,

    /// Sets the color used as ANSI yellow.
    pub yellow: Option<String>,
}

/// Sets the cursor shape for the profile. Possible values: "vintage" ( ▃ ), "bar" ( ┃,
/// default ), "underscore" ( ▁ ), "filledBox" ( █ ), "emptyBox" ( ▯ )
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CursorShape {
    Bar,

    #[serde(rename = "emptyBox")]
    EmptyBox,

    #[serde(rename = "filledBox")]
    FilledBox,

    Underscore,

    Vintage,
}

/// Defines the visibility of the scrollbar.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScrollbarState {
    Hidden,

    Visible,
}

/// A list of profiles and default settings that apply to all of them
#[derive(Debug, Serialize, Deserialize)]
pub struct ProfilesObject {
    /// The default settings that apply to every profile.
    pub defaults: Option<Profile>,

    pub list: Option<Vec<ProfileList>>,
}

/// A list of profiles and the properties specific to each.
///
/// Properties specific to a unique profile.
///
/// The default settings that apply to every profile.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    /// When useAcrylic is set to true, it sets the transparency of the window for the profile.
    /// Accepts floating point values from 0-1 (default 0.5).
    pub acrylic_opacity: Option<f64>,

    /// Controls how text is antialiased in the renderer. Possible values are "grayscale",
    /// "cleartype" and "aliased". Note that changing this setting will require starting a new
    /// terminal instance.
    pub antialiasing_mode: Option<AntialiasingMode>,

    /// Sets the background color of the profile. Overrides background set in color scheme if
    /// colorscheme is set. Uses hex color format: "#rrggbb".
    pub background: Option<String>,

    /// Sets the file location of the Image to draw over the window background.
    pub background_image: Option<String>,

    pub background_image_alignment: Option<BackgroundImageAlignment>,

    /// (Not in SettingsSchema.md)
    pub background_image_opacity: Option<f64>,

    /// Sets how the background image is resized to fill the window.
    pub background_image_stretch_mode: Option<BackgroundImageStretchMode>,

    /// Sets how the profile reacts to termination or failure to launch. Possible values:
    /// "graceful" (close when exit is typed or the process exits normally), "always" (always
    /// close) and "never" (never close). true and false are accepted as synonyms for "graceful"
    /// and "never" respectively.
    pub close_on_exit: Option<CloseOnExitUnion>,

    /// Name of the terminal color scheme to use. Color schemes are defined under "schemes".
    pub color_scheme: Option<String>,

    /// Array of colors used in the profile if colorscheme is not set. Colors use hex color
    /// format: "#rrggbb". Ordering is as follows: [black, red, green, yellow, blue, magenta,
    /// cyan, white, bright black, bright red, bright green, bright yellow, bright blue, bright
    /// magenta, bright cyan, bright white]
    pub color_table: Option<Vec<ColorTable>>,

    /// Executable used in the profile.
    pub commandline: Option<String>,

    /// A GUID reference to a connection type. Currently undocumented as of 0.3, this is used for
    /// Azure Cloud Shell
    pub connection_type: Option<String>,

    /// Sets the cursor color for the profile. Uses hex color format: "#rrggbb".
    pub cursor_color: Option<String>,

    /// Sets the percentage height of the cursor starting from the bottom. Only works when
    /// cursorShape is set to "vintage". Accepts values from 25-100.
    pub cursor_height: Option<i64>,

    /// Sets the cursor shape for the profile. Possible values: "vintage" ( ▃ ), "bar" ( ┃,
    /// default ), "underscore" ( ▁ ), "filledBox" ( █ ), "emptyBox" ( ▯ )
    pub cursor_shape: Option<CursorShape>,

    /// When set to true, enable retro terminal effects. This is an experimental feature, and its
    /// continued existence is not guaranteed.
    #[serde(rename = "experimental.retroTerminalEffect")]
    pub experimental_retro_terminal_effect: Option<bool>,

    /// Name of the font face used in the profile.
    pub font_face: Option<String>,

    /// Sets the font size.
    pub font_size: Option<i64>,

    /// Sets the foreground color of the profile. Overrides foreground set in color scheme if
    /// colorscheme is set. Uses hex color format: "#rrggbb".
    pub foreground: Option<String>,

    /// Unique identifier of the profile. Written in registry format:
    /// "{00000000-0000-0000-0000-000000000000}".
    pub guid: Option<String>,

    /// If set to true, the profile will not appear in the list of profiles. This can be used to
    /// hide default profiles and dynamically generated profiles, while leaving them in your
    /// settings file.
    pub hidden: Option<bool>,

    /// The number of lines above the ones displayed in the window you can scroll back to.
    pub history_size: Option<i64>,

    /// Image file location of the icon used in the profile. Displays within the tab and the
    /// dropdown menu.
    pub icon: Option<String>,

    /// Name of the profile. Displays in the dropdown menu.
    pub name: Option<String>,

    /// Sets the padding around the text within the window. Can have three different formats: "#"
    /// sets the same padding for all sides, "#, #" sets the same padding for left-right and
    /// top-bottom, and "#, #, #, #" sets the padding individually for left, top, right, and
    /// bottom.
    pub padding: Option<String>,

    /// Defines the visibility of the scrollbar.
    pub scrollbar_state: Option<ScrollbarState>,

    /// Sets the selection background color of the profile. Overrides selection background set in
    /// color scheme if colorscheme is set. Uses hex color format: "#rrggbb".
    pub selection_background: Option<String>,

    /// When set to true, the window will scroll to the command input line when typing. When set
    /// to false, the window will not scroll when you start typing.
    pub snap_on_input: Option<bool>,

    /// Stores the name of the profile generator that originated this profile.
    pub source: Option<String>,

    /// The directory the shell starts in when it is loaded.
    pub starting_directory: Option<String>,

    /// When set to true, tabTitle overrides the default title of the tab and any title change
    /// messages from the application will be suppressed. When set to false, tabTitle behaves as
    /// normal.
    pub suppress_application_title: Option<bool>,

    /// If set, will replace the name as the title to pass to the shell on startup. Some shells
    /// (like bash) may choose to ignore this initial value, while others (cmd, powershell) may
    /// use this value over the lifetime of the application.
    pub tab_title: Option<String>,

    /// When set to true, the window will have an acrylic background. When set to false, the
    /// window will have a plain, untextured background.
    pub use_acrylic: Option<bool>,
}

/// Properties are specific to each color scheme. ColorTool is a great tool you can use to
/// create and explore new color schemes. All colors use hex color format.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemeList {
    /// Sets the background color of the color scheme.
    pub background: Option<String>,

    /// Sets the color used as ANSI black.
    pub black: Option<String>,

    /// Sets the color used as ANSI blue.
    pub blue: Option<String>,

    /// Sets the color used as ANSI bright black.
    pub bright_black: Option<String>,

    /// Sets the color used as ANSI bright blue.
    pub bright_blue: Option<String>,

    /// Sets the color used as ANSI bright cyan.
    pub bright_cyan: Option<String>,

    /// Sets the color used as ANSI bright green.
    pub bright_green: Option<String>,

    /// Sets the color used as ANSI bright purple.
    pub bright_purple: Option<String>,

    /// Sets the color used as ANSI bright red.
    pub bright_red: Option<String>,

    /// Sets the color used as ANSI bright white.
    pub bright_white: Option<String>,

    /// Sets the color used as ANSI bright yellow.
    pub bright_yellow: Option<String>,

    /// Sets the color used as ANSI cyan.
    pub cyan: Option<String>,

    /// Sets the foreground color of the color scheme.
    pub foreground: Option<String>,

    /// Sets the color used as ANSI green.
    pub green: Option<String>,

    /// Name of the color scheme.
    pub name: Option<String>,

    /// Sets the color used as ANSI purple.
    pub purple: Option<String>,

    /// Sets the color used as ANSI red.
    pub red: Option<String>,

    /// Sets the selection background color of the color scheme.
    pub selection_background: Option<String>,

    /// Sets the color used as ANSI white.
    pub white: Option<String>,

    /// Sets the color used as ANSI yellow.
    pub yellow: Option<String>,
}
