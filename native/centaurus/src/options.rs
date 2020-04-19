/// The set of options, defaults, and related checking functions.
use rustler::{ NifUntaggedEnum };


#[derive(NifUntaggedEnum)]
#[derive(Debug)]
pub enum QuicOptions {
    Timeout(u64),
}
