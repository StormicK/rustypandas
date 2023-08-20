// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use dirs::data_local_dir;
use tauri::{Manager, SystemTray, SystemTrayMenu};

#[macro_use]
extern crate lazy_static;

mod models;
mod repositories;

use models::configuration::{ ConfigurationModel, ConfigurationModelTrait };
use repositories::configuration::repository::JsonConfigurationRepository;
use repositories::gif::repository::RESTGifRepository;

use dotenv::dotenv;

lazy_static! {
    static ref CONFIGURATION_MODEL: ConfigurationModel = {
        dotenv().ok();

        let gif_repository: RESTGifRepository =
            RESTGifRepository::new(std::env::var("GIPHY_API_KEY").unwrap());

        let path = data_local_dir().unwrap();
        let configuration_repository: JsonConfigurationRepository = JsonConfigurationRepository::new(format!("{}\\Packages\\Microsoft.WindowsTerminal_8wekyb3d8bbwe\\LocalState\\settings.json", path.to_str().unwrap()));
        
        ConfigurationModel::new(configuration_repository, gif_repository)
    };
}

#[tauri::command(rename_all = "snake_case")]
async fn update_gif(search_query: String) -> Result<(), ()> {
    let _ = CONFIGURATION_MODEL.update_gif(&search_query).await;

    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
async fn update_color_scheme(color_scheme: String) -> Result<(), ()> {
    let _ = CONFIGURATION_MODEL.update_color_scheme(&color_scheme).await;

    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
async fn get_color_schemes() -> Result<Vec<String>, ()> {
    match CONFIGURATION_MODEL.get_color_schemes().await {
        Ok(color_schemes) => Ok(color_schemes),
        Err(_) => Err(()),
    }
}

fn main() {
    let tray_menu = SystemTrayMenu::new();
    let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![update_gif, update_color_scheme, get_color_schemes])
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            tauri::SystemTrayEvent::LeftClick { .. } => {
                let window = app.get_window("main").unwrap();
                window.show().unwrap();
                window.set_focus().unwrap();
            }
            _ => {}
        })
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                event.window().hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
