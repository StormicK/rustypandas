use crate::models::configuration::{ ColorSchemesModel, ColorSchemesModelTrait };
use crate::repositories::configuration::repository::JsonConfigurationRepository;

#[tokio::test]
async fn test_get_color_schemes() {
    let path_to_config = format!("{}\\resources\\test\\config.json", env!("CARGO_MANIFEST_DIR"));
    println!("{}", path_to_config);

    let configuration_repository = JsonConfigurationRepository::new(String::from(path_to_config));
    let color_schemes_model = ColorSchemesModel::new(configuration_repository);

    let color_schemes = color_schemes_model.get_color_schemes().await.unwrap();
    assert_eq!(color_schemes.len(), 3);
    assert_eq!(color_schemes[0], String::from("Campbell"));
    assert_eq!(color_schemes[1], String::from("Vintage"));
    assert_eq!(color_schemes[2], String::from("RedPanda"));
}
