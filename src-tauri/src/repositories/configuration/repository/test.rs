use uuid::Uuid;

use crate::repositories::configuration::repository::{ JsonConfigurationRepository, TerminalConfigurationRepository };

async fn get_repository() -> Result<JsonConfigurationRepository, ()> {
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

    Ok(JsonConfigurationRepository::new(String::from(path_to_test_config)))
}

#[tokio::test]
async fn test_update_configuration_stays_same() {
    let repository = get_repository().await.unwrap();
    let configuration = repository.get_configuration().await.unwrap();
    repository.update_configuration(configuration.clone()).await.unwrap();
    let new_configuration = repository.get_configuration().await.unwrap();
    assert_eq!(serde_json::to_string_pretty(&configuration).unwrap(), serde_json::to_string_pretty(&new_configuration).unwrap());
}