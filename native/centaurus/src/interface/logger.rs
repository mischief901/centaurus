//! The Nif side of the Logger.
use crate::log;
use super::api::{ Result };

#[rustler::nif]
pub fn logger() -> Result<()> {
    log::start()?;
    Ok(())
}

