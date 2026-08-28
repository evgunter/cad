//! **SHELLFIX PR-2a reviewer R1 probes — head only** (they name
//! `topo::offset_planes_together`, which does not exist at the base).
//!
//! The corner solve attacked directly: a valence-4 planar corner whose
//! four planes DO concur (must solve, all four verified), the same
//! corner under a uniform offset where they do NOT (must refuse typed
//! and never build), and the conditioning meter's own arm.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]

use geom_core::{Band, Point3, Tol, Vec3};
use topo::{Body, ChartMove, FaceKey, ReplaceFaceError};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// The chamfered cube: an all-planar body every one of whose vertices
/// has FOUR distinct planes (one cube face, two strips, one corner
/// patch).
fn chamfered_cube() -> Body<f64> {
    let body = sweep::test_support::cube(1.0, Tol::witness());
    let edges: Vec<topo::EdgeKey> = body.edges().map(|(k, _)| k).collect();
    sweep::chamfer::chamfer_edges(&body, &edges, 0.1, band(), Tol::witness())
        .expect("a cube chamfers")
        .body
}

/// Charts: faces grouped by surface key, with the plane's origin and
/// stored normal.
fn charts(body: &Body<f64>) -> Vec<(Vec<FaceKey>, Point3<f64>, Vec3<f64>)> {
    let mut out: Vec<(topo::SurfaceKey, Vec<FaceKey>, Point3<f64>, Vec3<f64>)> = Vec::new();
    for (k, f) in body.faces() {
        match out.iter_mut().find(|(s, _, _, _)| *s == f.surface) {
            Some((_, v, _, _)) => v.push(k),
            None => {
                let Some(geom::Surface::Plane { origin, normal, .. }) = body.get_surface(f.surface)
                else {
                    panic!("every face here is a plane");
                };
                out.push((f.surface, vec![k], *origin, *normal));
            }
        }
    }
    out.into_iter().map(|(_, v, o, n)| (v, o, n)).collect()
}

fn centroid(body: &Body<f64>) -> Point3<f64> {
    let pts: Vec<Point3<f64>> = body
        .vertices()
        .filter_map(|(k, _)| body.get_vertex(k))
        .filter_map(|v| body.get_point(v.point))
        .copied()
        .collect();
    let n = pts.len() as f64;
    let mut s = Vec3::new(0.0, 0.0, 0.0);
    for p in &pts {
        s = s + Vec3::new(p.x, p.y, p.z);
    }
    Point3::new(s.x / n, s.y / n, s.z / n)
}

fn sorted_points(body: &Body<f64>) -> Vec<(f64, f64, f64)> {
    let mut pts: Vec<(f64, f64, f64)> = body
        .vertices()
        .filter_map(|(k, _)| body.get_vertex(k))
        .filter_map(|v| body.get_point(v.point))
        .map(|p| (p.x, p.y, p.z))
        .collect();
    pts.sort_by(|a, b| a.partial_cmp(b).expect("finite"));
    pts
}

/// The worst |n·(v − o)| over every vertex and every face incident to
/// it — zero exactly when every corner sits on every one of its own
/// planes.
fn worst_incidence(body: &Body<f64>) -> f64 {
    let mut worst: f64 = 0.0;
    for (_, v) in body.vertices() {
        let Some(em) = v.emanating else { continue };
        let p = *body.get_point(v.point).unwrap();
        for he in body.vertex_orbit(em).expect("orbit") {
            let lk = body.get_half_edge(he).unwrap().parent_loop;
            let fk = body.get_loop(lk).unwrap().face;
            let f = body.get_face(fk).unwrap();
            if let Some(geom::Surface::Plane { origin, normal, .. }) = body.get_surface(f.surface) {
                worst = worst.max(normal.dot(p - *origin).abs());
            }
        }
    }
    worst
}

// =====================================================================
// H1. A valence-4 corner whose four planes DO concur: MUST solve, and
// every one of the four must be satisfied by the answer.
//
// Construction: give each chart the offset that scales the whole body
// about a point by λ. Every plane is then the λ-scaled plane, so every
// corner — at any valence — concurs exactly at the λ-scaled vertex.
// =====================================================================

#[test]
fn h1_valence_four_concurring_corners_must_solve() {
    let mut body = chamfered_cube();
    let c = centroid(&body);
    let before = sorted_points(&body);
    let vol0 = topo::mass_properties(&body, Tol::witness()).unwrap().volume;
    let lambda = 0.9;
    let moves: Vec<ChartMove<f64>> = charts(&body)
        .into_iter()
        .map(|(faces, o, n)| ChartMove {
            faces,
            distance: (lambda - 1.0) * n.dot(o - c),
        })
        .collect();
    println!("[h1] {} charts, {} vertices", moves.len(), before.len());
    let r = topo::offset_planes_together(&mut body, &moves, band(), Tol::witness());
    match r {
        Ok(()) => {
            let vol = topo::mass_properties(&body, Tol::witness()).unwrap().volume;
            let after = sorted_points(&body);
            // Every vertex must be the λ-scaled original.
            let mut worst: f64 = 0.0;
            for ((x, y, z), (a, b, d)) in before.iter().zip(after.iter()) {
                let want = (
                    c.x + lambda * (x - c.x),
                    c.y + lambda * (y - c.y),
                    c.z + lambda * (z - c.z),
                );
                worst = worst
                    .max((a - want.0).abs())
                    .max((b - want.1).abs())
                    .max((d - want.2).abs());
            }
            println!(
                "[h1] SOLVES: worst vertex error {worst:.3e}; volume {vol:.17e} vs \
                 lambda^3*V0 {:.17e} (rel {:.3e}); worst incidence {:.3e}; tier3 {:?}",
                lambda.powi(3) * vol0,
                (vol - lambda.powi(3) * vol0).abs() / vol0,
                worst_incidence(&body),
                topo::validate_geometric(&body, Tol::witness()).is_ok()
            );
            assert!(worst < 1e-14, "a concurring valence-4 corner solved wrong");
        }
        Err(e) => panic!("[h1] a concurring valence-4 corner was REFUSED: {e}"),
    }
}

// =====================================================================
// H2. The same corners under a UNIFORM inward offset, where the four
// planes do NOT concur: must refuse typed, and must not build.
// =====================================================================

#[test]
fn h2_valence_four_non_concurring_corners_must_refuse_typed() {
    let mut body = chamfered_cube();
    let c = centroid(&body);
    let before = sorted_points(&body);
    let t = 0.05;
    let moves: Vec<ChartMove<f64>> = charts(&body)
        .into_iter()
        .map(|(faces, o, n)| {
            let h = n.dot(o - c);
            ChartMove {
                faces,
                distance: -t * h.signum(),
            }
        })
        .collect();
    let r = topo::offset_planes_together(&mut body, &moves, band(), Tol::witness());
    match r {
        Ok(()) => panic!("[h2] a NON-concurring valence-4 corner BUILT"),
        Err(e) => {
            println!("[h2] REFUSES: {e}");
            assert!(
                matches!(e, ReplaceFaceError::TogetherCorner { .. }),
                "[h2] the refusal must be the corner gate, got {e}"
            );
            assert_eq!(sorted_points(&body), before, "[h2] the operand moved");
        }
    }
}

// =====================================================================
// H3. The conditioning meter's arm. `|det| * (sum of |distance| over
// the CHARTS NAMED)` is compared against a band in meters, so the
// verdict on a corner is a function of the offset asked for and of how
// many charts the call names — not of the corner.
// =====================================================================

#[test]
fn h3_the_conditioning_meter_is_a_function_of_the_offset() {
    for d in [1e-2, 1e-4, 1e-6, 1e-8, 1e-9, 1e-10, 1e-11] {
        let mut body = sweep::test_support::cube(1.0, Tol::witness());
        let c = centroid(&body);
        let moves: Vec<ChartMove<f64>> = charts(&body)
            .into_iter()
            .map(|(faces, o, n)| {
                let h = n.dot(o - c);
                ChartMove {
                    faces,
                    distance: -d * h.signum(),
                }
            })
            .collect();
        match topo::offset_planes_together(&mut body, &moves, band(), Tol::witness()) {
            Ok(()) => println!("[h3] CUBE, inward {d:e}: solves"),
            Err(e) => println!("[h3] CUBE, inward {d:e}: REFUSES {e}"),
        }
    }
}

// =====================================================================
// H4. The documented-but-unenforced precondition: "every face of the
// body must appear exactly once across [the moves]".
// =====================================================================

#[test]
fn h4_a_face_named_twice() {
    let mut body = sweep::test_support::cube(1.0, Tol::witness());
    let c = centroid(&body);
    let t = 0.05;
    let mut moves: Vec<ChartMove<f64>> = charts(&body)
        .into_iter()
        .map(|(faces, o, n)| {
            let h = n.dot(o - c);
            ChartMove {
                faces,
                distance: -t * h.signum(),
            }
        })
        .collect();
    let dup = ChartMove {
        faces: moves[0].faces.clone(),
        distance: moves[0].distance,
    };
    moves.push(dup);
    let vol0 = topo::mass_properties(&body, Tol::witness()).unwrap().volume;
    match topo::offset_planes_together(&mut body, &moves, band(), Tol::witness()) {
        Ok(()) => {
            let vol = topo::mass_properties(&body, Tol::witness()).unwrap().volume;
            println!(
                "[h4] a face named TWICE was accepted: V0={vol0:.17e} V={vol:.17e}; \
                 the correct inward-{t} cube is {:.17e}; worst incidence {:.3e}; tier3 {:?}",
                (1.0 - 2.0 * t).powi(3),
                worst_incidence(&body),
                topo::validate_geometric(&body, Tol::witness()).is_ok()
            );
        }
        Err(e) => println!("[h4] a face named TWICE refuses: {e}"),
    }
}
