use crate::repositories::{
    configuration::{
        //dto::Scheme,
        dto_rework::testdto::SchemeList as Scheme,
        dto_rework::testdto::Profiles as Profiles,
        dto_rework::testdto::BackgroundImageStretchMode as BackgroundImageStretchMode,
        dto_rework::testdto::BackgroundImageAlignment as BackgroundImageAlignment,
        repository::{JsonConfigurationRepository, TerminalConfigurationRepository},
    },
    gif::repository::{GifRepository, RESTGifRepository},
};
use async_trait::async_trait;

use crate::models::errors::ConfigurationModelError;

#[async_trait]
pub trait ConfigurationModelTrait {
    async fn update_gif(&self, search_query: &str) -> Result<(), ConfigurationModelError>;
    async fn update_color_scheme(&self, color_scheme: &str) -> Result<(), ConfigurationModelError>;
    async fn get_color_schemes(&self) -> Result<Vec<String>, ConfigurationModelError>;
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
impl ConfigurationModelTrait for ConfigurationModel {
    async fn update_gif(&self, search_query: &str) -> Result<(), ConfigurationModelError> {
        let mut terminal_config = self.terminal_config_repository.get_configuration().await?;
        let panda_path = self.gif_repository.get_gif_by_search(search_query).await?;

        let scheme = Scheme {
            background: Option::from(String::from("#3D1F16")),
            black: Option::from(String::from("#401F10")),
            blue: Option::from(String::from("#314573")),
            bright_black: Option::from(String::from("#6E6E6E")),
            bright_blue: Option::from(String::from("#5476B7")),
            bright_cyan: Option::from(String::from("#8CD2E6")),
            bright_green: Option::from(String::from("#9EC54F")),
            bright_purple: Option::from(String::from("#B674C2")),
            bright_red: Option::from(String::from("#C1443D")),
            bright_white: Option::from(String::from("#FFFFFF")),
            bright_yellow: Option::from(String::from("#DBA15C")),
            cyan: Option::from(String::from("#4F8A99")),
            foreground: Option::from(String::from("#FCE7D2")),
            green: Option::from(String::from("#516C57")),
            name: Option::from(String::from("RedPanda")),
            purple: Option::from(String::from("#82578E")),
            red: Option::from(String::from("#B13D14")),
            selection_background: Option::from(String::from("#4F8A99")),
            white:Option::from( String::from("#DBD3CE")),
            yellow: Option::from(String::from("#DB8758")),
        };

        terminal_config.schemes.retain(|s| s.name.clone().expect("im lazy rn") != "RedPanda");
        terminal_config.schemes.push(scheme);
        
        match &mut terminal_config.profiles {
            Profiles::ProfileListArray (profiles) => {
                for profile in profiles.iter_mut() {
                    profile.background_image = Some(panda_path.clone());
                    profile.background_image_opacity = Some(0.27);
                    //profile.opacity = Some(97);
                    profile.background_image_stretch_mode = Some(BackgroundImageStretchMode::None);
                    profile.background_image_alignment = Some(BackgroundImageAlignment::BottomRight);
                };
            },
            Profiles::ProfilesObject(_) => {
                //ignore for now
            }
        }
        

        self.terminal_config_repository
            .update_configuration(terminal_config)
            .await?;

        Ok(())
    }

    async fn update_color_scheme(&self, color_scheme: &str) -> Result<(), ConfigurationModelError> {
        let mut terminal_config = self.terminal_config_repository.get_configuration().await?;
        
        match &mut terminal_config.profiles {
            Profiles::ProfileListArray (ref mut profiles) => {
                for profile in profiles.iter_mut() {
                    profile.color_scheme = Some(color_scheme.to_string());
                };
            },
            Profiles::ProfilesObject(_) => {
                //ignore for now
            }
        }

        self.terminal_config_repository
            .update_configuration(terminal_config)
            .await?;

        Ok(())
    }

    async fn get_color_schemes(&self) -> Result<Vec<String>, ConfigurationModelError> {
        let terminal_config = self.terminal_config_repository.get_configuration().await?;

        let color_schemes: Vec<String> = terminal_config
            .schemes
            .iter()
            .map(|s| s.name.clone().expect("lazyy rn", ).to_string())
            .collect();

        Ok(color_schemes)
    }
}
