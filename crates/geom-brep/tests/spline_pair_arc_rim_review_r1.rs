//! Review r1 (PR 5907): limb 3's refusal on an exact chart is pinned to
//! the cause the boundary-row tube gives, not to any tube refusal, so a
//! tube built about the wrong locus goes red.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::tol::band;
use geom::{Curve3, NurbsSurface};
use geom_brep::edge_nurbs::sub_arc_joint;
use geom_brep::ssi::OneArcRefusal;
use geom_brep::{PlaneNurbsRefusal, nurbs_pair_limbs};
use geom_core::spline::KnotVector;
use geom_core::{Point3, Vec3};

fn ring(turn: f64, profile: [(f64, f64); 2]) -> NurbsSurface<f64> {
    let m = 1;
    let h = turn / m as f64;
    let mut ku = vec![0.0; 3];
    for k in 1..m {
        ku.extend([sub_arc_joint(k, m); 2]);
    }
    ku.extend([1.0; 3]);
    let (mut control, mut weights) = (Vec::new(), Vec::new());
    for j in 0..=2 * m {
        let theta = j as f64 * h / 2.0;
        let (reach, w) = if j % 2 == 1 {
            (1.0 / (h / 2.0).cos(), (h / 2.0).cos())
        } else {
            (1.0, 1.0)
        };
        for (r, z) in profile {
            let r = r * reach;
            control.push(Point3::new(r * theta.cos(), r * theta.sin(), z));
            weights.push(w);
        }
    }
    NurbsSurface::new(
        KnotVector::clamped(ku, 2).unwrap(),
        KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
        control,
        weights,
    )
    .unwrap()
}

fn rim() -> Curve3<f64> {
    Curve3::Circle {
        center: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: 1.0,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

/// A short edge of the unit rim (its span √ε-scaled), clear of every `u` boundary, between a
/// cylinder and an annulus. With the rim inside both charts it certifies;
/// with it on the annulus' `v = 0` row (an exact chart) the tube, built
/// about the exact image's polyline, holds the rim on a face of every
/// box and its one-arc walk is UNDECIDED. A tube about any other locus
/// counts no solution instead.
#[test]
fn review_r1_an_exact_charts_tube_is_about_the_rim() {
    let turn = 1.2;
    // The fitted chart's chord miss falls as the span's square, so the
    // span shrinks as √ε to keep the interior chart clear of limb 2.
    let span = 0.03 * (crate::shared::tol::eps() / 1e-6).sqrt();
    let params = (0.5, 0.5 + span);
    let cyl_in = ring(turn, [(1.0, -1.0), (1.0, 1.0)]);
    let ann_in = ring(turn, [(0.5, 0.0), (2.0, 0.0)]);
    let ann_row = ring(turn, [(1.0, 0.0), (2.0, 0.0)]);
    let interior = nurbs_pair_limbs(&rim(), params, &cyl_in, &ann_in, 1.0, band());
    assert!(interior.is_ok(), "the rim inside both charts certifies: {interior:?}");
    for (what, a, b) in [
        ("cylinder × annulus row", &cyl_in, &ann_row),
        ("annulus row × cylinder", &ann_row, &cyl_in),
    ] {
        let got = nurbs_pair_limbs(&rim(), params, a, b, 1.0, band());
        assert!(
            matches!(
                got,
                Err(PlaneNurbsRefusal::TubeNotOneArc {
                    cause: OneArcRefusal::Undecided(_),
                    ..
                })
            ),
            "{what}: the tube about the exact image holds the rim on its face: {got:?}"
        );
    }
}
