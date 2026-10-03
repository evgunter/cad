//! **A rigid map moves a plane × NURBS edge certified near ε.**
//!
//! The lane's limb 2 (`ssi_hull_sup_chart`) bounds the residual
//! composite `S(P(t)) − C(t)` from the norms of its vector Bernstein
//! coefficients, which a rigid map leaves unchanged up to rounding, so
//! an edge certified a few percent under ε re-derives under it in every
//! frame. A bound folded from per-coordinate sups would not: for one
//! residual field it lies anywhere between the field's sup norm and √3
//! times it, depending on how the field sits against the axes.
//!
//! **The subject.** The `z = 1/2` plane against a rational
//! quarter-cylinder wall of radius 1, with the carrier the exact
//! rational arc of radius `1 + δ` — a residual field of exactly `δ`,
//! pointing radially. The pair is seated symmetric about the x axis
//! (the arc spans ±45°), where the per-coordinate fold reads
//! `√(1 + ½)·δ`; a rotation that turns the arc's chord off the axes
//! reads up to `√2·δ`. `δ` is calibrated from the lane's own bound so
//! the edge certifies at `(1 − 1/32)·ε`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use geom::{Curve3, NurbsCurve3, NurbsSurface, Surface};
use geom_core::spline::KnotVector;
use geom_core::{Affine3, Band, Point3, Tol, Vec3};
use test_utils::vacuity;
use topo::Body;

const W: f64 = core::f64::consts::FRAC_1_SQRT_2;

/// Where the bound is placed under ε, as a fraction of ε.
const MARGIN: f64 = 1.0 / 32.0;

/// The limb-2 bound at `δ = 0`, in metres: the floor every placement
/// sits on. Measured 3.242e-14 at every battery ε; pinned with 30%
/// headroom.
const FLOOR: f64 = 4.2e-14;

/// How far limb 1 may read from the field, in ulps of the fixture's
/// coordinate scale. Limb 1 is the length of `S(u*, v*) − C(t)`, a
/// difference of two points evaluated at that scale from nets a rigid
/// map has re-rounded, so its error is a few ulps of the coordinates
/// whatever `δ` is. Measured within 1.8 in every frame at every
/// battery ε.
const ROUNDING_ULPS: f64 = 4.0;

fn kv2() -> KnotVector {
    KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap()
}

fn kv1() -> KnotVector {
    KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap()
}

/// The seat: the quarter turn `[0°, 90°]` turned to `[−45°, 45°]`.
fn seat() -> Affine3<f64> {
    Affine3::rotation_about_axis(
        Point3::origin(),
        Vec3::new(0.0, 0.0, 1.0),
        -core::f64::consts::FRAC_PI_4,
    )
}

fn wall() -> NurbsSurface<f64> {
    let control = vec![
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(1.0, 0.0, 1.0),
        Point3::new(1.0, 1.0, 0.0),
        Point3::new(1.0, 1.0, 1.0),
        Point3::new(0.0, 1.0, 0.0),
        Point3::new(0.0, 1.0, 1.0),
    ];
    let m = seat();
    NurbsSurface::new(kv2(), kv1(), control, vec![1.0, 1.0, W, W, 1.0, 1.0])
        .unwrap()
        .map_points(|p| m.transform_point(p))
}

fn plane() -> Surface<f64> {
    Surface::Plane {
        origin: Point3::new(0.0, 0.0, 0.5),
        normal: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

/// The exact rational arc of radius `r` on the plane, `reversed` running
/// it the other way.
fn arc(r: f64, reversed: bool) -> NurbsCurve3<f64> {
    let m = seat();
    let mut control = vec![
        Point3::new(r, 0.0, 0.5),
        Point3::new(r, r, 0.5),
        Point3::new(0.0, r, 0.5),
    ];
    if reversed {
        control.reverse();
    }
    NurbsCurve3::new(kv2(), control, vec![1.0, W, 1.0])
        .unwrap()
        .map_points(|p| m.transform_point(p))
}

fn hull_at(delta: f64, band: Band) -> geom_brep::PlaneNurbsLimbs<f64> {
    geom_brep::plane_nurbs_limbs::<f64>(&arc(1.0 + delta, false), &plane(), &wall(), 1.0, band)
        .unwrap_or_else(|e| panic!("δ = {delta:e}: the seated edge certifies: {e}"))
}

/// The `δ` that lands the seated edge's bound `MARGIN·ε` under ε, with
/// the bound's floor at `δ = 0` it was placed from, or `None` where the run's ε is too close to the bound's floor at δ = 0
/// to place one. The bound is affine in `δ` to well inside the margin
/// (its floor plus a fixed multiple of `δ`), so two readings
/// place it.
fn calibrated_delta(row: &str) -> Option<(f64, f64)> {
    let tol = Tol::witness();
    let eps = tol.eps();
    let band = Band::linear(tol).unwrap();
    // The readings that place δ are taken where any of them certifies.
    let probe = Band::new(64.0 * eps, 640.0 * eps).unwrap();
    let floor = hull_at(0.0, probe).hull_sup;
    println!("rigid map, plane × NURBS: the bound's floor at δ = 0 is {floor:e} at ε = {eps:e}");
    // Pinned at every ε, so a floor that grew cannot turn these rows
    // into a green skip: the stand-down below is only for an ε the
    // pinned floor genuinely reaches.
    assert!(
        floor <= FLOOR,
        "the bound's floor at δ = 0 rose to {floor:e} (pinned at {FLOOR:e})"
    );
    if floor >= eps / 8.0 {
        vacuity::stood_down(
            row,
            &format!(
                "the bound's floor at δ = 0 ({floor:e}) is within 8× of ε = {eps:e}, \
                 so no edge can be placed just under ε here; nothing about a rigid \
                 map is asserted at this ε"
            ),
        );
        return None;
    }
    let half = hull_at(0.5 * eps, probe).hull_sup;
    let delta = 0.5 * eps * (eps * (1.0 - MARGIN) - floor) / (half - floor);
    let placed = hull_at(delta, band).hull_sup;
    assert!(
        placed <= eps && placed >= eps * (1.0 - 2.0 * MARGIN),
        "the seated edge must certify just under ε: hull_sup/ε = {}",
        placed / eps
    );
    Some((delta, floor))
}

/// A two-face lamina: the plane face and the wall face, bounded by two
/// edges that both run the arc, each re-described as the plane × NURBS
/// `Intersection` through the door that mints the class.
fn lamina(delta: f64) -> Body<f64> {
    let tol = Tol::witness();
    let forward = arc(1.0 + delta, false);
    let (p0, p1) = (forward.eval(0.0), forward.eval(1.0));
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(p0, true).unwrap();
    let made = body
        .mev_line(
            topo::MevSite::Lone {
                r#loop: seed.r#loop,
            },
            p1,
            tol,
        )
        .unwrap();
    let first = body.get_edge(made.edge).unwrap().clone();
    let closed = body
        .mef_chord(
            topo::MefSite::Chords {
                he1: first.he_minus,
                he2: first.he_plus,
            },
            tol,
        )
        .unwrap();
    let face_surface = |surface| topo::FaceSurface::New {
        surface,
        sense: true,
    };
    let pl = body
        .set_face_surface(seed.face, face_surface(plane()))
        .unwrap();
    let wl = body
        .set_face_surface(closed.face, face_surface(Surface::Nurbs(Arc::new(wall()))))
        .unwrap();
    for (edge, carrier) in [(made.edge, forward), (closed.edge, arc(1.0 + delta, true))] {
        body.set_edge_curve(
            edge,
            geom_brep::EdgeCurveSpec {
                description: geom_brep::EdgeDescriptionSpec::Intersection {
                    s1: pl,
                    s2: wl,
                    witness: carrier.eval(0.5),
                },
                carrier: Curve3::Nurbs(Arc::new(carrier)),
                param_start: 0.0,
                param_end: 1.0,
            },
            tol,
        )
        .unwrap_or_else(|e| panic!("the seated edge certifies through the attach door: {e}"));
    }
    body
}

/// Rigid maps about four axes at eight angles each.
fn rotations() -> Vec<(String, Affine3<f64>)> {
    let axes = [
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(0.3, -0.4, 0.8),
    ];
    let mut out = Vec::new();
    for axis in axes {
        for k in 1..=8 {
            let angle = f64::from(k) * 0.135;
            out.push((
                format!("({}, {}, {}) by {angle:.3}", axis.x, axis.y, axis.z),
                Affine3::rotation_about_axis(Point3::origin(), axis.normalize(), angle),
            ));
        }
    }
    out
}

/// **Every rigid map moves the body**: the edge's limb 2 re-derives
/// under ε in every frame, the oblique ones included.
#[test]
fn every_rigid_map_moves_an_edge_certified_near_eps() {
    let Some((delta, _)) = calibrated_delta("rigid map, plane × NURBS edge near ε") else {
        return;
    };
    let body = lamina(delta);
    for (name, map) in rotations() {
        topo::transform_rigid(&body, &map, Tol::witness())
            .unwrap_or_else(|e| panic!("{name}: the image of an edge certified near ε moves: {e}"));
    }
}

/// **Nothing in the certificate reads the frame.** Over the same
/// rotations, at a band loose enough that every image certifies and
/// reports its limbs, each limb re-derives within a width that does
/// not scale with the field: limb 1 reads the field itself to within
/// the rounding of the coordinates, the tube re-derives, and limb 2
/// moves by no more than its own floor, the part of the bound the
/// field does not set.
#[test]
fn the_certificate_re_derives_within_rounding_under_the_map() {
    let Some((delta, floor)) = calibrated_delta("rigid map, plane × NURBS limbs") else {
        return;
    };
    let eps = Tol::witness().eps();
    let loose = Band::new(4.0 * eps, 40.0 * eps).unwrap();
    let carrier = arc(1.0 + delta, false);
    // The residual field, closed form: the arc's radius less the
    // wall's, exact (Sterbenz).
    let field = (1.0 + delta) - 1.0;
    let ulp = f64::EPSILON
        * wall()
            .control()
            .iter()
            .map(|p| p.distance(Point3::origin()))
            .fold(0.0, f64::max);
    let off_field = |on_locus: f64| (on_locus - field).abs() / ulp;
    let seated = hull_at(delta, loose);
    assert!(
        off_field(seated.on_locus_max) <= ROUNDING_ULPS,
        "seated: limb 1 reads {:e} against the field's {field:e}, {:.2} ulps off",
        seated.on_locus_max,
        off_field(seated.on_locus_max)
    );
    let (mut worst_limb1, mut worst_limb2): (f64, f64) = (0.0, 0.0);
    for (name, map) in rotations() {
        let Surface::Plane {
            origin,
            normal,
            u_ref,
        } = plane()
        else {
            unreachable!()
        };
        let mapped_plane = Surface::Plane {
            origin: map.transform_point(origin),
            normal: map.linear * normal,
            u_ref: map.linear * u_ref,
        };
        let image = geom_brep::plane_nurbs_limbs::<f64>(
            &carrier.map_points(|p| map.transform_point(p)),
            &mapped_plane,
            &wall().map_points(|p| map.transform_point(p)),
            1.0,
            loose,
        )
        .unwrap_or_else(|e| panic!("{name}: the image certifies at the loose band: {e}"));
        let rel = |a: f64, b: f64| ((a - b) / b).abs();
        // The ladder rung, in metres: the chart pads beside it divide it
        // by the frame's box-norm speeds, which a rotation may move.
        let rung = |t: geom_brep::SsiTube<f64>| match t {
            geom_brep::SsiTube::Chart { rung, .. } => rung,
            other => panic!("{name}: the plane × NURBS lane proves a chart tube: {other:?}"),
        };
        let limb1 = off_field(image.on_locus_max);
        assert!(
            limb1 <= ROUNDING_ULPS,
            "{name}: limb 1 reads {:e} against the field's {field:e}, {limb1:.2} ulps off",
            image.on_locus_max
        );
        assert!(
            rung(image.tube) == rung(seated.tube)
                && rel(image.tube_transversality, seated.tube_transversality) < 1e-6
                && image.tube_boxes == seated.tube_boxes,
            "{name}: the tube is frame-invariant here"
        );
        let limb2 = (image.hull_sup - seated.hull_sup).abs() / floor;
        assert!(
            limb2 <= 1.0,
            "{name}: limb 2 moved {:e} m under a rigid map, {limb2:.3} of its floor \
             {floor:e} (hull_sup/ε {:.6} against {:.6} seated)",
            (image.hull_sup - seated.hull_sup).abs(),
            image.hull_sup / eps,
            seated.hull_sup / eps
        );
        worst_limb1 = worst_limb1.max(limb1);
        worst_limb2 = worst_limb2.max(limb2);
    }
    println!(
        "rigid map, plane × NURBS: limb 1 within {worst_limb1:.2} ulps of the field; \
         limb 2 seated at {:.5} ε, moves by up to {worst_limb2:.3} of its floor",
        seated.hull_sup / eps
    );
}
