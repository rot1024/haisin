use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Article(pub Vec<Post>);

#[derive(Debug, Serialize)]
pub struct Post {
    pub author: Author,
    pub id: String,
    pub title: String,
    pub image_url: String,
    pub published_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct Author {
    pub id: String,
    pub name: String,
    pub alias: String,
    pub icon_url: String,
}

#[async_trait(?Send)]
pub trait Source {
    type Err;

    async fn fetch(&self, name: &str) -> Result<Article, Self::Err>;
}
