use actix_web::Result;
use fanbox::Fanbox;
use haisin::{Article, Error, Source};
use std::str::FromStr;

mod fanbox;

#[derive(Debug)]
pub enum SourceType {
    Fanbox,
}

impl FromStr for SourceType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "fanbox" => Ok(Self::Fanbox),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Sources {
    fanbox: Fanbox,
}

impl Sources {
    pub fn new() -> Self {
        Self { fanbox: Fanbox }
    }

    pub async fn fetch(&self, t: SourceType, name: &str) -> Option<Result<Article>> {
        let src: &dyn Source<Err = actix_web::Error> = match t {
            SourceType::Fanbox => Some(&self.fanbox),
        }?;
        let res = src.fetch(name).await;
        match res {
            Err(Error::NotFound) => None,
            Err(Error::Misc(err)) => Some(Err(err)),
            Ok(result) => Some(Ok(result)),
        }
    }
}
