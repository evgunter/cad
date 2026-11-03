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
//! cleared on the exact image, and [`ArcRimRefusal`] that they
//! fell back to the fitted image and refused there. An exact image read
//! in band takes the fitted image as a refused one does, and its fitted
//! limb's refusal stands as itself.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::tol::band;
use geom::{Curve3, NurbsSurface};
use geom_brep::Pcurve;
use geom_brep::edge_nurbs::sub_arc_joint;
use geom_brep::ssi::OneArcRefusal;
use geom_brep::{
    ArcRimRefusal, PcurveCheck, PlaneNurbsLimbs, PlaneNurbsRefusal, SsiLimb, nurbs_pair_limbs,
};
use geom_core::spline::KnotVector;
use geom_core::test_support::fixture_lever;
use geom_core::{Bounds, Decide, Point3, Vec3};

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
    Curve3::circle(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        r,
        Vec3::new(1.0, 0.0, 0.0),
    )
}

fn limbs(
    carrier: &Curve3<f64>,
    params: (f64, f64),
    a: &NurbsSurface<f64>,
    b: &NurbsSurface<f64>,
) -> Limbs {
    nurbs_pair_limbs(carrier, params, a, b, fixture_lever(1.0), band())
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
        Err(PlaneNurbsRefusal::ArcRim(ArcRimRefusal { iso, limb, .. })) => Some((*iso, *limb)),
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

/// A distance inside the band: past `zero()`, short of `escalate()`.
fn in_band() -> f64 {
    let b = band();
    0.5 * (b.zero() + b.escalate())
}

/// The pair lane at `T`, every fixture lifted.
fn limbs_at<T: Decide + Bounds + geom_core::CertifiedEnclosure>(
    carrier: &Curve3<f64>,
    params: (f64, f64),
    a: &NurbsSurface<f64>,
    b: &NurbsSurface<f64>,
) -> Result<PlaneNurbsLimbs<T>, PlaneNurbsRefusal> {
    nurbs_pair_limbs(
        &carrier.map_scalar(T::from_f64),
        (T::from_f64(params.0), T::from_f64(params.1)),
        &a.map_scalar(T::from_f64),
        &b.map_scalar(T::from_f64),
        fixture_lever(T::one()),
        band(),
    )
}

/// The arc-rim pick at `T` on `chart`, every fixture lifted.
fn pick_at<T: Decide>(
    carrier: &Curve3<f64>,
    (t0, span): (f64, f64),
    chart: &NurbsSurface<f64>,
) -> Result<Option<Pcurve<T>>, geom_core::Indeterminate> {
    geom_brep::pick_arc_rim(
        &carrier.map_scalar(T::from_f64),
        (T::from_f64(t0), T::from_f64(span)),
        &geom::Surface::Nurbs(chart.map_scalar(T::from_f64).into()),
        band(),
    )
}

/// **A pick read in band takes the fitted image, recording nothing.**
/// The rim moved into the band off both walls' rows: the pick's probe is
/// undecided on each chart, so each chart reads its fitted image, as a
/// definite miss does (D3: a lane choice in band takes the general
/// lane), recording nothing: the escalation is the fitted image's own
/// limb 1, which reads the rim's distance from the cylinder in band,
/// not the exact image's.
fn an_in_band_pick_reads_the_fitted_image<T: Decide + Bounds + geom_core::CertifiedEnclosure>() {
    let carrier = rim(1.0 + in_band());
    for chart in [cylinder(1, TURN), annulus(1, TURN)] {
        assert!(
            pick_at::<T>(&carrier, (0.0, TURN), &chart).is_err(),
            "the pick is undecided on each wall's row, at {}",
            T::NAME
        );
    }
    let got = limbs_at::<T>(&carrier, (0.0, TURN), &cylinder(1, TURN), &annulus(1, TURN));
    assert!(
        matches!(
            got,
            Err(PlaneNurbsRefusal::Escalated {
                limb: SsiLimb::OnLocus,
                ..
            })
        ),
        "the fitted image's limb 1 reads the rim in band, at {}: {:?}",
        T::NAME,
        got.as_ref().map(|_| ())
    );
}

#[test]
fn an_in_band_pick_reads_the_fitted_image_at_f64() {
    an_in_band_pick_reads_the_fitted_image::<f64>();
}

#[test]
fn an_in_band_pick_reads_the_fitted_image_at_interval() {
    an_in_band_pick_reads_the_fitted_image::<geom_core::Interval>();
}

/// The cylinder in two sub-arcs with the second's middle control point
/// pushed off the axis by `push`: the row parts from the circle by
/// `push / (1 + cos(h/2))` at that sub-arc's middle, and the probe's
/// points (the ends and a quarter in) stay on it.
fn bent(push: f64) -> NurbsSurface<f64> {
    ring(2, TURN, [(1.0, 0.0), (1.0, 1.0)], |j| {
        if j == 3 { 1.0 + push } else { 1.0 }
    })
}

/// The push that parts [`bent`]'s row from the circle by `in_band()`.
fn in_band_push() -> f64 {
    in_band() * (1.0 + (TURN / 4.0).cos())
}

/// **A certificate read in band takes the fitted image, recording
/// nothing.** The cylinder's row bent inside the band over its second
/// sub-arc: the pick shows the row, and its certificate reads the part
/// in band. The chart reads its fitted image, as a definite refusal
/// does, recording nothing: the escalation is the fitted image's own
/// limb 1, which reads the carrier's distance from the bent wall in
/// band, not the exact image's.
fn an_in_band_certificate_reads_the_fitted_image<
    T: Decide + Bounds + geom_core::CertifiedEnclosure,
>() {
    let carrier = rim(1.0);
    let wall = bent(in_band_push());
    let image = pick_at::<T>(&carrier, (0.0, TURN), &wall)
        .expect("the pick decides")
        .expect("the probe shows the bent row");
    let certified = geom_brep::PcurveCache::certify(
        image,
        T::zero(),
        T::from_f64(TURN),
        &carrier.map_scalar(T::from_f64),
        &geom::Surface::Nurbs(wall.map_scalar(T::from_f64).into()),
        band(),
    );
    assert!(
        matches!(
            certified,
            Err(geom_brep::PcurveCertifyError::Escalated { .. })
        ),
        "the row's certificate reads the bend in band, at {}: {:?}",
        T::NAME,
        certified.as_ref().map(|_| ())
    );
    let got = limbs_at::<T>(&carrier, (0.0, TURN), &wall, &annulus(2, TURN));
    assert!(
        matches!(
            got,
            Err(PlaneNurbsRefusal::Escalated {
                limb: SsiLimb::OnLocus,
                ..
            })
        ),
        "the fitted image's limb 1 reads the carrier in band, at {}: {:?}",
        T::NAME,
        got.as_ref().map(|_| ())
    );
}

#[test]
fn an_in_band_certificate_reads_the_fitted_image_at_f64() {
    an_in_band_certificate_reads_the_fitted_image::<f64>();
}

#[test]
fn an_in_band_certificate_reads_the_fitted_image_at_interval() {
    an_in_band_certificate_reads_the_fitted_image::<geom_core::Interval>();
}

/// **An in-band closed form does not stop the measured step.** A rim
/// short of the row's whole arc by an angle inside the band: each
/// closed-form image, which maps the edge onto the whole row, misses it
/// in band at its end, and the row's own arc holds it. The pick takes
/// the measured image, over the row's whole turn.
fn an_in_band_closed_form_gives_way_to_the_measured_row<T: Decide + Bounds>() {
    let carrier = rim(1.0);
    let span = TURN - in_band();
    let chart = cylinder(1, TURN);
    let closed = geom_brep::probe_pick(
        "pcurve_iso_arc_row_direction",
        &carrier.map_scalar(T::from_f64),
        (T::zero(), T::from_f64(span)),
        &geom::Surface::Nurbs(chart.map_scalar(T::from_f64).into()),
        band(),
        geom_brep::arc_rim_candidates(&chart.map_scalar(T::from_f64), T::zero(), T::from_f64(span)),
    );
    assert!(
        closed.is_err(),
        "the closed-form step alone is undecided, at {}",
        T::NAME
    );
    let got = pick_at::<T>(&carrier, (0.0, span), &chart);
    let Ok(Some(Pcurve::IsoArc { angle, .. })) = &got else {
        panic!("the measured row is picked, at {}: {got:?}", T::NAME)
    };
    assert!(
        (Bounds::lo(*angle) - TURN).abs() < 1e-12,
        "the row's own turn, at {}: {angle:?}",
        T::NAME
    );
}

#[test]
fn an_in_band_closed_form_gives_way_to_the_measured_row_at_f64() {
    an_in_band_closed_form_gives_way_to_the_measured_row::<f64>();
}

#[test]
fn an_in_band_closed_form_gives_way_to_the_measured_row_at_interval() {
    an_in_band_closed_form_gives_way_to_the_measured_row::<geom_core::Interval>();
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

/// The unit circle about `z` at height `z`, `t` its angle from `x̂`: on
/// the cylinder's wall wherever `0 ≤ z ≤ 1`.
fn rim_at(z: f64) -> Curve3<f64> {
    Curve3::circle(
        Point3::new(0.0, 0.0, z),
        Vec3::new(0.0, 0.0, 1.0),
        1.0,
        Vec3::new(1.0, 0.0, 0.0),
    )
}

/// The plane lane at `T` against the cylinder: the plane `z = h`, every
/// fixture lifted.
fn plane_lane_at<T: Decide + Bounds + geom_core::CertifiedEnclosure>(
    carrier: &Curve3<f64>,
    (t0, t1): (f64, f64),
    h: f64,
) -> Result<geom_brep::PlaneNurbsIncidence<T>, PlaneNurbsRefusal> {
    let plane = geom::Surface::Plane {
        origin: Point3::new(0.0, 0.0, h),
        normal: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    geom_brep::plane_nurbs_incidence(
        &carrier.map_scalar(T::from_f64),
        (T::from_f64(t0), T::from_f64(t1)),
        &plane.map_scalar(T::from_f64),
        &geom::Surface::Nurbs(cylinder(1, TURN).map_scalar(T::from_f64).into()),
        None,
        band(),
    )
}

/// **A fitted refusal after an in-band exact image stands as itself.**
/// The circle on the cylinder's wall `in_band()` above its `v = 0` row,
/// cut by the plane at that height: the pick reads every row in band,
/// so the wall reads its fitted image, which holds the circle (limb 1)
/// and misses its arc between feet past the band (limb 2). That limb's
/// refusal is the wall's whole answer: no exact image refused, so none
/// is named.
fn a_fitted_refusal_after_an_in_band_exact_image_stands_as_itself<
    T: Decide + Bounds + geom_core::CertifiedEnclosure,
>() {
    let carrier = rim_at(in_band());
    assert!(
        pick_at::<T>(&carrier, (0.0, TURN), &cylinder(1, TURN)).is_err(),
        "the pick reads the row in band, at {}",
        T::NAME
    );
    let got = plane_lane_at::<T>(&carrier, (0.0, TURN), in_band());
    assert!(
        matches!(got, Err(PlaneNurbsRefusal::Limb { .. })),
        "the fitted limb's refusal stands as itself, at {}: {:?}",
        T::NAME,
        got.as_ref().map(|_| ())
    );
}

#[test]
fn a_fitted_refusal_after_an_in_band_exact_image_stands_as_itself_at_f64() {
    a_fitted_refusal_after_an_in_band_exact_image_stands_as_itself::<f64>();
}

#[test]
fn a_fitted_refusal_after_an_in_band_exact_image_stands_as_itself_at_interval() {
    a_fitted_refusal_after_an_in_band_exact_image_stands_as_itself::<geom_core::Interval>();
}

/// **An in-band exact image gives way to a fitted image that
/// certifies.** The same circle over an arc short enough that the
/// fitted image's chords stay within ε (its 32 segments each bow by
/// `R·(span/32)²/8`): the pick reads the row in band, and the wall's
/// fitted image certifies the arc.
fn an_in_band_exact_image_gives_way_to_a_certified_fitted_image<
    T: Decide + Bounds + geom_core::CertifiedEnclosure,
>() {
    let carrier = rim_at(in_band());
    let span = 16.0 * band().zero().sqrt();
    let params = (0.25, 0.25 + span);
    assert!(
        pick_at::<T>(&carrier, (params.0, span), &cylinder(1, TURN)).is_err(),
        "the pick reads the row in band, at {}",
        T::NAME
    );
    let got = plane_lane_at::<T>(&carrier, params, in_band());
    assert!(
        got.is_ok(),
        "the fitted image certifies the arc, at {}: {:?}",
        T::NAME,
        got.as_ref().map(|_| ())
    );
}

#[test]
fn an_in_band_exact_image_gives_way_to_a_certified_fitted_image_at_f64() {
    an_in_band_exact_image_gives_way_to_a_certified_fitted_image::<f64>();
}

#[test]
fn an_in_band_exact_image_gives_way_to_a_certified_fitted_image_at_interval() {
    an_in_band_exact_image_gives_way_to_a_certified_fitted_image::<geom_core::Interval>();
}
