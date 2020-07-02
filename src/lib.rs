//! The libazdice library exists to enable the rolling of fairly involved dice strings, as well
//! as for the making of fairly complex dice collections (here called `DiceBag`s).
//! It also facilitates their rolling and the construction of the resulting distributions.
//! The intened use is in TTRPGs as well as in rigorous, systematic testing of TTRPG game balance
//! by developers.
//!
//! NB: This dice roller was not made for raw speed. Rather it was made to be able to enable a wide
//! variety of rolls and make complete complete reports based of those rolls.
//!

// This is here purely to make clippy shut up, because we are not at home at coding convention and fashion changing
// every second release of rust.
#![allow(clippy::while_let_on_iterator)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::needless_range_loop)]

pub mod distribution;
pub mod externalise;
pub mod parse;
mod tests;

pub use parse::parse;
