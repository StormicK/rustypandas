mod client;
use client::giphy_client::GiphyClient;

mod model;
use model::ClientError;

use std::fs::{File, create_dir_all};
use std::path::PathBuf;
use std::io::Write;
use std::env;
use dirs::data_local_dir;

const PROGRAM_NAME: &str = "giphy_client";

#[derive(Debug)]
enum ProgramError {
    ClientError(ClientError),
    DirNotFound(String),
    IoError(std::io::Error),
}

#[tokio::main]
async fn main() -> Result<(), ProgramError> {
    let mut path = create_program_dir()?;
    path.push("red_panda.gif");
    println!("Saving to {:?}", &path);

    let mut file = match File::create(path) {
        Ok(file) => file,
        Err(e) => return Err(ProgramError::IoError(e)),
    };

    let url = fetch_giphy_image_url().await?;
    
    let image_bytes = retrieve_url_as_bytes(&url).await?;
        
    match file.write_all(&image_bytes) {
        Ok(_) => return Ok(()),
        Err(e) => return Err(ProgramError::IoError(e)),
    };
}

fn create_program_dir() -> Result<PathBuf, ProgramError> {
    if let Some(mut path) = data_local_dir() {
        path.push(PROGRAM_NAME);
        match create_dir_all(&path) {
            Ok(_) => return Ok(path),
            Err(e) => return Err(ProgramError::IoError(e)),
        }
    } else {
        Err(ProgramError::DirNotFound(String::from("No local data directory found")))
    }
}

async fn fetch_giphy_image_url() -> Result<String, ProgramError> {
    dotenv::dotenv().ok();
    let api_key = match env::var("GIPHY_API_KEY") {
        Ok(api_key) => api_key,
        Err(e) => return Err(ProgramError::DirNotFound(e.to_string())),
    };

    let giphy_client = GiphyClient::new(api_key);
    
    let giphy_image = match giphy_client.execute_search("red panda").await {
        Ok(giphy_image) => giphy_image,
        Err(e) => return Err(ProgramError::ClientError(e)),
    };

    Ok(giphy_image)
}

async fn retrieve_url_as_bytes(url: &str) -> Result<Vec<u8>, ProgramError> {
    let response = match reqwest::get(url).await {
        Ok(response) => response,
        Err(e) => return Err(ProgramError::ClientError(model::ClientError::ReqwestError(e))),
    };

    let response_bytes = match response.bytes().await {
        Ok(response_bytes) => response_bytes,
        Err(e) => return Err(ProgramError::ClientError(model::ClientError::ReqwestError(e))),
    };

    Ok(response_bytes.to_vec())
}






