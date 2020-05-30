/// The set of options, defaults, and related checking functions.
//use rustler::{ NifUntaggedEnum };

#[derive(Debug)]
#[derive(NifStruct)]
#[module = "Centaurus.Types.Options"]
#[rustler(encode, decode)]
pub struct QuicOptions {
    pub timeout: Option<u64>,
}
