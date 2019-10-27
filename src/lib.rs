use serde::{
    de::{self, Visitor, MapAccess},
    Serialize,
    Deserialize, Deserializer};
use crate::PackageJsonError::ParseError;
use std::marker::PhantomData;
use std::str::FromStr;
use std::fmt;

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Issues {
    pub url: Option<url::Url>,
    pub email: Option<String>
}

impl FromStr for Issues {
    type Err = url::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        url::Url::from_str(s).map(|u| Issues {
            url: Some(u),
            email: None
        })
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
    pub bugs: Option<Issues>
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
        .map_err(|r| PackageJsonError::ParseError(r.to_string().to_owned()))
}

fn optional_string_or_struct<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where T: Deserialize<'de> + FromStr<Err = url::ParseError>, D: Deserializer<'de> {
    struct OptionalStringOrStruct<T>(PhantomData<fn() -> T>);
    
    impl<'de, T> Visitor<'de> for OptionalStringOrStruct<T>
        where T: Deserialize<'de> + FromStr<Err = url::ParseError>
    {
        type Value = T;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or map")
        }

        fn visit_str<E>(self, value: &str) -> Result<T, E> where E: de::Error {
            Ok(FromStr::from_str(value).unwrap())
        }

        fn visit_map<M>(self, map: M) -> Result<T, M::Error> where M: MapAccess<'de> {
            Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))
        }
    }
    let deserialized = deserializer.deserialize_any(OptionalStringOrStruct(PhantomData));
    deserialized.map(|r| Some(r))
}
