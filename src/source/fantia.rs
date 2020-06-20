use actix_web::{client::Client, error::ErrorBadGateway, Result};
use async_trait::async_trait;
use chrono::{FixedOffset, TimeZone};
use haisin::{Article, Author, AuthorFieldName, Error, Field, Post, PostFieldName};
use lazy_static::lazy_static;
use regex::Regex;
use scraper::{Html, Selector};

const ORIGIN: &str = "https://fantia.jp";

#[derive(Debug, Clone)]
pub struct Fantia;

#[async_trait(?Send)]
impl super::Source for Fantia {
    type Err = actix_web::Error;

    async fn fetch(&self, name: &str) -> Result<Article, Error<Self::Err>> {
        let mut resp = Client::default()
            .get(format!(
                "{}/fanclubs/{}/posts?utf8=%E2%9C%93&q%5Bs%5D=newer",
                ORIGIN, name
            ))
            .send()
            .await
            .map_err(|e| Error::Misc(e.into()))?;

        if resp.status() == 404 {
            return Err(Error::NotFound);
        } else if !resp.status().is_success() {
            return Err(Error::Misc(ErrorBadGateway("bad gateway")));
        }

        let body = resp.body().await.map_err(|e| Error::Misc(e.into()))?;
        let body_str = std::str::from_utf8(&body).map_err(|e| Error::Misc(e.into()))?;
        let doc = Html::parse_document(body_str);

        Ok(parse(doc, name).map_err(|e| Error::ParseError(e))?)
    }
}

fn parse(doc: Html, id: &str) -> Result<Article, Field> {
    lazy_static! {
        static ref S1: Selector = Selector::parse(".fanclub-summary h1 a").unwrap();
        static ref S2: Selector = Selector::parse(".post-inner").unwrap();
        static ref S_POST_TITLE: Selector = Selector::parse(".post-title").unwrap();
        static ref S_POST_DATE: Selector = Selector::parse(".post-date").unwrap();
        static ref S_POST_TEXT: Selector = Selector::parse(".post-text").unwrap();
        static ref S_IMG: Selector = Selector::parse("img").unwrap();
        static ref S_A: Selector = Selector::parse("a").unwrap();
        static ref S_SPAN: Selector = Selector::parse("span").unwrap();
        static ref RE_POST_ID: Regex = Regex::new("^/posts/(.+)$").unwrap();
    }

    let author_name: String = doc
        .select(&S1)
        .next()
        .map(|e| e.text().collect())
        .ok_or(Field::Author(AuthorFieldName::Name))?;

    let author = Author {
        id: id.into(),
        name: author_name.clone(),
        alias: id.into(),
        icon_url: "".into(),
    };

    let posts: Vec<Post> = doc
        .select(&S2)
        .map(|e| {
            let path = e
                .select(&S_A)
                .next()
                .and_then(|e| e.value().attr("href"))
                .ok_or(Field::Post(PostFieldName::Id))?;

            let id = RE_POST_ID
                .captures_iter(path)
                .next()
                .and_then(|m| m.get(1))
                .map(|m| m.as_str())
                .ok_or(Field::Post(PostFieldName::Id))?;

            let title = e
                .select(&S_POST_TITLE)
                .next()
                .ok_or(Field::Post(PostFieldName::Title))?
                .text()
                .collect();

            // <span class="post-date">2020-06-17 17:32</span>
            // or
            // <span class="post-date recently"><span class="mr-5">2020-06-17 17:32</span><span>更新</span></span>
            let date = e
                .select(&S_POST_DATE)
                .next()
                .map(|e| e.select(&S_SPAN).next().unwrap_or(e))
                .map(|e| e.text().collect::<String>())
                .ok_or(Field::Post(PostFieldName::PublishedAt))?;

            let published_at = FixedOffset::east(9 * 3600)
                .datetime_from_str(&date, "%Y-%m-%d %H:%M")
                .map_err(|_| Field::Post(PostFieldName::PublishedAt))?
                .into();

            Ok(Post {
                author: author.clone(),
                id: id.into(),
                title,
                image_url: e
                    .select(&S_IMG)
                    .next()
                    .and_then(|e| e.value().attr("src"))
                    .map(|s| s.into()),
                published_at,
                updated_at: published_at,
                summary: e.select(&S_POST_TEXT).next().map(|e| e.text().collect()),
                content: None,
                url: format!("{}{}", ORIGIN, path),
            })
        })
        .collect::<Result<Vec<Post>, _>>()?;

    Ok(Article {
        id: id.into(),
        url: format!("{}/fanclubs/{}", ORIGIN, &author_name),
        title: format!("{} | ファンティア[Fantia]", &author_name),
        posts,
    })
}
