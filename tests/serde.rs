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
    assert_eq!(res.bugs.unwrap().url.unwrap().to_string(), "https://github.com/owner/project/issues");
}
/*
#[test]
fn deserialize_bugs_object() {
    let json = r#"{"bugs": {
        "url": "https://github.com/owner/project/issues",
        "email": "project@hostname.com"
    }}"#;
    match parse_contents(json) {
        Ok(res) => {
            let bugs = res.bugs.unwrap();
            if let OrString::T(i) = bugs {
                assert_eq!(i.email.unwrap(), "project@hostname.com");
                assert_eq!(i.url.unwrap(), url::Url::parse("https://github.com/owner/project/issues").unwrap());
            } else {
                panic!("Enum is or wrong type")
            }           
        },
        Err(e) => {
            println!("{:?}", e)
        }
    }
}
*/
