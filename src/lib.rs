#![feature(box_syntax, box_patterns)]
#![feature(try_from)]

extern crate arccstr;
extern crate distributary;
extern crate msql_srv;
extern crate nom_sql;
extern crate regex;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate slog;

mod convert;
mod schema;
mod soup_backend;
mod utils;

pub use soup_backend::SoupBackend;