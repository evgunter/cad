//! Review probes for EDIT-PICK (PR 2721, lane `pick-r2`, frozen head
//! `428431ab`): the box-entry guard on `ray_triangle` measured off the
//! corpus, on geometry built for the purpose.
//!
//! Two questions the PR body answers by corpus measurement, asked here
//! by construction:
//!
//! 1. **Is "a refused genuine graze is answered by a sibling" geometry
//!    or fixture luck?** Real cylinders at several tessellation
//!    budgets, axis rays aimed exactly at cap and rim vertices (the
//!    tie-break row's shape); the unguarded predicate (main's kernel)
//!    against the guarded one, over every triangle. A ray whose
//!    unguarded nearest is the aimed vertex and whose guarded nearest
//!    is beyond it (or a miss) is a graze the GUARD lost.
//! 2. **Can a noise `t` land above its entry, inside its box?** Rays in
//!    a triangle's plane, random triangles; a guarded acceptance whose
//!    ray point is nowhere near the triangle is the residual the spec
//!    asked to measure, reached without a corpus.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use bvh::{Aabb, Ray};
use editor_core::resolve::ray_triangle;
use geom_core::{Point2, Point3, Tol, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::{Extrusion, extrude};
use test_utils::fuzz;
use topo::Body;

fn cylinder(r: f64, h: f64) -> Body<f64> {
    let disc = <ProfileLoop<f64> as RawLoop<f64>>::new(vec![
        ProfileVertex::new(Point2::new(-r, 0.0), 1.0),
        ProfileVertex::new(Point2::new(r, 0.0), 1.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![disc])
        .validate(Tol::witness())
        .expect("a disc validates");
    extrude(&profile, Extrusion::Distance(h), Tol::witness())
        .expect("a disc extrudes")
        .body
}

struct Tri {
    corners: [Point3<f64>; 3],
    patch: usize,
}

fn triangles(mesh: &mesh::Mesh) -> Vec<Tri> {
    let mut out = Vec::new();
    for (pi, patch) in mesh.patches.iter().enumerate() {
        for tri in &patch.triangles {
            out.push(Tri {
                corners: tri.map(|i| mesh.positions[i as usize]),
                patch: pi,
            });
        }
    }
    out
}

/// Möller–Trumbore's determinant, as `ray_triangle` computes it.
fn det(ray: &Ray, tri: &[Point3<f64>; 3]) -> f64 {
    let e1: Vec3<f64> = tri[1] - tri[0];
    let e2: Vec3<f64> = tri[2] - tri[0];
    e1.dot(ray.dir.cross(e2))
}

/// `pick_face`'s answer over every triangle — the lexicographic
/// minimum of `(t, flat position)` — with the guard (each candidate's
/// own box entry) or without it (`−∞`, main's kernel).
fn nearest(tris: &[Tri], ray: &Ray, guarded: bool) -> Option<(f64, usize)> {
    let mut best: Option<(f64, usize)> = None;
    for (i, tri) in tris.iter().enumerate() {
        let b = Aabb::from_points(tri.corners).unwrap();
        let Some(t_enter) = ray.slab_enter(&b) else {
            continue;
        };
        let floor = if guarded { t_enter } else { f64::NEG_INFINITY };
        if let Some(t) = ray_triangle(ray, &tri.corners, floor)
            && best.is_none_or(|(bt, bi)| (t, i) < (bt, bi))
        {
            best = Some((t, i));
        }
    }
    best
}

/// **Probe 1.** Axis rays through every cap and rim vertex of real
/// cylinders: the guard must never turn the vertex graze into an
/// answer beyond it. Red = a graze lost to the guard on EVERY
/// incident triangle, with no sibling to answer.
#[test]
fn a_vertex_graze_refused_by_the_guard_is_always_answered_by_a_sibling() {
    let tol = Tol::witness();
    let mut lost = Vec::new();
    let mut refused_but_rescued = 0usize;
    let mut grazes = 0usize;
    let mut rays = 0usize;
    for &(r, h) in &[(0.5, 1.0), (1.0, 2.0), (0.37, 0.61)] {
        let body = cylinder(r, h);
        for &chordal in &[0.05, 0.02, 0.008] {
            let mesh = mesh::tessellate(&body, chordal, tol).expect("tessellates");
            let tris = triangles(&mesh);
            let (lo_z, hi_z) = mesh
                .positions
                .iter()
                .fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), p| {
                    (lo.min(p.z), hi.max(p.z))
                });
            let dirs = [
                Vec3::new(1.0, 0.0, 0.0),
                Vec3::new(-1.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
                Vec3::new(0.0, -1.0, 0.0),
                Vec3::new(0.0, 0.0, 1.0),
                Vec3::new(0.0, 0.0, -1.0),
            ];
            for v in &mesh.positions {
                if v.z != lo_z && v.z != hi_z {
                    continue;
                }
                for dir in dirs {
                    for &reach in &[1.48, 3.0, 2.5] {
                        rays += 1;
                        let ray = Ray {
                            origin: *v - dir * reach,
                            dir,
                        };
                        let unguarded = nearest(&tris, &ray, false);
                        let guarded = nearest(&tris, &ray, true);
                        let Some((ut, ui)) = unguarded else {
                            continue;
                        };
                        // The aimed vertex is a true hit at t = reach;
                        // only rays whose unguarded answer IS that
                        // graze can be lost by the guard.
                        if (ut - reach).abs() > 1e-9 {
                            continue;
                        }
                        grazes += 1;
                        // A genuine (non-noise) graze refused on some
                        // triangle: the guard's own class.
                        let refused_genuine = tris.iter().enumerate().any(|(i, tri)| {
                            let b = Aabb::from_points(tri.corners).unwrap();
                            let Some(t_enter) = ray.slab_enter(&b) else {
                                return false;
                            };
                            let un = ray_triangle(&ray, &tri.corners, f64::NEG_INFINITY);
                            let gu = ray_triangle(&ray, &tri.corners, t_enter);
                            let _ = i;
                            matches!(un, Some(t) if (t - reach).abs() < 1e-9)
                                && gu.is_none()
                                && det(&ray, &tri.corners).abs() > 1e-15
                        });
                        match guarded {
                            Some((gt, _)) if (gt - reach).abs() < 1e-9 => {
                                if refused_genuine {
                                    refused_but_rescued += 1;
                                }
                            }
                            other => lost.push(format!(
                                "r {r} h {h} δ {chordal}: {dir:?} through {v:?} at reach {reach}: \
                                 unguarded {ut} (patch {}, det {:e}); guarded {other:?}",
                                tris[ui].patch,
                                det(&ray, &tris[ui].corners)
                            )),
                        }
                    }
                }
            }
        }
    }
    println!(
        "# {rays} aimed rays; {grazes} graze at the vertex unguarded; {refused_but_rescued} had a \
         genuine graze refused by the guard and a sibling answered; {} lost",
        lost.len()
    );
    assert!(grazes > 100, "the probe reached the graze class: {grazes}");
    assert!(
        lost.is_empty(),
        "{} vertex grazes were lost by the guard on every incident triangle:\n{}",
        lost.len(),
        lost.join("\n")
    );
}

/// Squared distance from `p` to the closed triangle (the standard
/// region walk), for saying whether an accepted ray point is anywhere
/// near the triangle.
fn dist2_to_triangle(p: Point3<f64>, tri: &[Point3<f64>; 3]) -> f64 {
    // Ericson, Real-Time Collision Detection §5.1.5.
    let (a, b, c) = (tri[0], tri[1], tri[2]);
    let ab = b - a;
    let ac = c - a;
    let ap = p - a;
    let d1 = ab.dot(ap);
    let d2 = ac.dot(ap);
    if d1 <= 0.0 && d2 <= 0.0 {
        return ap.norm_squared();
    }
    let bp = p - b;
    let d3 = ab.dot(bp);
    let d4 = ac.dot(bp);
    if d3 >= 0.0 && d4 <= d3 {
        return bp.norm_squared();
    }
    let vc = d1 * d4 - d3 * d2;
    if vc <= 0.0 && d1 >= 0.0 && d3 <= 0.0 {
        let v = d1 / (d1 - d3);
        return (p - (a + ab * v)).norm_squared();
    }
    let cp = p - c;
    let d5 = ab.dot(cp);
    let d6 = ac.dot(cp);
    if d6 >= 0.0 && d5 <= d6 {
        return cp.norm_squared();
    }
    let vb = d5 * d2 - d1 * d6;
    if vb <= 0.0 && d2 >= 0.0 && d6 <= 0.0 {
        let w = d2 / (d2 - d6);
        return (p - (a + ac * w)).norm_squared();
    }
    let va = d3 * d6 - d5 * d4;
    if va <= 0.0 && (d4 - d3) >= 0.0 && (d5 - d6) >= 0.0 {
        let w = (d4 - d3) / ((d4 - d3) + (d5 - d6));
        return (p - (b + (c - b) * w)).norm_squared();
    }
    let denom = 1.0 / (va + vb + vc);
    let v = vb * denom;
    let w = vc * denom;
    (p - (a + ab * v + ac * w)).norm_squared()
}

/// **Probe 2.** A ray in a triangle's plane, the triangle's box entry
/// well below where the ray reaches it: the guard passes a noise `t`
/// at a ray point that is not on the triangle. Red = such an
/// acceptance exists (the spec's "residual", constructed).
#[test]
fn a_noise_t_above_its_entry_passes_the_guard() {
    let mut rng = fuzz::start("pick-r2: in-plane rays against the guarded predicate");
    let mut accepted = 0usize;
    let mut garbage = Vec::new();
    let mut noise_dets = 0usize;
    let cases = fuzz::scaled(20_000);
    for _ in 0..cases {
        let pt = |rng: &mut fuzz::Rng| {
            Point3::new(
                rng.range(-2.0, 2.0),
                rng.range(-2.0, 2.0),
                rng.range(-2.0, 2.0),
            )
        };
        let tri = [pt(&mut rng), pt(&mut rng), pt(&mut rng)];
        let e1 = tri[1] - tri[0];
        let e2 = tri[2] - tri[0];
        // Origin and direction in the plane, up to the rounding of
        // their own construction.
        let origin = tri[0] + e1 * rng.range(-1.5, 2.5) + e2 * rng.range(-1.5, 2.5);
        let dir = e1 * rng.range(-1.0, 1.0) + e2 * rng.range(-1.0, 1.0);
        if dir.norm_squared() < 1e-6 {
            continue;
        }
        let ray = Ray { origin, dir };
        let b = Aabb::from_points(tri).unwrap();
        let Some(t_enter) = ray.slab_enter(&b) else {
            continue;
        };
        let Some(t) = ray_triangle(&ray, &tri, t_enter) else {
            continue;
        };
        accepted += 1;
        let d = det(&ray, &tri);
        if d.abs() < 1e-15 {
            noise_dets += 1;
        }
        let p = ray.origin + ray.dir * t;
        let off = dist2_to_triangle(p, &tri).sqrt();
        if off > 1e-6 {
            garbage.push(format!(
                "det {d:e}: t {t} (entry {t_enter}) at {p:?}, {off:.3} from the triangle {tri:?}"
            ));
        }
    }
    println!(
        "# {cases} in-plane rays: {accepted} accepted by the guarded predicate, {noise_dets} of \
         them at |det| < 1e-15, {} at a point off the triangle",
        garbage.len()
    );
    assert!(
        garbage.is_empty(),
        "{} guarded acceptances at a ray point that is not on the triangle (first five):\n{}",
        garbage.len(),
        garbage.iter().take(5).cloned().collect::<Vec<_>>().join("\n")
    );
}
