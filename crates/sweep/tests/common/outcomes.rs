//! **A boolean's outcome as one comparable string**: the body's whole
//! `Debug` when it builds, `Empty`, or the refusal's `Debug`. Two runs
//! of one scene that a suite claims are one outcome (an undeclared run
//! against its declared twin, D10) compare equal here. What a suite
//! reads OFF a result it built, so it routes here beside
//! [`super::bitdump`] ([`super`]'s routing rule).
//!
//! The body is read alone, not the whole `BooleanBody`: the naming and
//! contact records list a coincidence in the order the run met it, and
//! a declared pair is met before one the boolean found itself.
//!
//! **Deliberately not absorbed**, and the whole of it:
//! [`super::bitdump`], which writes a body out as a file for a
//! base/head diff rather than comparing two runs in one; and
//! [`super::germ_pair::same_door`], which compares two refusals of one
//! configuration in two poses, whose margins agree within the zero band
//! rather than bit for bit.

use topo::{BooleanError, BooleanResult};

/// The outcome of a boolean: `Body(<the body's Debug>)`, `Empty`, or
/// `Err(<the refusal's Debug>)`.
pub fn outcome(r: &Result<BooleanResult<f64>, BooleanError>) -> String {
    match r {
        Ok(BooleanResult::Body(bb)) => format!("Body({:?})", bb.body),
        Ok(BooleanResult::Empty) => "Empty".to_owned(),
        Err(e) => format!("Err({e:?})"),
    }
}
