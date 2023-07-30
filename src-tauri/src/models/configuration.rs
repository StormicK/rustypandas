use crate::repositories::{
    configuration::{
        dto::{Scheme, Theme},
        repository::{JsonConfigurationRepository, TerminalConfigurationRepository},
    },
    gif::repository::{GifRepository, RESTGifRepository},
};
use async_trait::async_trait;

use crate::models::errors::ConfigurationModelError;

#[async_trait]
pub trait PresetConfigurator {
    async fn refresh_panda(&self, search_query: &str) -> Result<(), ConfigurationModelError>;
}

#[derive(Debug)]
pub struct ConfigurationModel {
    terminal_config_repository: JsonConfigurationRepository,
    gif_repository: RESTGifRepository,
}

impl ConfigurationModel {
    pub fn new(
        terminal_config_repository: JsonConfigurationRepository,
        gif_repository: RESTGifRepository,
    ) -> Self {
        Self {
            terminal_config_repository,
            gif_repository,
        }
    }
}

#[async_trait]
impl PresetConfigurator for ConfigurationModel {
    async fn refresh_panda(&self, search_query: &str) -> Result<(), ConfigurationModelError> {
        let mut terminal_config = self.terminal_config_repository.get_configuration().await?;
        let panda_path = self.gif_repository.get_gif_by_search(search_query).await?;

        let scheme = Scheme {
            background: String::from("#3D1F16"),
            black: String::from("#401F10"),
            blue: String::from("#314573"),
            bright_black: String::from("#6E6E6E"),
            bright_blue: String::from("#5476B7"),
            bright_cyan: String::from("#8CD2E6"),
            bright_green: String::from("#9EC54F"),
            bright_purple: String::from("#B674C2"),
            bright_red: String::from("#C1443D"),
            bright_white: String::from("#FFFFFF"),
            bright_yellow: String::from("#DBA15C"),
            cursor_color: String::from("#FFFFFF"),
            cyan: String::from("#4F8A99"),
            foreground: String::from("#FCE7D2"),
            green: String::from("#516C57"),
            name: String::from("RedPanda"),
            purple: String::from("#82578E"),
            red: String::from("#B13D14"),
            selection_background: String::from("#4F8A99"),
            white: String::from("#DBD3CE"),
            yellow: String::from("#DB8758"),
        };

        terminal_config.schemes.push(scheme);

        let theme = Theme {
            name: String::from("RedPanda"),
            tab: crate::repositories::configuration::dto::Tab {
                background: Some(String::from("#3D1F16")),
                show_close_button: String::from("always"),
                unfocused_background: Some(String::from("#3D1F16")),
            },
            window: crate::repositories::configuration::dto::Window {
                application_theme: String::from("dark"),
                use_mica: false,
            },
        };

        terminal_config.themes.push(theme);
        terminal_config.theme = Some(String::from("RedPanda"));

        for profile in terminal_config.profiles.list.iter_mut() {
            profile.background_image = Some(panda_path.clone());
            profile.background_image_opacity = Some(0.27);
            profile.opacity = Some(97);
            profile.background_image_stretch_mode = Some(String::from("none"));
            profile.background_image_alignment = Some(String::from("bottomRight"));
            profile.color_scheme = Some(String::from("RedPanda"));
        }

        println!("Current config! {:?}", terminal_config);

        self.terminal_config_repository
            .update_configuration(terminal_config)
            .await?;

        Ok(())
    }
}
