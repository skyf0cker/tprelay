use serde::{Deserialize, Serialize};
use std::error::Error;
use std::result::Result;

#[derive(Deserialize, Serialize)]
pub struct TelegraphResp {
    ok: bool,
    result: Pagination,
}

#[derive(Deserialize, Serialize)]
pub struct Pagination {
    pages: Vec<Post>,
}

#[derive(Deserialize, Serialize)]
pub struct Post {
    // #[serde(skip_serializing)]
    pub path: String,
    pub title: String,
    pub url: String,
    pub description: String,
    pub author_name: String,
}

impl Post {
    pub fn is_ignore(&self) -> bool {
        self.path.contains("!ignore") || self.title.contains("!ignore")
    }
}

pub struct Client {
    token: String,
    client: reqwest::Client,
}

impl Client {
    pub fn new(token: &str) -> Self {
        Client {
            token: token.to_string(),
            client: reqwest::Client::new(),
        }
    }

    pub async fn get_page_list(&self, page: u32, limit: u32) -> Result<Vec<Post>, Box<dyn Error>> {
        let resp = self
            .client
            .get(format!(
                "https://api.telegra.ph/getPageList?access_token={}&offset={}&limit={}",
                self.token, page * limit, limit
            ))
            .send()
            .await?
            .json::<TelegraphResp>()
            .await?;

        if !resp.ok {
            return Err("get page list failed".into());
        }

        return Ok(resp
            .result
            .pages
            .into_iter()
            .filter(|p| !p.is_ignore())
            .collect());
    }
}
