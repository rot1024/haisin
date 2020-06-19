use actix_web::{
    client::Client,
    error::{ErrorBadGateway, ErrorNotFound},
    Error, Result,
};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use feeder::{Article, Author, Post};
use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Fanbox;

#[derive(Debug, Deserialize)]
struct Resp {
    #[serde(skip_deserializing)]
    id: String,
    body: Body,
}

impl Into<Article> for Resp {
    fn into(self) -> Article {
        let baseurl = format!("https://{}.fanbox.cc", &self.id);
        Article {
            id: self.id,
            url: baseurl.clone(),
            title: format!(
                "{} | FANBOX",
                self.body
                    .items
                    .get(0)
                    .map(|p| p.user.name.clone())
                    .unwrap_or("".into())
            ),
            posts: self
                .body
                .items
                .into_iter()
                .map(|i| {
                    let url = format!("{}/posts/{}", &baseurl, &i.id);
                    Post {
                        author: Author {
                            id: i.user.user_id,
                            name: i.user.name,
                            alias: i.creator_id,
                            icon_url: i.user.icon_url,
                        },
                        id: url.clone(),
                        title: i.title,
                        image_url: i.cover_image_url,
                        published_at: i.published_datetime,
                        updated_at: i.updated_datetime,
                        summary: i.excerpt.clone(),
                        content: i.excerpt,
                        url,
                    }
                })
                .collect(),
        }
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
    cover_image_url: Option<String>,
    #[serde(rename = "publishedDatetime")]
    published_datetime: DateTime<Utc>,
    #[serde(rename = "updatedDatetime")]
    updated_datetime: DateTime<Utc>,
    #[serde(rename = "creatorId")]
    creator_id: String,
    user: User,
    excerpt: Option<String>,
    // body
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
        let mut resp = Client::default()
            .get(format!(
                "https://api.fanbox.cc/post.listCreator?creatorId={}&limit=10",
                name
            ))
            .set_header("origin", format!("https://{}.fanbox.cc", name))
            .timeout(Duration::from_secs(10))
            .send()
            .await?;

        if resp.status() == 404 {
            return Err(ErrorNotFound("not found"));
        } else if !resp.status().is_success() {
            return Err(ErrorBadGateway("bad gateway"));
        }

        let mut body: Resp = resp.json().await?;

        body.id = name.into();

        Ok(body.into())
    }
}
