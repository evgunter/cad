//! Review probe (delta): production reads of a conic's centre that
//! clippy -D warnings and conic-centre-reads.sh both pass.
#![allow(dead_code)]

use geom_core::Point3;

use crate::{Conic, Ellipse};

#[allow(clippy :: style)]
pub(crate) fn g1_spaced(e: &Ellipse<f64>) -> Point3<f64> {
    Conic::frame(e).origin()
}
#[allow(clippy::
    style)]
pub(crate) fn g2_split_group(e: &Ellipse<f64>) -> Point3<f64> {
    e.center()
}
#[allow(clippy
    ::disallowed_methods)]
pub(crate) fn g3_split_lint(e: &Ellipse<f64>) -> Point3<f64> {
    Conic::frame(e).origin()
}
#[path = "../../../tools/review/outside.rs"]
mod outside;
#[allow(renamed_and_removed_lints, clippy::disallowed_method)]
pub(crate) fn g4_renamed(e: &Ellipse<f64>) -> Point3<f64> {
    Conic::frame(e).origin()
}
