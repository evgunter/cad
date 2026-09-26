//! The exact ray/triangle test off the corpus, on geometry built for
//! the purpose. These rows came in as review probes (branch
//! `review/pick-r2`) against a box-entry guard the unit withdrew;
//! each now asserts the behaviour of the certified determinant.
//!
//! 1. **Real cylinders, axis rays aimed exactly at cap and rim
//!    vertices** (the tie-break row's shape): no candidate whose
//!    determinant is genuinely non-zero is refused at the
//!    certification, and the aimed vertex is answered at `reach`.
//! 2. **Rays in a triangle's plane**: an accepted candidate whose
//!    ray point is not on the triangle is the noise answer the
//!    certification exists to refuse — zero of them.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::pick::{AXES, aimed, det_and_conditioning};
use bvh::Ray;
use editor_core::resolve::{TSpan, crossing, ray_triangle};
use geom_core::{Point2, Point3, Tol};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::{Extrusion, extrude};
use test_utils::fuzz;
use topo::Body;

fn cylinder(r: f64, h: f64) -> Body<f64> {
    let disc = bulge_loop(vec![
        (Point2::new(-r, 0.0), 1.0),
        (Point2::new(r, 0.0), 1.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![disc])
        .validate(Tol::witness())
        .expect("a disc validates");
    extrude(&profile, Extrusion::Distance(h), Tol::witness())
        .expect("a disc extrudes")
        .body
}

fn triangles(mesh: &mesh::Mesh) -> Vec<[Point3<f64>; 3]> {
    let mut out = Vec::new();
    for patch in &mesh.patches {
        for tri in &patch.triangles {
            out.push(tri.map(|i| mesh.positions[i as usize]));
        }
    }
    out
}

/// `pick_face`'s certified tie over every triangle:
/// [`TSpan::survivors`] called, not restated. The door answers one of
/// these when they are one face and refuses with them when they are
/// several; either way a graze is ANSWERED FOR when the aimed point
/// is one of their parameters.
fn survivors(tris: &[[Point3<f64>; 3]], ray: &Ray) -> Vec<(f64, usize)> {
    let hits: Vec<(TSpan, usize)> = tris
        .iter()
        .enumerate()
        .filter_map(|(i, tri)| ray_triangle(ray, tri).map(|s| (s, i)))
        .collect();
    let spans: Vec<TSpan> = hits.iter().map(|&(s, _)| s).collect();
    TSpan::survivors(&spans)
        .into_iter()
        .map(|w| (hits[w].0.t, hits[w].1))
        .collect()
}

/// **Row 1.** Axis rays through every cap and rim vertex of real
/// cylinders at three budgets: a candidate whose conditioning is at
/// or above `1e-12` — four orders above the certification's own
/// bound — is never refused at the determinant, and enough of the
/// aimed rays are answered at the vertex for the row to have reached
/// the graze class. A deterministic enumeration, not a search: the
/// floor is a witness count over a fixed corpus.
#[test]
fn no_genuine_determinant_is_refused_on_a_cylinders_vertex_grazes() {
    let tol = Tol::witness();
    let mut grazes = 0usize;
    let mut rays = 0usize;
    let mut refused_genuine = Vec::new();
    let mut refused_at_det = 0usize;
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
            for v in &mesh.positions {
                if v.z != lo_z && v.z != hi_z {
                    continue;
                }
                for dir in AXES {
                    for &reach in &[1.48, 3.0, 2.5] {
                        rays += 1;
                        let ray = aimed(*v, dir, reach);
                        for tri in &tris {
                            let (det, cond) = det_and_conditioning(&ray, tri);
                            let certified = crossing(&ray, tri).is_some();
                            if !certified && det != 0.0 {
                                refused_at_det += 1;
                            }
                            if cond >= 1e-12 && !certified {
                                refused_genuine.push(format!(
                                    "r {r} h {h} δ {chordal}: {dir:?} through {v:?}: det {det:e} \
                                     (conditioning {cond:e}) refused on {tri:?}"
                                ));
                            }
                        }
                        if survivors(&tris, &ray)
                            .iter()
                            .any(|&(t, _)| (t - reach).abs() < 1e-9)
                        {
                            grazes += 1;
                        }
                    }
                }
            }
        }
    }
    println!(
        "# {rays} aimed rays; {grazes} answered at the aimed vertex; {refused_at_det} candidates \
         refused at the determinant"
    );
    assert!(grazes > 100, "the row reached the graze class: {grazes}");
    assert!(
        refused_genuine.is_empty(),
        "{} candidates with a genuine determinant refused at the certification:\n{}",
        refused_genuine.len(),
        refused_genuine.join("\n")
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

/// **Row 2.** A ray in a triangle's plane (origin and direction
/// built from the triangle's own edges, so out of the plane only by
/// the rounding of that construction): an acceptance is a noise
/// answer unless its ray point lies on the triangle. Zero acceptances
/// off the triangle. Shape: counterexample search (varying seed,
/// effort dial).
#[test]
fn an_in_plane_ray_never_answers_a_point_off_the_triangle() {
    let mut rng = fuzz::start("pick: in-plane rays against the certified determinant");
    let mut accepted = 0usize;
    let mut refused = 0usize;
    let mut garbage = Vec::new();
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
        let origin = tri[0] + e1 * rng.range(-1.5, 2.5) + e2 * rng.range(-1.5, 2.5);
        let dir = e1 * rng.range(-1.0, 1.0) + e2 * rng.range(-1.0, 1.0);
        if dir.norm_squared() < 1e-6 {
            continue;
        }
        let ray = Ray { origin, dir };
        let Some(span) = ray_triangle(&ray, &tri) else {
            refused += 1;
            continue;
        };
        accepted += 1;
        let t = span.t;
        let p = ray.origin + ray.dir * t;
        let off = dist2_to_triangle(p, &tri).sqrt();
        if off > 1e-6 {
            let (det, cond) = det_and_conditioning(&ray, &tri);
            garbage.push(format!(
                "det {det:e} (conditioning {cond:e}): t {t} at {p:?}, {off:.3} from the \
                 triangle {tri:?}; {}",
                fuzz::replay()
            ));
        }
    }
    println!(
        "# {cases} in-plane rays: {refused} refused, {accepted} accepted, {} at a point off \
         the triangle",
        garbage.len()
    );
    assert!(
        garbage.is_empty(),
        "{} acceptances at a ray point that is not on the triangle (first five):\n{}",
        garbage.len(),
        garbage
            .iter()
            .take(5)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n")
    );
}
