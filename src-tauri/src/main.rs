// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use dirs::data_local_dir;
use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayMenu};

#[macro_use]
extern crate lazy_static;

mod models;
mod repositories;

use models::configuration::{ConfigurationModel, ConfigurationModelTrait};
use repositories::configuration::repository::JsonConfigurationRepository;
use repositories::gif::repository::RESTGifRepository;

use dotenv::dotenv;

lazy_static! {
    static ref CONFIGURATION_MODEL: ConfigurationModel = {
        dotenv().ok();

        let gif_repository: RESTGifRepository =
            RESTGifRepository::new(std::env::var("GIPHY_API_KEY").unwrap());

        let path = data_local_dir().unwrap();
        let configuration_repository: JsonConfigurationRepository =
            JsonConfigurationRepository::new(format!(
                "{}\\Packages\\Microsoft.WindowsTerminal_8wekyb3d8bbwe\\LocalState\\settings.json",
                path.to_str().unwrap()
            ));

        ConfigurationModel::new(configuration_repository, gif_repository)
    };
}

#[tauri::command(rename_all = "snake_case")]
async fn get_profiles() -> Result<Vec<String>, ()> {
    match CONFIGURATION_MODEL.get_profiles().await {
        Ok(profiles) => Ok(profiles),
        Err(_) => {
            println!("Error getting profiles");
            Err(())
        }
    }
}

#[tauri::command(rename_all = "snake_case")]
async fn set_current_profile(profile: String) -> Result<(), ()> {
    match CONFIGURATION_MODEL.set_current_profile(&profile).await {
        Ok(_) => Ok(()),
        Err(_) => {
            println!("Error setting current profile");
            Err(())
        }
    }
}

#[tauri::command(rename_all = "snake_case")]
async fn get_current_profile() -> Result<String, ()> {
    match CONFIGURATION_MODEL.get_current_profile().await {
        Ok(profile) => Ok(profile),
        Err(_) => {
            println!("Error getting current profile");
            Err(())
        }
    }
}

#[tauri::command(rename_all = "snake_case")]
async fn set_gif(search_query: String) -> Result<(), ()> {
    match CONFIGURATION_MODEL.set_gif(&search_query).await {
        Ok(_) => Ok(()),
        Err(_) => {
            println!("Error setting gif");
            Err(())
        }
    }
}

#[tauri::command(rename_all = "snake_case")]
async fn set_color_scheme(color_scheme: String) -> Result<(), ()> {
    match CONFIGURATION_MODEL.set_color_scheme(&color_scheme).await {
        Ok(_) => Ok(()),
        Err(_) => {
            println!("Error setting color scheme");
            Err(())
        }
    }
}

#[tauri::command(rename_all = "snake_case")]
async fn get_color_schemes() -> Result<Vec<String>, ()> {
    match CONFIGURATION_MODEL.get_color_schemes().await {
        Ok(color_schemes) => Ok(color_schemes),
        Err(_) => {
            println!("Error getting color schemes");
            Err(())
        }
    }
}

#[tauri::command(rename_all = "snake_case")]
async fn get_current_color_scheme() -> Result<String, ()> {
    match CONFIGURATION_MODEL.get_color_scheme().await {
        Ok(color_scheme) => Ok(color_scheme),
        Err(_) => {
            println!("Error getting current color scheme");
            Ok(String::from(""))
        }
    }
}

fn main() {
    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("quit".to_string(), "Quit").accelerator("Cmd+Q"));
    let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            set_gif,
            set_color_scheme,
            get_color_schemes,
            get_profiles,
            get_current_profile,
            set_current_profile,
            get_current_color_scheme
        ])
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            tauri::SystemTrayEvent::LeftClick { .. } => {
                let window = app.get_window("main").unwrap();
                window.show().unwrap();
                window.set_focus().unwrap();
            }
            tauri::SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                _ => {}
            },
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
