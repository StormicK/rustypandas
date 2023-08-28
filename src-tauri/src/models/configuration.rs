mod color_schemes;
mod profile;

use crate::repositories::{
    configuration::repository::JsonConfigurationRepository, gif::repository::RESTGifRepository,
};
use async_trait::async_trait;

use crate::models::errors::ModelError;

use self::{
    color_schemes::{ColorSchemesModel, ColorSchemesModelTrait},
    profile::{ProfileModel, ProfileModelTrait},
};

#[async_trait]
pub trait ConfigurationModelTrait {
    async fn get_color_schemes(&self) -> Result<Vec<String>, ModelError>;
    async fn set_color_scheme(&self, color_scheme: &str) -> Result<(), ModelError>;
    async fn get_color_scheme(&self) -> Result<String, ModelError>;

    async fn get_profiles(&self) -> Result<Vec<String>, ModelError>;
    async fn set_current_profile(&self, profile: &str) -> Result<(), ModelError>;
    async fn get_current_profile(&self) -> Result<String, ModelError>;
    async fn set_gif(&self, search_query: &str) -> Result<(), ModelError>;
}

#[derive(Debug)]
pub struct ConfigurationModel {
    profile_model: ProfileModel,
    color_schemes_model: ColorSchemesModel,
}

impl ConfigurationModel {
    pub fn new(
        terminal_config_repository: JsonConfigurationRepository,
        gif_repository: RESTGifRepository,
    ) -> Self {
        Self {
            color_schemes_model: ColorSchemesModel::new(terminal_config_repository.clone()),
            profile_model: ProfileModel::new(terminal_config_repository, gif_repository),
        }
    }
}

#[async_trait]
impl ConfigurationModelTrait for ConfigurationModel {
    async fn get_color_schemes(&self) -> Result<Vec<String>, ModelError> {
        self.color_schemes_model.get_color_schemes().await
    }

    async fn set_color_scheme(&self, color_scheme: &str) -> Result<(), ModelError> {
        self.profile_model.set_color_scheme(color_scheme).await
    }

    async fn get_color_scheme(&self) -> Result<String, ModelError> {
        self.profile_model.get_color_scheme().await
    }

    async fn get_profiles(&self) -> Result<Vec<String>, ModelError> {
        self.profile_model.get_profiles().await
    }

    async fn set_current_profile(&self, profile: &str) -> Result<(), ModelError> {
        self.profile_model.set_current_profile(profile).await
    }

    async fn get_current_profile(&self) -> Result<String, ModelError> {
        self.profile_model.get_current_profile().await
    }

    async fn set_gif(&self, search_query: &str) -> Result<(), ModelError> {
        self.profile_model.set_gif(search_query).await
    }
}
