//! **A rim between two spline walls is metered against its exact image**
//! (`geom_brep::nurbs_pair_limbs`): a circle carrier that is a boundary
//! row of a chart reads limbs 1 and 2 on that chart through the
//! row's `IsoArc` image and its certificate, the plane lane's arc-rim
//! arm, and reads its fitted image only where no exact image certifies.
//!
//! The walls are rings of revolution about `z`: a cylinder and a flat
//! annulus meeting at right angles along their shared unit circle, the
//! `v = 0` row of each. The degree-1 image fitted through that rim's
//! feet misses it by 5.3e-5 m at a break, past the band at every
//! ε; the exact image's envelope is a few ulps.
//!
//! Every row here reaches limb 3 or refuses before it, and the tube
//! refuses a rim on both charts' boundary
//! (`work/ssiedge/a-two-chart-tube-cannot-prove-one-arc-along-a-chart-boundary.md`),
//! so the rows read which limb refused: a fitted limb 1 or 2 is the
//! fitted image's verdict, limb 3 says limbs 1 and 2 cleared.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::tol::band;
use geom::{Curve3, NurbsSurface};
use geom_brep::edge_nurbs::sub_arc_joint;
use geom_brep::{PcurveCheck, PlaneNurbsLimbs, PlaneNurbsRefusal, SsiLimb, nurbs_pair_limbs};
use geom_core::spline::KnotVector;
use geom_core::{Point3, Vec3};

type Limbs = Result<PlaneNurbsLimbs<f64>, PlaneNurbsRefusal>;

/// The rim's turn on the open rings.
const TURN: f64 = 1.2;

/// A ring of revolution about `z` through `turn` from `x̂`, in `m`
/// rational quadratic sub-arcs (`u`), its `v` rows the profile points
/// `(radius, height)` of `profile`, joined linearly. `bulge(j)` scales
/// the `j`-th `u` control point off the axis: `1` for a true ring.
fn ring(
    m: usize,
    turn: f64,
    profile: [(f64, f64); 2],
    bulge: impl Fn(usize) -> f64,
) -> NurbsSurface<f64> {
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
            let r = r * reach * bulge(j);
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

/// The unit cylinder `z ∈ [0, 1]`: its `v = 0` row is the rim.
fn cylinder(m: usize, turn: f64) -> NurbsSurface<f64> {
    ring(m, turn, [(1.0, 0.0), (1.0, 1.0)], |_| 1.0)
}

/// The annulus `r ∈ [1, 2]` in `z = 0`: its `v = 0` row is the rim.
fn annulus(m: usize, turn: f64) -> NurbsSurface<f64> {
    ring(m, turn, [(1.0, 0.0), (2.0, 0.0)], |_| 1.0)
}

/// The circle of radius `r` about `z` in `z = 0`, `t` its angle from `x̂`.
fn rim(r: f64) -> Curve3<f64> {
    Curve3::Circle {
        center: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: r,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

fn limbs(
    carrier: &Curve3<f64>,
    params: (f64, f64),
    a: &NurbsSurface<f64>,
    b: &NurbsSurface<f64>,
) -> Limbs {
    nurbs_pair_limbs(carrier, params, a, b, 1.0, band())
}

/// Whether `got` cleared limbs 1 and 2: it certified, or refused at the
/// tube.
fn past_incidence(got: &Limbs) -> bool {
    matches!(
        got,
        Ok(_)
            | Err(PlaneNurbsRefusal::TubeNotOneArc { .. } | PlaneNurbsRefusal::TubeStraddles { .. })
    )
}

/// The refusing limb of a fitted limb-1 or limb-2 refusal, definite or
/// undecided.
fn fitted_limb(got: &Limbs) -> Option<SsiLimb> {
    match got {
        Err(PlaneNurbsRefusal::Limb { limb, .. } | PlaneNurbsRefusal::Escalated { limb, .. })
            if *limb != SsiLimb::Tube =>
        {
            Some(*limb)
        }
        _ => None,
    }
}

/// **The rim reads its exact image on both charts.** Each wall's `v = 0`
/// row is the rim; its fitted image misses it past the band, which is
/// where the lane refused before it read the row. It now clears limbs 1
/// and 2 in both operand orders.
#[test]
fn a_rim_on_both_walls_rows_reads_past_the_fitted_images_miss() {
    let (carrier, params) = (rim(1.0), (0.0, TURN));
    let (a, b) = (cylinder(1, TURN), annulus(1, TURN));
    for (what, a, b) in [
        ("cylinder × annulus", &a, &b),
        ("annulus × cylinder", &b, &a),
    ] {
        let got = limbs(&carrier, params, a, b);
        assert!(
            past_incidence(&got),
            "{what}: limbs 1 and 2 clear on the exact images: {got:?}"
        );
    }
}

/// The bilinear chart `z = h` over `[-0.5, 2]²`, which holds the rim in
/// its interior and offers no exact image of it.
fn flat(h: f64) -> NurbsSurface<f64> {
    let k = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let p = |x, y| Point3::new(x, y, h);
    NurbsSurface::new(
        k.clone(),
        k,
        vec![p(-0.5, -0.5), p(-0.5, 2.0), p(2.0, -0.5), p(2.0, 2.0)],
        vec![1.0; 4],
    )
    .unwrap()
}

/// **A chart with no exact image falls back to its fitted image.** The
/// cylinder holds the rim as its row; the flat chart holds it inside,
/// where the fitted image's chord misses it by `≈ 1.8e-4` m. The lane
/// refuses on that chart's fitted limb, in both orders, whichever side
/// reads exactly.
#[test]
fn the_chart_with_no_exact_image_reads_its_fitted_one() {
    let (carrier, params) = (rim(1.0), (0.0, TURN));
    let (wall, plane) = (cylinder(1, TURN), flat(0.0));
    for (what, a, b) in [
        ("cylinder × flat", &wall, &plane),
        ("flat × cylinder", &plane, &wall),
    ] {
        let got = limbs(&carrier, params, a, b);
        assert!(
            fitted_limb(&got).is_some(),
            "{what}: the flat chart's fitted image refuses: {got:?}"
        );
    }
}

/// **An image the probe holds but the certificate refuses is not read.**
/// The cylinder in two sub-arcs with the second's middle control point
/// pushed `1e-3` off the axis: the probe's three points (the ends and a
/// quarter in, in the first sub-arc) lie on the row, but the row is no
/// circle over its second sub-arc, which the certificate reads. The
/// carrier is off the wall there, so the fitted path refuses on the
/// cylinder.
#[test]
fn an_image_its_certificate_refuses_is_not_read() {
    let (carrier, params) = (rim(1.0), (0.0, TURN));
    let bent = ring(2, TURN, [(1.0, 0.0), (1.0, 1.0)], |j| {
        if j == 3 { 1.0 + 1e-3 } else { 1.0 }
    });
    let probe = geom_brep::pick_arc_rim(
        &carrier,
        (0.0, TURN),
        &geom::Surface::Nurbs(bent.clone().into()),
        band(),
    )
    .expect("the pick decides")
    .expect("the probe holds the bent row");
    assert!(
        geom_brep::PcurveCache::certify(
            probe,
            0.0,
            TURN,
            &carrier,
            &geom::Surface::Nurbs(bent.clone().into()),
            band()
        )
        .is_err(),
        "the certificate refuses the bent row"
    );
    let got = limbs(&carrier, params, &bent, &annulus(2, TURN));
    assert!(
        fitted_limb(&got).is_some(),
        "the bent cylinder's fitted image refuses: {got:?}"
    );
}

/// **An exact image that escalates escalates the rim.** The rim moved
/// into the band off both walls' rows: the pick's probe on the first
/// chart is undecided, and the fitted image does not make it definite.
#[test]
fn an_exact_image_that_escalates_escalates() {
    let b = band();
    let carrier = rim(1.0 + 0.5 * (b.zero() + b.escalate()));
    let got = limbs(&carrier, (0.0, TURN), &cylinder(1, TURN), &annulus(1, TURN));
    let Err(
        e @ PlaneNurbsRefusal::ArcRimEscalated {
            check: PcurveCheck::MapResidual,
            ..
        },
    ) = &got
    else {
        panic!("the pick escalates at the cylinder's row: {got:?}");
    };
    assert!(
        matches!(
            e.decision(),
            Some((
                geom_brep::CertCheck::PlaneNurbsHullValue,
                geom_brep::recourse::RefusedArm::Undecided(_)
            ))
        ),
        "the escalation reads as the exact image's map residual: {:?}",
        e.decision()
    );
}

/// **On a closed ring, the image is the row's arc the edge lies on.**
/// The whole turn in four sub-arcs, the `u` boundaries one ruling. A rim
/// clear of the seam reads its row over the branch the edge lies on, and
/// clears limbs 1 and 2; one across the seam lies on no single arc of
/// the row, so no image holds it, and the fitted image, whose feet jump
/// across the chart at the seam, refuses.
#[test]
fn a_closed_ring_reads_the_arc_its_rim_lies_on() {
    let tau = core::f64::consts::TAU;
    let (a, b) = (cylinder(4, tau), annulus(4, tau));
    let carrier = rim(1.0);
    let clear = limbs(&carrier, (0.5, 1.5), &a, &b);
    assert!(
        past_incidence(&clear),
        "a rim clear of the seam reads its row: {clear:?}"
    );
    let shifted = limbs(&carrier, (0.5 + tau, 1.5 + tau), &a, &b);
    assert!(
        past_incidence(&shifted),
        "and a turn on, the same row: {shifted:?}"
    );
    let across = limbs(&carrier, (-0.2, 0.4), &a, &b);
    assert!(
        !past_incidence(&across),
        "a rim across the seam reads no single arc of the row: {across:?}"
    );
}
