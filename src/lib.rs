use serde::{Serialize, Deserialize};
use std::error::Error;
use crate::PackageJsonError::ParseError;

#[derive(Serialize, Deserialize, Debug)]
pub struct PackageJson {
    pub name: Option<String>,
    pub version: Option<semver::Version>
}

#[derive(Debug, Clone)]
pub enum PackageJsonError{
    ParseError(String)
}

impl std::fmt::Display for PackageJsonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError(ref m) => write!(f, "Failed to parse package.json: {}", m)
        }
    }
}


impl std::error::Error for PackageJsonError {
    fn description(&self) -> &str {
        match self {
            ParseError(ref m) => m
        }
    }
}

pub fn parse_contents(contents: &str) -> Result<PackageJson, PackageJsonError> {
    serde_json::from_str(&contents)
        .map_err(|r| PackageJsonError::ParseError(r.description().to_owned()))
}
