use async_trait::async_trait;
use chrono::{DateTime, Utc};
use field_types::FieldName;
use serde::Serialize;
use std::fmt::{self, Display};

mod renderer;

#[derive(Debug, Serialize, Clone, FieldName)]
pub struct Article {
    pub id: String,
    pub url: String,
    pub title: String,
    pub posts: Vec<Post>,
}

#[derive(Debug, Serialize, Clone, FieldName)]
pub struct Post {
    pub author: Author,
    pub id: String,
    pub title: String,
    pub image_url: Option<String>,
    pub published_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub summary: Option<String>,
    pub content: Option<String>,
    pub url: String,
}

#[derive(Debug, Serialize, Clone, FieldName)]
pub struct Author {
    pub id: String,
    pub name: String,
    pub alias: String,
    pub icon_url: String,
}

#[async_trait(?Send)]
pub trait Source {
    type Err;

    async fn fetch(&self, name: &str) -> Result<Article, Error<Self::Err>>;
}

#[derive(Debug)]
pub enum Field {
    Article(ArticleFieldName),
    Post(PostFieldName),
    Author(AuthorFieldName),
}

impl Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            *f,
            "{}.{}",
            match self {
                Self::Article(_) => "article",
                Self::Post(_) => "post",
                Self::Author(_) => "author",
            },
            match self {
                Self::Article(n) => n.name(),
                Self::Post(n) => n.name(),
                Self::Author(n) => n.name(),
            }
        )
    }
}

#[derive(Debug)]
pub enum Error<T> {
    NotFound,
    ParseError(Field),
    Misc(T),
}
