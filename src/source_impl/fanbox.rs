use actix_web::{client::Client, Error, Result};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use feeder::{Article, Author, Post};
use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Fanbox;

#[derive(Debug, Deserialize)]
struct Resp {
    body: Body,
}

impl Into<Article> for Resp {
    fn into(self) -> Article {
        Article(
            self.body
                .items
                .into_iter()
                .map(|i| Post {
                    id: i.id,
                    author: Author {
                        id: i.user.user_id,
                        alias: i.creator_id,
                        icon_url: i.user.icon_url,
                        name: i.user.name,
                    },
                    title: i.title,
                    published_at: i.published_datetime,
                    image_url: i.cover_image_url,
                })
                .collect(),
        )
    }
}

#[derive(Debug, Deserialize)]
struct Body {
    items: Vec<Item>,
}

#[derive(Debug, Deserialize)]
struct Item {
    id: String,
    title: String,
    #[serde(rename = "coverImageUrl")]
    cover_image_url: String,
    #[serde(rename = "publishedDatetime")]
    published_datetime: DateTime<Utc>,
    #[serde(rename = "creatorId")]
    creator_id: String,
    user: User,
}

#[derive(Debug, Deserialize)]
struct User {
    #[serde(rename = "userId")]
    user_id: String,
    name: String,
    #[serde(rename = "iconUrl")]
    icon_url: String,
}

#[async_trait(?Send)]
impl super::Source for Fanbox {
    type Err = Error;

    async fn fetch(&self, name: &str) -> Result<Article, Self::Err> {
        let resp: Resp = Client::default()
            .get(format!(
                "https://api.fanbox.cc/post.listCreator?creatorId={}&limit=10",
                name
            ))
            .set_header("origin", format!("https://{}.fanbox.cc", name))
            .timeout(Duration::from_secs(10))
            .send()
            .await?
            .json()
            .await?;

        Ok(resp.into())
    }
}
