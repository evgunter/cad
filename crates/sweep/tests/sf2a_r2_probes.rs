//! VERBS-SHELLFIX PR-2a — R2 blinded-review probes (#1081, PR #1126).
//!
//! Unique-signal attacks on the simultaneous-offset door:
//!
//! 1. a valence-4 corner whose planes do NOT concur after a uniform
//!    inset (the chamfered cube — every vertex is 4-valent, and the
//!    inset of a chamfered cube is not a chamfered cube) must refuse
//!    typed, never build;
//! 2. a valence-4 corner whose planes DO concur (the same cube under a
//!    CRAFTED move set: corner-plane distance chosen so all four
//!    planes meet again) must build, all four planes verified, with
//!    the wall pinned to a closed form only a right corner can hit;
//! 3. the bevel / kite / triangle fixtures re-pinned to closed-form
//!    volumes derived INDEPENDENTLY here (the shipped acceptance pins
//!    only the hexagon — a wrong corner on the other three builds a
//!    valid body and only the volume nets it);
//! 4. the door's documented-but-unenforced preconditions (a face named
//!    twice across moves; one `ChartMove` spanning two planes) — what
//!    actually happens when a caller breaks them.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]

use geom::Surface;
use geom_core::{Band, Point2, Tol};
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};
use sweep::chamfer::chamfer_edges;
use sweep::test_support::cube;
use sweep::{Extrusion, extrude};
use topo::{Body, ChartMove, EdgeKey, FaceKey, ReplaceFaceError, ShellError};

const FIT_TOL: f64 = 1e-6;

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn prism(pts: &[(f64, f64)], h: f64) -> Body<f64> {
    let lp = ProfileLoop::new(
        pts.iter()
            .map(|&(x, y)| ProfileVertex::new(p2(x, y), 0.0))
            .collect(),
    );
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .expect("a polygon is a valid profile");
    extrude(&profile, Extrusion::Distance(h), Tol::witness())
        .expect("a polygon extrudes")
        .body
}

fn all_edges(body: &Body<f64>) -> Vec<EdgeKey> {
    body.edges().map(|(k, _)| k).collect()
}

/// `a³ − 6ad² + (16/3)d³` — the chamfered cube's own closed form
/// (restated from `verbs_chamfer.rs`, where it is derived).
fn chamfered_cube_volume(a: f64, d: f64) -> f64 {
    a.powi(3) - 6.0 * a * d * d + (16.0 / 3.0) * d.powi(3)
}

/// The convex polygon inset by `t`: each edge's line moved inward,
/// vertex `i` re-derived as the crossing of its two adjacent moved
/// lines. Independent of the kernel's arithmetic on purpose.
fn inset(pts: &[(f64, f64)], t: f64) -> Vec<(f64, f64)> {
    let n = pts.len();
    let line = |i: usize| -> (f64, f64, f64, f64) {
        let (px, py) = pts[i];
        let (qx, qy) = pts[(i + 1) % n];
        let (dx, dy) = (qx - px, qy - py);
        let l = dx.hypot(dy);
        // Left normal — inward for a CCW footprint.
        let (nx, ny) = (-dy / l, dx / l);
        (px + t * nx, py + t * ny, dx, dy)
    };
    (0..n)
        .map(|i| {
            let (x0, y0, dx0, dy0) = line((i + n - 1) % n);
            let (x1, y1, dx1, dy1) = line(i);
            let det = dx0 * (-dy1) - (-dx1) * dy0;
            let a = ((x1 - x0) * (-dy1) - (-dx1) * (y1 - y0)) / det;
            (x0 + a * dx0, y0 + a * dy0)
        })
        .collect()
}

fn shoelace(pts: &[(f64, f64)]) -> f64 {
    let n = pts.len();
    (0..n)
        .map(|i| {
            let (x0, y0) = pts[i];
            let (x1, y1) = pts[(i + 1) % n];
            x0 * y1 - x1 * y0
        })
        .sum::<f64>()
        / 2.0
}

// ---------------------------------------------------------------------
// 1. The valence-4 corner that does NOT concur: refuse, never build.
// ---------------------------------------------------------------------

/// **The chamfered cube is the valence-past-3 attack the acceptance
/// never ran**: 24 vertices, 48 edges — every corner is FOUR distinct
/// planes. A uniform inset of a chamfered cube is NOT a chamfered
/// cube (at the vertex `(0, d, d)` the first triple solves to
/// `x + y + z = 2d + t(2√2 − 1)` while the moved corner plane wants
/// `2d + t√3`; `2√2 − 1 ≠ √3`), so `shell` must refuse typed with the
/// concurrence arm — a door that solved the first triple and ASSUMED
/// the fourth would build here, and only this row would know.
#[test]
fn r2a_valence4_nonconcurring_corner_refuses_typed() {
    let tol = Tol::witness();
    let body = cube(1.0, tol);
    let out = chamfer_edges(&body, &all_edges(&body), 0.2, band(), tol)
        .expect("a cube's twelve edges chamfer");
    let chamfered = out.body;
    assert_eq!(
        (chamfered.vertices().count(), chamfered.edges().count()),
        (24, 48),
        "every vertex 4-valent (2E/V = 4)"
    );
    let e = topo::shell(&chamfered, 0.02, FIT_TOL, band(), tol)
        .expect_err("a chamfered cube's corners do not concur under a uniform inset");
    let ShellError::Face { error, .. } = e else {
        panic!("not the offset door's refusal: {e}");
    };
    let ReplaceFaceError::TogetherCorner { planes, what, .. } = *error else {
        panic!("must refuse at the corner gate, got {error}");
    };
    println!("[r2a] chamfered cube: TogetherCorner planes={planes} what={what}");
    assert_eq!(planes, 4, "the corner is valence 4");
    assert!(
        what.contains("concur"),
        "the concurrence arm, not the singular arm: {what}"
    );
    // And the operand is untouched by the refusal.
    assert_eq!(topo::validate_geometric(&chamfered, tol), Ok(()));
}

// ---------------------------------------------------------------------
// 2. The valence-4 corner that DOES concur: build, verified, pinned.
// ---------------------------------------------------------------------

/// **The same 24 four-valent corners made to CONCUR by construction**:
/// square faces moved by `t`, strips by `s`, corner triangles by
/// `c = (2s√2 − t)/√3` — exactly the distance that makes the fourth
/// plane pass through the first triple's solution at every vertex (the
/// cube's symmetry group is transitive on them). The door must BUILD
/// this: 24 corner solves, each verifying a fourth plane at `Zero`.
/// The result is the chamfered cube of `a − 2t` with setback
/// `d + s√2 − 2t`, and the volume is pinned to that closed form —
/// which a corner solved to any wrong point cannot hit.
#[test]
fn r2a_valence4_concurring_corner_builds_in_closed_form() {
    let tol = Tol::witness();
    let (a, d, t, s) = (1.0, 0.2, 0.02, 0.02);
    let c = (2.0 * s * core::f64::consts::SQRT_2 - t) / 3.0_f64.sqrt();
    let body = cube(a, tol);
    let out = chamfer_edges(&body, &all_edges(&body), d, band(), tol)
        .expect("a cube's twelve edges chamfer");
    let mut chamfered = out.body;

    // One ChartMove per face (each face its own plane), the distance
    // signed along the STORED normal so every plane moves INWARD.
    let centroid = {
        let (mut x, mut y, mut z, mut n) = (0.0, 0.0, 0.0, 0.0);
        for (k, _) in chamfered.vertices() {
            let p = chamfered
                .get_vertex(k)
                .and_then(|v| chamfered.get_point(v.point))
                .unwrap();
            x += p.x;
            y += p.y;
            z += p.z;
            n += 1.0;
        }
        (x / n, y / n, z / n)
    };
    let mut moves: Vec<ChartMove<f64>> = Vec::new();
    for (k, f) in chamfered.faces() {
        let Some(Surface::Plane { origin, normal, .. }) = chamfered.get_surface(f.surface) else {
            panic!("a chamfered cube carries planes only");
        };
        let nonzero = [normal.x, normal.y, normal.z]
            .iter()
            .filter(|v| v.abs() > 1e-9)
            .count();
        let dist = match nonzero {
            1 => t, // a shrunk cube face
            2 => s, // a strip
            3 => c, // a corner triangle
            _ => unreachable!(),
        };
        let toward_inside = normal.x * (centroid.0 - origin.x)
            + normal.y * (centroid.1 - origin.y)
            + normal.z * (centroid.2 - origin.z);
        let signed = if toward_inside > 0.0 { dist } else { -dist };
        moves.push(ChartMove {
            faces: vec![k],
            distance: signed,
        });
    }
    topo::offset_planes_together(&mut chamfered, &moves, band(), tol)
        .expect("24 four-valent corners, every fourth plane verified Zero");
    assert_eq!(topo::validate_geometric(&chamfered, tol), Ok(()), "tier 3");
    let props = topo::mass_properties(&chamfered, tol).expect("props");
    let want = chamfered_cube_volume(a - 2.0 * t, d + s * core::f64::consts::SQRT_2 - 2.0 * t);
    println!(
        "[r2a] concurring valence-4 inset: volume {} want {want}",
        props.volume
    );
    assert!(
        (props.volume - want).abs() <= 1e-12,
        "the inset chamfered cube's closed form is {want}, got {}",
        props.volume
    );
}

// ---------------------------------------------------------------------
// 3. The three unpinned oblique prisms, pinned independently.
// ---------------------------------------------------------------------

/// **The shipped acceptance pins only the hexagon's volume; the bevel,
/// kite and triangle rows assert just "hollows, two shells, tier 3" —
/// which a corner solved to a wrong point also satisfies.** Re-pinned
/// here against closed forms derived independently (each footprint
/// inset by `t` via its own adjacent-line crossings, wall = A·h −
/// A_inset·(h − 2t)).
#[test]
fn r2a_bevel_kite_triangle_walls_in_closed_form() {
    let tol = Tol::witness();
    let (t, h) = (0.02, 0.25);
    for (what, pts) in [
        (
            "a box with one bevelled side",
            vec![(0.0, 0.0), (0.4, 0.0), (0.3, 0.3), (0.0, 0.3)],
        ),
        (
            "a kite",
            vec![(0.0, 0.0), (0.2, -0.1), (0.4, 0.0), (0.2, 0.3)],
        ),
        (
            "a 58/58/64 triangle",
            vec![(0.0, 0.0), (0.3, 0.0), (0.15, 0.26)],
        ),
    ] {
        let body = prism(&pts, h);
        let hollow = topo::shell(&body, t, FIT_TOL, band(), tol)
            .unwrap_or_else(|e| panic!("{what} hollows, got {e}"));
        let props = topo::mass_properties(&hollow, tol).expect("props");
        let want = shoelace(&pts) * h - shoelace(&inset(&pts, t)) * (h - 2.0 * t);
        println!("[r2a] {what}: wall {} want {want}", props.volume);
        assert!(
            (props.volume - want).abs() <= 1e-12,
            "{what}: closed form {want}, got {}",
            props.volume
        );
    }
}

// ---------------------------------------------------------------------
// 4. The documented-but-unenforced preconditions of the public door.
// ---------------------------------------------------------------------

/// **`moves` docs say "every face of the body must appear exactly
/// once across them" — only ABSENCE is enforced.** A face named twice
/// (its chart moved twice over) sails through the scope gate; this row
/// measures what the door then does. The body is a box, the duplicate
/// is one extra move of the top chart: if the door builds, the top
/// face's plane has been moved twice while its corners moved once —
/// tier-3 invalidity or a wrong volume, from a public door, silently.
#[test]
fn r2a_a_face_named_twice_across_moves() {
    let tol = Tol::witness();
    let mut body = prism(&[(0.0, 0.0), (2.0, 0.0), (2.0, 3.0), (0.0, 3.0)], 4.0);
    let centroid = (1.0, 1.5, 2.0);
    let mut moves: Vec<ChartMove<f64>> = Vec::new();
    for (k, f) in body.faces() {
        let Some(Surface::Plane { origin, normal, .. }) = body.get_surface(f.surface) else {
            panic!("a box is planes");
        };
        let toward_inside = normal.x * (centroid.0 - origin.x)
            + normal.y * (centroid.1 - origin.y)
            + normal.z * (centroid.2 - origin.z);
        let signed = if toward_inside > 0.0 { 0.25 } else { -0.25 };
        moves.push(ChartMove {
            faces: vec![k],
            distance: signed,
        });
    }
    // The duplicate: the first chart named AGAIN, same distance.
    moves.push(ChartMove {
        faces: moves[0].faces.clone(),
        distance: moves[0].distance,
    });
    let before = topo::mass_properties(&body, tol).expect("props").volume;
    match topo::offset_planes_together(&mut body, &moves, band(), tol) {
        Err(e) => println!("[r2a] duplicate face: refused, {e}"),
        Ok(()) => {
            let tier3 = topo::validate_geometric(&body, tol);
            let vol = topo::mass_properties(&body, tol).map(|p| p.volume);
            println!(
                "[r2a] duplicate face: BUILT (operand was volume {before}); tier3 = {tier3:?}, \
                 volume = {vol:?} (a correct inset would be 1.5·2.5·3.5 = 13.125)"
            );
        }
    }
}

/// **`ChartMove.faces` docs say "they must share one surface key" —
/// nothing checks it.** One move naming faces of TWO different planes
/// re-points the second face at the first's minted surface in the
/// mutation pass. This row measures whether anything downstream
/// refuses before that body is adopted.
#[test]
fn r2a_one_move_spanning_two_planes() {
    let tol = Tol::witness();
    let mut body = prism(&[(0.0, 0.0), (2.0, 0.0), (2.0, 3.0), (0.0, 3.0)], 4.0);
    let centroid = (1.0, 1.5, 2.0);
    let mut per_face: Vec<(FaceKey, f64)> = Vec::new();
    for (k, f) in body.faces() {
        let Some(Surface::Plane { origin, normal, .. }) = body.get_surface(f.surface) else {
            panic!("a box is planes");
        };
        let toward_inside = normal.x * (centroid.0 - origin.x)
            + normal.y * (centroid.1 - origin.y)
            + normal.z * (centroid.2 - origin.z);
        per_face.push((k, if toward_inside > 0.0 { 0.25 } else { -0.25 }));
    }
    // The first move swallows the second face; the rest are honest.
    let mut moves: Vec<ChartMove<f64>> = vec![ChartMove {
        faces: vec![per_face[0].0, per_face[1].0],
        distance: per_face[0].1,
    }];
    for &(k, d) in &per_face[2..] {
        moves.push(ChartMove {
            faces: vec![k],
            distance: d,
        });
    }
    match topo::offset_planes_together(&mut body, &moves, band(), tol) {
        Err(e) => println!("[r2a] two-plane ChartMove: refused, {e}"),
        Ok(()) => {
            let tier3 = topo::validate_geometric(&body, tol);
            let vol = topo::mass_properties(&body, tol).map(|p| p.volume);
            println!("[r2a] two-plane ChartMove: BUILT; tier3 = {tier3:?}, volume = {vol:?}");
        }
    }
}

// ---------------------------------------------------------------------
// 5. Boundary rows: one curved face among planes; a drum stays put.
// ---------------------------------------------------------------------

/// **One curved face among planes takes the per-chart path and still
/// refuses at the old door** — here an OBLIQUE-planar body wearing a
/// cylindrical bore, so the planar corners the new door could solve
/// coexist with a curved chart. The branch predicate is per-BODY, so
/// the whole body must go the old way and refuse where it always did.
#[test]
fn r2a_one_curved_face_among_oblique_planes_refuses_at_the_old_door() {
    let tol = Tol::witness();
    // A hexagonal prism with ONE side bulged into an arc: every flat
    // side oblique to its neighbours (the class the new door fixed),
    // one curved face (the bulged wall) putting the body outside the
    // door.
    let r = 0.2;
    let hex: Vec<(f64, f64)> = (0..6)
        .map(|i| {
            let a = core::f64::consts::TAU * f64::from(i) / 6.0;
            (r * a.cos(), r * a.sin())
        })
        .collect();
    let outer = ProfileLoop::new(
        hex.iter()
            .enumerate()
            .map(|(i, &(x, y))| ProfileVertex::new(p2(x, y), if i == 0 { 0.2 } else { 0.0 }))
            .collect(),
    );
    let profile = Profile::new(SketchPlane::xy(), vec![outer])
        .validate(tol)
        .expect("a one-arc hexagon validates");
    let body = extrude(&profile, Extrusion::Distance(0.25), tol)
        .expect("a one-arc hexagon extrudes")
        .body;
    let curved = body
        .faces()
        .filter(|(_, f)| !matches!(body.get_surface(f.surface), Some(Surface::Plane { .. })))
        .count();
    assert!(curved > 0, "the bore is a curved chart");
    let e = topo::shell(&body, 0.02, FIT_TOL, band(), tol)
        .expect_err("one curved face puts the whole body outside the simultaneous door");
    println!("[r2a] one-arc hexagon: {e}");
    if let ShellError::Face { ref error, .. } = e {
        assert!(
            !matches!(
                **error,
                ReplaceFaceError::TogetherNonPlanar { .. }
                    | ReplaceFaceError::TogetherPartialSet { .. }
                    | ReplaceFaceError::TogetherCorner { .. }
            ),
            "the new door's gates must not fire on the per-chart path: {error}"
        );
    }
}

/// **The straight-footprint-vertex prism, through `shell` itself.**
/// The PR's door table says the coplanar-adjacent corner was "built
/// silently" before and is `TogetherCorner` now — but its acceptance
/// row exercises only the door, called directly. This row measures
/// what the VERB does at head (the base half runs in a separate
/// worktree at the merge base): if the base build was CORRECT (the
/// straight vertex has perpendicular planes only, so the sequential
/// transport lands exactly right), then head has converted a correct
/// silent build into a refusal, and the table row's "built silently"
/// is doing quiet work.
#[test]
fn r2a_straight_vertex_prism_through_shell_at_head() {
    let tol = Tol::witness();
    let body = prism(
        &[(0.0, 0.0), (0.5, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
        0.4,
    );
    match topo::shell(&body, 0.05, FIT_TOL, band(), tol) {
        Ok(hollow) => {
            let props = topo::mass_properties(&hollow, tol).expect("props");
            println!(
                "[r2a] straight-vertex prism at head: HOLLOWS, wall {} (closed form 0.157)",
                props.volume
            );
        }
        Err(e) => println!("[r2a] straight-vertex prism at head: refuses, {e}"),
    }
}

/// **All distances zero refuses as "singular"** — the conditioning
/// meter is `|det|·Σ|dᵢ|`, so a zero total offset zeroes the meter for
/// every triple and the refusal message blames the planes ("every
/// triple ... is singular") on a perfectly conditioned box. Recorded
/// as a measurement of the meter's edge, not asserted as a defect.
#[test]
fn r2a_zero_total_offset_reports_singular() {
    let tol = Tol::witness();
    let mut body = prism(&[(0.0, 0.0), (2.0, 0.0), (2.0, 3.0), (0.0, 3.0)], 4.0);
    let moves: Vec<ChartMove<f64>> = body
        .faces()
        .map(|(k, _)| ChartMove {
            faces: vec![k],
            distance: 0.0,
        })
        .collect();
    let e = topo::offset_planes_together(&mut body, &moves, band(), tol)
        .expect_err("a zero-offset call has a zero meter");
    println!("[r2a] all-zero distances: {e}");
    assert!(
        matches!(e, ReplaceFaceError::TogetherCorner { .. }),
        "the meter refuses at the corner gate: {e}"
    );
}
