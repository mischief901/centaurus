/// The set of options, defaults, and related checking functions.
use rustler::{ NifUntaggedEnum };


#[derive(NifUntaggedEnum)]
pub enum QuicOptions {
    Timeout(u64),
}
