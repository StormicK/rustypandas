use std::sync::Mutex;

use crate::repositories::{
    configuration::{
        dto::Scheme,
        repository::{JsonConfigurationRepository, TerminalConfigurationRepository},
    },
    gif::repository::{GifRepository, RESTGifRepository},
};
use async_trait::async_trait;
use serde_json::Value;

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
            current_profile: Mutex::new(None),
        }
    }

    fn extract_string(&self, value: &serde_json::Map<String, Value>, attribute: &str) -> Result<String, ConfigurationModelError> {
        match &value[attribute] {
            Value::String(s) => Ok(s.clone()),
            _ => Err(ConfigurationModelError::ConfigurationFailedError()),
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
            let profile_name = match self.extract_string(&profile, "name") {
                Ok(s) => s,
                Err(_) => return Err(ConfigurationModelError::ConfigurationFailedError()), 
            };
            
            if profile_name == self.get_current_profile().await? {
                println!("Setting gif to: {}", gif_path);
                profile.insert(String::from("backgroundImage"), Value::String(gif_path.clone()));
                profile.entry(String::from("backgroundImageOpacity")).or_insert(Value::Number(serde_json::Number::from_f64(0.27).unwrap()));
                profile.insert(String::from("backgroundImageStretchMode"), Value::String(String::from("none")));
                profile.insert(String::from("backgroundImageAlignment"), Value::String(String::from("bottomRight")));
                break;
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
            let profile_name = match self.extract_string(&profile, "name") {
                Ok(s) => s,
                _ => return Err(ConfigurationModelError::ConfigurationFailedError()),
            };

            if profile_name == self.get_current_profile().await? {
                println!("Setting color scheme to: {}", color_scheme);
                profile["colorScheme"] = Value::String(color_scheme.to_string());
            }
        }

        self.terminal_config_repository
            .update_configuration(terminal_config)
            .await?;

        Ok(())
    }

    async fn get_profiles(&self) -> Result<Vec<String>, ConfigurationModelError> {
        let terminal_config = self.terminal_config_repository.get_configuration().await?;

        let profiles = terminal_config
            .profiles
            .list
            .iter()
            .map(|profile| {
                match self.extract_string(&profile, "name") {
                    Ok(s) => Ok(s),
                    _ => return Err(ConfigurationModelError::ConfigurationFailedError()),
                }
            }).collect::<Result<Vec<String>, ConfigurationModelError>>()?;

        println!("{:?}", profiles);

        Ok(profiles)
    }

    async fn set_current_profile(&self, value: &str) -> Result<(), ConfigurationModelError> {
        let mut terminal_config = self.terminal_config_repository.get_configuration().await?;

        for profile in terminal_config.profiles.list.iter_mut() {
            let profile_name = match self.extract_string(&profile, "name") {
                Ok(s) => s,
                _ => return Err(ConfigurationModelError::ConfigurationFailedError()),
            };

            if profile_name == value {
                println!("Setting profile to: {}", profile_name);
                let mut locked_profile = self.current_profile.lock().unwrap();
                locked_profile.replace(profile["name"].as_str().unwrap().to_string());
            }
        }

        self.terminal_config_repository
            .update_configuration(terminal_config)
            .await?;

        Ok(())
    }

    async fn get_current_profile(&self) -> Result<String, ConfigurationModelError> {
        let terminal_config = self.terminal_config_repository.get_configuration().await?;

        let current_profile = self.current_profile.lock().unwrap();
        let current_profile = match current_profile.as_ref() {
            Some(profile) => {
                println!("Getting current profile");
                profile.clone()
            },
            None => {
                println!("Getting current default profile");
                let default_profile_guid = terminal_config.default_profile;
                let default_profile = terminal_config
                    .profiles
                    .list
                    .iter()
                    .find(|profile| {
                        let profile_guid = match self.extract_string(&profile, "guid") {
                            Ok(s) => s,
                            _ => return false,
                        };
                        profile_guid == default_profile_guid
                    });
                
                let default_profile = match default_profile {
                    Some(value) => value,
                    _ => return Err(ConfigurationModelError::ConfigurationFailedError()),
                };
                
                match self.extract_string(default_profile, "name") {
                    Ok(s) => s,
                    _ => return Err(ConfigurationModelError::ConfigurationFailedError()),
                }
            },
        };

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
