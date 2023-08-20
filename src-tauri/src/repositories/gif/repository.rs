use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::PathBuf;

use async_trait::async_trait;
use dirs::data_local_dir;
use reqwest::Client;
use url::Url;
use rand::Rng;

use crate::repositories::gif::dto::GiphyResponse;
use crate::repositories::gif::errors::RepositoryError;

#[async_trait]
pub trait GifRepository {
    async fn get_gif_by_search(&self, query: &str) -> Result<String, RepositoryError>;
}

#[derive(Debug)]
pub struct RESTGifRepository {
    api_key: String,
    client: Client,
}

impl RESTGifRepository {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: Client::new(),
        }
    }

    fn build_search_url(&self, query: &str) -> Result<String, RepositoryError> {
        let url = Url::parse_with_params(
            "https://api.giphy.com/v1/gifs/search",
            [
                ("api_key", self.api_key.to_string()),
                ("q", query.to_string()),
                ("limit", String::from("30")),
            ],
        )?;

        Ok(String::from(url.as_str()))
    }

    async fn save_url_to_app_directory(&self, url: &str) -> Result<String, RepositoryError> {
        let mut path = Self::create_program_dir()?;
        path.push(format!("{}.gif", get_random_numbers_as_string()));

        let mut file = File::create(path.clone())?;

        let image_bytes = Self::retrieve_url_as_bytes(url).await?;
        file.write_all(&image_bytes)?;

        //get the path as string
        let path = match path.into_os_string().into_string() {
            Ok(path) => path,
            Err(_) => {
                return Err(RepositoryError::DataAccessError(String::from(
                    "OsString could not be converted to String",
                )))
            }
        };

        Ok(path)
    }

    fn create_program_dir() -> Result<PathBuf, RepositoryError> {
        let mut path = data_local_dir().ok_or(RepositoryError::DataAccessError(String::from(
            "Could not find local data directory",
        )))?;
        path.push("panda-dir");

        create_dir_all(&path)?;
        return Ok(path);
    }

    async fn retrieve_url_as_bytes(url: &str) -> Result<Vec<u8>, RepositoryError> {
        let response = reqwest::get(url).await?;
        let response_bytes = response.bytes().await?;

        Ok(response_bytes.to_vec())
    }
}

#[async_trait]
impl GifRepository for RESTGifRepository {
    async fn get_gif_by_search(&self, query: &str) -> Result<String, RepositoryError> {
        println!("Searching for: \"{}\"", query);

        let url = self.build_search_url(&query)?;

        let response = self.client.get(&url).send();
        let response = response.await?;

        let giphy_response = response.json::<GiphyResponse>().await?;

        let giphy_data = get_random_item(&giphy_response.data)
            .ok_or(RepositoryError::NotFoundError(String::from(
                "No images found",
            )))?;

        Ok(self
            .save_url_to_app_directory(&giphy_data.images.original.url)
            .await?)
    }
}

fn get_random_item<T>(vec: &Vec<T>) -> Option<&T> {
    if vec.is_empty() {
        return None;
    }

    let index = rand::thread_rng().gen_range(0..vec.len());
    Some(&vec[index])
}

fn get_random_numbers_as_string() -> String {
    let mut rng = rand::thread_rng();
    let mut result = String::new();

    result.push_str(&rng.gen_range(0..25).to_string());    

    result
}