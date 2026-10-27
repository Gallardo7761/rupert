use nekosbest::{
    Category,
    client::{Client, ClientConfig},
};

use crate::error::GifError;

#[derive(Debug, Clone)]
pub struct Gif {
    pub url: String,
    pub anime: Option<String>,
}

pub struct GifClient {
    client: Client,
}

impl GifClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(ClientConfig::default()),
        }
    }

    pub async fn random(&self, category: Category) -> Result<Gif, GifError> {
        let response = nekosbest::get_with_client(&self.client, category).await?;
        Ok(Gif {
            url: response.url,
            anime: response.details.try_into_gif().ok().map(|d| d.anime_name),
        })
    }
}

impl Default for GifClient {
    fn default() -> Self {
        Self::new()
    }
}
