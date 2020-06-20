use actix_web::{error::ErrorInternalServerError, Result};
use haisin::{Article, Error, Source};
use std::str::FromStr;

mod fanbox;
mod fantia;

#[derive(Debug)]
pub enum SourceType {
    Fanbox,
    Fantia,
}

impl FromStr for SourceType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "fanbox" => Ok(Self::Fanbox),
            "fantia" => Ok(Self::Fantia),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Sources {
    fanbox: fanbox::Fanbox,
    fantia: fantia::Fantia,
}

impl Sources {
    pub fn new() -> Self {
        Self {
            fanbox: fanbox::Fanbox,
            fantia: fantia::Fantia,
        }
    }

    pub async fn fetch(&self, t: SourceType, name: &str) -> Option<Result<Article>> {
        let res = self.source(t).fetch(name).await;

        match res {
            Err(Error::NotFound) => None,
            Err(Error::Misc(err)) => Some(Err(err)),
            Err(Error::ParseError(f)) => Some(Err(ErrorInternalServerError(format!(
                "failed to parse: {}",
                f
            )))),
            Ok(result) => Some(Ok(result)),
        }
    }

    fn source(&self, t: SourceType) -> Box<&(dyn Source<Err = actix_web::Error>)> {
        match t {
            SourceType::Fanbox => Box::new(&self.fanbox),
            SourceType::Fantia => Box::new(&self.fantia),
        }
    }
}
