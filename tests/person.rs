extern crate read_package_json;
use read_package_json::*;
use std::str::FromStr;

#[test]
fn parse_shorthand_person() {
    let person = "Barney Rubble <b@rubble.com> (http://barneyrubble.tumblr.com/)";
    let parsed = Person::from_str(person).unwrap();

    assert_eq!(parsed.name, "Barney Rubble");
    assert_eq!(parsed.email.unwrap(), "b@rubble.com");
    assert_eq!(
        parsed.url.unwrap(),
        url::Url::parse("http://barneyrubble.tumblr.com").unwrap()
    );
}

#[test]
fn parse_shorthand_person_reversed() {
    let person = "Barney Rubble (http://barneyrubble.tumblr.com/) <b@rubble.com>";
    let parsed = Person::from_str(person).unwrap();

    assert_eq!(parsed.name, "Barney Rubble");
    assert_eq!(parsed.email.unwrap(), "b@rubble.com");
    assert_eq!(
        parsed.url.unwrap(),
        url::Url::parse("http://barneyrubble.tumblr.com").unwrap()
    );
}

#[test]
fn parse_partial_person_url() {
    let person = "Barney Rubble (http://barneyrubble.tumblr.com/)";
    let parsed = Person::from_str(person).unwrap();

    assert_eq!(parsed.name, "Barney Rubble");
    assert!(parsed.email.is_none());
    assert_eq!(
        parsed.url.unwrap(),
        url::Url::parse("http://barneyrubble.tumblr.com").unwrap()
    );
}

#[test]
fn parse_partial_person_email() {
    let person = "Barney Rubble <b@rubble.com>";
    let parsed = Person::from_str(person).unwrap();

    assert_eq!(parsed.name, "Barney Rubble");
    assert_eq!(parsed.email.unwrap(), "b@rubble.com");
    assert!(parsed.url.is_none());
}

#[test]
fn parse_person_empty() {
    let person = "";
    let parsed = Person::from_str(person);

    assert!(parsed.is_err())
}

#[test]
fn parse_person_missing_name() {
    let person = "<b@rubble.com>";
    let parsed = Person::from_str(person);

    assert!(parsed.is_err())
}
