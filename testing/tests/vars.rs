#[macro_use]
extern crate askama;

use askama::Template;

#[derive(Template)]
#[template(source = "{% let (a, b) = s %}{{ a }}{{ b }}{{ s.0 }}{{ s.1}}", ext = "txt")]
struct LetTemplate<'a> {
    s: (&'a str, &'a str),
}

#[test]
fn test_let() {
    let t = LetTemplate { s: ("foo", "bar") };
    assert_eq!(t.render().unwrap(), "foobarfoobar");
}


#[derive(Template)]
#[template(path = "let-decl.html")]
struct LetDeclTemplate<'a> {
    cond: bool,
    s: &'a str,
}

#[test]
fn test_let_decl() {
    let t = LetDeclTemplate { cond: false, s: "bar" };
    assert_eq!(t.render().unwrap(), "bar");
}
