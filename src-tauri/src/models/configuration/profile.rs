use std::sync::Mutex;

use async_trait::async_trait;
use serde_json::Value;

use crate::repositories::{
    configuration::repository::{JsonConfigurationRepository, TerminalConfigurationRepository},
    gif::repository::{GifRepository, RESTGifRepository},
};

use crate::models::errors::ModelError;

#[async_trait]
pub trait ProfileModelTrait {
    async fn get_profiles(&self) -> Result<Vec<String>, ModelError>;
    async fn set_current_profile(&self, profile: &str) -> Result<(), ModelError>;
    async fn get_current_profile(&self) -> Result<String, ModelError>;
    async fn set_gif(&self, search_query: &str) -> Result<(), ModelError>;

    async fn set_color_scheme(&self, color_scheme: &str) -> Result<(), ModelError>;
    async fn get_color_scheme(&self) -> Result<String, ModelError>;
}

#[derive(Debug)]
pub struct ProfileModel {
    terminal_config_repository: JsonConfigurationRepository,
    gif_repository: RESTGifRepository,
    current_profile: Mutex<Option<String>>,
}

impl ProfileModel {
    pub fn new(
        terminal_config_repository: JsonConfigurationRepository,
        gif_repository: RESTGifRepository,
    ) -> Self {
        Self {
            terminal_config_repository,
            gif_repository,
            current_profile: Mutex::new(None),
        }
    }

    fn extract_string(
        &self,
        value: &serde_json::Map<String, Value>,
        attribute: &str,
    ) -> Result<String, ModelError> {
        match value.get(attribute) {
            Some(Value::String(s)) => Ok(s.clone()),
            _ => Err(ModelError::ConfigurationFailedError()),
        }
    }
}

#[async_trait]
impl ProfileModelTrait for ProfileModel {
    async fn get_profiles(&self) -> Result<Vec<String>, ModelError> {
        let terminal_config = self.terminal_config_repository.get_configuration().await?;

        terminal_config
            .profiles
            .list
            .iter()
            .map(|profile| match self.extract_string(&profile, "name") {
                Ok(s) => Ok(s),
                _ => return Err(ModelError::ConfigurationFailedError()),
            })
            .collect::<Result<Vec<String>, ModelError>>()
            .or_else(|_| Err(ModelError::ConfigurationFailedError()))
    }

    async fn set_current_profile(&self, value: &str) -> Result<(), ModelError> {
        let mut terminal_config = self.terminal_config_repository.get_configuration().await?;

        for profile in terminal_config.profiles.list.iter_mut() {
            if self.extract_string(&profile, "name")? == value {
                println!("Setting profile to: {}", value);
                let mut locked_profile = self.current_profile.lock().unwrap();
                locked_profile.replace(profile["name"].as_str().unwrap().to_string());
            }
        }

        self.terminal_config_repository
            .update_configuration(terminal_config)
            .await?;

        Ok(())
    }

    async fn get_current_profile(&self) -> Result<String, ModelError> {
        let terminal_config = self.terminal_config_repository.get_configuration().await?;

        let current_profile = self.current_profile.lock().unwrap();
        let current_profile = match current_profile.as_ref() {
            Some(profile) => profile.clone(),
            None => {
                let default_profile_guid = terminal_config.default_profile;
                let default_profile = terminal_config.profiles.list.iter().find(|profile| {
                    let profile_guid = match self.extract_string(&profile, "guid") {
                        Ok(s) => s,
                        _ => return false,
                    };
                    profile_guid == default_profile_guid
                });

                let default_profile = match default_profile {
                    Some(value) => value,
                    _ => return Err(ModelError::ConfigurationFailedError()),
                };

                self.extract_string(default_profile, "name")?
            }
        };

        Ok(current_profile.clone())
    }

    async fn set_gif(&self, search_query: &str) -> Result<(), ModelError> {
        println!("Setting gif to: {}", search_query);
        let mut terminal_config = self.terminal_config_repository.get_configuration().await?;
        let gif_path = self.gif_repository.get_gif_by_search(search_query).await?;

        for profile in terminal_config.profiles.list.iter_mut() {
            let profile_name = self.extract_string(&profile, "name")?;

            if profile_name == self.get_current_profile().await? {
                println!("Setting gif to: {}", gif_path);
                profile.insert(
                    String::from("backgroundImage"),
                    Value::String(gif_path.clone()),
                );
                profile
                    .entry(String::from("backgroundImageOpacity"))
                    .or_insert(Value::Number(serde_json::Number::from_f64(0.27).unwrap()));
                profile.insert(
                    String::from("backgroundImageStretchMode"),
                    Value::String(String::from("none")),
                );
                profile.insert(
                    String::from("backgroundImageAlignment"),
                    Value::String(String::from("bottomRight")),
                );
                break;
            }
        }

        self.terminal_config_repository
            .update_configuration(terminal_config)
            .await?;

        Ok(())
    }

    async fn set_color_scheme(&self, color_scheme: &str) -> Result<(), ModelError> {
        let mut terminal_config = self.terminal_config_repository.get_configuration().await?;

        for profile in terminal_config.profiles.list.iter_mut() {
            let profile_name = self.extract_string(&profile, "name")?;

            if profile_name == self.get_current_profile().await? {
                println!("Setting color scheme to: {}", color_scheme);
                profile.insert(
                    String::from("colorScheme"),
                    Value::String(color_scheme.to_string()),
                );
            }
        }

        self.terminal_config_repository
            .update_configuration(terminal_config)
            .await?;

        Ok(())
    }

    async fn get_color_scheme(&self) -> Result<String, ModelError> {
        let terminal_config = self.terminal_config_repository.get_configuration().await?;

        let current_profile = self.get_current_profile().await?;
        let current_profile = terminal_config.profiles.list.iter().find(|profile| {
            let profile_name = match self.extract_string(&profile, "name") {
                Ok(s) => s,
                _ => return false,
            };
            profile_name == current_profile
        });

        let current_profile = match current_profile {
            Some(value) => value,
            _ => return Err(ModelError::ConfigurationFailedError()),
        };

        match self.extract_string(&current_profile, "colorScheme") {
            Ok(s) => Ok(s),
            _ => Err(ModelError::ConfigurationFailedError()),
        }
    }
}
