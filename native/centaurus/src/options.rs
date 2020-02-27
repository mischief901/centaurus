/// The set of options, defaults, and related checking functions.

use serde::{ Serialize, Deserialize };

#[derive(Debug, Serialize, Deserialize)]
pub struct QuicOptions {
    timeout: u64,
}
