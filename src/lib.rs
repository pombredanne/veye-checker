
extern crate sha2;
extern crate base64;
extern crate sha1;
extern crate md5;
extern crate hyper;
//extern crate hyper_rustls;
extern crate hyper_native_tls;
extern crate rustc_serialize; //TODO: replace it with serde
extern crate csv;
extern crate regex;
extern crate serde;
extern crate toml;
extern crate walkdir;

#[macro_use]
extern crate serde_derive;


pub mod api;
pub mod product;
pub mod checker;
pub mod configs;

pub mod tasks;