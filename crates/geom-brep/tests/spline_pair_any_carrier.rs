//! **The spline-pair edge certificates take a carrier of any kind**
//! (C1 rung 3, C2): the plane × NURBS and NURBS × NURBS lanes read a
//! line, circle or ellipse through the carrier-enclosure door
//! (`geom_brep::carrier_net`) exactly as they read a spline, and a
//! closed form refuses where, and by what, its spline twin does.
//!
//! The verdicts are the lanes' own: a line's degree-1 chart image
//! certifies on a flat wall, and a circle's or an ellipse's refuses at
//! the hull wherever its degree-1 image's chord error passes the band,
//! with that error measured. Each row offers the same locus as a
//! spline carrier and asks the same verdict of it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use crate::shared::fixture::quarter_cylinder_wall;
use crate::shared::tol::{band, eps};
use geom::{Curve3, NurbsCurve3, NurbsSurface, Surface};
use geom_brep::keys::SurfaceKey;
use geom_brep::{
    CarrierEnclosureRefusal, CertifyError, EdgeCurve, EdgeCurveSpec, EdgeDescriptionSpec,
    NurbsLane, PlaneNurbsLimbs, PlaneNurbsRefusal, SsiLimb, SurfacePair, nurbs_pair_limbs,
    plane_nurbs_limbs,
};
use geom_core::spline::KnotVector;
use geom_core::test_support::upper;
use geom_core::{Point3, Vec3};
use slotmap::SlotMap;

type Limbs = Result<PlaneNurbsLimbs<f64>, PlaneNurbsRefusal>;

/// A bilinear wall through four corners, `control[iu * 2 + iv]`.
fn bilinear(corners: [Point3<f64>; 4]) -> NurbsSurface<f64> {
    let k = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    NurbsSurface::new(k.clone(), k, corners.to_vec(), vec![1.0; 4]).unwrap()
}

/// The flat wall `z = 0` over `[0, 1]²`, its chart the identity.
fn flat_wall() -> NurbsSurface<f64> {
    let p = |x, y| Point3::new(x, y, 0.0);
    bilinear([p(0.0, 0.0), p(0.0, 1.0), p(1.0, 0.0), p(1.0, 1.0)])
}

/// The off-iso diagonal on the flat wall, `t ∈ [0, 1]`.
fn diagonal() -> Curve3<f64> {
    Curve3::Line {
        origin: Point3::new(0.1, 0.1, 0.0),
        dir: Vec3::new(0.6, 0.8, 0.0),
    }
}

/// The vertical plane through the diagonal.
fn diagonal_plane() -> Surface<f64> {
    Surface::Plane {
        origin: Point3::new(0.1, 0.1, 0.0),
        normal: Vec3::new(0.8, -0.6, 0.0),
        u_ref: Vec3::new(0.6, 0.8, 0.0),
    }
}

/// The vertical bilinear wall through the diagonal, reaching past its
/// ends.
fn diagonal_wall() -> NurbsSurface<f64> {
    let at = |t: f64, z| Point3::new(0.1 + 0.6 * t, 0.1 + 0.8 * t, z);
    bilinear([at(-0.1, -0.5), at(-0.1, 0.5), at(1.1, -0.5), at(1.1, 0.5)])
}

/// A spline twin of a closed form over `[t0, t1]`: a degree-1 segment
/// for a line, the exact rational quadratic arc for a circle or an
/// ellipse of half-angle under π/2, on the closed form's own interval.
fn twin(carrier: &Curve3<f64>, (t0, t1): (f64, f64)) -> Curve3<f64> {
    let net = match *carrier {
        Curve3::Line { .. } => NurbsCurve3::new(
            KnotVector::clamped(vec![t0, t0, t1, t1], 1).unwrap(),
            vec![carrier.eval(t0), carrier.eval(t1)],
            vec![1.0, 1.0],
        ),
        Curve3::Circle {
            center,
            axis,
            radius,
            u_ref,
        } => arc(center, u_ref * radius, axis.cross(u_ref) * radius, (t0, t1)),
        Curve3::Ellipse {
            center,
            axis,
            major,
            minor,
            u_ref,
        } => arc(center, u_ref * major, axis.cross(u_ref) * minor, (t0, t1)),
        _ => unreachable!("the rows twin a line, a circle or an ellipse"),
    }
    .unwrap();
    Curve3::Nurbs(Arc::new(net))
}

/// `c + u·cos θ + v·sin θ` over `[a, b]` as a rational quadratic.
fn arc(
    c: Point3<f64>,
    u: Vec3<f64>,
    v: Vec3<f64>,
    (a, b): (f64, f64),
) -> Result<NurbsCurve3<f64>, geom_core::spline::SplineError> {
    let (h, m) = (0.5 * (b - a), 0.5 * (a + b));
    let at = |t: f64| c + u * t.cos() + v * t.sin();
    let mid = c + (u * m.cos() + v * m.sin()) / h.cos();
    NurbsCurve3::new(
        KnotVector::clamped(vec![a, a, a, b, b, b], 2).unwrap(),
        vec![at(a), mid, at(b)],
        vec![1.0, h.cos(), 1.0],
    )
}

/// A height-`z` unit circle about the z axis, `t` its angle: on the
/// quarter-cylinder wall.
fn circle(z: f64) -> Curve3<f64> {
    Curve3::Circle {
        center: Point3::new(0.0, 0.0, z),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: 1.0,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

/// The plane `z = h`.
fn horizontal(h: f64) -> Surface<f64> {
    Surface::Plane {
        origin: Point3::new(0.0, 0.0, h),
        normal: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

/// The quarter-cylinder wall's section by the plane `z = 0.4 + 0.3·x`:
/// `(cos t, sin t, 0.4 + 0.3·cos t)`.
fn ellipse() -> (Curve3<f64>, Surface<f64>) {
    let k = 0.3_f64;
    let s = (1.0 + k * k).sqrt();
    let axis = Vec3::new(-k / s, 0.0, 1.0 / s);
    (
        Curve3::Ellipse {
            center: Point3::new(0.0, 0.0, 0.4),
            axis,
            major: s,
            minor: 1.0,
            u_ref: Vec3::new(1.0 / s, 0.0, k / s),
        },
        Surface::Plane {
            origin: Point3::new(0.0, 0.0, 0.4),
            normal: axis,
            u_ref: Vec3::new(1.0 / s, 0.0, k / s),
        },
    )
}

/// A short arc of the circle from `0.7` rad whose degree-1 chart image
/// stays inside the band at every ε: `scale·√(ε/10⁻⁹)` rad, so the
/// chord error, quadratic in the arc, scales with ε.
fn short(scale: f64) -> (f64, f64) {
    (0.7, 0.7 + scale * (eps() / 1e-9).sqrt().min(20.0))
}

/// The verdict a row compares between a closed form and its twin:
/// certified, or refused at a limb.
#[derive(Debug, PartialEq)]
enum Verdict {
    Certified,
    Refused(SsiLimb),
}

/// The verdict, with the bound it was reached on: a limb's definite
/// refusal or its undecided one alike.
fn verdict(name: &str, got: &Limbs) -> (Verdict, f64) {
    match got {
        Ok(l) => (Verdict::Certified, l.hull_sup),
        Err(PlaneNurbsRefusal::Limb { limb, margin }) => (Verdict::Refused(*limb), upper(*margin)),
        // A miss inside the band refuses undecided, on the bound it read.
        Err(PlaneNurbsRefusal::Escalated { limb, cause }) => {
            (Verdict::Refused(*limb), upper(cause.margin))
        }
        Err(other) => panic!("{name}: certifies or refuses at a limb with its bound: {other:?}"),
    }
}

/// **Plane × spline, each closed-form kind.** The off-iso diagonal on
/// the flat wall certifies, its hull bound within ε; a circle and an
/// ellipse over 1.3 rad of the quarter-cylinder wall refuse at the
/// hull on the degree-1 image's chord error, measured; a circle over
/// a short arc ([`short`]) certifies. Each spline twin reaches the same
/// verdict.
#[test]
fn a_closed_form_carrier_on_a_plane_by_spline_pair_meets_its_twins_verdict() {
    let (ellipse, tilted) = ellipse();
    let rows = [
        (
            "line",
            diagonal(),
            (0.0, 1.0),
            diagonal_plane(),
            flat_wall(),
        ),
        (
            "circle, 1.3 rad",
            circle(0.5),
            (0.1, 1.4),
            horizontal(0.5),
            quarter_cylinder_wall(),
        ),
        (
            "circle, short",
            circle(0.5),
            short(0.002),
            horizontal(0.5),
            quarter_cylinder_wall(),
        ),
        (
            "ellipse, 1.3 rad",
            ellipse,
            (0.1, 1.4),
            tilted,
            quarter_cylinder_wall(),
        ),
    ];
    for (name, carrier, params, plane, wall) in rows {
        let got = plane_nurbs_limbs(&carrier, params, &plane, &wall, 1.0, band());
        let (closed, bound) = verdict(name, &got);
        let spline = twin(&carrier, params);
        let (twinned, twin_bound) = verdict(
            name,
            &plane_nurbs_limbs(&spline, params, &plane, &wall, 1.0, band()),
        );
        println!("{name}: {closed:?} on {bound:e} m; its twin {twinned:?} on {twin_bound:e} m");
        assert_eq!(
            std::mem::discriminant(&closed),
            std::mem::discriminant(&twinned),
            "{name}: the twin's verdict: {closed:?}, {twinned:?}"
        );
        match closed {
            Verdict::Certified => assert!(
                bound <= eps() && twin_bound <= eps(),
                "{name}: both bounds within ε: {bound:e}, {twin_bound:e}"
            ),
            Verdict::Refused(limb) => {
                assert!(
                    matches!(limb, SsiLimb::HullSup | SsiLimb::HullValueChart),
                    "{name}: limb 2 refuses: {limb:?}"
                );
                assert!(
                    [bound, twin_bound]
                        .iter()
                        .all(|b| (eps()..1e-3).contains(b)),
                    "{name}: a miss past ε, measured: {bound:e}, {twin_bound:e}"
                );
            }
        }
        let want = if name.contains("1.3") {
            Verdict::Refused(SsiLimb::HullSup)
        } else {
            Verdict::Certified
        };
        assert_eq!(
            std::mem::discriminant(&closed),
            std::mem::discriminant(&want),
            "{name}: {closed:?}"
        );
    }
}

/// The horizontal bilinear chart `z = h` over `[-0.1, 1.1]²`.
fn horizontal_wall(h: f64) -> NurbsSurface<f64> {
    let p = |x, y| Point3::new(x, y, h);
    bilinear([p(-0.1, -0.1), p(-0.1, 1.1), p(1.1, -0.1), p(1.1, 1.1)])
}

/// **NURBS × NURBS, a closed-form carrier.** The diagonal between the
/// flat wall and the vertical wall through it certifies; a circle
/// between the quarter-cylinder wall and a horizontal spline chart
/// certifies over a short arc ([`short`]; the horizontal chart's own
/// image of it is curved, so shorter than on the plane) and refuses at
/// the hull over 1.3 rad. Each
/// spline twin reaches the same verdict.
#[test]
fn a_closed_form_carrier_between_two_spline_charts_meets_its_twins_verdict() {
    let rows = [
        (
            "line",
            diagonal(),
            (0.0, 1.0),
            flat_wall(),
            diagonal_wall(),
            Verdict::Certified,
        ),
        (
            "circle, short",
            circle(0.5),
            short(0.002),
            quarter_cylinder_wall(),
            horizontal_wall(0.5),
            Verdict::Certified,
        ),
        (
            "circle, 1.3 rad",
            circle(0.5),
            (0.1, 1.4),
            quarter_cylinder_wall(),
            horizontal_wall(0.5),
            Verdict::Refused(SsiLimb::HullSup),
        ),
    ];
    for (name, carrier, params, a, b, want) in rows {
        let got = nurbs_pair_limbs(&carrier, params, &a, &b, 1.0, band());
        let (closed, bound) = verdict(name, &got);
        let spline = twin(&carrier, params);
        let (twinned, twin_bound) = verdict(
            name,
            &nurbs_pair_limbs(&spline, params, &a, &b, 1.0, band()),
        );
        println!("{name}: {closed:?} on {bound:e} m; its twin {twinned:?} on {twin_bound:e} m");
        assert_eq!(
            std::mem::discriminant(&closed),
            std::mem::discriminant(&want),
            "{name}: {closed:?}"
        );
        assert_eq!(
            std::mem::discriminant(&closed),
            std::mem::discriminant(&twinned),
            "{name}: the twin's verdict: {closed:?}, {twinned:?}"
        );
        if closed == Verdict::Certified {
            assert!(
                bound <= eps() && twin_bound <= eps(),
                "{name}: both bounds within ε: {bound:e}, {twin_bound:e}"
            );
        } else {
            assert!(
                [bound, twin_bound]
                    .iter()
                    .all(|b| (eps()..1e-3).contains(b)),
                "{name}: a miss past ε, measured: {bound:e}, {twin_bound:e}"
            );
        }
    }
}

/// A wall on `[0, 1]²` that leaves `z = 0` by `peak` between the
/// image schedule's stations and touches it, level, at each: in `u`,
/// one degree-4 Bézier per station span with controls
/// `(0, 0, peak/0.375, 0, 0)` (its middle Bernstein is `6/16` at the
/// span's middle), the spans meeting level; in `v`, linear.
fn bumped_wall(peak: f64) -> NurbsSurface<f64> {
    let spans = geom_brep::edge_nurbs::PXN_FIT_SAMPLES as usize - 1;
    let mut knots = vec![0.0; 5];
    for i in 1..spans {
        #[allow(clippy::cast_precision_loss)]
        knots.extend([i as f64 / spans as f64; 4]);
    }
    knots.extend([1.0; 5]);
    let ku = KnotVector::clamped(knots, 4).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let n = 4 * spans + 1;
    let mut control = Vec::with_capacity(2 * n);
    for j in 0..n {
        #[allow(clippy::cast_precision_loss)]
        let x = j as f64 / (n - 1) as f64;
        let z = if j % 4 == 2 { peak / 0.375 } else { 0.0 };
        control.extend([Point3::new(x, 0.0, z), Point3::new(x, 1.0, z)]);
    }
    NurbsSurface::new(ku, kv, control, vec![1.0; 2 * n]).unwrap()
}

/// **A carrier 2ε off the wall between samples is refused at limb 2,
/// with its measured bound.** The line `y = 1/2, z = 0` lies on the
/// plane `y = 1/2` and touches the bumped wall at every station of the
/// schedule, so limb 1 and the image's feet read zero; between them
/// the wall stands `2ε` off it, and only limb 2's hull sees that. `2ε`
/// is inside the band, so the hull's verdict is undecided and says so
/// with the bound it read. The line and its spline twin alike.
#[test]
fn a_carrier_two_epsilon_off_the_wall_between_samples_escalates_at_limb_two() {
    let line = Curve3::Line {
        origin: Point3::new(0.0, 0.5, 0.0),
        dir: Vec3::new(1.0, 0.0, 0.0),
    };
    let plane = Surface::Plane {
        origin: Point3::new(0.0, 0.5, 0.0),
        normal: Vec3::new(0.0, 1.0, 0.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let wall = bumped_wall(2.0 * eps());
    for (name, carrier) in [("line", line.clone()), ("spline", twin(&line, (0.0, 1.0)))] {
        match plane_nurbs_limbs(&carrier, (0.0, 1.0), &plane, &wall, 1.0, band()) {
            Err(PlaneNurbsRefusal::Escalated { limb, cause }) => {
                assert!(
                    matches!(limb, SsiLimb::HullSup | SsiLimb::HullValueChart),
                    "{name}: limb 2 is what cannot decide: {limb:?}"
                );
                let read = upper(cause.margin);
                assert!(
                    (1.9 * eps()..=4.0 * eps()).contains(&read),
                    "{name}: the bound it read is the 2ε the wall stands off: {read:e}"
                );
            }
            other => panic!("{name}: a 2ε miss between samples escalates at limb 2: {other:?}"),
        }
    }
}

/// **The door's typed refusal reaches the edge certificate.** A spiric
/// whose square root's argument reaches zero at `v = π` (offset equal
/// to the inner radius), on its plane `x = 3/2` and over the flat wall
/// `z = 0` it crosses, is refused at limb 2 by the carrier-enclosure
/// door, naming the kind and the span that holds `π`, not as malformed
/// enclosure data.
#[test]
fn a_carrier_the_door_cannot_enclose_is_refused_by_kind_and_span() {
    let spiric = Curve3::Spiric {
        center: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
        major_radius: 2.0,
        minor_radius: 0.5,
        offset: 1.5,
    };
    let plane = Surface::Plane {
        origin: Point3::new(1.5, 0.0, 0.0),
        normal: Vec3::new(1.0, 0.0, 0.0),
        u_ref: Vec3::new(0.0, 1.0, 0.0),
    };
    let p = |x, y| Point3::new(x, y, 0.0);
    let wall = bilinear([p(1.0, -1.0), p(1.0, 1.0), p(2.0, -1.0), p(2.0, 1.0)]);
    let got = plane_nurbs_limbs(&spiric, (3.0, 3.3), &plane, &wall, 1.0, band());
    let Err(PlaneNurbsRefusal::CarrierEnclosure(CarrierEnclosureRefusal { kind, span })) = got
    else {
        panic!("the door's refusal, typed: {got:?}")
    };
    assert_eq!(kind, geom::CurveKind::Spiric, "named by kind");
    let pi = core::f64::consts::PI;
    assert!(
        span.0 <= pi && pi <= span.1 && span.1 - span.0 < 0.3 / 16.0,
        "named by the span holding π, one span of the grid: {span:?}"
    );
}

/// The arena and spec of an `Intersection` of `a` and `b` over
/// `carrier` on `params`, its witness the carrier's midpoint.
fn door_spec(
    a: Surface<f64>,
    b: Surface<f64>,
    carrier: Curve3<f64>,
    (t0, t1): (f64, f64),
) -> (
    impl Fn(SurfaceKey) -> Option<Surface<f64>>,
    EdgeCurveSpec<f64>,
) {
    let mut arena: SlotMap<SurfaceKey, Surface<f64>> = SlotMap::with_key();
    let (s1, s2) = (arena.insert(a), arena.insert(b));
    let witness = carrier.eval(0.5 * (t0 + t1));
    (
        move |k| arena.get(k).cloned(),
        EdgeCurveSpec {
            description: EdgeDescriptionSpec::Intersection {
                pair: SurfacePair::new(s1, s2),
                witness,
            },
            carrier,
            param_start: t0,
            param_end: t1,
        },
    )
}

/// **Through the certification door**: the diagonal as a `Line` on the
/// plane × wall pair and on the wall × wall pair certifies, recording
/// the lane's bound, and a circle over 1.3 rad on the plane × wall pair
/// refuses at limb 2 with its measured miss — where the door
/// refused every non-spline carrier before it read a limb.
#[test]
fn the_door_certifies_a_closed_form_carrier_on_a_spline_pair() {
    let lane = Some(NurbsLane::certified());
    let line = diagonal();
    let ends = (line.eval(0.0), line.eval(1.0));
    for (name, a, b) in [
        (
            "plane × wall",
            diagonal_plane(),
            Surface::Nurbs(Arc::new(flat_wall())),
        ),
        (
            "wall × wall",
            Surface::Nurbs(Arc::new(diagonal_wall())),
            Surface::Nurbs(Arc::new(flat_wall())),
        ),
    ] {
        let (arena, spec) = door_spec(a, b, line.clone(), (0.0, 1.0));
        let edge = EdgeCurve::certify_via(spec, ends.0, ends.1, arena, band(), lane)
            .unwrap_or_else(|e| panic!("{name}: the line certifies through the door: {e}"));
        assert!(
            edge.certificate().max_residual <= eps(),
            "{name}: within ε: {:e}",
            edge.certificate().max_residual
        );
    }
    let (t0, t1) = (0.1, 1.4);
    let arc = circle(0.5);
    let (arena, spec) = door_spec(
        horizontal(0.5),
        Surface::Nurbs(Arc::new(quarter_cylinder_wall())),
        arc.clone(),
        (t0, t1),
    );
    let got = EdgeCurve::certify_via(spec, arc.eval(t0), arc.eval(t1), arena, band(), lane);
    let Err(CertifyError::PlaneNurbs(PlaneNurbsRefusal::Limb { limb, margin })) = got else {
        panic!("the circle refuses at a limb through the door: {got:?}")
    };
    assert!(
        matches!(limb, SsiLimb::HullSup | SsiLimb::HullValueChart),
        "limb 2 refuses: {limb:?}"
    );
    assert!(
        (eps()..1e-3).contains(&upper(margin)),
        "a miss past ε, measured: {:e}",
        upper(margin)
    );
}

/// **D7's promoted line certifies wherever its spline did.** An off-iso
/// straight edge between a plane and a flat bilinear wall, stated as a
/// degree-1 spline and as the `Line` D7 recognition promotes it to (its
/// first control point the origin, the unit chord the direction, on the
/// same interval because the chord has unit length): both certify
/// through the door on bounds within roundoff of each other, and the
/// chart image the certificate derives (the mint's own producer,
/// `FittedLane::general_image`) is the same net for both.
#[test]
fn a_promoted_line_certifies_where_its_spline_did_on_the_same_image() {
    let lane = Some(NurbsLane::certified());
    let line = diagonal();
    let spline = twin(&line, (0.0, 1.0));
    let ends = (line.eval(0.0), line.eval(1.0));
    let mut bounds = Vec::new();
    for carrier in [&spline, &line] {
        let (arena, spec) = door_spec(
            diagonal_plane(),
            Surface::Nurbs(Arc::new(flat_wall())),
            carrier.clone(),
            (0.0, 1.0),
        );
        let edge = EdgeCurve::certify_via(spec, ends.0, ends.1, arena, band(), lane)
            .unwrap_or_else(|e| panic!("{:?} certifies: {e}", carrier.kind()));
        bounds.push(edge.certificate().max_residual);
    }
    println!("spline, then line: bounds {bounds:?}");
    assert!(
        bounds.iter().all(|b| *b <= eps()) && (bounds[0] - bounds[1]).abs() <= 1e-14,
        "the same bound, within ε and within roundoff of each other: {bounds:?}"
    );
    let fitted = geom_brep::FittedLane::<f64>::certified();
    let images: Vec<Vec<(f64, f64)>> = [&spline, &line]
        .into_iter()
        .map(|c| {
            let image = fitted.general_image(c, (0.0, 1.0), &flat_wall()).unwrap();
            assert_eq!(image.knots().domain(), (0.0, 1.0), "on the edge's interval");
            image.control().iter().map(|p| (p.x, p.y)).collect()
        })
        .collect();
    let apart = images[0]
        .iter()
        .zip(&images[1])
        .map(|(a, b)| (a.0 - b.0).abs().max((a.1 - b.1).abs()))
        .fold(0.0, f64::max);
    assert!(
        images[0].len() == images[1].len() && apart <= 1e-15,
        "one image, its feet within roundoff: {apart:e}"
    );
}
