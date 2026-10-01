//! **A rigid map can refuse a plane × NURBS edge certified near ε.**
//!
//! The lane's limb 2 (`ssi_hull_sup_chart`) folds the residual
//! composite's three whole-domain per-coordinate sups Euclidean, so its
//! bound for one residual field lies anywhere between the field's sup
//! norm and √3 times it, depending on how the field's directions sit
//! against the coordinate axes. A rotation moves that fold, and the
//! lane re-derives it in the new frame, so an edge certified a few
//! percent under ε re-derives above it.
//!
//! **The subject.** The `z = 1/2` plane against a rational
//! quarter-cylinder wall of radius 1, with the carrier the exact
//! rational arc of radius `1 + δ` — a residual field of exactly `δ`,
//! pointing radially. The pair is seated symmetric about the x axis
//! (the arc spans ±45°), where the fold reads `√(1 + ½)·δ`; a rotation
//! that turns the arc's chord off the axes reads up to `√2·δ`. `δ` is
//! calibrated from the lane's own bound so the edge certifies at
//! `(1 − 1/32)·ε`.
//!
//! Open as
//! `work/ssi/a-rigid-map-re-derives-the-plane-nurbs-edge-certificate-in-a-frame-that-moves-it.md`;
//! the rows here pin the reproduction and are ignored until that row
//! picks a remedy:
//!
//!     cargo test -p topo --test all rigid_map_near_eps_plane_nurbs -- --ignored --nocapture

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use geom::{Curve3, NurbsCurve3, NurbsSurface, Surface};
use geom_core::spline::KnotVector;
use geom_core::{Affine3, Band, Point3, Tol, Vec3};
use topo::Body;

const W: f64 = core::f64::consts::FRAC_1_SQRT_2;

/// Where the bound is placed under ε, as a fraction of ε.
const MARGIN: f64 = 1.0 / 32.0;

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

/// The `δ` that lands the seated edge's bound `MARGIN·ε` under ε. The
/// bound is affine in `δ` to well inside the margin (a rounding floor
/// plus a fixed multiple of `δ`), so two readings place it.
fn calibrated_delta() -> f64 {
    let tol = Tol::witness();
    let eps = tol.eps();
    let band = Band::linear(tol).unwrap();
    let floor = hull_at(0.0, band).hull_sup;
    assert!(
        floor < eps / 8.0,
        "the subject needs ε well above the bound's rounding floor ({floor:e} at ε = {eps:e}); \
         run it at the default ε or a coarser one"
    );
    let half = hull_at(0.5 * eps, band).hull_sup;
    let delta = 0.5 * eps * (eps * (1.0 - MARGIN) - floor) / (half - floor);
    let placed = hull_at(delta, band).hull_sup;
    assert!(
        placed <= eps && placed >= eps * (1.0 - 2.0 * MARGIN),
        "the seated edge must certify just under ε: hull_sup/ε = {}",
        placed / eps
    );
    delta
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
        body.set_edge_curve_nurbs_lane(
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

/// Whether `e` is the lane's limb-2 refusal: definite above the band,
/// or in it.
fn is_hull_refusal(e: &topo::TransformError) -> bool {
    match e {
        topo::TransformError::Certify { source, .. } => match source {
            geom_brep::CertifyError::PlaneNurbs(geom_brep::PlaneNurbsRefusal::Limb {
                limb,
                ..
            }) => *limb == geom_brep::SsiLimb::HullSup,
            geom_brep::CertifyError::Escalated { check, .. } => {
                *check == geom_brep::CertCheck::PlaneNurbsHull
            }
            _ => false,
        },
        _ => false,
    }
}

/// **The reproduction.** The body moves through the transform under
/// the rotations about x, which keep the arc's chord on an axis, and
/// refuses on limb 2 under oblique ones.
#[test]
#[ignore = "pins an open SSI row; run command in the module docs"]
fn the_image_of_an_edge_certified_near_eps_refuses_re_derivation() {
    let body = lamina(calibrated_delta());
    let mut refused = Vec::new();
    let mut moved = 0;
    for (name, map) in rotations() {
        match topo::transform_rigid(&body, &map, Tol::witness()) {
            Ok(_) => moved += 1,
            Err(e) if is_hull_refusal(&e) => refused.push(name),
            Err(e) => panic!("{name}: only limb 2 moves here: {e}"),
        }
    }
    println!(
        "rigid map, plane × NURBS: {moved} maps move the body, {} refuse on limb 2: {refused:?}",
        refused.len()
    );
    assert!(
        !refused.is_empty(),
        "no rotation moved the bound past ε, so the edge does not reproduce the refusal"
    );
}

/// **Which term moves.** Over the same rotations, at a band loose
/// enough that every image certifies: the sampled on-locus residual and
/// every tube limb are frame-invariant to the printed digits, and limb 2
/// moves with the fold of the true residual field's per-coordinate sups.
#[test]
#[ignore = "pins an open SSI row; run command in the module docs"]
fn only_the_folded_hull_bound_moves_under_the_map() {
    let eps = Tol::witness().eps();
    let delta = calibrated_delta();
    let loose = Band::new(4.0 * eps, 40.0 * eps).unwrap();
    let carrier = arc(1.0 + delta, false);
    let seated = hull_at(delta, loose);
    let mut worst: f64 = 0.0;
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
        // The true residual field is δ along the outward radius; its
        // per-coordinate sups in the mapped frame, folded Euclidean.
        let mut sups = [0.0f64; 3];
        for i in 0..=4000 {
            let p = carrier.eval(f64::from(i) / 4000.0);
            let r = map.linear * (Vec3::new(p.x, p.y, 0.0).normalize() * delta);
            for (s, c) in sups.iter_mut().zip([r.x, r.y, r.z]) {
                *s = s.max(c.abs());
            }
        }
        let fold = sups.iter().map(|s| s * s).sum::<f64>().sqrt();
        println!(
            "{name}: hull_sup/ε {:.5}, fold/δ {:.5}, hull_sup/fold {:.5}",
            image.hull_sup / eps,
            fold / delta,
            image.hull_sup / fold
        );
        let rel = |a: f64, b: f64| ((a - b) / b).abs();
        // The ladder rung, in metres: the chart pads beside it divide it
        // by the frame's box-norm speeds, which a rotation may move.
        let rung = |t: geom_brep::SsiTube<f64>| match t {
            geom_brep::SsiTube::Chart { rung, .. } => rung,
            other => panic!("{name}: the plane × NURBS lane proves a chart tube: {other:?}"),
        };
        assert!(
            rel(image.on_locus_max, seated.on_locus_max) < 1e-6
                && rung(image.tube) == rung(seated.tube)
                && rel(image.tube_transversality, seated.tube_transversality) < 1e-6
                && image.tube_boxes == seated.tube_boxes,
            "{name}: limb 1 and the tube are frame-invariant here"
        );
        worst = worst.max(image.hull_sup / seated.hull_sup);
    }
    println!("rigid map, plane × NURBS: limb 2 grows by up to {worst:.4}×");
    assert!(
        worst > 1.0 / (1.0 - MARGIN),
        "some rotation moves limb 2 by more than the margin"
    );
}
