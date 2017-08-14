//#[cfg(feature = "iron")]

#[macro_use]
extern crate askama;
extern crate iron;

use askama::Template;
use iron::{status, Response};
use iron::response::WriteBody;

#[derive(Template, WriteBody)]
#[template(path = "hello.html")]
struct HelloTemplate<'a> {
    name: &'a str,
}

#[test]
fn test_iron() {
    let rsp = Response::with((status::Ok, Box::new(HelloTemplate { name: "world" })));
}
