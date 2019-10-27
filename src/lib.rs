mod parsers;

use crate::parsers::optional_string_or_struct;
use crate::PackageJsonError::ParseError;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
#[macro_use]
extern crate lazy_static;

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Issues {
    pub url: Option<url::Url>,
    pub email: Option<String>,
}

impl FromStr for Issues {
    type Err = PackageJsonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        url::Url::parse(s).map(|u| Issues {
            url: Some(u),
            email: None,
        }).map_err(|_| PackageJsonError::ParseError("Issues URL is malformed".to_owned()))
    }
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Person {
    pub name: String,
    pub email: Option<String>,
    pub url: Option<url::Url>,
}

impl FromStr for Person {
    type Err = PackageJsonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref NAME_REGEX: Regex = Regex::new(r"^([^\(<]+)").unwrap();
            static ref URL_REGEX: Regex = Regex::new(r"\(([^\)]+)\)").unwrap();
            static ref EMAIL_REGEX: Regex = Regex::new(r"<([^>]+)>").unwrap();
        }
        let name = NAME_REGEX
            .captures(s)
            .and_then(|f| f.get(1))
            .map(|f| f.as_str().trim().to_owned())
            .ok_or(PackageJsonError::MalformedShorthand(
                "Person is missing name".to_string(),
            ))?;
        let email = EMAIL_REGEX
            .captures(s)
            .and_then(|f| f.get(1))
            .map(|f| f.as_str())
            .map(str::to_owned);
        let url = URL_REGEX
            .captures(s)
            .and_then(|f| f.get(1))
            .map(|f| f.as_str())
            .map(url::Url::from_str)
            .transpose()
            .map_err(|r| {
                PackageJsonError::MalformedShorthand(format!("Person URL field is malformed: {}", r))
            })?;
        Ok(Person { name, url, email })
    }
}

#[derive(Default, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct PackageJson {
    pub name: String,
    pub version: Option<semver::Version>,
    pub description: String,
    pub keywords: std::vec::Vec<String>,
    pub homepage: Option<url::Url>,
    #[serde(deserialize_with = "optional_string_or_struct")]
    pub bugs: Option<Issues>,
    pub license: String,
    #[serde(deserialize_with = "optional_string_or_struct")]
    pub author: Option<Person>,
    pub contributors: Vec<Person>,
}

#[derive(Debug, Clone)]
pub enum PackageJsonError {
    ParseError(String),
    MalformedShorthand(String),
}

impl std::fmt::Display for PackageJsonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError(ref m) => write!(f, "Failed to parse package.json: {}", m),
            PackageJsonError::MalformedShorthand(ref m) => write!(
                f,
                "Failed to parse shorthand into object. {} is invalid.",
                m
            ),
        }
    }
}

impl std::error::Error for PackageJsonError {
    fn description(&self) -> &str {
        match self {
            ParseError(ref m) => m,
            PackageJsonError::MalformedShorthand(ref m) => m,
        }
    }
}

pub fn parse_contents(contents: &str) -> Result<PackageJson, PackageJsonError> {
    serde_json::from_str(&contents)
        .map_err(|r| PackageJsonError::ParseError(r.to_string().to_owned()))
}
