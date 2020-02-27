/// The set of options, defaults, and related checking functions.
use rustler_codegen::{ NifUntaggedEnum };


#[derive(NifUntaggedEnum)]
pub enum QuicOptions {
    Timeout(u64),
}
