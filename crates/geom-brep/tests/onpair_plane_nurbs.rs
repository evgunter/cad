//! **`OnPair` on a plane × spline pair**: incidence on both surfaces
//! through the plane × NURBS lane's limbs 1 and 2
//! ([`geom_brep::plane_nurbs_incidence`]), and nothing about how they
//! meet. The plane side is its closed-form residual at each sample; the
//! wall side the foot residual and the composite sup of
//! `|S(P(t)) − C(t)|` over the chart image.
//!
//! The fixture is M7-8's: the rational quarter-cylinder wall and a plane
//! through its `u = 0` ruling, and a flat patch lying in a plane, the
//! pair D2 leaves conventional (it under-determines their shared side).
//! The certificate states incidence alone, so the TANGENT plane `x = 1`
//! certifies too, though D2's honest form there is a
//! `TangentIntersection`: which form an edge must carry is tier 3's
//! question, not this certificate's. An `Approx` wall is metered against
//! its fit.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use crate::shared::fixture::{quarter_cylinder_wall, segment, transverse_plane};
use crate::shared::tol::band;
use geom::{Curve3, NurbsCurve3, NurbsSurface, Surface};
use geom_brep::keys::SurfaceKey;
use geom_brep::{
    ArcRimRefusal, CertifyError, EdgeCurve, EdgeCurveSpec, EdgeDescription, EdgeDescriptionSpec,
    PlaneNurbsRefusal, SurfacePair,
};
use geom_core::test_support::upper;
use geom_core::{Point3, Tol, Vec3};
use slotmap::SlotMap;

/// The `x = 1` plane, tangent to [`quarter_cylinder_wall`] along its
/// `u = 0` ruling.
fn tangent_plane() -> Surface<f64> {
    Surface::Plane {
        origin: Point3::new(1.0, 0.0, 0.0),
        normal: Vec3::new(1.0, 0.0, 0.0),
        u_ref: Vec3::new(0.0, 1.0, 0.0),
    }
}

/// The ruling, `(1, 0, z)` for `z ∈ [0, 1]`, displaced by `off` along
/// `x`: off the wall, and on the `y = 0` plane.
fn ruling(off: f64) -> NurbsCurve3<f64> {
    segment(
        Point3::new(1.0 + off, 0.0, 0.0),
        Point3::new(1.0 + off, 0.0, 1.0),
    )
}

/// Certifies `carrier` as an `OnPair` of `plane` and `wall` through the
/// door, with `lane`.
fn on_pair(
    plane: Surface<f64>,
    wall: Surface<f64>,
    carrier: NurbsCurve3<f64>,
) -> Result<EdgeCurve<f64>, CertifyError> {
    let mut arena: SlotMap<SurfaceKey, Surface<f64>> = SlotMap::with_key();
    let (s1, s2) = (arena.insert(plane), arena.insert(wall));
    let ends = (carrier.eval(0.0), carrier.eval(1.0));
    let spec = EdgeCurveSpec {
        description: EdgeDescriptionSpec::on_pair(SurfacePair::new(s1, s2)),
        carrier: Curve3::Nurbs(Arc::new(carrier)),
        param_start: 0.0,
        param_end: 1.0,
    };
    EdgeCurve::certify(
        spec,
        ends.0,
        ends.1,
        move |k| arena.get(k).cloned(),
        1.0,
        band(),
        geom_brep::NurbsLane::certified(),
    )
}

fn wall() -> Surface<f64> {
    Surface::Nurbs(Arc::new(quarter_cylinder_wall()))
}

fn assert_on_pair(edge: &EdgeCurve<f64>, what: &str) {
    assert!(
        matches!(edge.description(), EdgeDescription::OnPair { .. }),
        "{what}: {:?}",
        edge.description()
    );
    assert!(
        edge.certificate().max_residual <= Tol::witness().get().eps,
        "{what}: the recorded residual is within ε: {:e}",
        edge.certificate().max_residual
    );
}

/// The ruling on the transverse plane certifies as an `OnPair`, its
/// recorded residual the lane's certified sup.
#[test]
fn a_ruling_on_a_transverse_plane_certifies_on_the_pair() {
    let edge = on_pair(transverse_plane(), wall(), ruling(0.0)).expect("the ruling lies on both");
    assert_on_pair(&edge, "the transverse plane");
}

/// **Incidence alone.** The tangent plane meets the wall along the same
/// ruling with parallel normals, a tangency the wall's curvature
/// determines, so D2's honest form there is a `TangentIntersection`, and
/// the `Intersection` lane refuses it `NotTransverse`
/// (`m7_8_plane_nurbs_edge`). The pair's certificate still passes,
/// incidence being all it states: holding the edge to the form D2 asks
/// of it is tier 3's must-carry check, which no door here is asking.
#[test]
fn a_ruling_on_the_tangent_plane_certifies_incidence_alone() {
    let edge = on_pair(tangent_plane(), wall(), ruling(0.0)).expect("the ruling lies on both");
    assert_on_pair(&edge, "the tangent plane");
}

/// **The wall-side falsifier.** The ruling moved to `x = 1 + off` stays
/// exactly on the `y = 0` plane and leaves only the wall, so the lane's
/// foot residual alone refuses it, with the measured miss.
#[test]
fn a_carrier_off_the_wall_refuses_with_the_measured_miss() {
    let off = 1e3 * Tol::witness().get().eps;
    match on_pair(transverse_plane(), wall(), ruling(off)) {
        Err(CertifyError::PlaneNurbs(PlaneNurbsRefusal::Limb { margin, .. })) => {
            let value = upper(margin);
            assert!(value >= 0.5 * off, "the measured miss: {value:e}");
        }
        other => panic!("a carrier {off:e} m off the wall must refuse typed: {other:?}"),
    }
}

/// **The pair D2 leaves conventional.** A flat NURBS patch in the plane
/// `x = 0` and that plane itself: the two surfaces are one, so they
/// under-determine any locus on both, and the patch's `y = 0` side
/// rests on the pair.
#[test]
fn a_side_a_flat_patch_shares_with_its_plane_certifies_on_the_pair() {
    let edge = on_pair(
        Surface::Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(1.0, 0.0, 0.0),
            u_ref: Vec3::new(0.0, 1.0, 0.0),
        },
        Surface::Nurbs(Arc::new(flat_patch())),
        segment(Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 0.0, 1.0)),
    )
    .expect("the side lies on both");
    assert_on_pair(&edge, "the coincident flat patch");
}

/// The flat patch `x = 0` over `y, z ∈ [0, 1]`.
fn flat_patch() -> NurbsSurface<f64> {
    let k = geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    NurbsSurface::new(
        k.clone(),
        k,
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 0.0, 1.0),
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(0.0, 1.0, 1.0),
        ],
        vec![1.0; 4],
    )
    .unwrap()
}

/// **An `Approx` wall is metered against its fit.** The flat patch
/// `x = 0` over `y, z ∈ [0, 1]` offset by `d`: its fit is the plane
/// `x = ±d` itself, and its `y = 0` side lies on the `y = 0` plane.
#[test]
fn a_side_of_an_approx_wall_certifies_on_the_pair() {
    let approx =
        geom_brep::offset_fit::approx_offset_surface_at(Arc::new(flat_patch()), 0.05, 1e-6, band())
            .expect("the flat patch's offset fits");
    let side = segment(approx.eval(0.0, 0.0), approx.eval(0.0, 1.0));
    assert_eq!(
        (side.eval(0.0).y, side.eval(1.0).y),
        (0.0, 0.0),
        "the side lies on the y = 0 plane"
    );
    let edge = on_pair(transverse_plane(), approx, side).expect("the side lies on both");
    assert_on_pair(&edge, "the Approx wall");
}

/// Two spline charts have no incidence lane: the pair refuses as before.
#[test]
fn two_spline_charts_still_refuse_the_pair() {
    assert!(matches!(
        on_pair(
            wall(),
            Surface::Nurbs(Arc::new(quarter_cylinder_wall())),
            ruling(0.0)
        ),
        Err(CertifyError::Unimplemented)
    ));
}

/// **The fit, not the description.** The parabolic cylinder
/// `z = 3v²`, extruded along `x`, offset by `d = 0.05` and fitted
/// coarsely (to `1e-3`): every `v`-iso of the true offset is a straight
/// line in a plane `y = const`, and at `v = 0.75` the fit lies a
/// measured distance off it. The true offset line, on its plane, refuses
/// with that miss wherever the band reads it, and certifies where the
/// band is coarser: the lane meters the fit. A lane that metered the
/// description would certify it at every band.
#[test]
fn an_approx_wall_is_metered_against_its_fit() {
    let kv2 =
        geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let control = (0..9)
        .map(|k| {
            let (i, j) = (k / 3, k % 3);
            Point3::new(
                f64::from(i) * 0.5,
                f64::from(j) * 0.5,
                [0.0, 0.0, 3.0][j as usize],
            )
        })
        .collect();
    let base = NurbsSurface::new(kv2.clone(), kv2, control, vec![1.0; 9]).unwrap();
    let d = 0.05;
    let approx = geom_brep::offset_fit::approx_offset_surface_at(Arc::new(base), d, 1e-3, band())
        .expect("the parabolic cylinder's offset fits");
    // The true offset at `v`: `(x, v − 6v·d/n, 3v² + d/n)`, `n = √(1 + 36v²)`.
    let v = 0.75_f64;
    let n = (1.0 + 36.0 * v * v).sqrt();
    let (y, z) = (v - 6.0 * v * d / n, 3.0 * v * v + d / n);
    let line = segment(Point3::new(0.0, y, z), Point3::new(1.0, y, z));
    // The fit's distance from the line within the plane `y`, through
    // the fit's iso that meets the line's `y` at mid-span: an upper
    // bound on its distance from the fit.
    let mut w = v;
    for _ in 0..60 {
        w -= 0.9 * (approx.eval(0.5, w).y - y);
    }
    let miss = (0..=40)
        .map(|i| {
            let p = approx.eval(f64::from(i) / 40.0, w);
            (p.y - y).hypot(p.z - z)
        })
        .fold(0.0_f64, f64::max);
    let plane = Surface::Plane {
        origin: Point3::new(0.0, y, 0.0),
        normal: Vec3::new(0.0, 1.0, 0.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let got = on_pair(plane, approx, line);
    println!("the fit lies {miss:e} off the description's line");
    if miss > 2.0 * band().zero() {
        match got {
            Err(CertifyError::PlaneNurbs(PlaneNurbsRefusal::Limb { margin, .. })) => {
                // The limb's margin is a foot distance at the sample it
                // refused at, at most the slice's miss.
                let value = upper(margin);
                assert!(
                    value > band().zero() && value <= miss,
                    "the refusal carries a miss off the fit within {miss:e}: {value:e}"
                );
            }
            other => panic!("the description's line lies {miss:e} off the fit: {other:?}"),
        }
    } else if miss < 0.5 * band().zero() {
        assert_on_pair(&got.expect("within the band of the fit"), "the coarse fit");
    }
}

/// The quarter-circle rim at `z = 0` of [`quarter_cylinder_wall`],
/// radius `r` about `z`, from `x̂` through `angle`.
fn rim(r: f64, angle: f64) -> (Curve3<f64>, f64) {
    (
        Curve3::Circle {
            center: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: r,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        },
        angle,
    )
}

/// The `z = 0` plane, the wall's bottom cap.
fn cap() -> Surface<f64> {
    Surface::Plane {
        origin: Point3::new(0.0, 0.0, 0.0),
        normal: Vec3::new(0.0, 0.0, -1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

/// The quarter rim's own image on the wall: the arc-rim pick's, which
/// its certificate holds.
fn quarter_image() -> geom_brep::Pcurve<f64> {
    let (carrier, angle) = rim(1.0, core::f64::consts::FRAC_PI_2);
    geom_brep::pick_arc_rim(&carrier, (0.0, angle), &wall(), band())
        .expect("the pick decides")
        .expect("one candidate is the rim's row")
}

/// An `OnPair` of the cap and the wall: its spec, its two ends, the
/// surfaces, and the wall's key.
type ArcSpec = (
    EdgeCurveSpec<f64>,
    (Point3<f64>, Point3<f64>),
    SlotMap<SurfaceKey, Surface<f64>>,
    SurfaceKey,
);

/// [`ArcSpec`] over `carrier` on `[t0, t1]`.
fn arc_spec(carrier: Curve3<f64>, (t0, t1): (f64, f64)) -> ArcSpec {
    let mut arena: SlotMap<SurfaceKey, Surface<f64>> = SlotMap::with_key();
    let (s1, s2) = (arena.insert(cap()), arena.insert(wall()));
    let ends = (carrier.eval(t0), carrier.eval(t1));
    let spec = EdgeCurveSpec {
        description: EdgeDescriptionSpec::on_pair(SurfacePair::new(s1, s2)),
        carrier,
        param_start: t0,
        param_end: t1,
    };
    (spec, ends, arena, s2)
}

/// Certifies `carrier` over `[0, angle]` as an `OnPair` of the cap and
/// the wall, the wall's image `stated` where given.
fn arc_on_pair(
    (carrier, angle): (Curve3<f64>, f64),
    stated: Option<geom_brep::Pcurve<f64>>,
) -> Result<EdgeCurve<f64>, CertifyError> {
    let (spec, ends, arena, wall_key) = arc_spec(carrier, (0.0, angle));
    let stated = stated.map(|pcurve| geom_brep::StatedImage {
        surface: wall_key,
        pcurve,
    });
    EdgeCurve::certify_stated(
        spec,
        ends.0,
        ends.1,
        move |k| arena.get(k).cloned(),
        1.0,
        band(),
        geom_brep::NurbsLane::certified(),
        stated.as_ref(),
    )
}

/// **An arc rim certifies on its pair through its exact image.** The
/// fitted image's hull refuses a circle's foot path; the arc-rim
/// certificate compares the wall's boundary row with the circle in the
/// row's own spline space. The pick's image certifies, and so does the
/// same image stated, to the same bound.
#[test]
fn an_arc_rim_certifies_on_its_pair_through_its_exact_image() {
    let quarter = rim(1.0, core::f64::consts::FRAC_PI_2);
    let derived = arc_on_pair(quarter.clone(), None).expect("the rim lies on both");
    assert_on_pair(&derived, "the arc rim");
    let stated = arc_on_pair(quarter, Some(quarter_image())).expect("the stated image certifies");
    assert_eq!(
        stated.certificate().max_residual,
        derived.certificate().max_residual,
        "the stated image and the pick's are one image"
    );
}

/// **Off the wall, both causes.** The rim at radius `1 + off` lies on
/// the cap and `off` off the wall. No boundary-row image holds it at the
/// pick's probe (`iso: None`); the on-wall rim's image stated for it
/// refuses at its map residual, and the refusal reads as that check,
/// not as the fitted path's bound, which an arc's bent foot path
/// inflates whether the rim is on the wall or off it.
#[test]
fn an_arc_rim_off_its_wall_refuses_with_both_causes() {
    let off = 1e3 * Tol::witness().get().eps;
    let carrier = rim(1.0 + off, core::f64::consts::FRAC_PI_2);
    assert!(
        matches!(
            arc_on_pair(carrier.clone(), None),
            Err(CertifyError::PlaneNurbs(PlaneNurbsRefusal::ArcRim(
                ArcRimRefusal::Refused { iso: None, .. }
            )))
        ),
        "no image of the wall's rows holds a rim {off:e} off it"
    );
    let refused = arc_on_pair(carrier, Some(quarter_image()));
    let Err(
        e @ CertifyError::PlaneNurbs(PlaneNurbsRefusal::ArcRim(ArcRimRefusal::Refused {
            iso: Some(geom_brep::PcurveCheck::MapResidual),
            ..
        })),
    ) = &refused
    else {
        panic!("the stated image refuses at its map residual: {refused:?}");
    };
    assert!(
        matches!(
            e.decision(),
            Some((
                geom_brep::CertCheck::PlaneNurbsHullValue,
                geom_brep::recourse::RefusedArm::SignCertain(None)
            ))
        ),
        "the refusal reads as the exact image's check: {:?}",
        e.decision()
    );
    assert!(
        e.to_string()
            .contains("exact image along a boundary row of its wall misses it"),
        "the refusal names the exact image's check: {e}"
    );
}

/// **An escalated pick escalates.** The rim `2ε` off the wall sits in
/// the band at the pick's probe: the exact image's reading is
/// undecided, and the fitted path does not make it definite (D4 ¶3) —
/// the pcurve mint's rule, which the pick is.
#[test]
fn an_arc_rim_whose_exact_image_escalates_escalates() {
    let b = band();
    let off = 0.5 * (b.zero() + b.escalate());
    let carrier = rim(1.0 + off, core::f64::consts::FRAC_PI_2);
    assert!(
        geom_brep::pick_arc_rim(&carrier.0, (0.0, carrier.1), &wall(), b).is_err(),
        "the pick escalates at the rim's row"
    );
    match arc_on_pair(carrier, None) {
        Err(CertifyError::Escalated {
            check: geom_brep::CertCheck::PlaneNurbsHullValue,
            ..
        }) => {}
        other => panic!("an exact image that escalates escalates the rim: {other:?}"),
    }
}

/// **A partial rim takes the row over the arc the row measures.** Half
/// the quarter arc lies on the wall. No closed-form image (the edge's
/// interval onto the whole row) holds it at the pick's probe, nor
/// certifies it; the measured step's does: the whole row over its own
/// arc, the quarter rim's own image, in which the half rim's span lies.
/// The pair certifies through it.
#[test]
fn a_partial_arc_rim_takes_the_rows_arc() {
    let half = rim(1.0, core::f64::consts::FRAC_PI_4);
    assert!(
        geom_brep::arc_rim_candidates(&quarter_cylinder_wall(), 0.0, half.1)
            .into_iter()
            .all(|p| {
                geom_brep::PcurveCache::certify(p, 0.0, half.1, &half.0, &wall(), band()).is_err()
            }),
        "no full-row image over the edge's interval certifies a partial rim"
    );
    let picked = geom_brep::pick_arc_rim(&half.0, (0.0, half.1), &wall(), band())
        .expect("the pick decides")
        .expect("the measured step holds the partial rim");
    assert_same_arc(&picked, &quarter_image(), "the partial rim's image");
    geom_brep::PcurveCache::certify(picked, 0.0, half.1, &half.0, &wall(), band())
        .expect("the row over its own arc certifies the partial rim");
    let edge = arc_on_pair(half, None).expect("the partial rim certifies on its pair");
    assert_on_pair(&edge, "the partial rim, by its exact image");
}

/// `got` and `want` compared as rows ([`test_utils::arc_row`]).
fn assert_same_arc(got: &geom_brep::Pcurve<f64>, want: &geom_brep::Pcurve<f64>, what: &str) {
    test_utils::arc_row::assert_same_row(&arc_row(got), &arc_row(want), what);
}

fn arc_row(image: &geom_brep::Pcurve<f64>) -> test_utils::arc_row::ArcRow {
    let geom_brep::Pcurve::IsoArc {
        p0,
        pd,
        t0,
        angle,
        breaks,
    } = image
    else {
        panic!("an IsoArc image: {image:?}")
    };
    test_utils::arc_row::ArcRow {
        p0: [p0.x, p0.y],
        pd: [pd.x, pd.y],
        breaks: format!("{breaks:?}"),
        t0: *t0,
        angle: *angle,
    }
}

/// **The measured arc is moved onto the branch the edge lies on.** The
/// half rim stated a whole turn on, `[τ + π/8, τ + 3π/8]`: the row's
/// start corner reads at azimuth `0`, and the arc that holds the span
/// starts at `τ`, a turn from the principal read. The pick decides that
/// turn and the image certifies over the span; read at the principal
/// branch, the image's arc would hold none of it.
#[test]
fn the_measured_arc_takes_the_branch_its_edge_lies_on() {
    let (carrier, quarter) = rim(1.0, core::f64::consts::FRAC_PI_2);
    let tau = core::f64::consts::TAU;
    let (t0, span) = (tau + quarter / 4.0, quarter / 2.0);
    let picked = geom_brep::pick_arc_rim(&carrier, (t0, span), &wall(), band())
        .expect("the pick decides")
        .expect("the measured step holds the rim a turn on");
    let row = arc_row(&picked);
    assert!(
        (row.t0 - tau).abs() < 1e-12 && (row.angle - quarter).abs() < 1e-12,
        "the row's arc a turn on: ({}, {})",
        row.t0,
        row.angle
    );
    geom_brep::PcurveCache::certify(picked, t0, t0 + span, &carrier, &wall(), band())
        .expect("the image certifies over the span a turn on");
}

/// **A polynomial row states no turn.** The quarter wall's net with unit
/// weights is no circle's rational form: its rows' interior weight is
/// `1`, which states a zero turn, so the measured step offers no image
/// rather than one of zero arc. The wall's own rational rows offer one
/// per row and direction.
#[test]
fn a_polynomial_row_offers_no_measured_arc() {
    let (carrier, quarter) = rim(1.0, core::f64::consts::FRAC_PI_2);
    let rational = quarter_cylinder_wall();
    let control: Vec<Point3<f64>> = rational.control().to_vec();
    let polynomial = NurbsSurface::new(
        crate::shared::fixture::kv2(),
        crate::shared::fixture::kv1(),
        control,
        vec![1.0; 6],
    )
    .unwrap();
    let span = (quarter / 4.0, quarter / 2.0);
    assert_eq!(
        geom_brep::arc_rim_measured(&carrier, &rational, span, band()).len(),
        4,
        "the rational wall: one measured image per row and direction"
    );
    assert!(
        geom_brep::arc_rim_measured(&carrier, &polynomial, span, band()).is_empty(),
        "a polynomial row offers no measured image"
    );
}

/// **An edge running past its image's arc is refused at the span**,
/// before its samples are read. The quarter rim's image holds every
/// sub-span of its arc, and refuses a span reaching before its start or
/// past its end, typed: the envelope bounds nothing off the arc.
#[test]
fn an_iso_arc_refuses_a_span_past_its_arc() {
    let (carrier, quarter) = rim(1.0, core::f64::consts::FRAC_PI_2);
    let certify = |t0: f64, t1: f64| {
        geom_brep::PcurveCache::certify(quarter_image(), t0, t1, &carrier, &wall(), band())
    };
    certify(0.2, 0.9).expect("a span inside the arc certifies");
    for (t0, t1) in [(-0.01, 0.9), (0.2, quarter + 0.01)] {
        match certify(t0, t1) {
            Err(geom_brep::PcurveCertifyError::IsoUnsupported { what }) => assert!(
                what.starts_with("an arc rim whose edge runs past its image's arc"),
                "[{t0}, {t1}]: refused at the span: {what}"
            ),
            other => panic!("[{t0}, {t1}] runs past the arc [0, {quarter}]: {other:?}"),
        }
    }
}

/// **A stated image is certified as it stands, not swapped for the
/// pick's.** The wall's other row stated as the rim's image refuses,
/// though the pick's own image holds. The lane does fall back, soundly,
/// to the fitted path, which refuses an arc here; `iso: Some` tells the
/// two paths apart: it is the stated image's refusal, where a pick would
/// have certified.
#[test]
fn a_wrong_stated_image_is_not_swapped_for_the_pick() {
    let quarter = rim(1.0, core::f64::consts::FRAC_PI_2);
    let wrong = geom_brep::arc_rim_candidates(&quarter_cylinder_wall(), 0.0, quarter.1)
        .into_iter()
        .find(|p| {
            geom_brep::PcurveCache::certify(p.clone(), 0.0, quarter.1, &quarter.0, &wall(), band())
                .is_err()
        })
        .expect("a candidate that is not the rim's row");
    match arc_on_pair(quarter, Some(wrong)) {
        Err(CertifyError::PlaneNurbs(PlaneNurbsRefusal::ArcRim(ArcRimRefusal::Refused {
            iso: Some(_),
            ..
        }))) => {}
        other => panic!("a wrong stated image refuses as itself: {other:?}"),
    }
}

/// **A split child certifies at rest as it did at the op.** The
/// `[0, π/4]` half of the quarter rim, with its parent's stored image
/// stated — whose arc covers it — certifies, and so does the child
/// alone, through the image the pick measures for it, which is the
/// parent's. Re-certification reads the stored image as the op did
/// (`recertify_stated`, tier 3's).
#[test]
fn a_split_child_recertifies_through_its_parents_stored_image() {
    let (carrier, _) = rim(1.0, core::f64::consts::FRAC_PI_2);
    let span = (0.0, core::f64::consts::FRAC_PI_4);
    let (spec, ends, arena, wall_key) = arc_spec(carrier, span);
    let stated = geom_brep::StatedImage {
        surface: wall_key,
        pcurve: quarter_image(),
    };
    let surfaces = |k| arena.get(k).cloned();
    let lane = geom_brep::NurbsLane::<f64>::certified;
    let child = EdgeCurve::certify_stated(
        spec.clone(),
        ends.0,
        ends.1,
        surfaces,
        1.0,
        band(),
        lane(),
        Some(&stated),
    )
    .expect("the child certifies through its parent's image");
    let alone = EdgeCurve::certify(spec, ends.0, ends.1, surfaces, 1.0, band(), lane())
        .expect("the child alone certifies through the image picked for it");
    assert!(
        (alone.certificate().max_residual - child.certificate().max_residual).abs() < 1e-12,
        "the picked image and the parent's are one row: {} / {}",
        alone.certificate().max_residual,
        child.certificate().max_residual
    );
    child
        .recertify_stated(ends.0, ends.1, surfaces, 1.0, band(), lane(), Some(&stated))
        .expect("the child re-certifies through the stored image");
}
