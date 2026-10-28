//! **A rim between two spline walls is metered against its exact image**
//! (`geom_brep::nurbs_pair_limbs`): a circle carrier that is a boundary
//! row of a chart reads limbs 1 and 2 on that chart through the
//! row's `IsoArc` image and its certificate, the plane lane's arc-rim
//! arm, and reads its fitted image only where no exact image certifies.
//!
//! The walls are rings of revolution about `z`: a cylinder and a flat
//! annulus meeting at right angles along their shared unit circle, the
//! `v = 0` row of each. Measured 2026-10-28 at ε 1e-9: the degree-1 image
//! fitted through the whole rim's feet misses it by 5.3e-5 m at a break,
//! where the exact image's certificate passes.
//!
//! An exact image is a boundary row, and limb 3's tube refuses a rim on a
//! chart's boundary row by name, [`OneArcRefusal::AlongSide`]
//! (`work/splice/a-two-chart-tube-cannot-prove-one-arc-along-a-chart-boundary.md`),
//! so no row here reaches `Ok` through an exact chart. The rows read
//! which limb refused: limb 3's refusal along the row says limbs 1 and 2
//! cleared on the exact image, and [`ArcRimRefusal::Refused`] that they
//! fell back to the fitted image and refused there.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::tol::band;
use geom::{Curve3, NurbsSurface};
use geom_brep::edge_nurbs::sub_arc_joint;
use geom_brep::ssi::OneArcRefusal;
use geom_brep::{
    ArcRimRefusal, PcurveCheck, PlaneNurbsLimbs, PlaneNurbsRefusal, SsiLimb, nurbs_pair_limbs,
};
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

/// Whether `got` refused at limb 3 because the rim lies along chart
/// `chart`'s boundary row, which only limbs 1 and 2 cleared reach.
fn tube_along_row(got: &Limbs, chart: u32) -> bool {
    matches!(
        got,
        Err(PlaneNurbsRefusal::TubeNotOneArc {
            cause: OneArcRefusal::AlongSide { chart: c },
            ..
        }) if *c == chart
    )
}

/// The exact image's refusing check and the fitted limb, where `got`
/// fell back to a chart's fitted image and refused there.
fn fell_back(got: &Limbs) -> Option<(Option<PcurveCheck>, SsiLimb)> {
    match got {
        Err(PlaneNurbsRefusal::ArcRim(ArcRimRefusal::Refused { iso, limb, .. })) => {
            Some((*iso, *limb))
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
            tube_along_row(&got, 0),
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
/// cylinder holds the rim as its row; the flat chart holds it inside, so
/// no candidate holds it there, and its fitted image's chord misses it
/// (1.8e-4 m at ε 1e-9, measured 2026-10-28). The lane refuses on that
/// chart's fitted limb, carrying that no image held, in both orders.
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
            matches!(fell_back(&got), Some((None, _))),
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
        matches!(fell_back(&got), Some((Some(_), _))),
        "the bent cylinder's fitted image refuses, past its image's refusal: {got:?}"
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
        e @ PlaneNurbsRefusal::ArcRim(ArcRimRefusal::Escalated {
            check: PcurveCheck::MapResidual,
            ..
        }),
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
        tube_along_row(&clear, 0),
        "a rim clear of the seam reads its row: {clear:?}"
    );
    let shifted = limbs(&carrier, (0.5 + tau, 1.5 + tau), &a, &b);
    assert!(
        tube_along_row(&shifted, 0),
        "and a turn on, the same row: {shifted:?}"
    );
    let across = limbs(&carrier, (-0.2, 0.4), &a, &b);
    assert!(
        matches!(fell_back(&across), Some((None, _))),
        "a rim across the seam reads no single arc of the row: {across:?}"
    );
}

/// **An exact chart's tube is built about the rim.** A short edge of the
/// unit rim, clear of every `u` boundary, its span `√ε`-scaled so the
/// fitted chart's chord miss, quadratic in the span, stays inside the
/// band. Inside both rings it certifies through the fitted images. With
/// it on the annulus' `v = 0` row, that chart reads its exact image and
/// the tube is built about its polyline: the rim lies on a face of every
/// box, and the tube refuses along the annulus' row, the chart it is in
/// each order. A tube about any other locus finds no solution in its
/// boxes and refuses `Count` instead.
#[test]
fn an_exact_charts_tube_is_about_the_rim() {
    let span = 0.03 * (crate::shared::tol::eps() / 1e-6).sqrt();
    let params = (0.5, 0.5 + span);
    let carrier = rim(1.0);
    let inside_cylinder = ring(1, TURN, [(1.0, -1.0), (1.0, 1.0)], |_| 1.0);
    let inside_annulus = ring(1, TURN, [(0.5, 0.0), (2.0, 0.0)], |_| 1.0);
    let interior = limbs(&carrier, params, &inside_cylinder, &inside_annulus);
    assert!(
        interior.is_ok(),
        "the rim inside both rings certifies: {interior:?}"
    );
    let row = annulus(1, TURN);
    for (what, a, b, chart) in [
        ("cylinder × annulus row", &inside_cylinder, &row, 1),
        ("annulus row × cylinder", &row, &inside_cylinder, 0),
    ] {
        let got = limbs(&carrier, params, a, b);
        assert!(
            tube_along_row(&got, chart),
            "{what}: the tube about the exact image holds the rim on its face: {got:?}"
        );
    }
}

/// **A rim ending on both charts' sides refuses by them.** The unit rim
/// inside both rings, each ring turning only the `√ε`-scaled span of
/// [`an_exact_charts_tube_is_about_the_rim`], whose interior rim
/// certifies: here the rim runs the whole turn, so each of its ends
/// lies on a `u` side of both charts at once, where the end box's walk
/// isolates no root. The tube names the two sides rather than reading
/// undecided.
#[test]
fn a_rim_ending_on_both_charts_sides_refuses_by_them() {
    let turn = 0.03 * (crate::shared::tol::eps() / 1e-6).sqrt();
    let inside_cylinder = ring(1, turn, [(1.0, -1.0), (1.0, 1.0)], |_| 1.0);
    let inside_annulus = ring(1, turn, [(0.5, 0.0), (2.0, 0.0)], |_| 1.0);
    let got = limbs(&rim(1.0), (0.0, turn), &inside_cylinder, &inside_annulus);
    assert!(
        matches!(
            got,
            Err(PlaneNurbsRefusal::TubeNotOneArc {
                cause: OneArcRefusal::EndOnSides { sides: 2 },
                ..
            })
        ),
        "each end lies on a u side of both charts: {got:?}"
    );
}
