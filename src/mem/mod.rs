mod findpattern;
mod module;
mod process;
mod snapshot;
pub(crate) mod csgo;

pub use crate::mem::findpattern::*;
pub use crate::mem::module::*;
pub use crate::mem::process::*;
pub use crate::mem::snapshot::*;

pub trait Constructor {
    fn new() -> Self;
}
