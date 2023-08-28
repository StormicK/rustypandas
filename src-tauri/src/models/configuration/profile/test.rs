// async fn get_profiles(&self) -> Result<Vec<String>, ModelError>;
// async fn set_current_profile(&self, profile: &str) -> Result<(), ModelError>;
// async fn get_current_profile(&self) -> Result<String, ModelError>;
// async fn set_gif(&self, search_query: &str) -> Result<(), ModelError>;
// async fn set_color_scheme(&self, color_scheme: &str) -> Result<(), ModelError>;
// async fn get_color_scheme(&self) -> Result<String, ModelError>;

use crate::models::configuration::{ ProfileModel, ProfileModelTrait };
use crate::repositories::configuration::repository::JsonConfigurationRepository;
use crate::repositories::gif::repository::RESTGifRepository;
use uuid::Uuid;

async fn get_profile_model() -> Result<ProfileModel, ()> {
    dotenv::dotenv().ok();
    let path_to_config = format!("{}\\resources\\test\\config.json", env!("CARGO_MANIFEST_DIR"));
    let local_data_dir = dirs::data_local_dir().unwrap();
    let path_to_test_config = format!(
        "{}\\rustypandas-test\\{}\\settings.json",
        local_data_dir.to_str().unwrap(),
        Uuid::new_v4().to_string()
    );
    if let Some(parent_dir) = std::path::Path::new(&path_to_test_config).parent() {
        std::fs::create_dir_all(parent_dir).unwrap();
    }
    std::fs::copy(path_to_config, &path_to_test_config).unwrap();

    let configuration_repository = JsonConfigurationRepository::new(String::from(path_to_test_config));
    let gif_repository: RESTGifRepository = RESTGifRepository::new(std::env::var("GIPHY_API_KEY").unwrap());
    Ok(ProfileModel::new(configuration_repository, gif_repository))
}

#[tokio::test]
async fn test_get_profiles_will_return_all_profiles() {
    let profile_model = get_profile_model().await.unwrap();
    let profiles = profile_model.get_profiles().await.unwrap();
    assert_eq!(profiles.len(), 6);
    assert_eq!(profiles[0], String::from("Windows PowerShell"));
    assert_eq!(profiles[1], String::from("Eingabeaufforderung"));
    assert_eq!(profiles[2], String::from("PowerShell"));
    assert_eq!(profiles[3], String::from("Azure Cloud Shell"));
    assert_eq!(profiles[4], String::from("Developer Command Prompt for VS 2022"));
    assert_eq!(profiles[5], String::from("Developer PowerShell for VS 2022"));
}

#[tokio::test]
async fn test_get_current_profile_will_return_current_profile() {
    let profile_model = get_profile_model().await.unwrap();
    let current_profile = profile_model.get_current_profile().await.unwrap();
    assert_eq!(current_profile, String::from("PowerShell"));
}

#[tokio::test]
async fn test_set_current_profile_will_set_current_profile() {
    let profile_model = get_profile_model().await.unwrap();
    profile_model.set_current_profile("Windows PowerShell").await.unwrap();
    let current_profile = profile_model.get_current_profile().await.unwrap();
    assert_eq!(current_profile, String::from("Windows PowerShell"));
}

#[tokio::test]
async fn test_get_gif_will_return_gif() {
    let profile_model = get_profile_model().await.unwrap();
    let gif = profile_model.get_gif().await.unwrap();
    assert_eq!(gif, String::from("C:\\Users\\André Bruns\\AppData\\Local\\panda-dir\\7.gif"));
}

#[tokio::test]
async fn test_set_gif_will_set_gif() {
    let profile_model = get_profile_model().await.unwrap();
    profile_model.set_current_profile("Windows PowerShell").await.unwrap();
    profile_model.set_gif("test").await.unwrap();
    let gif = profile_model.get_gif().await.unwrap();
    assert!(!gif.is_empty());
}

#[tokio::test]
async fn test_get_color_scheme_will_return_color_scheme() {
    let profile_model = get_profile_model().await.unwrap();
    let color_scheme = profile_model.get_color_scheme().await.unwrap();
    assert_eq!(color_scheme, String::from("Campbell"));
}