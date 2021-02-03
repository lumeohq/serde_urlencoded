use matches::assert_matches;
use serde_derive::Serialize;
use serde_urlencoded::ser::Error;

#[derive(Serialize)]
struct NewType<T>(T);

#[test]
fn serialize_newtype_i32() {
    let params = &[("field", Some(NewType(11)))];
    assert_eq!(
        serde_urlencoded::to_string(params),
        Ok("field=11".to_owned())
    );
}

#[test]
fn serialize_option_map_int() {
    let params = &[("first", Some(23)), ("middle", None), ("last", Some(42))];

    assert_eq!(
        serde_urlencoded::to_string(params),
        Ok("first=23&last=42".to_owned())
    );
}

#[test]
fn serialize_option_map_string() {
    let params = &[
        ("first", Some("hello")),
        ("middle", None),
        ("last", Some("world")),
    ];

    assert_eq!(
        serde_urlencoded::to_string(params),
        Ok("first=hello&last=world".to_owned())
    );
}

#[test]
fn serialize_option_map_bool() {
    let params = &[("one", Some(true)), ("two", Some(false))];

    assert_eq!(
        serde_urlencoded::to_string(params),
        Ok("one=true&two=false".to_owned())
    );
}

#[test]
fn serialize_map_bool() {
    let params = &[("one", true), ("two", false)];

    assert_eq!(
        serde_urlencoded::to_string(params),
        Ok("one=true&two=false".to_owned())
    );
}

#[derive(Serialize)]
enum X {
    A,
    B,
    C,
}

#[test]
fn serialize_unit_enum() {
    let params = &[("one", X::A), ("two", X::B), ("three", X::C)];
    assert_eq!(
        serde_urlencoded::to_string(params),
        Ok("one=A&two=B&three=C".to_owned())
    );
}

#[derive(Serialize)]
struct Unit;

#[test]
fn serialize_unit_struct() {
    assert_eq!(serde_urlencoded::to_string(Unit), Ok("".to_owned()));
}

#[test]
fn serialize_unit_type() {
    assert_eq!(serde_urlencoded::to_string(()), Ok("".to_owned()));
}

#[test]
fn serialize_list_of_str() {
    let params = &[("list", vec!["hello", "world"])];

    assert_eq!(
        serde_urlencoded::to_string(params),
        Ok("list%5B%5D=hello&list%5B%5D=world".to_owned())
    );
}

#[test]
fn serialize_multiple_lists() {
    #[derive(Serialize)]
    struct Lists {
        xs: Vec<bool>,
        ys: Vec<u32>,
    }

    let params = Lists {
        xs: vec![true, false],
        ys: vec![3, 2, 1],
    };

    assert_eq!(
        serde_urlencoded::to_string(params),
        Ok(
            "xs%5B%5D=true&xs%5B%5D=false&ys%5B%5D=3&ys%5B%5D=2&ys%5B%5D=1"
                .to_owned()
        )
    );
}

#[test]
fn serialize_nested_list() {
    let params = &[("list", vec![vec![0u8]])];
    assert_matches!(
        serde_urlencoded::to_string(params),
        Err(Error::Custom(s)) if s.contains("unsupported")
    )
}

#[test]
fn serialize_list_of_option() {
    let params = &[("list", vec![Some(10), Some(100)])];
    assert_eq!(
        serde_urlencoded::to_string(params),
        Ok("list%5B%5D=10&list%5B%5D=100".to_owned())
    );
}

#[test]
fn serialize_list_of_newtype() {
    let params = &[("list", vec![NewType("test".to_owned())])];
    assert_eq!(
        serde_urlencoded::to_string(params),
        Ok("list%5B%5D=test".to_owned())
    );
}

#[test]
fn serialize_list_of_enum() {
    let params = &[("item", vec![X::A, X::B, X::C])];
    assert_eq!(
        serde_urlencoded::to_string(params),
        Ok("item%5B%5D=A&item%5B%5D=B&item%5B%5D=C".to_owned())
    );
}

#[test]
fn serialize_map() {
    let mut s = std::collections::BTreeMap::new();
    s.insert("hello", "world");
    s.insert("seri", "alize");
    s.insert("matrix", "ruma");

    let encoded = serde_urlencoded::to_string(s).unwrap();
    assert_eq!("hello=world&matrix=ruma&seri=alize", encoded);
}
