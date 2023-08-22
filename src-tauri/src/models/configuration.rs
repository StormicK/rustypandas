use std::sync::Mutex;

use crate::repositories::{
    configuration::{
        dto::Scheme,
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
    async fn get_profiles(&self) -> Result<Vec<String>, ConfigurationModelError>;
    async fn set_current_profile(&self, profile: &str) -> Result<(), ConfigurationModelError>;
    async fn get_current_profile(&self) -> Result<String, ConfigurationModelError>;
}

#[derive(Debug)]
pub struct ConfigurationModel {
    terminal_config_repository: JsonConfigurationRepository,
    gif_repository: RESTGifRepository,
    current_profile: Mutex<Option::<String>>,
}

impl ConfigurationModel {
    pub fn new(
        terminal_config_repository: JsonConfigurationRepository,
        gif_repository: RESTGifRepository,
    ) -> Self {
        Self {
            terminal_config_repository: terminal_config_repository,
            gif_repository: gif_repository,
            current_profile: Mutex::from(None),
        }
    }
}

#[async_trait]
impl ConfigurationModelTrait for ConfigurationModel {
    async fn update_gif(&self, search_query: &str) -> Result<(), ConfigurationModelError> {
        let mut terminal_config = self.terminal_config_repository.get_configuration().await?;
        let gif_path = self.gif_repository.get_gif_by_search(search_query).await?;

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

        terminal_config.schemes.retain(|s| s.name != "RedPanda");
        terminal_config.schemes.push(scheme);

        for profile in terminal_config.profiles.list.iter_mut() {
            if profile["name"].as_str().unwrap().to_string() == self.get_current_profile().await? {
                println!("Setting gif to: {}", gif_path);
                profile.insert(String::from("backgroundImage"), serde_json::Value::String(gif_path.clone()));
                profile.entry(String::from("backgroundImageOpacity")).or_insert(serde_json::Value::Number(serde_json::Number::from_f64(0.27).unwrap()));
                profile.insert(String::from("backgroundImageStretchMode"), serde_json::Value::String(String::from("none")));
                profile.insert(String::from("backgroundImageAlignment"), serde_json::Value::String(String::from("bottomRight")));
            }
        }

        self.terminal_config_repository
            .update_configuration(terminal_config)
            .await?;

        Ok(())
    }

    async fn update_color_scheme(&self, color_scheme: &str) -> Result<(), ConfigurationModelError> {
        let mut terminal_config = self.terminal_config_repository.get_configuration().await?;

        for profile in terminal_config.profiles.list.iter_mut() {
            if profile["name"].as_str().unwrap().to_string() == self.get_current_profile().await? {
                println!("Setting color scheme to: {}", color_scheme);
                profile["colorScheme"] = serde_json::Value::String(color_scheme.to_string());
            }
        }

        self.terminal_config_repository
            .update_configuration(terminal_config)
            .await?;

        Ok(())
    }

    async fn get_profiles(&self) -> Result<Vec<String>, ConfigurationModelError> {
        let terminal_config = self.terminal_config_repository.get_configuration().await?;

        let profiles: Vec<String> = terminal_config
            .profiles
            .list
            .iter()
            .map(|s| s["name"].as_str().unwrap().to_string())
            .collect();

        println!("{:?}", profiles);

        Ok(profiles)
    }

    async fn set_current_profile(&self, profile: &str) -> Result<(), ConfigurationModelError> {
        let mut terminal_config = self.terminal_config_repository.get_configuration().await?;

        for terminal_profile in terminal_config.profiles.list.iter_mut() {
            if terminal_profile["name"].as_str().unwrap().to_string() == profile {
                println!("Setting profile to: {}", profile);
                let mut locked_profile = self.current_profile.lock().unwrap();
                locked_profile.replace(terminal_profile["name"].as_str().unwrap().to_string());
            }
        }

        self.terminal_config_repository
            .update_configuration(terminal_config)
            .await?;

        Ok(())
    }

    async fn get_current_profile(&self) -> Result<String, ConfigurationModelError> {
        let terminal_config = self.terminal_config_repository.get_configuration().await?;

        //get stuff from arc
        let current_profile = self.current_profile.lock().unwrap();
        let current_profile = match current_profile.as_ref() {
            Some(profile) => profile.clone(),
            None => {
                let default_profile = terminal_config
                    .profiles
                    .list
                    .iter()
                    .filter(|p| p["guid"].as_str().unwrap() == terminal_config.default_profile);
                
                default_profile
                    .map(|s| s["name"].as_str().unwrap().to_string())
                    .collect::<Vec<String>>()[0].clone()
            },
        };

        println!("Current profile: {}", current_profile);
        Ok(current_profile.clone())
    }

    async fn get_color_schemes(&self) -> Result<Vec<String>, ConfigurationModelError> {
        let terminal_config = self.terminal_config_repository.get_configuration().await?;

        let color_schemes: Vec<String> = terminal_config
            .schemes
            .iter()
            .map(|s| s.name.to_string())
            .collect();

        Ok(color_schemes)
    }
}
