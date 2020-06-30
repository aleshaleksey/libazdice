//! The libazdice library exists to enable the rolling of fairly involved dice strings, as well
//! as for the making of fairly complex dice collections (here called `DiceBag`s).
//! It also facilitates their rolling and the construction of the resulting distributions.
//! The intened use is in TTRPGs as well as in rigorous, systematic testing of TTRPG game balance
//! by developers.
pub mod parse;
pub mod distribution;
pub mod externalise;
mod tests;

pub use parse::parse;
