use std::{fmt::Display, str::FromStr};

use serde::Deserialize;
use thiserror::Error;

use crate::validate::Validate;

#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
#[serde(try_from = "String")]
pub struct BulletId(String);

#[derive(Debug, Error)]
#[error("Formatting error: Only all small kebab case allowed")]
pub struct BulletIdError;

impl FromStr for BulletId {
    type Err = BulletIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().all(|c| c.is_alphanumeric() || c == '-') {
            Ok(BulletId(s.to_string()))
        } else {
            Err(BulletIdError)
        }
    }
}

impl TryFrom<String> for BulletId {
    type Error = BulletIdError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl Display for BulletId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::write!(f, "{}", self.0)
    }
}

#[derive(Debug, Deserialize)]
pub struct Bullet {
    pub id: BulletId,
    pub text: String,
    pub tags: Vec<String>,
}

#[derive(Error, Debug, Clone)]
#[error("Empty text for bullet id: {id}")]
pub struct EmptyBulletError {
    id: String,
}

impl Validate for Bullet {
    type Error = EmptyBulletError;

    fn validate(&self) -> Vec<Self::Error> {
        let mut res = vec![];
        if self.text.trim().is_empty() {
            res.push(Self::Error {
                id: self.id.0.to_string(),
            })
        }
        res
    }
}
