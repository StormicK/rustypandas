use reqwest::Client;
use url::Url;
use async_trait::async_trait;

use crate::gif::dto::GiphyResponse;
use crate::gif::errors::RepositoryError;

#[async_trait]
pub trait GifRepository {
    async fn execute_search(&self, query: &str) -> Result<String, RepositoryError>;
}

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
                ("limit", String::from("1")),
            ],
        )?;
        
        Ok(String::from(url.as_str()))
    }
}

#[async_trait]
impl GifRepository for RESTGifRepository {
    async fn execute_search(&self, query: &str) -> Result<String, RepositoryError> {
        let url = self.build_search_url(query)?;

        let response = self.client.get(&url).send();
        let response = response.await?;

        let giphy_response = response
            .json::<GiphyResponse>()
            .await?;

        let giphy_data = giphy_response.data.first().ok_or(RepositoryError::NotFoundError(String::from("No images found")))?;
        Ok(giphy_data.images.original.url.to_string())
    }
}
