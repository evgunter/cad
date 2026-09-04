//! **A CONCAVE closed rim has a band, and it is the same band.** The
//! blend of a closed rim is a torus about the rim's own spine on either
//! material side: on a convex rim the surgery replaces the two support
//! strips between the rim and the trimlines with the band and REMOVES
//! material, on a concave rim the identical replacement ADDS it. The
//! only material-side facts anywhere in the carve are folds of the
//! chain's stored convexity verdict — the side the arms rest the ball
//! on (`battery::curved_arm`, `arms::plane_sphere_blend`) and the band
//! face's sense bit (`Convexity::blend_sense`) — never a sampled normal.
//!
//! These rows are the concave side of what the convex closed-rim suites
//! already pin, designed against one finding: a fold that is hardcoded
//! convex is INVISIBLE to every convex fixture and red only on concave
//! carves. So every concave row here reds when the arms' fold is put
//! back to the bare sense bit, and every convex twin beside it stays
//! green — which is what makes the twins evidence and not decoration.
//!
//! - **The waist** (the ANNULUS door, a seam-split rim between two
//!   cones): one band, tier-3 valid, and the volume is the source's plus
//!   the Pappus fill derived in the row, with `volume_pad == 0.0`.
//! - **The convex twin** on the same body cuts, `V₁ < V₀`, so the two
//!   signs sit side by side over one fixture.
//! - **The boss** `cube ∪ ball` (the LADDER door: a ring of the top
//!   plane, two ring-free half-caps) — the concave twin of the die's pip,
//!   whose spherical-cap fill is a washer integral in closed form and is
//!   the pip's own cut with its sign flipped.
//! - **Naming totality** on the concave band, in both directions.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom::{Curve3, Surface};
use geom_core::{Tol, Vec3};
use sweep::blend::build::fillet_edges;
use sweep::test_support::{ball_poled_z, cube, rim_arcs_at, waisted};
use topo::boolean::{BooleanOp, SweepStrategy, boolean_op_with};
use topo::{
    Body, BooleanDeclarations, EdgeKey, FaceKey, VertexKey, mass_properties, validate_geometric,
};

fn tol() -> Tol {
    Tol::witness()
}

/// `(vertices, edges, faces)`.
fn census(b: &Body<f64>) -> (usize, usize, usize) {
    (b.vertices().count(), b.edges().count(), b.faces().count())
}

/// A volume with the closed-form-inventory claim CHECKED: every face of
/// these bodies is analytic — planes, cones, spheres and the band's
/// torus — so the pad is exactly zero, not merely small (the revolve's
/// iso-rectangle argument, `verbs_arms1_annulus.rs`).
fn volume(body: &Body<f64>) -> f64 {
    let props = mass_properties(body, tol()).expect("mass properties compute");
    assert_eq!(
        props.volume_pad, 0.0,
        "closed-form inventory: the volume pad is exactly zero"
    );
    props.volume
}

/// The census delta a two-crossing rim band leaves, on either door.
///
/// Vertices `+2`: four feet minted (a host and a mate foot per
/// crossing) and the two rim vertices retired. Edges `+3`: four seam
/// splits and four trimlines minted; the two rim arcs, the two host
/// rim-side seam pieces and the carry-through crossing's mate piece
/// retired (the closure crossing's mate piece survives as the band's
/// slit). Faces `+1`: four strips minted, three merged away — the band.
const TWO_CROSSING_DELTA: (usize, usize, usize) = (2, 3, 1);

fn assert_delta(before: (usize, usize, usize), after: (usize, usize, usize), what: &str) {
    assert_eq!(
        (after.0 - before.0, after.1 - before.1, after.2 - before.2),
        TWO_CROSSING_DELTA,
        "{what}: census {before:?} → {after:?} is the two-crossing band's delta"
    );
}

// ------------------------------------------------------------------
// The waist: the annulus door, material added.
// ------------------------------------------------------------------

/// **The waist's material-adding fill, by Pappus** — nothing of the
/// kernel enters.
///
/// In the meridian half-plane `(x, y)` the waist vertex is
/// `V = (x_v, y_v) = (0.5, 0.5)`, where the lower generator (from
/// `(1, 0)`, direction `(−1, 1)/√2`) meets the upper one (to `(1, 1)`,
/// direction `(1, 1)/√2`). The material is on the axis side, so the
/// VOID wedge at `V` opens toward `+x` between the two generators and
/// is `90°`; the rim is concave. The rolling ball of radius `r` rests in
/// that void, tangent to both generators: its centre is on the wedge's
/// bisector (the `+x` ray from `V`) at distance `r/sin 45° = r√2`, so
/// `C = (x_v + r√2, y_v)`, and its feet are `r` from `V` along each
/// generator, `F± = (x_v + r/√2, y_v ± r/√2)`.
///
/// The fill region is the curvilinear triangle `V, F−, F+` bounded by
/// the two generators and the fillet arc — the kite `V F− C F+` minus
/// the circular sector at `C` between the feet. The kite is two right
/// triangles of legs `r, r`, area `r²`; the sector's angle is
/// `π − π/2 = π/2`, area `πr²/4`; so the fill's area is `r²(1 − π/4)`.
///
/// Its first moment about the axis, `∫ x dA`:
/// - the kite is symmetric about `y = y_v` and each of its two
///   triangles has centroid `x = x_v + r/√2` (the mean of `x_v`,
///   `x_v + r/√2` and `x_v + r√2`), so `∫_kite x dA = r²(x_v + r/√2)`;
/// - the sector's centroid lies `4√2 r/(3π)` from `C` toward `V`
///   (`2R sin θ / 3θ` at half-angle `θ = π/4`), so
///   `∫_sector x dA = (πr²/4)(x_v + r√2) − √2 r³/3`.
///
/// Subtracting and collecting,
/// `∫_fill x dA = x_v r²(1 − π/4) + √2 r³(5/6 − π/4)`, and Pappus gives
/// `ΔV = 2π ∫_fill x dA`. Both brackets are positive, as the fill lies
/// on the `+x` side of `V`.
fn waist_fill(x_v: f64, r: f64) -> f64 {
    2.0 * PI * (x_v * r * r * (1.0 - PI / 4.0) + 2f64.sqrt() * r.powi(3) * (5.0 / 6.0 - PI / 4.0))
}

const WAIST_R: f64 = 0.05;

/// **The waist carves**: one annulus band, tier-3 valid, the
/// two-crossing census delta, and `V₁ = V₀ + ΔV` with `ΔV` the Pappus
/// fill above — `V₁ > V₀` is the point.
#[test]
fn the_waist_carves_one_annulus_band_and_adds_the_pappus_fill() {
    let source = waisted(tol());
    let arcs = rim_arcs_at(&source, 0.5, 0.5);
    assert_eq!(arcs.len(), 2, "the waist rim is seam-split into two arcs");
    let v0 = volume(&source);
    assert!(
        (v0 - 7.0 * PI / 12.0).abs() < 1e-12,
        "the source is two frusta, 7π/12: {v0}"
    );

    let out = fillet_edges(&source, &arcs, WAIST_R, tol())
        .unwrap_or_else(|e| panic!("the concave waist carves, got {e:?}"));
    assert_eq!(out.band_faces.len(), 1, "one annulus band");
    validate_geometric(&out.body, tol()).unwrap_or_else(|e| panic!("tier-3 valid, got {e:?}"));
    assert_delta(census(&source), census(&out.body), "the waist");

    let v1 = volume(&out.body);
    assert!(
        v1 > v0,
        "a concave band ADDS material: V₁ = {v1} must exceed V₀ = {v0}"
    );
    let want = waist_fill(0.5, WAIST_R);
    assert!(
        ((v1 - v0) - want).abs() < 1e-14,
        "the fill is the Pappus closed form: measured {} vs derived {want}",
        v1 - v0
    );
}

/// **The convex twin of the same body**: the base rim, through the same
/// door, cuts — `V₁ < V₀` — so the two signs sit side by side over one
/// fixture. Green under the sense-bit mutant, which is what makes the
/// concave row's red a statement about the fold and not about the body.
#[test]
fn the_convex_twin_of_the_same_body_cuts() {
    let source = waisted(tol());
    let v0 = volume(&source);
    for (name, rim_y) in [("the base", 0.0), ("the top", 1.0)] {
        let arcs = rim_arcs_at(&source, 1.0, rim_y);
        assert_eq!(arcs.len(), 2, "{name} rim is seam-split into two arcs");
        let out = fillet_edges(&source, &arcs, WAIST_R, tol())
            .unwrap_or_else(|e| panic!("{name} rim carves, got {e:?}"));
        assert_eq!(out.band_faces.len(), 1, "{name}: one annulus band");
        validate_geometric(&out.body, tol())
            .unwrap_or_else(|e| panic!("{name}: tier-3 valid, got {e:?}"));
        assert_delta(census(&source), census(&out.body), name);
        let v1 = volume(&out.body);
        assert!(
            v1 < v0,
            "{name}: a convex band REMOVES material: V₁ = {v1} must fall below V₀ = {v0}"
        );
    }
}

// ------------------------------------------------------------------
// The boss: the ladder door, material added.
// ------------------------------------------------------------------

const SLAB: f64 = 1.0;
const BALL_R: f64 = 0.09;
/// How far the ball's cap rises above (boss) or dips below (pip) the
/// slab's top face — the die's own pip depth.
const CAP_H: f64 = 0.05;
const BOSS_R: f64 = 0.02;

/// `slab ∪ ball` (a boss) or `slab ∖ ball` (a pip), through the public
/// boolean door. The ball's centre sits `R − H` inside the slab for the
/// boss and `R − H` above it for the pip, so both caps have height `H`
/// and both rims have radius `√(R² − (R − H)²)`.
fn slab_with(op: BooleanOp) -> Body<f64> {
    let cz = match op {
        BooleanOp::Union => SLAB - (BALL_R - CAP_H),
        BooleanOp::Subtract => SLAB + (BALL_R - CAP_H),
        BooleanOp::Intersect => unreachable!("only the two die shapes are built here"),
    };
    let ball = ball_poled_z(BALL_R, Vec3::new(0.5, 0.5, cz), tol());
    boolean_op_with(
        op,
        &cube(SLAB, tol()),
        &ball,
        &BooleanDeclarations::none(),
        SweepStrategy::Realized,
        tol(),
    )
    .unwrap_or_else(|e| panic!("the boolean builds the {op:?} body, got {e}"))
    .body()
    .expect("a body")
    .body
    .clone()
}

/// The edges between a plane face and a sphere face: the one rim of a
/// pip or a boss, as the arcs the sphere's chart seam split it into.
fn plane_sphere_rim(body: &Body<f64>) -> Vec<EdgeKey> {
    use geom_brep::SurfaceKind;
    use topo::query::{self, SurfaceKindSet};
    query::all_edges(body)
        .into_iter()
        .filter(|&k| {
            query::edge_adjacent_matches(
                body,
                k,
                SurfaceKindSet::just(SurfaceKind::Plane),
                SurfaceKindSet::just(SurfaceKind::Sphere),
            )
        })
        .collect()
}

/// The slab's top face: the plane at `z = SLAB` with outward normal `+z`.
fn top_face(body: &Body<f64>) -> FaceKey {
    body.faces()
        .find(|(_, f)| {
            matches!(body.get_surface(f.surface),
                Some(Surface::Plane { origin, normal, .. })
                    if (origin.z - SLAB).abs() < 1e-12 && normal.z > 0.5)
        })
        .map(|(k, _)| k)
        .expect("the slab's top face")
}

/// The radius of a ring read as one circle — every edge of a pip's or a
/// boss's ring carries the same circle.
fn ring_radius(body: &Body<f64>, face: FaceKey) -> f64 {
    let fd = body.get_face(face).unwrap();
    let [ring] = fd.rings[..] else {
        panic!(
            "the top face carries exactly one ring, got {}",
            fd.rings.len()
        )
    };
    let topo::LoopBoundary::Cycle { first } = body.get_loop(ring).unwrap().boundary else {
        panic!("a ring is a cycle")
    };
    let he = body.loop_cycle(first).unwrap()[0];
    let e = body.get_edge(body.get_half_edge(he).unwrap().edge).unwrap();
    let Curve3::Circle { radius, .. } = *body
        .get_curve_geom(e.curve)
        .unwrap()
        .certified()
        .unwrap()
        .carrier()
    else {
        panic!("a ring edge is a circle arc")
    };
    radius
}

/// **The boss's material-adding fill, as a washer integral** — nothing
/// of the kernel enters.
///
/// In the meridian half-plane `(ρ, z)` about the boss's axis: the top
/// plane is `z = z_p`, the sphere has centre `(0, z_c)` (below the
/// plane, `z_c < z_p`) and radius `R`, and the rim is where they meet.
/// The rolling ball of radius `r` rests in the void — ABOVE the plane,
/// OUTSIDE the sphere — so its centre is at `z_b = z_p + r` on the
/// offset sphere of radius `R + r`: `ρ_b = s = √((R + r)² − d²)` with
/// `d = z_b − z_c`. Its foot on the plane is `(s, z_p)`; its foot on the
/// sphere is the point of the sphere on the ray from the sphere's centre
/// through the ball's, at height `z_f = z_c + d·R/(R + r)`.
///
/// The fill is the region between the plane, the sphere and the fillet
/// arc: for `z ∈ [z_p, z_f]` it runs from the sphere's
/// `ρ_s(z) = √(R² − (z − z_c)²)` out to the arc's
/// `ρ_a(z) = s − √(r² − (z − z_b)²)` (the arc's inner half, facing the
/// sphere), the two meeting tangentially at `z_f`. Revolving a washer
/// slice gives `ΔV = π ∫ (ρ_a² − ρ_s²) dz`. With `u = z − z_b`, so
/// `u ∈ [−r, u₁]` and `u₁ = z_f − z_b = −d·r/(R + r)`,
///
/// `ρ_a² − ρ_s² = (s² + r² − u²) − 2s√(r² − u²) − (R² − (u + d)²)`,
///
/// every term of which has an elementary antiderivative
/// (`∫√(r² − u²) du = ½[u√(r² − u²) + r² asin(u/r)]`).
fn boss_fill(z_p: f64, z_c: f64, big_r: f64, r: f64) -> f64 {
    let z_b = z_p + r;
    let d = z_b - z_c;
    let s = ((big_r + r).powi(2) - d * d).sqrt();
    let u1 = -d * r / (big_r + r);
    let arc = |u: f64| 0.5 * (u * (r * r - u * u).max(0.0).sqrt() + r * r * (u / r).asin());
    let anti = |u: f64| {
        ((s * s + r * r) * u - u.powi(3) / 3.0 - 2.0 * s * arc(u))
            - (big_r * big_r * u - (u + d).powi(3) / 3.0)
    };
    PI * (anti(u1) - anti(-r))
}

/// **The boss carves through the LADDER door and adds its cap fill.**
///
/// The die's pips are `slab ∖ ball` and route to the ladder (the rim is
/// a ring of the top plane, the cap two ring-free half-caps); the boss
/// `slab ∪ ball` is the same shape with the sphere's material on the
/// other side, so the rim is CONCAVE. It reaches the same door — the
/// top face keeps exactly one ring across the carve, the plane's hole
/// having WIDENED from the rim circle to the trim circle, which for a
/// boss means the band rests outside the boss's footprint — carves
/// tier-3 valid with the two-crossing delta, and its `ΔV` is the washer
/// closed form above. The pip is carved beside it: the two bodies are
/// mirror images through the top plane, so the boss's fill is the pip's
/// cut with its sign flipped, and both signs sit side by side.
#[test]
fn the_boss_carves_a_concave_ladder_band_and_adds_the_cap_fill() {
    let boss = slab_with(BooleanOp::Union);
    let pip = slab_with(BooleanOp::Subtract);
    let rim_r = (BALL_R * BALL_R - (BALL_R - CAP_H).powi(2)).sqrt();

    let mut delta = Vec::with_capacity(2);
    for (name, body, concave) in [("the boss", &boss, true), ("the pip", &pip, false)] {
        let arcs = plane_sphere_rim(body);
        assert_eq!(
            arcs.len(),
            2,
            "{name}'s rim is two arcs across the cap's seam"
        );
        let top = top_face(body);
        assert!(
            (ring_radius(body, top) - rim_r).abs() < 1e-12,
            "{name}: the rim is the top face's one ring"
        );
        let v0 = volume(body);
        let out = fillet_edges(body, &arcs, BOSS_R, tol())
            .unwrap_or_else(|e| panic!("{name} carves, got {e:?}"));
        assert_eq!(out.band_faces.len(), 1, "{name}: one ladder band");
        validate_geometric(&out.body, tol())
            .unwrap_or_else(|e| panic!("{name}: tier-3 valid, got {e:?}"));
        assert_delta(census(body), census(&out.body), name);
        // The ladder's signature: the top face keeps its key and its one
        // ring, and that ring is now the (wider) trim circle.
        let trim_r = ring_radius(&out.body, top);
        assert!(
            trim_r > rim_r,
            "{name}: the plane's hole widens from the rim ({rim_r}) to the trim ({trim_r})"
        );
        let v1 = volume(&out.body);
        if concave {
            assert!(
                v1 > v0,
                "{name}: a concave band ADDS material, {v1} vs {v0}"
            );
        } else {
            assert!(
                v1 < v0,
                "{name}: a convex band REMOVES material, {v1} vs {v0}"
            );
        }
        delta.push(v1 - v0);
    }
    let [fill, cut] = delta[..] else {
        unreachable!("two bodies")
    };
    let want = boss_fill(SLAB, SLAB - (BALL_R - CAP_H), BALL_R, BOSS_R);
    assert!(
        (fill - want).abs() < 1e-14,
        "the boss's fill is the washer closed form: measured {fill} vs derived {want}"
    );
    assert!(
        (fill + cut).abs() < 1e-14,
        "mirror images through the top plane: the fill {fill} is the cut {cut} negated"
    );
}

// ------------------------------------------------------------------
// Naming totality on a concave band.
// ------------------------------------------------------------------

/// **Every output entity of the concave band is a recorded mint or a
/// survivor, and every retirement names a SOURCE key** — the same shape
/// `blend_seam_split_rim::a_seam_split_band_records_every_birth_and_every_death`
/// pins on a convex band, on the waist. The naming rows do not read the
/// material side, and this is where that is checked rather than said.
#[test]
fn a_concave_band_records_every_birth_and_every_death() {
    let source = waisted(tol());
    let arcs = rim_arcs_at(&source, 0.5, 0.5);
    let out = fillet_edges(&source, &arcs, WAIST_R, tol())
        .unwrap_or_else(|e| panic!("the waist carves, got {e:?}"));
    let rec = out
        .naming
        .as_ref()
        .expect("the rim phase records its births");

    let minted_edges: Vec<EdgeKey> = rec
        .rim_trims
        .iter()
        .map(|(e, _, _)| *e)
        .chain(rec.meridian_remnants.iter().map(|(e, _)| *e))
        .chain(rec.slits.iter().map(|(e, _)| *e))
        .collect();
    for (k, _) in out.body.edges() {
        assert!(
            minted_edges.contains(&k) || source.get_edge(k).is_some(),
            "output edge {k:?} is neither minted nor a survivor"
        );
    }
    for e in &rec.dead.edges {
        assert!(
            source.get_edge(*e).is_some(),
            "a retirement names a source edge, got {e:?}"
        );
        assert!(
            !out.body.edges().any(|(k, _)| k == *e) || minted_edges.contains(e),
            "a retired edge does not survive: {e:?}"
        );
    }
    for v in &rec.dead.vertices {
        assert!(
            source.get_vertex(*v).is_some(),
            "a retirement names a source vertex, got {v:?}"
        );
        assert!(
            !out.body.vertices().any(|(k, _)| k == *v),
            "a retired vertex does not survive: {v:?}"
        );
    }
    let minted_vertices: Vec<VertexKey> = rec
        .rim_feet
        .iter()
        .map(|(v, _)| *v)
        .chain(rec.meridian_splits.iter().map(|(v, _)| *v))
        .collect();
    for (k, _) in out.body.vertices() {
        assert!(
            minted_vertices.contains(&k) || source.get_vertex(k).is_some(),
            "output vertex {k:?} is neither a recorded mint nor a survivor"
        );
    }
    assert_eq!(
        rec.rim_feet.len(),
        2,
        "one host foot per crossing, and this rim has two"
    );
    assert_eq!(
        rec.meridian_splits.len(),
        2,
        "one mate foot per crossing, and this rim has two"
    );
    assert_eq!(
        rec.dead.vertices.len(),
        2,
        "both seam vertices are retired, and nothing else is"
    );
    let mut banded: Vec<EdgeKey> = rec
        .bands
        .iter()
        .flat_map(|(_, edges)| edges.iter().copied())
        .collect();
    banded.sort_unstable();
    let mut requested = arcs.clone();
    requested.sort_unstable();
    assert_eq!(
        banded, requested,
        "the band row names exactly the requested arcs"
    );
    assert_eq!(rec.bands.len(), 1, "one band row");
    assert_eq!(
        rec.bands[0].0, out.band_faces[0],
        "the band row names the band face"
    );
}
