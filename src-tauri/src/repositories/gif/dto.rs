use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GiphyResponse {
    pub data: Vec<GiphyData>,
}

#[derive(Debug, Deserialize)]
pub struct GiphyData {
    pub images: GiphyImages,
}

#[derive(Debug, Deserialize)]
pub struct GiphyImages {
    pub original: GiphyImage,
}

#[derive(Debug, Deserialize)]
pub struct GiphyImage {
    pub url: String,
}
