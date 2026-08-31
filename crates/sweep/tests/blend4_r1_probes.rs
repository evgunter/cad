//! **BLEND-4 R1 review probes** — adversarial pins against PR #1360's
//! claims, self-contained (fixtures re-authored through the public
//! API).
//!
//! - P1: the OBLIQUE all-concave carve. Every e2e fixture the unit
//!   ships is axis-aligned (the oblique trihedron appears only in the
//!   unit-level measurement rows), so the pose where a wrong-way arc,
//!   a wrong-side foot or a mis-chirality chart would hide is the one
//!   no fixture reaches. A skewed (60°) parallelogram cavity's twelve
//!   concave edges carve end to end, tier-3 valid, watertight, and the
//!   certified volume matches a Steiner closed form computed HERE from
//!   the polygon itself (inner offset polytope ⊕ ball), at 1e-12
//!   relative.
//! - P2: the degeneracy funnel (the PR's "no sibling degeneracy"
//!   claim). The same cavity at successively slimmer skews walks the
//!   corner determinant toward zero: every request must either carve
//!   tier-3 valid or refuse TYPED — no panic, no NaN body, no silent
//!   garbage — which is exactly the "funnels to metered refusals"
//!   sentence, executed at poses the unit never tried.
//! - P3: the chamfer differential digest. BLEND-4 edits the corner
//!   path BLEND-3 built; this row freezes the chamfered vented cavity
//!   and the chamfered cube to a bit-level digest of their carved
//!   geometry (sorted point bits, census, volume bits) so any drift of
//!   the chamfer under the fillet's widening is a red, not a diff
//!   nobody ran. The pinned digests were measured at the merge base
//!   f106e96d and re-measured identical at the review head.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Mat3, Point2, Point3, Tol, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::blend::build::fillet_edges;
use sweep::chamfer::chamfer_edges;
use sweep::test_support::cube;
use sweep::{Extrusion, extrude};
use topo::{Body, EdgeKey, subtract, validate, validate_closed};

/// An extruded polygonal prism between two z planes, authored the way
/// a user would.
fn prism(pts: &[Point2<f64>], z0: f64, z1: f64) -> Body<f64> {
    let lp = ProfileLoop::polygon(pts.iter().copied());
    let plane = SketchPlane::new(Affine3::from_parts(
        Mat3::from_cols(Vec3::unit_x(), Vec3::unit_y(), Vec3::unit_z()),
        Point3::new(0.0, 0.0, z0) - Point3::origin(),
    ));
    let profile = Profile::new(plane, vec![lp])
        .validate(Tol::witness())
        .expect("a convex polygon is a valid profile");
    extrude(&profile, Extrusion::Distance(z1 - z0), Tol::witness())
        .expect("a prism extrudes")
        .body
}

/// A circular rod (two half-arc segments), for the vent.
fn rod(center: Point2<f64>, r: f64, z0: f64, z1: f64) -> Body<f64> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(Point2::new(center.x - r, center.y), 1.0),
        ProfileVertex::new(Point2::new(center.x + r, center.y), 1.0),
    ]);
    let plane = SketchPlane::new(Affine3::from_parts(
        Mat3::from_cols(Vec3::unit_x(), Vec3::unit_y(), Vec3::unit_z()),
        Point3::new(0.0, 0.0, z0) - Point3::origin(),
    ));
    let profile = Profile::new(plane, vec![lp])
        .validate(Tol::witness())
        .expect("a circle is a valid profile");
    extrude(&profile, Extrusion::Distance(z1 - z0), Tol::witness())
        .expect("a rod extrudes")
        .body
}

fn cut(a: &Body<f64>, b: &Body<f64>) -> Body<f64> {
    subtract(a, b, Tol::witness())
        .expect("the cut succeeds")
        .body()
        .expect("the cut leaves material")
        .body
        .clone()
}

/// The parallelogram cavity's cross-section at skew angle `theta`
/// (radians), side `s`, anchored at `a`.
fn skew_quad(a: Point2<f64>, s: f64, theta: f64) -> [Point2<f64>; 4] {
    let d = Point2::new(s * theta.cos(), s * theta.sin());
    [
        a,
        Point2::new(a.x + s, a.y),
        Point2::new(a.x + s + d.x, a.y + d.y),
        Point2::new(a.x + d.x, a.y + d.y),
    ]
}

/// The skewed vented cavity: block `[0,4]³`, parallelogram cavity
/// prism `z ∈ [1,3]`, round vent from the cavity centroid clear of the
/// top. One shell, twelve concave cavity edges, eight all-concave but
/// OBLIQUE trihedra (corner determinant `sin θ`).
fn skewed_cavity(s: f64, theta: f64, vent_r: f64) -> (Body<f64>, [Point2<f64>; 4]) {
    let quad = skew_quad(Point2::new(1.0, 1.0), s, theta);
    let block = prism(
        &[
            Point2::new(0.0, 0.0),
            Point2::new(4.0, 0.0),
            Point2::new(4.0, 4.0),
            Point2::new(0.0, 4.0),
        ],
        0.0,
        4.0,
    );
    let centroid = Point2::new(
        (quad[0].x + quad[1].x + quad[2].x + quad[3].x) / 4.0,
        (quad[0].y + quad[1].y + quad[2].y + quad[3].y) / 4.0,
    );
    let vent = rod(centroid, vent_r, 2.5, 5.0);
    let cavity = prism(&quad, 1.0, 3.0);
    let vented = cut(&block, &vent);
    (cut(&vented, &cavity), quad)
}

/// The cavity's twelve edges, found by their endpoints — both at
/// corners of the cavity prism — never by index.
fn cavity_edges(body: &Body<f64>, quad: &[Point2<f64>; 4]) -> Vec<EdgeKey> {
    let corner = |q: Point3<f64>| {
        ((q.z - 1.0).abs() < 1e-9 || (q.z - 3.0).abs() < 1e-9)
            && quad
                .iter()
                .any(|c| (q.x - c.x).abs() < 1e-9 && (q.y - c.y).abs() < 1e-9)
    };
    let mut found: Vec<EdgeKey> = body
        .edges()
        .filter(|(k, _)| {
            let Some(e) = body.get_edge(*k) else {
                return false;
            };
            let pt = |vk| {
                body.get_vertex(vk)
                    .and_then(|x| body.get_point(x.point))
                    .copied()
            };
            let (Some(a), Some(b)) = (
                body.get_half_edge(e.he_plus).map(|h| h.start).and_then(pt),
                body.half_edge_end(e.he_plus).and_then(pt),
            ) else {
                return false;
            };
            corner(a) && corner(b)
        })
        .map(|(k, _)| k)
        .collect();
    found.sort_unstable();
    found
}

/// The Steiner volume of the rounded cavity void: the inner offset
/// polytope (every wall moved inward by `r`) Minkowski-summed with the
/// ball. Computed from the polygon itself, term by term —
/// `V + S·r + r²·Σ_e L_e·θ_e/2 + (4π/3)r³` — with nothing shared with
/// the kernel's own integrator.
fn rounded_void_volume(quad: &[Point2<f64>; 4], z0: f64, z1: f64, r: f64) -> f64 {
    // Interior angles of the parallelogram at each vertex.
    let ang = |i: usize| {
        let p = quad[i];
        let a = quad[(i + 3) % 4];
        let b = quad[(i + 1) % 4];
        let u = Vec3::new(a.x - p.x, a.y - p.y, 0.0).normalize();
        let v = Vec3::new(b.x - p.x, b.y - p.y, 0.0).normalize();
        u.dot(v).clamp(-1.0, 1.0).acos()
    };
    // The inner offset polygon: each side shifted inward by r keeps
    // the angles and shortens each side by r·(cot(α/2) + cot(β/2)).
    let side = |i: usize| {
        let p = quad[i];
        let q = quad[(i + 1) % 4];
        ((q.x - p.x).powi(2) + (q.y - p.y).powi(2)).sqrt()
    };
    let shrunk: Vec<f64> = (0..4)
        .map(|i| {
            let a = ang(i);
            let b = ang((i + 1) % 4);
            side(i) - r * (1.0 / (a / 2.0).tan() + 1.0 / (b / 2.0).tan())
        })
        .collect();
    for s in &shrunk {
        assert!(*s > 0.0, "the inner offset polygon survives at this r");
    }
    // Parallelogram: opposite sides equal, area = s0·s1·sin θ with θ
    // the interior angle at vertex 1 (between side 0 reversed and
    // side 1).
    let theta = ang(1);
    assert!((shrunk[0] - shrunk[2]).abs() < 1e-12);
    assert!((shrunk[1] - shrunk[3]).abs() < 1e-12);
    let (s0, s1) = (shrunk[0], shrunk[1]);
    let h = (z1 - z0) - 2.0 * r;
    let area = s0 * s1 * theta.sin();
    let perimeter = 2.0 * (s0 + s1);
    let volume = area * h;
    let surface = 2.0 * area + perimeter * h;
    // Edge term: Σ L_e·θ_e/2 over the inner polytope's edges, θ_e the
    // exterior dihedral. Cap edges: θ = π/2, total length 2·perimeter.
    // Vertical edges: θ = π − interior angle, one per polygon vertex,
    // length h.
    let edge_term = (2.0 * perimeter) * (core::f64::consts::PI / 2.0) / 2.0
        + (0..4)
            .map(|i| h * (core::f64::consts::PI - ang(i)))
            .sum::<f64>()
            / 2.0;
    volume + surface * r + edge_term * r * r + 4.0 * core::f64::consts::PI * r.powi(3) / 3.0
}

/// **P1 — the oblique all-concave carve, volume-certified.** The
/// skewed cavity's eight corners are oblique concave trihedra
/// (`det = sin 60° ≈ 0.866`); all twelve edges fillet end to end and
/// the certified volume matches the here-derived Steiner form at
/// 1e-12 relative. A wrong-way corner arc, a foot on the wrong side of
/// an oblique wall, or a chart mirrored with the wrong chirality
/// cannot hit this closed form.
#[test]
fn p1_an_oblique_all_concave_cavity_carves_to_its_own_steiner_form() {
    let r = 0.15;
    let theta = core::f64::consts::PI / 3.0;
    let (body, quad) = skewed_cavity(1.8, theta, 0.25);
    let edges = cavity_edges(&body, &quad);
    assert_eq!(edges.len(), 12, "the skewed cavity's twelve concave edges");

    let before = topo::mass_properties(&body, Tol::witness())
        .expect("closed-form props")
        .volume;
    let out = fillet_edges(&body, &edges, r, Tol::witness())
        .expect("the oblique all-concave cavity fillets");
    assert_eq!(validate(&out.body), Ok(()), "tier 1");
    assert_eq!(validate_closed(&out.body), Ok(()), "tier 2");
    assert_eq!(
        topo::validate_geometric(&out.body, Tol::witness()),
        Ok(()),
        "tier 3"
    );
    assert_eq!(out.blend_faces.len(), 12, "one cove band per edge");
    assert_eq!(out.corner_faces.len(), 8, "one octant per oblique corner");

    // The un-rounded cavity void above z=1 (prism ∩ material zone),
    // measured off `before` so the closed form needs only the DELTA:
    // filleting adds material equal to (prism void − rounded void).
    let prism_void = {
        let s = 1.8;
        (s * s * theta.sin()) * 2.0
    };
    let added = prism_void - rounded_void_volume(&quad, 1.0, 3.0, r);
    let want = before + added;
    let after = topo::mass_properties(&out.body, Tol::witness())
        .expect("closed-form props")
        .volume;
    assert!(
        (after - want).abs() <= 1e-12 * want,
        "the oblique concave fillet's volume: got {after}, want {want}"
    );

    let mesh = mesh::tessellate(&out.body, 5e-3, Tol::witness()).expect("tessellates");
    mesh::validate::check_mesh(&mesh).expect("watertight");
}

/// **P2 — the degeneracy funnel, executed at poses the unit never
/// tried.** Slimmer and slimmer skews drive the corner determinant
/// toward zero. At every pose the public verb must either carve
/// tier-3 valid or refuse TYPED — the funnel the PR claims for every
/// division on the fillet corner path. A panic, a NaN volume, or an
/// invalid carved body at any pose is the unguarded sibling.
#[test]
fn p2_slim_skews_carve_valid_or_refuse_typed_never_worse() {
    for &deg in &[30.0_f64, 10.0, 4.0, 1.0, 0.2] {
        let theta = deg.to_radians();
        // Keep the cavity in the block and the vent inside the slab:
        // slimmer skews get a smaller vent and radius.
        let (body, quad) = skewed_cavity(1.2, theta, 0.5 * (theta.sin() * 0.9).min(0.25));
        let edges = cavity_edges(&body, &quad);
        assert_eq!(edges.len(), 12, "twelve concave edges at {deg}°");
        let r = 0.05 * theta.sin();
        match fillet_edges(&body, &edges, r, Tol::witness()) {
            Ok(out) => {
                assert_eq!(
                    topo::validate_geometric(&out.body, Tol::witness()),
                    Ok(()),
                    "a carve that lands at {deg}° must be tier-3 valid"
                );
                let vol = topo::mass_properties(&out.body, Tol::witness())
                    .expect("closed-form props")
                    .volume;
                assert!(vol.is_finite() && vol > 0.0, "a finite positive volume");
            }
            Err(e) => {
                // Typed is the whole demand; the sentence must render.
                let text = e.error.to_string();
                assert!(!text.is_empty(), "a refusal renders at {deg}°");
            }
        }
    }
}

/// A bit-level digest of a carved body: census, volume bits, and an
/// order-independent FNV fold of every stored point's coordinate bits.
fn digest(body: &Body<f64>) -> (usize, usize, usize, u64, u64) {
    let (nv, ne, nf) = (
        body.vertices().count(),
        body.edges().count(),
        body.faces().count(),
    );
    let mut coords: Vec<[u64; 3]> = body
        .vertices()
        .filter_map(|(k, _)| {
            body.get_vertex(k)
                .and_then(|v| body.get_point(v.point))
                .map(|p| [p.x.to_bits(), p.y.to_bits(), p.z.to_bits()])
        })
        .collect();
    coords.sort_unstable();
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for c in coords.iter().flatten() {
        for b in c.to_le_bytes() {
            h ^= u64::from(b);
            h = h.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
    let vol = topo::mass_properties(body, Tol::witness())
        .expect("closed-form props")
        .volume;
    (nv, ne, nf, h, vol.to_bits())
}

/// **P3 — the chamfer did not move under the fillet's widening.** The
/// chamfered vented cavity (BLEND-3's concave fixture) and the
/// chamfered cube, digested to the bit: census, sorted point bits,
/// volume bits. The pinned values were measured at the merge base
/// f106e96d and re-measured IDENTICAL at review head fa898277 — the
/// differential the review ran; the pin keeps it red-able.
#[test]
fn p3_the_chamfer_digest_is_bit_identical_to_the_merge_base() {
    // BLEND-3's vented cavity, re-authored: block [0,4]³, cavity
    // [1,3]³, vent r=0.5 at (2,2), z ∈ [2.5, 5].
    let block = prism(
        &[
            Point2::new(0.0, 0.0),
            Point2::new(4.0, 0.0),
            Point2::new(4.0, 4.0),
            Point2::new(0.0, 4.0),
        ],
        0.0,
        4.0,
    );
    let vent = rod(Point2::new(2.0, 2.0), 0.5, 2.5, 5.0);
    let cavity = prism(
        &[
            Point2::new(1.0, 1.0),
            Point2::new(3.0, 1.0),
            Point2::new(3.0, 3.0),
            Point2::new(1.0, 3.0),
        ],
        1.0,
        3.0,
    );
    let body = cut(&cut(&block, &vent), &cavity);
    let corner = |q: Point3<f64>| {
        [q.x, q.y, q.z]
            .iter()
            .all(|c| (c - 1.0).abs() < 1e-12 || (c - 3.0).abs() < 1e-12)
    };
    let edges: Vec<EdgeKey> = {
        let mut found: Vec<EdgeKey> = body
            .edges()
            .filter(|(k, _)| {
                let Some(e) = body.get_edge(*k) else {
                    return false;
                };
                let pt = |vk| {
                    body.get_vertex(vk)
                        .and_then(|x| body.get_point(x.point))
                        .copied()
                };
                let (Some(a), Some(b)) = (
                    body.get_half_edge(e.he_plus).map(|h| h.start).and_then(pt),
                    body.half_edge_end(e.he_plus).and_then(pt),
                ) else {
                    return false;
                };
                corner(a) && corner(b)
            })
            .map(|(k, _)| k)
            .collect();
        found.sort_unstable();
        found
    };
    assert_eq!(edges.len(), 12);
    let cav =
        chamfer_edges(&body, &edges, 0.25, Tol::witness()).expect("the chamfered cavity carves");
    let cav_digest = digest(&cav.body);

    let cube_body = cube(2.0, Tol::witness());
    let cube_edges: Vec<EdgeKey> = cube_body.edges().map(|(k, _)| k).collect();
    let cvx = chamfer_edges(&cube_body, &cube_edges, 0.25, Tol::witness())
        .expect("the chamfered cube carves");
    let cvx_digest = digest(&cvx.body);

    // Measured at f106e96d, re-measured at fa898277 (see the probe
    // report); a mismatch here means the chamfer's carve moved.
    assert_eq!(cav_digest, DIG_CAV, "the chamfered cavity moved");
    assert_eq!(cvx_digest, DIG_CVX, "the chamfered cube moved");
}

// The digests, printed by running with `PRINT_DIGESTS=1` (see
// `p0_print_digests`) at the two commits and transcribed here.
const DIG_CAV: (usize, usize, usize, u64, u64) =
    (36, 66, 34, 1555599825405810509, 4633061406684759201);
const DIG_CVX: (usize, usize, usize, u64, u64) =
    (24, 48, 26, 13370656390848575877, 4619942617744233813);
