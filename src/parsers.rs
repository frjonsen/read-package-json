use serde::{
    de::{self, Visitor, MapAccess},
    Deserialize, Deserializer};

use std::marker::PhantomData;
use std::str::FromStr;
use std::fmt;

pub fn optional_string_or_struct<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where T: Deserialize<'de> + FromStr<Err = crate::PackageJsonError>, D: Deserializer<'de> {
    struct OptionalStringOrStruct<T>(PhantomData<fn() -> T>);

    impl<'de, T> Visitor<'de> for OptionalStringOrStruct<T>
        where T: Deserialize<'de> + FromStr<Err = crate::PackageJsonError>
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
