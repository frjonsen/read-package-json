extern crate read_package_json;
use read_package_json::*;

#[test]
fn deserialize_version() {
    let json = r#"{"version": "1.0.0"}"#;
    let res = parse_contents(json).unwrap();

    assert_eq!(res.version.clone().unwrap().major, 1);
}

#[test]
fn deserialize_name() {
    let json = r#"{"name": "apackage"}"#;
    let res = parse_contents(json).unwrap();

    assert_eq!(res.name.unwrap(), "apackage");
}

