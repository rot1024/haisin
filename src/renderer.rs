use crate::Article;
use atom_syndication::{Content, Entry, Feed, Link, Person};
use chrono::{DateTime, NaiveDateTime, Utc};
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref DEFAULT_UPDATED_AT: DateTime<Utc> =
        DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc);
}

impl Into<Feed> for Article {
    fn into(self) -> Feed {
        let author = Person {
            name: self
                .posts
                .get(0)
                .map(|u| u.author.name.escape())
                .unwrap_or_else(|| "".into()),
            email: None,
            uri: None,
        };

        let updated = self
            .posts
            .iter()
            .map(|p| &p.updated_at)
            .max()
            .unwrap_or(&DEFAULT_UPDATED_AT)
            .clone();

        let entries = self
            .posts
            .into_iter()
            .map(|p| {
                let url = &p.url;
                Entry {
                    title: p.title.escape(),
                    id: url.into(),
                    updated: p.updated_at.into(),
                    authors: vec![author.clone()],
                    categories: vec![],
                    contributors: vec![],
                    links: vec![Link {
                        href: url.clone(),
                        mime_type: Some("text/html".into()),
                        rel: "alternate".into(),
                        title: Some(p.title),
                        hreflang: None,
                        length: None,
                    }],
                    published: Some(p.published_at.into()),
                    rights: None,
                    source: None,
                    summary: p.summary.map(|s| s.escape()),
                    content: p.content.map(|c| Content {
                        content_type: Some("text".into()),
                        src: Some(url.into()),
                        value: Some(c.escape()),
                    }),
                    extensions: HashMap::new(),
                }
            })
            .collect();

        Feed {
            title: self.title.escape(),
            id: self.url,
            updated: updated.into(),
            authors: vec![author],
            categories: vec![],
            contributors: vec![],
            generator: None,
            icon: None,
            links: vec![],
            logo: None,
            rights: None,
            subtitle: None,
            entries,
            extensions: HashMap::new(),
            namespaces: HashMap::new(),
        }
    }
}

trait Escape {
    fn escape(&self) -> Self;
}

impl Escape for String {
    fn escape(&self) -> Self {
        self.replace("&", "&amp;")
    }
}
