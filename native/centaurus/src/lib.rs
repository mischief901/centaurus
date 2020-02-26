
#![feature(try_trait)]

mod conn;
mod net;

use conn::Connection;
use net::{ Testing, Rustler };

use std::{
    net::SocketAddr,
};

fn test -> Connection<Testing> {
    unimplemented!();
}

fn new(addr : SocketAddr) -> Connection<> {
    

}

    

