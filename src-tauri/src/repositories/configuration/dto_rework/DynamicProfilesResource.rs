use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum DynamicProfilesResource {
    WindowsTerminalWsl,
    WindowsTerminalAzure,
    WindowsTerminalPowershellCore,
}

impl DynamicProfilesResource {
    pub const WINDOWS_TERMINAL_WSL: &'static str = "Windows.Terminal.Wsl";
    pub const WINDOWS_TERMINAL_AZURE: &'static str = "Windows.Terminal.Azure";
    pub const WINDOWS_TERMINAL_POWERSHELL_CORE: &'static str = "Windows.Terminal.PowershellCore";
}