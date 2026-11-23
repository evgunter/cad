//! Review probe: production reads of a conic's centre that clippy and
//! `scripts/gates/conic-centre-reads.sh` both pass.
#![allow(dead_code)]

use geom_core::Point3;

use crate::{Circle, Conic, Ellipse};

pub fn f07_circle_placed(c: &Circle<f64>) -> Point3<f64> {
    let mut out = Point3::origin();
    let _ = c.placed(|f| {
        out = f.origin();
        *f
    });
    out
}
#[allow(clippy::style)]
pub fn f09_group_allow(e: &Ellipse<f64>) -> Point3<f64> {
    Conic::frame(e).origin()
}
#[expect(clippy::style, reason = "a group expectation names no lint")]
pub fn f14_group_expect(e: &Ellipse<f64>) -> Point3<f64> {
    e.center()
}
pub fn f12_unit_in_plane(e: &Ellipse<f64>, p: Point3<f64>) -> (f64, f64) {
    e.unit_in_plane(p)
}
