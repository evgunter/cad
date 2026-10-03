//! **Which declared pairs meet along a curve.** A seam declaration names
//! two faces that touch along a curve, and the declaration door refuses
//! a pair that touches along none of it (`seam_locus_untouched`). A
//! suite that declares a seam over two FACE SETS (two half-walls
//! against two half-caps) therefore names only the pairs that meet:
//! [`meeting`] keeps those, read here by the suite, not by the kernel.
//!
//! A pair meets when, either way round:
//!
//! - **the two faces share a stretch of a boundary curve**: an edge of
//!   each on one line or one circle, their spans overlapping by more
//!   than a nanometre (exact, in closed form);
//! - **they share a vertex** (a half-wall and the half-cap across the
//!   rim's vertex, which the door reads at that point); or
//! - **a boundary edge of one runs inside the other face** (the stadium:
//!   the slab's edge lies on the rod wall's interior). This one is
//!   SAMPLED, five points per edge, on the face's own containment door.

use geom::SurfaceKind;
use geom_core::{Band, Point3, Tol, Vec3};
use topo::{Body, BooleanCoincidence, BooleanDeclarations, FaceContainment, FaceKey};

/// `decls` (A faces from `x`, B faces from `y`) with every seam pair
/// whose faces do not meet dropped; every other pair kept.
pub fn meeting(
    x: &Body<f64>,
    y: &Body<f64>,
    mut decls: BooleanDeclarations,
) -> BooleanDeclarations {
    decls
        .coincident_faces
        .retain(|d| d.class != BooleanCoincidence::Seam || meets(x, d.a, y, d.b));
    decls
}

/// Whether faces `fa` of `x` and `fb` of `y` meet along a curve.
pub fn meets(x: &Body<f64>, fa: FaceKey, y: &Body<f64>, fb: FaceKey) -> bool {
    let (ea, eb) = (edges(x, fa), edges(y, fb));
    let ends = |e: &[geom_brep::EdgeCurve<f64>]| -> Vec<Point3<f64>> {
        e.iter()
            .flat_map(|c| {
                let (t0, t1) = c.params();
                [c.carrier().eval(t0), c.carrier().eval(t1)]
            })
            .collect()
    };
    let (va, vb) = (ends(&ea), ends(&eb));
    ea.iter().any(|a| eb.iter().any(|b| share_stretch(a, b)))
        || va
            .iter()
            .any(|p| vb.iter().any(|q| (*p - *q).norm() < 1e-9))
        || inside_points(&ea).into_iter().any(|p| on_face(y, fb, p))
        || inside_points(&eb).into_iter().any(|p| on_face(x, fa, p))
}

/// The certified curves of `face`'s boundary edges.
fn edges(b: &Body<f64>, face: FaceKey) -> Vec<geom_brep::EdgeCurve<f64>> {
    b.edges()
        .filter(|(_, e)| {
            [e.he_plus, e.he_minus]
                .into_iter()
                .any(|h| b.face_of_half_edge(h) == Some(face))
        })
        .filter_map(|(_, e)| {
            b.get_curve_geom(e.curve)
                .and_then(|g| g.certified())
                .cloned()
        })
        .collect()
}

/// Five points inside each curve.
fn inside_points(curves: &[geom_brep::EdgeCurve<f64>]) -> Vec<Point3<f64>> {
    curves
        .iter()
        .flat_map(|c| {
            let (t0, t1) = c.params();
            (1..6).map(move |k| c.carrier().eval(t0 + (t1 - t0) * f64::from(k) / 6.0))
        })
        .collect()
}

/// Whether two edges lie on one line or one circle with spans that
/// overlap by more than a nanometre.
fn share_stretch(a: &geom_brep::EdgeCurve<f64>, b: &geom_brep::EdgeCurve<f64>) -> bool {
    const EPS: f64 = 1e-9;
    let tau = core::f64::consts::TAU;
    match (a.carrier(), b.carrier()) {
        (geom::Curve3::Line { origin, dir }, geom::Curve3::Line { .. }) => {
            let d = *dir / dir.norm();
            let off = |p: Point3<f64>| {
                let w = p - *origin;
                (w - d * w.dot(d)).norm()
            };
            let span = |c: &geom_brep::EdgeCurve<f64>| {
                let (t0, t1) = c.params();
                let (u, w) = (
                    (c.carrier().eval(t0) - *origin).dot(d),
                    (c.carrier().eval(t1) - *origin).dot(d),
                );
                (u.min(w), u.max(w))
            };
            let (b0, b1) = b.params();
            if off(b.carrier().eval(b0)) > EPS || off(b.carrier().eval(b1)) > EPS {
                return false;
            }
            let ((l0, h0), (l1, h1)) = (span(a), span(b));
            h0.min(h1) - l0.max(l1) > EPS
        }
        (
            geom::Curve3::Circle {
                center,
                axis,
                radius,
                u_ref,
            },
            geom::Curve3::Circle {
                center: c2,
                axis: a2,
                radius: r2,
                ..
            },
        ) => {
            if (*center - *c2).norm() > EPS
                || axis.cross(*a2).norm() > EPS
                || (radius - r2).abs() > EPS
            {
                return false;
            }
            let v: Vec3<f64> = axis.cross(*u_ref);
            let angle = |p: Point3<f64>| {
                let w = p - *center;
                w.dot(v).atan2(w.dot(*u_ref)).rem_euclid(tau)
            };
            // Each arc as `[lo, lo + len]` in the first circle's sense.
            let arc = |c: &geom_brep::EdgeCurve<f64>, own_axis: Vec3<f64>| {
                let (t0, t1) = c.params();
                let len = t1 - t0;
                let lo = if own_axis.dot(*axis) < 0.0 {
                    angle(c.carrier().eval(t1))
                } else {
                    angle(c.carrier().eval(t0))
                };
                (lo, lo + len)
            };
            let ((l0, h0), (l1, h1)) = (arc(a, *axis), arc(b, *a2));
            [-tau, 0.0, tau]
                .iter()
                .any(|k| (h0.min(h1 + k) - l0.max(l1 + k)) * radius > EPS)
        }
        _ => false,
    }
}

/// Whether `p` lies on `face`: on its carrier, and in or on its region.
fn on_face(b: &Body<f64>, face: FaceKey, p: Point3<f64>) -> bool {
    let band = Band::linear(Tol::witness()).expect("the witness band");
    let Some(s) = b.get_face(face).and_then(|f| b.get_surface(f.surface)) else {
        return false;
    };
    if geom_brep::implicit_residual(s, p).abs() > 1e-9 {
        return false;
    }
    let read = match s {
        geom::Surface::Plane { normal, .. } => topo::contfp(b, face, *normal, p, band).ok(),
        _ if s.kind() != SurfaceKind::Plane => topo::curved_face_containment(b, face, p, band)
            .ok()
            .flatten(),
        _ => None,
    };
    matches!(read, Some(r) if r != FaceContainment::Out)
}
