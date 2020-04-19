#![feature(try_trait)]
#![feature(async_closure)]

mod config;
mod conn;
mod error;
mod interface;
mod options;
mod runtime;
mod state;

#[macro_use]
extern crate rustler;
//#[macro_use]
extern crate tokio;
extern crate webpki;
