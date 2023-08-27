use crate::repositories::gif::repository::{ RESTGifRepository, GifRepository };

async fn get_repository() -> Result<RESTGifRepository, ()> {
    dotenv::dotenv().ok();
    Ok(RESTGifRepository::new(std::env::var("GIPHY_API_KEY").unwrap()))
}

#[tokio::test]
async fn get_gif_by_search_will_download_gif() {
    let repository = get_repository().await.unwrap();
    let gif = repository.get_gif_by_search("red panda").await.unwrap();
    assert!(std::path::Path::new(&gif).exists());
}