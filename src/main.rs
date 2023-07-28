mod repositories;
use repositories::gif::repository::{RESTGifRepository, GifRepository};
use repositories::configuration::repository::{JsonConfigurationRepository, TerminalConfigurationRepository};

mod errors;
use errors::ProgramError;

use std::fs::{File, create_dir_all};
use std::path::PathBuf;
use std::io::{Write, self};
use std::env;
use dirs::data_local_dir;

const PROGRAM_NAME: &str = "giphy_client";

#[tokio::main]
async fn main() -> Result<(), ProgramError> {
    let mut path = create_program_dir()?;
    path.push("red_panda.gif");
    println!("Saving to {:?}", &path);

    let mut file = match File::create(path.clone()) {
        Ok(file) => file,
        Err(e) => return Err(ProgramError::IoError(e)),
    };

    let url = fetch_giphy_image_url().await?;
    
    let image_bytes = retrieve_url_as_bytes(&url).await?;
        
    file.write_all(&image_bytes)?;

    let json_terminal_config_path = "C:\\Users\\André Bruns\\AppData\\Local\\Packages\\Microsoft.WindowsTerminal_8wekyb3d8bbwe\\LocalState\\settings.json";
    let json_terminal_config_repository = JsonConfigurationRepository::new(json_terminal_config_path.to_string());
    let mut terminal_config = json_terminal_config_repository.get_configuration().await?;
    println!("Current terminal configuration: {:?}", terminal_config);

    for profile in terminal_config.profiles.list.iter_mut() {
        //set background to the panda we wrote to disk
        profile.background_image = Some(path.to_str().unwrap().to_string());
    }

    json_terminal_config_repository.update_configuration(terminal_config).await?;

    Ok(())
}

fn create_program_dir() -> Result<PathBuf, ProgramError> {
    let mut path = data_local_dir().ok_or(ProgramError::IoError(io::Error::new(
        io::ErrorKind::NotFound,
        "Could not find local data directory",
    )))?;

    path.push(PROGRAM_NAME);
    
    match create_dir_all(&path) {
        Ok(_) => return Ok(path),
        Err(e) => return Err(ProgramError::IoError(e)),
    };
}

async fn fetch_giphy_image_url() -> Result<String, ProgramError> {
    dotenv::dotenv().ok();
    let api_key = env::var("GIPHY_API_KEY")?;
    let giphy_client = RESTGifRepository::new(api_key);
    let giphy_image = giphy_client.execute_search("red panda").await?;

    Ok(giphy_image)
}

async fn retrieve_url_as_bytes(url: &str) -> Result<Vec<u8>, ProgramError> {
    let response = reqwest::get(url).await?;
    let response_bytes = response.bytes().await?;

    Ok(response_bytes.to_vec())
}






