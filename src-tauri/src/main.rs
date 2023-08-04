// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use dirs::data_local_dir;
use tauri::{Manager, SystemTray, SystemTrayMenu};

#[macro_use]
extern crate lazy_static;

mod controllers;
mod models;
mod repositories;

use controllers::configuration::{ ConfigurationController, ConfigurationControllerTrait };
use models::configuration::ConfigurationModel;
use repositories::configuration::repository::JsonConfigurationRepository;
use repositories::gif::repository::RESTGifRepository;

use dotenv::dotenv;

lazy_static! {
    static ref CONFIGURATION_CONTROLLER: ConfigurationController = {
        dotenv().ok();

        let gif_repository: RESTGifRepository =
            RESTGifRepository::new(std::env::var("GIPHY_API_KEY").unwrap());

        //get appdata directory
        let path = data_local_dir().unwrap();
        let configuration_repository: JsonConfigurationRepository = JsonConfigurationRepository::new(format!("{}\\Packages\\Microsoft.WindowsTerminal_8wekyb3d8bbwe\\LocalState\\settings.json", path.to_str().unwrap()));
        let configuration_model: ConfigurationModel =
            ConfigurationModel::new(configuration_repository, gif_repository);

        controllers::configuration::ConfigurationController::new(configuration_model)
    };
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command(rename_all = "snake_case")]
async fn update_gif(search_query: String) -> Result<(), ()> {
    let _ = CONFIGURATION_CONTROLLER.update_gif(&search_query).await;

    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
async fn update_color_scheme(color_scheme: String) -> Result<(), ()> {
    let _ = CONFIGURATION_CONTROLLER.update_color_scheme(&color_scheme).await;

    Ok(())
}


fn main() {
    let tray_menu = SystemTrayMenu::new();
    let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![update_gif, update_color_scheme])
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
