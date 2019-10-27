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

    assert_eq!(res.name, "apackage");
}

#[test]
fn deserialize_missing_name() {
    let json = "{}";
    let res = parse_contents(json).unwrap();
    assert_eq!(res.name, "");
}

#[test]
fn deserialize_bugs_string() {
    let json = r#"{"bugs": "https://github.com/owner/project/issues"}"#;
    let res = parse_contents(json).unwrap();
    assert_eq!(
        res.bugs.unwrap().url.unwrap(),
        url::Url::parse("https://github.com/owner/project/issues").unwrap()
    );
}

#[test]
fn derserialize_bugs_object() {
    let json = r#"{"bugs": {
        "url": "https://github.com/owner/project/issues",
        "email": "project@hostname.com"
    }}"#;
    let res = parse_contents(json).unwrap();
    let bugs = res.bugs.unwrap();
    let url = bugs.url.unwrap();
    let email = bugs.email.unwrap();
    assert_eq!(url, url::Url::parse("https://github.com/owner/project/issues").unwrap());
    assert_eq!(email, "project@hostname.com");
}

#[test]
fn deserialize_author_only_name() {
    let json = r#"{
        "author": {
            "name": "An Author"
    }}"#;
    let res = parse_contents(json).unwrap();
    let author = res.author.unwrap();
    assert_eq!(author.name, "An Author");
    assert!(author.email.is_none());
    assert!(author.url.is_none());
}

#[test]
fn deserialize_author_full_object() {
    let json = r#"{
        "author": {
            "name": "An Author",
            "url": "http://example.com",
            "email": "project@hostname.com"
    }}"#;
    let res = parse_contents(json).unwrap();
    let author = res.author.unwrap();
    assert_eq!(author.name, "An Author");
    assert_eq!(author.email.unwrap(), "project@hostname.com");
    //println!("{:?}", author.url.unwrap().as_str());
    assert_eq!(author.url.unwrap(), url::Url::parse("http://example.com").unwrap());
}

