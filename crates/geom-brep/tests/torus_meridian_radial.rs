//! **A torus meridian centred on the torus axis has no radial**, and
//! every torus consumer refuses it under the decision that finds that
//! out, `props_meridian_radial`.
//!
//! The torus's major radius sits inside the zero band, so a meridian
//! centred on (or a tenth of ε off) the axis passes
//! `props_meridian_fit`; what is left to say no is the anchor
//! meridian's chart frame, whose radial decides to zero length. The
//! exact-zero row reaches the orientation classify with a poisoned
//! tangent unless the radial is decided first; the off-axis row has a
//! radial the band calls zero, which a bare normalize turns into a
//! definite direction.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::tol::{band, eps};
use core::f64::consts::FRAC_PI_2;
use geom::{Curve3, Surface};
use geom_brep::props::boundary_material_sign;
use geom_brep::{LoopEdge, PropsError, curved_face, require_iso_rectangle};
use geom_core::{Point3, Vec3};

/// A torus about `z` with major radius `off` and minor radius 1, and a
/// one-edge loop on its meridian in the `xz` plane centred at
/// `(off, 0, 0)` — on the major circle, so the meridian fits.
fn fixture(off: f64) -> (Surface<f64>, Vec<LoopEdge<f64>>) {
    let torus = Surface::Torus {
        center: Point3::origin(),
        axis: Vec3::unit_z(),
        major_radius: off,
        minor_radius: 1.0,
        u_ref: Vec3::unit_x(),
    };
    let meridian = Curve3::Circle {
        center: Point3::new(off, 0.0, 0.0),
        axis: Vec3::unit_y(),
        radius: 1.0,
        u_ref: Vec3::unit_x(),
    };
    let edge = LoopEdge::hand_built(meridian, 0.0, FRAC_PI_2, true, 0, 1);
    (torus, vec![edge])
}

#[test]
fn a_meridian_on_the_torus_axis_refuses_as_having_no_radial() {
    let want = PropsError::NotIsoRectangle {
        what: "props_meridian_radial",
    };
    for off in [0.0, 0.1 * eps()] {
        let (torus, outer) = fixture(off);
        assert_eq!(
            curved_face(&torus, &outer, true, band()).err(),
            Some(want.clone()),
            "curved_face, meridian centre {off:e} off the axis"
        );
        assert_eq!(
            boundary_material_sign(&torus, &outer, band()).err(),
            Some(want.clone()),
            "boundary_material_sign, meridian centre {off:e} off the axis"
        );
        assert_eq!(
            require_iso_rectangle(&torus, &outer, band()).err(),
            Some(want.clone()),
            "require_iso_rectangle, meridian centre {off:e} off the axis"
        );
    }
}
