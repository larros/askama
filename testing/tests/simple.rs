#[macro_use]
extern crate askama;
#[macro_use]
extern crate serde_json;

use askama::Template;
use serde_json::Value;

#[derive(Template)]
#[template(path = "simple.html", print = "code")]
struct VariablesTemplate<'a> {
    strvar: &'a str,
    num: i64,
    i18n: String,
}

#[test]
fn test_variables() {
    let s = VariablesTemplate {
        strvar: "foo",
        num: 42,
        i18n: "Iñtërnâtiônàlizætiøn".to_string(),
    };
    assert_eq!(s.render().unwrap(), "\nhello world, foo\n\
                                     with number: 42\n\
                                     Iñtërnâtiônàlizætiøn is important\n\
                                     in vars too: Iñtërnâtiônàlizætiøn");
}


#[derive(Template)]
#[template(path = "if.html")]
struct IfTemplate {
    cond: bool,
}

#[test]
fn test_if() {
    let s = IfTemplate { cond: true };
    assert_eq!(s.render().unwrap(), "true");
}


#[derive(Template)]
#[template(path = "else.html")]
struct ElseTemplate {
    cond: bool,
}

#[test]
fn test_else() {
    let s = ElseTemplate { cond: false };
    assert_eq!(s.render().unwrap(), "false");
}


#[derive(Template)]
#[template(path = "else-if.html")]
struct ElseIfTemplate {
    cond: bool,
    check: bool,
}

#[test]
fn test_else_if() {
    let s = ElseIfTemplate { cond: false, check: true };
    assert_eq!(s.render().unwrap(), "checked");
}


#[derive(Template)]
#[template(path = "literals.html")]
struct LiteralsTemplate {}

#[test]
fn test_literals() {
    let s = LiteralsTemplate {};
    assert_eq!(s.render().unwrap(), "a");
}


struct Holder {
    a: usize,
}

#[derive(Template)]
#[template(path = "attr.html")]
struct AttrTemplate {
    inner: Holder,
}

#[test]
fn test_attr() {
    let t = AttrTemplate { inner: Holder { a: 5 } };
    assert_eq!(t.render().unwrap(), "5");
}


struct NestedHolder {
    holder: Holder,
}

#[derive(Template)]
#[template(path = "nested-attr.html")]
struct NestedAttrTemplate {
    inner: NestedHolder,
}

#[test]
fn test_nested_attr() {
    let t = NestedAttrTemplate {
        inner: NestedHolder { holder: Holder { a: 5 } }
    };
    assert_eq!(t.render().unwrap(), "5");
}


#[derive(Template)]
#[template(path = "option.html")]
struct OptionTemplate<'a> {
    var: Option<&'a str>,
}

#[test]
fn test_option() {
    let some = OptionTemplate { var: Some("foo") };
    assert_eq!(some.render().unwrap(), "some: foo");
    let none = OptionTemplate { var: None };
    assert_eq!(none.render().unwrap(), "none");
}


#[derive(Template)]
#[template(path = "generics.html")]
struct GenericsTemplate<T: std::fmt::Display, U = u8>
    where U: std::fmt::Display {
    t: T,
    u: U,
}

#[test]
fn test_generics() {
    let t = GenericsTemplate { t: "a", u: 42 };
    assert_eq!(t.render().unwrap(), "a42");
}


#[derive(Template)]
#[template(path = "json.html")]
struct JsonTemplate<'a> {
    foo: &'a str,
    bar: &'a Value,
}

#[test]
fn test_json() {
    let val =  json!({"arr": [ "one", 2, true, null ]});
    let t = JsonTemplate { foo: "a", bar: &val };
    // Note: the json filter lacks a way to specify initial indentation
    assert_eq!(t.render().unwrap(), r#"{
  "foo": "a",
  "bar": {
  "arr": [
    "one",
    2,
    true,
    null
  ]
}
}"#);
}

#[derive(Template)]
#[template(path = "composition.html")]
struct CompositionTemplate {
    foo: IfTemplate,
}

#[test]
fn test_composition() {
    let t = CompositionTemplate { foo: IfTemplate { cond: true } };
    assert_eq!(t.render().unwrap(), "composed: true");
}
