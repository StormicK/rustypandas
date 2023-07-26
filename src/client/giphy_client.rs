use reqwest::Client;
use url::Url;

use crate::model::{ClientError, GiphyResponse};

#[derive(Debug)]
pub struct GiphyClient {
    api_key: String,
    client: Client,
}

impl GiphyClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: Client::new(),
        }
    }

    fn build_search_url(&self, query: &str) -> Result<String, ClientError> {
        let url = Url::parse_with_params(
            "https://api.giphy.com/v1/gifs/search",
            &[
                ("api_key", self.api_key.to_string()),
                ("q", query.to_string()),
                ("limit", String::from("1")),
            ],
        );
        match url {
            Ok(url) => Ok(String::from(url.as_str())),
            Err(_) => Err(ClientError::SearchError(String::from("Invalid URL"))),
        }
    }

    pub async fn execute_search(&self, query: &str) -> Result<String, ClientError> {
        let url = self.build_search_url(query);
        let url = match url {
            Ok(url) => url,
            Err(e) => return Err(e),
        };

        let response = self.client.get(&url).send();
        let response = response.await.map_err(|e| ClientError::ReqwestError(e))?;

        let giphy_response = response
            .json::<GiphyResponse>()
            .await
            .map_err(|e| ClientError::ReqwestError(e))?;

        let giphy_data = giphy_response.data.first().ok_or(ClientError::ParseError);

        let giphy_data = match giphy_data {
            Ok(giphy_data) => giphy_data,
            Err(_) => return Err(ClientError::SearchError(String::from("No data found"))),
        };

        let giphy_image = &giphy_data.images.original.url;
        Ok(giphy_image.to_string())
    }
}
