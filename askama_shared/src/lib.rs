#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate nom;
extern crate quote;
extern crate syn;

#[cfg(feature = "serde-json")]
extern crate serde;
#[cfg(feature = "serde-json")]
extern crate serde_json;

pub use errors::{Error, Result};
pub mod filters;
pub mod path;
pub use parser::parse;
pub use generator::generate;

mod generator;
mod parser;

use std::fmt::{self, Display, Formatter};

pub enum MarkupDisplay<'a, T> where T: 'a + Display {
    Safe(&'a T),
    Unsafe(&'a T),
}

impl<'a, T> MarkupDisplay<'a, T> where T: 'a + Display {
    pub fn mark_safe(&mut self) {
        *self = match *self {
            MarkupDisplay::Unsafe(t) => MarkupDisplay::Safe(t),
            _ => { return; },
        }
    }
}

impl<'a, T> From<&'a T> for MarkupDisplay<'a, T> where T: 'a + Display {
    fn from(t: &'a T) -> MarkupDisplay<'a, T> {
        MarkupDisplay::Unsafe(t)
    }
}

impl<'a, T> From<usize> for MarkupDisplay<'a, T> where T: 'a + Display {
    fn from(t: usize) -> MarkupDisplay<'a, T> {
        MarkupDisplay::Unsafe(t)
    }
}

impl<'a, T> Display for MarkupDisplay<'a, T> where T: 'a + Display {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            MarkupDisplay::Unsafe(t) => {
                write!(f, "{}", filters::escape(&t).map_err(|_| std::fmt::Error)?)
            },
            MarkupDisplay::Safe(t) => {
                t.fmt(f)
            },
        }
    }
}

mod errors {
    error_chain! {
        foreign_links {
            Fmt(::std::fmt::Error);
            Json(::serde_json::Error) #[cfg(feature = "serde-json")];
        }
    }
}
