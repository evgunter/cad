//! Orientation checking for the swept and lofted corpus: readers for the
//! shipped charts, the face-facing probe, and the two LEVEL-SET indexes
//! it decides against.
//!
//! **Routing rule** (`sweep::test_support`'s, applied here): an item
//! lives at the narrowest home all of its consumers can reach.
//! `sweep::test_support` is narrower than this module and holds
//! FIXTURES the library can build; `common/mod.rs` holds section
//! authoring; this module holds what a suite CHECKS of a body it
//! built; a helper used by one suite stays in that suite. Nothing here
//! is `pub` that only this module uses.
//!
//! A wall's outward normal is `sense_sign · (S_u × S_v)`. Checking it
//! means asking, of a point just off the wall, which side the material
//! is on — and the answer has to come from somewhere that never reads a
//! `sense`, a winding or a normal, or the check is circular. Everything
//! here reads POSITIONS off the shipped charts and nothing else.
//!
//! # Level sets, and the two ways to index them
//!
//! A body skinned from one profile through a stack of placements has
//! planar cross-sections: at chart parameter `v` every wall's iso-curve
//! is `Σ_j B_j(v) · A_j` applied to the profile's control points, and
//! since the `B_j` are a partition of unity that sum is one AFFINE map
//! applied to a planar section. So the level ring at `v` is planar for
//! any placements at all, straight path or turning — [`level_ring`]
//! asserts it rather than resting on the derivation.
//!
//! What differs between bodies is how a query point is matched to its
//! level. Where the stack does not turn much, the level height above a
//! plane oriented against a fixed axis falls monotonically and one
//! bisection finds the level; that is the loft corpus's index, and its
//! preconditions are its own. Where the stack turns through a full
//! revolution both halves fail — see [`LevelIndex`], which indexes by
//! continuity instead and enumerates every level a point lies on.

#![allow(dead_code)]

use geom::{NurbsSurface, Surface};
use geom_core::{Point3, Vec3};
use sweep::Lofted;
use topo::boolean::SolidContainment;
use topo::{Body, FaceKey};

// ---------------------------------------------------------------------
// Reading the shipped charts
// ---------------------------------------------------------------------

/// A wall's skinned chart and the knot parameters that FRACTIONAL chart
/// coordinates name on it. Fractions rather than knot values, so walls
/// with different spans are sampled on the same level set of `v`.
pub fn chart_at(
    body: &Body<f64>,
    face: FaceKey,
    su: f64,
    sv: f64,
) -> (&NurbsSurface<f64>, f64, f64) {
    let f = body.get_face(face).expect("wall face resolves");
    let Some(Surface::Nurbs(s)) = body.get_surface(f.surface) else {
        panic!("a skinned wall carries a NURBS chart");
    };
    let (u0, u1) = s.knots_u().domain();
    let (v0, v1) = s.knots_v().domain();
    (s, u0 + (u1 - u0) * su, v0 + (v1 - v0) * sv)
}

/// One wall's POSITION at fractional chart coordinates.
///
/// Every oracle in this module reads this and never [`wall_outward_at`],
/// which is what makes their independence from the datum under test
/// structural rather than conventional: they have no access to a
/// `sense`, a winding or a normal to be circular with. It also keeps
/// `normalize()` — poison on a degenerate jet — out of a path that only
/// ever wants `S(u, v)`.
///
/// Public for the same reason: a suite stating what its FIXTURE is —
/// which way a roll turns, how far a ring rotates — wants positions and
/// must not reach for a normal to say it with.
pub fn wall_point_at(body: &Body<f64>, face: FaceKey, su: f64, sv: f64) -> Point3<f64> {
    let (s, u, v) = chart_at(body, face, su, sv);
    s.eval(u, v)
}

/// One wall's point and the OUTWARD normal it claims: the shipped
/// surface's `S_u × S_v` with the stored `sense` folded in.
pub fn wall_outward_at(
    body: &Body<f64>,
    face: FaceKey,
    su: f64,
    sv: f64,
) -> (Point3<f64>, Vec3<f64>) {
    let f = body.get_face(face).expect("wall face resolves");
    let (s, u, v) = chart_at(body, face, su, sv);
    let jet = s.ders(u, v);
    (
        s.eval(u, v),
        jet.du.cross(jet.dv).normalize() * f.sense_sign::<f64>(),
    )
}

/// One wall's mid-chart point and outward normal.
pub fn wall_outward(body: &Body<f64>, face: FaceKey) -> (Point3<f64>, Vec3<f64>) {
    wall_outward_at(body, face, 0.5, 0.5)
}

/// The chord from the bottom level to the top one — the stacking
/// direction, where the stack has one.
pub fn stack_axis(lofted: &Lofted<f64>) -> Vec3<f64> {
    let first = lofted.side_faces[0][0];
    wall_point_at(&lofted.body, first, 0.5, 1.0) - wall_point_at(&lofted.body, first, 0.5, 0.0)
}

// ---------------------------------------------------------------------
// The probe
// ---------------------------------------------------------------------

/// A MATERIAL-SIDE oracle: where a query point lies relative to the
/// solid, decided without reading any face's `sense`.
pub type Oracle<'a> = dyn Fn(Point3<f64>) -> SolidContainment + 'a;

/// The chart grid the turning fixtures are probed on: three columns
/// across `u` at five levels along `v`. Mid-chart alone cannot see a
/// twist — it is one level set of a chart that has many. The columns
/// avoid `u = 0` and `u = 1`, where a probe would sit on a profile
/// CORNER and its inward step would land on the neighbouring wall.
pub fn along_v() -> Vec<(f64, f64)> {
    let vs = [0.05, 0.25, 0.5, 0.75, 0.95];
    [0.25, 0.5, 0.75]
        .into_iter()
        .flat_map(|u| vs.map(|v| (u, v)))
        .collect()
}

/// Step `delta` off a wall both ways along its claimed outward normal
/// and ask the oracle. Returns `(inward_side, outward_side)` — a
/// flipped `sense` swaps them, which is what makes the pair two-sided.
fn probe_sides(
    oracle: &Oracle<'_>,
    p: Point3<f64>,
    outward: Vec3<f64>,
    delta: f64,
) -> (SolidContainment, SolidContainment) {
    (oracle(p - outward * delta), oracle(p + outward * delta))
}

/// The orientation claim, wall by wall and sample by sample. The
/// ORIENTATION assertion runs first and is what a wrong bit trips; the
/// `sense = true` value pin follows it, so a production flip reports
/// the material-side failure rather than a stored-value mismatch.
///
/// `expect` is the wall count the caller believes the fixture has, so a
/// fixture that quietly loses a wall reports it. A probe COUNT is not
/// also returned: it would be `expect · samples.len()` by construction
/// and an assertion on it cannot fail.
pub fn assert_walls_face_out(
    lofted: &Lofted<f64>,
    oracle: &Oracle<'_>,
    samples: &[(f64, f64)],
    delta: f64,
    expect: usize,
) {
    assert_probe_step(delta);
    let mut n = 0;
    for (li, walls) in lofted.side_faces.iter().enumerate() {
        for (si, &fk) in walls.iter().enumerate() {
            for &(su, sv) in samples {
                let (p, outward) = wall_outward_at(&lofted.body, fk, su, sv);
                let (inward_side, outward_side) = probe_sides(oracle, p, outward, delta);
                assert_eq!(
                    (inward_side, outward_side),
                    (SolidContainment::In, SolidContainment::Out),
                    "loop {li} segment {si} at chart ({su}, {sv}) = {p:?}: the wall \
                     claims {outward:?} is outward, so the oracle must read material \
                     against it and void along it"
                );
            }
            assert!(
                lofted.body.get_face(fk).unwrap().sense,
                "loop {li} segment {si}: the honest orientation above is the one \
                 the skinned chart encodes as sense = true"
            );
            n += 1;
        }
    }
    assert_eq!(n, expect, "every wall of the fixture was probed");
}

/// The CAPS' orientation, from the same oracle, at the one interior
/// point a planar face hands over for free: the centroid of the level
/// ring it closes.
///
/// A cap carries no chart to sample, so it is probed at one point
/// rather than on a grid — which is enough, because a plane's normal is
/// the same vector everywhere on it, so there is no second sample that
/// could disagree.
///
/// PRECONDITION: the section's outer ring must contain its own
/// centroid. Every fixture that probes caps today is a centred square
/// or a circle; a notched or holed section would need an interior point
/// chosen rather than averaged, and would fail here loudly rather than
/// quietly reporting the wrong side.
pub fn assert_caps_face_out(lofted: &Lofted<f64>, oracle: &Oracle<'_>, delta: f64) {
    assert_probe_step(delta);
    for (what, face, t) in [("bottom", lofted.bottom, 0.0), ("top", lofted.top, 1.0)] {
        let f = lofted.body.get_face(face).expect("cap face resolves");
        let Some(Surface::Plane { normal, .. }) = lofted.body.get_surface(f.surface) else {
            panic!("{what} cap carries a plane");
        };
        let outward = *normal * f.sense_sign::<f64>();
        let p = ring_centroid(lofted, t);
        assert_eq!(
            probe_sides(oracle, p, outward, delta),
            (SolidContainment::In, SolidContainment::Out),
            "the {what} cap at {p:?} claims {outward:?} is outward, so the oracle \
             must read material against it and void along it"
        );
    }
}

/// The centroid of the outer ring's mid-`u` samples at `v`-fraction
/// `t` — an interior point of the cap for a section that contains its
/// own centroid, under the precondition [`assert_caps_face_out`]
/// states. Public so a suite measuring a ring's rotation about the
/// stack has one spelling of "the middle of the level" to measure
/// from.
pub fn ring_centroid(lofted: &Lofted<f64>, t: f64) -> Point3<f64> {
    let walls = &lofted.side_faces[0];
    let mut acc = Vec3::new(0.0, 0.0, 0.0);
    for &fk in walls {
        let p = wall_point_at(&lofted.body, fk, 0.5, t);
        acc = acc + Vec3::new(p.x, p.y, p.z);
    }
    #[allow(clippy::cast_precision_loss)]
    let n = walls.len() as f64;
    Point3::new(acc.x / n, acc.y / n, acc.z / n)
}

/// A probe step has to clear the level polyline's own error, or a
/// verdict about the polyline stops being a verdict about the solid.
///
/// This is the half of *a claim about a shared helper is a claim about
/// every caller* that a sampling-neutrality check does not reach:
/// [`CHORD_ERR_BOUND`] is a fixed number and is only sound RELATIVE to
/// the caller's `delta`. `delta` exists here and nowhere else in this
/// module, so the coupling is enforced here instead of being asserted
/// in a message.
///
/// An order of magnitude, and the shipped steps clear it with room:
/// 0.02 on the helices and on two of the loft fixtures, 0.05 and 0.06
/// on the elbows, against a floor of 0.01.
const PROBE_STEP_OVER_CHORD: f64 = 10.0;

fn assert_probe_step(delta: f64) {
    assert!(
        delta >= PROBE_STEP_OVER_CHORD * CHORD_ERR_BOUND,
        "a probe step of {delta} does not clear the level polyline's own chord \
         bound {CHORD_ERR_BOUND} by the order of magnitude that makes parity \
         against the polyline an answer about the solid"
    );
}

// ---------------------------------------------------------------------
// Level sets
// ---------------------------------------------------------------------

/// Samples per wall on a level-set polyline.
///
/// The number is not derived from any fixture and is not meant to be:
/// what makes it sound is that [`level_ring`] MEASURES the chord error
/// every time and refuses above [`CHORD_ERR_BOUND`]. The two worst
/// walls in the suites that use this today are the holed plate's hole
/// (radius 1, half a turn per wall, ≈ 3.3e-4) and the rational elbow's
/// semicircle (radius ¼, ≈ 7.5e-5); a suite whose walls curve harder
/// does not need this constant re-derived, it needs the measured bound
/// to fire, which it will.
const LEVEL_SAMPLES: usize = 64;

/// The least number of samples that must go round a level ring before
/// its Newell sum means anything. A ring read at ONE sample per wall
/// degenerates when the profile has fewer than three walls: a circle
/// section is two semicircular arcs, its two midpoints make a 2-gon,
/// and the Newell terms of a 2-gon cancel exactly — the sum is the zero
/// vector and normalizing it yields NaN, silently, at every level.
const RING_PLANE_MIN_SAMPLES: usize = 4;

/// How large the Newell sum must be against the ring's own extent for
/// its direction to be a plane normal rather than rounding noise.
/// Twice the ring's projected area over the square of its SPREAD — the
/// farthest sample from `ring[0]`, which under-reads a true diameter
/// when `ring[0]` sits mid-side and so makes the guard weaker rather
/// than tighter. A square reads ≈ 1; a ring collapsed onto a line
/// reads 0.
const RING_PLANE_MIN_AREA: f64 = 1e-6;

/// The plane of the level ring at `v`-fraction `t`, normal UNORIENTED:
/// a point on it and a unit normal whose sign follows the ring's
/// traversal. Read off the first loop's walls — the ring is planar
/// ([`level_ring`] asserts it on the full sample), so a handful of
/// samples fixes its plane and an index need not pay for a whole
/// polyline per step.
///
/// Newell over the whole ring, rather than three picked points, so no
/// one near-collinear sample can decide the normal. Hand-rolled rather
/// than `geom_brep::newell_plane`: that door certifies planarity
/// against the running ε, and an oracle whose validity moves with ε is
/// the trap #619 fell into. The planarity precondition here is a fixed
/// geometric bound.
fn level_ring_plane(lofted: &Lofted<f64>, t: f64) -> (Point3<f64>, Vec3<f64>) {
    let walls = &lofted.side_faces[0];
    let per_wall = RING_PLANE_MIN_SAMPLES.div_ceil(walls.len());
    let mut ring: Vec<Point3<f64>> = Vec::with_capacity(walls.len() * per_wall);
    for &fk in walls {
        for k in 0..per_wall {
            #[allow(clippy::cast_precision_loss)]
            let su = (k as f64 + 0.5) / per_wall as f64;
            ring.push(wall_point_at(&lofted.body, fk, su, t));
        }
    }
    let mut n = Vec3::new(0.0, 0.0, 0.0);
    let mut spread = 0.0_f64;
    for i in 0..ring.len() {
        let (a, b) = (ring[i], ring[(i + 1) % ring.len()]);
        spread = spread.max((a - ring[0]).norm());
        n = n + Vec3::new(
            (a.y - b.y) * (a.z + b.z),
            (a.z - b.z) * (a.x + b.x),
            (a.x - b.x) * (a.y + b.y),
        );
    }
    assert!(
        n.norm() > RING_PLANE_MIN_AREA * spread * spread,
        "the level ring at v-fraction {t} encloses no area to take a normal from \
         ({} over a spread of {spread}) — {} samples round the ring is not a \
         ring",
        n.norm(),
        ring.len()
    );
    (ring[0], n.normalize())
}

/// The body's own level set at `v`-fraction `t`, against a plane the
/// caller's index has already oriented: one closed polyline per loop,
/// walls in traversal order, sampled off the shipped charts.
///
/// Two PRECONDITIONS, asserted rather than assumed, because the ring is
/// worthless without them:
///
/// - the level set is **planar**, or the polyline is not a
///   cross-section of anything (the module header derives why it is,
///   for any placements; this measures it);
/// - the polyline's **chord error** stays far under the probe steps,
///   or parity answers about the polyline stop being answers about the
///   solid. Estimated per wall from consecutive triples: on a circle
///   of radius `R` sampled at angular step `θ` the middle sample sits
///   `R(1 − cos θ)` off its neighbours' chord, four times the chord's
///   own sagitta `R(1 − cos θ/2)`. Wall JUNCTIONS are excluded — a
///   profile corner is a real corner, not sampling error.
///
/// The chord bound is fixed and the probe step is the caller's, so the
/// two are coupled; [`assert_probe_step`] holds the caller's end.
/// How far a level polyline may sit off its own iso-curves. Fixed
/// rather than ε-keyed: an oracle whose validity moves with ε is the
/// fragility #619 was faulted for. [`assert_probe_step`] is the other
/// half — this number only means anything against a caller's step.
const CHORD_ERR_BOUND: f64 = 1e-3;

fn level_ring(
    lofted: &Lofted<f64>,
    plane: (Point3<f64>, Vec3<f64>),
    t: f64,
) -> Vec<Vec<Point3<f64>>> {
    let (origin, n) = plane;
    let (mut off_plane, mut chord_err) = (0.0_f64, 0.0_f64);
    let mut loops = Vec::with_capacity(lofted.side_faces.len());
    for walls in &lofted.side_faces {
        let mut poly = Vec::with_capacity(walls.len() * LEVEL_SAMPLES);
        for &fk in walls {
            let run = poly.len();
            for i in 0..LEVEL_SAMPLES {
                #[allow(clippy::cast_precision_loss)]
                let su = i as f64 / LEVEL_SAMPLES as f64;
                let p = wall_point_at(&lofted.body, fk, su, t);
                off_plane = off_plane.max((p - origin).dot(n).abs());
                poly.push(p);
            }
            for i in run + 1..poly.len() - 1 {
                let mid = poly[i - 1] + (poly[i + 1] - poly[i - 1]) * 0.5;
                chord_err = chord_err.max((poly[i] - mid).norm() * 0.25);
            }
        }
        loops.push(poly);
    }
    assert!(
        off_plane < 1e-9,
        "the level set at v-fraction {t} must be planar for its polyline to be \
         a cross-section: off-plane by {off_plane}"
    );
    assert!(
        chord_err < CHORD_ERR_BOUND,
        "the level polyline at v-fraction {t} must track its iso-curves far \
         inside every caller's probe step ([`assert_probe_step`] enforces the \
         other side of that): chord error {chord_err}"
    );
    loops
}

/// Crossing parity against a level set, in the level plane's own
/// frame: the material side of a closed planar curve, which is defined
/// without any orientation at all — a hole loop needs no special case,
/// its crossings simply cancel the plate's.
///
/// **A DECLARED COPY** — the outermost of the tree's ray-parity
/// implementations. Reuse is blocked in every
/// direction: `topo::splitting::containment::point_in_loop` is public
/// but takes `(&Body, LoopKey)` and walks the B-rep, so it cannot
/// consume a sampled polyline; `chart_region::point_in_polygon`,
/// `profile::validate::point_in_loop` and the shared walk the first
/// two now share (`topo::ray_parity::ray_verdict`, `pub(crate)`) are
/// private to their crates; and any `Decide`-certified door — which
/// the shared walk is — would tie this oracle's validity to the
/// running ε, which is the fragility #619 was faulted for.
fn level_set_contains(
    loops: &[Vec<Point3<f64>>],
    plane: (Point3<f64>, Vec3<f64>),
    q: Point3<f64>,
) -> bool {
    let (origin, n) = plane;
    let (e1, e2) = n.orthonormal_basis();
    let uv = |p: Point3<f64>| ((p - origin).dot(e1), (p - origin).dot(e2));
    let (qx, qy) = uv(q);
    let mut crossings = 0usize;
    for poly in loops {
        for i in 0..poly.len() {
            let (a, b) = (uv(poly[i]), uv(poly[(i + 1) % poly.len()]));
            if (a.1 > qy) != (b.1 > qy) && a.0 + (qy - a.1) / (b.1 - a.1) * (b.0 - a.0) > qx {
                crossings += 1;
            }
        }
    }
    crossings % 2 == 1
}

// ---------------------------------------------------------------------
// Index 1: a fixed stacking chord, one monotone height, one bisection
// ---------------------------------------------------------------------

/// How closely a level plane's normal must sit to a fixed stacking
/// chord for that chord to orient it. [`level_plane`] needs this at
/// EVERY level and refuses the body otherwise; a swept body whose path
/// turns has no such chord, which is the condition [`LevelIndex`]
/// exists for. Shared, so that a suite asserting this index cannot run
/// on its fixture and the index itself cannot drift apart.
pub const FIXED_AXIS_GUARD_COS: f64 = 0.1;

/// The plane of the level ring at `v`-fraction `t`, oriented along a
/// fixed stacking chord.
///
/// Newell's sign follows the ring's traversal, so a plane whose normal
/// is near-perpendicular to the chord cannot be oriented against it
/// reliably at all — that is what this guard is for. It is NOT the
/// bisection's precondition; `loft_contains` checks that one directly.
fn level_plane(lofted: &Lofted<f64>, axis: Vec3<f64>, t: f64) -> (Point3<f64>, Vec3<f64>) {
    let (origin, n) = level_ring_plane(lofted, t);
    let along = if n.dot(axis) < 0.0 { -1.0 } else { 1.0 };
    assert!(
        (n.dot(axis) * along) / axis.norm() > FIXED_AXIS_GUARD_COS,
        "the level plane at v-fraction {t} must be orientable against the \
         stacking chord: cos = {}",
        n.dot(axis) * along / axis.norm()
    );
    (origin, n * along)
}

/// The body's own level set at `v`-fraction `t`, against the plane the
/// stacking chord orients.
fn level_set(lofted: &Lofted<f64>, t: f64) -> Vec<Vec<Point3<f64>>> {
    level_ring(lofted, level_plane(lofted, stack_axis(lofted), t), t)
}

/// Levels the bisection's premise is checked on. Coarse: it must
/// catch a fan that folds back, not resolve one.
const MONOTONE_SCAN: usize = 64;

/// **The level-set oracle**: is `q` inside the stacked solid? `q`'s
/// height above the level plane falls as the level rises past it, so
/// `q`'s own level set is found by bisection and the caller never
/// needs to know how `v` maps to space. Past either cap is `Out` by
/// construction.
///
/// PRECONDITION, asserted rather than assumed, and asserted DIRECTLY:
/// `height` must be monotone. [`level_plane`]'s orientability guard is
/// a weaker and different condition — measured on the elbow, a 120°
/// turn clears it at `cos = 0.28` while `height` has already stopped
/// being monotone — so guarding that instead would leave a body whose
/// level is ambiguous answering from whichever root the bisection
/// lands on.
/// A post-condition cannot substitute: every spurious root satisfies
/// `height ≈ 0`, and which root was found is exactly what is at stake.
/// The scan is a scan — it certifies at its own resolution, and it
/// fires rather than lies.
pub fn loft_contains(lofted: &Lofted<f64>, q: Point3<f64>) -> SolidContainment {
    let axis = stack_axis(lofted);
    let height = |t: f64| {
        let (p, n) = level_plane(lofted, axis, t);
        (q - p).dot(n)
    };
    #[allow(clippy::cast_precision_loss)]
    let scan: Vec<f64> = (0..=MONOTONE_SCAN)
        .map(|i| height(i as f64 / MONOTONE_SCAN as f64))
        .collect();
    for (i, w) in scan.windows(2).enumerate() {
        assert!(
            w[1] < w[0],
            "the level height must fall monotonically along v for a query \
             point's level to be well defined: at step {i} of {MONOTONE_SCAN} \
             it rises, {} then {}",
            w[0],
            w[1]
        );
    }
    if scan[0] <= 0.0 || scan[MONOTONE_SCAN] >= 0.0 {
        return SolidContainment::Out;
    }
    let (mut lo, mut hi) = (0.0_f64, 1.0_f64);
    for _ in 0..40 {
        let mid = (lo + hi) * 0.5;
        if height(mid) > 0.0 {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    let t = (lo + hi) * 0.5;
    if level_set_contains(&level_set(lofted, t), level_plane(lofted, axis, t), q) {
        SolidContainment::In
    } else {
        SolidContainment::Out
    }
}

// ---------------------------------------------------------------------
// Index 2: continuity along the stack, every root enumerated
// ---------------------------------------------------------------------
//
// TWO indexes over one set of level rings, deliberately. `LevelIndex`
// subsumes `loft_contains` — a monotone height has exactly one root —
// and the corpus's sixteen loft rows could be moved onto it. They are
// not. The turning-path rows below assert that index 1 could NOT have
// run on their fixtures, against the constant index 1 guards with, so
// the two cannot silently converge either.

/// Level planes sampled along `v`. Two things have to be bought with
/// this number, and only one of them is argued:
///
/// - the CONTINUITY chain, which needs consecutive normals to agree.
///   A full revolution of frame roll advances the ring normal by ~0.7°
///   per step here, and the chain asserts the agreement anyway.
/// - the ROOT resolution, which is not argued at all — it is measured
///   per query, by re-counting the sign changes on the half-density
///   subsample. On the corpus's two-turn helix that check FIRES at
///   `LEVELS = 128` (9 roots against 5) and is satisfied from 256 up,
///   so the value below carries one doubling over the measured floor.
const LEVELS: usize = 512;

/// How closely consecutive level normals must agree for the continuity
/// chain to mean anything. This is a LOCAL condition and it is the
/// whole point: it says nothing about where the stack ends up.
const CONTINUITY_COS: f64 = 0.9;

/// A body's level planes, indexed by CONTINUITY along the stack — the
/// oracle for a swept body whose path turns through a full revolution.
///
/// # Why the loft corpus's index cannot be widened to reach here
///
/// That index orients every level plane against one fixed chord
/// ([`stack_axis`]) and then finds a query point's level by bisecting a
/// height it requires to be monotone. On a helix BOTH halves fail, and
/// they fail for different reasons — which is why this is a different
/// oracle and not a looser bound.
///
/// **There is no fixed axis to orient against.** The level planes are
/// the path's normal planes, so their normals sweep a cone of half
/// angle `atan(R/k)` about the helix axis while the chord from the
/// first section to the last is nearly the axis itself. The MODEL for
/// the corpus helix (`R = 1`, `k = pitch/2π = 0.0637`) is
/// `cos = k/√(R²+k²) = 0.06353`, constant in the path parameter — and
/// the shipped body is not the model. Measured over all 513 sampled
/// levels of the built bodies:
///
/// ```text
///                level 0     min       max     levels above 0.1
///   half turn    0.01108   0.01108   0.99346      486 / 513
///   whole turn   0.06352   0.05746   0.12918       18 / 513
///   two turns    0.06352   0.05749   0.12918       18 / 513
/// ```
///
/// So the cosine RANGES, it does not sit at the model, and on every
/// fixture some levels clear `0.1` while others do not — on the half
/// turn most of them do. **The guard is sized against the MINIMUM**,
/// which is what the fixed-axis index needs: it requires every level to
/// be orientable and refuses the body on the first one that is not.
/// Lowering the guard does not help either, because a near-perpendicular
/// Newell normal cannot be oriented against that chord reliably at all —
/// the sign it is being asked to fix is the undetermined thing.
///
/// **The level is not unique.** For the exact helix `c(a) = (R cos a,
/// R sin a, k a)` and a query point `q = (R, 0, z₀)` on the starting
/// section's radius, the height against the normal plane at `a` is
///
/// ```text
/// h(a) · √(R² + k²) = −R² sin a − k² a + k z₀,
/// ```
///
/// so `dh/da ∝ −R² cos a − k²`, which is POSITIVE for `a` in roughly
/// the second and third quadrants. The height stops being monotone
/// after about a quarter turn and a query point genuinely lies on
/// several level planes at once — 2 of them on the whole-turn fixture,
/// between 3 and 9 on the two-turn one. Bisecting would return
/// whichever root it landed on, and every spurious root satisfies
/// `h ≈ 0`, so no post-condition could tell them apart.
///
/// # What replaces it
///
/// Orient by CONTINUITY from `t = 0` — each level normal is flipped to
/// agree with its predecessor, and the agreement is asserted — so no
/// global direction is needed at all. Then, instead of assuming one
/// root, ENUMERATE them: `q` is in the body iff `q` lies inside the
/// level ring of some level, and `q` can only lie inside a ring whose
/// plane holds `q`, so the roots of `h` are exactly the candidate
/// levels. Test the ring at each; the body contains `q` iff one of them
/// claims it.
///
/// **What is enumerated is SIGN CHANGES, not roots**, and the two are
/// the same set only where `h` crosses transversally. A tangential root
/// — `h` grazing zero and returning — is invisible here, and invisible
/// to the refinement guard below as well, since that guard compares
/// COUNTS at two densities and both densities miss the same tangency.
/// The derivation above is therefore strictly stronger than what runs.
/// On a tube whose half-width is far under the path's curvature radius
/// (0.08 against ≈ 1 on this corpus) transversality is comfortable, but
/// it is a premise of the completeness claim and is not checked.
///
/// This is exactly the loft index where that one is valid — a monotone
/// height has one root — and it is defined where that one is not.
///
/// # Preconditions, all asserted
///
/// - consecutive level normals agree within [`CONTINUITY_COS`], or the
///   chain that orients them is arbitrary (in [`LevelIndex::build`]);
/// - the root count does not change when the sample density is halved,
///   or the enumeration is missing roots and the answer is a scan
///   artefact (in [`LevelIndex::contains`]). It compares root COUNTS
///   and not root POSITIONS, so two densities can agree on a count
///   while both resolve the wrong levels; what would replace it is a
///   check that each root's bracket survives refinement, which nothing
///   on this corpus has needed;
/// - at most one level's ring claims the point, or containment is not a
///   function of position at `q` (in [`LevelIndex::contains`]). Read
///   that one way only: two claims mean the body overlaps itself at
///   `q`, but ONE claim does not mean it does not. A section swept
///   along an arc tighter than the section is wide builds, passes
///   tier 1, and is answered by this index without a refusal. This is
///   an orientation oracle, not a self-intersection detector;
/// - each ring is planar and tracks its iso-curves (in [`level_ring`]).
///
/// Every one of them fires rather than lies.
pub struct LevelIndex<'a> {
    lofted: &'a Lofted<f64>,
    /// `(point, unit normal)` at `t = i / LEVELS`, normals oriented by
    /// continuity from `t = 0`.
    planes: Vec<(Point3<f64>, Vec3<f64>)>,
}

impl<'a> LevelIndex<'a> {
    /// Sample and continuity-orient the level planes.
    pub fn build(lofted: &'a Lofted<f64>) -> Self {
        let mut planes: Vec<(Point3<f64>, Vec3<f64>)> = Vec::with_capacity(LEVELS + 1);
        for i in 0..=LEVELS {
            #[allow(clippy::cast_precision_loss)]
            let t = i as f64 / LEVELS as f64;
            let (p, n) = level_ring_plane(lofted, t);
            let n = match planes.last() {
                None => n,
                Some(&(_, prev)) => {
                    let n = if n.dot(prev) < 0.0 { n * -1.0 } else { n };
                    assert!(
                        n.dot(prev) > CONTINUITY_COS,
                        "level planes {} and {i} of {LEVELS} turn by more than the \
                         continuity chain can follow: cos = {} — the sampling is too \
                         coarse for this stack, so which way each plane points is a \
                         guess",
                        i - 1,
                        n.dot(prev)
                    );
                    n
                }
            };
            planes.push((p, n));
        }
        Self { lofted, planes }
    }

    /// The continuity-oriented level planes, `t = i / LEVELS`.
    pub fn planes(&self) -> &[(Point3<f64>, Vec3<f64>)] {
        &self.planes
    }

    /// How far the level planes' normal turns along the whole stack,
    /// accumulated over the samples rather than compared end to end.
    ///
    /// For a swept body this is the path tangent's total turn, so it is
    /// the direct measure of whether the chart ROLLS: a straight path
    /// holds every level plane parallel and reads 0, and a whole
    /// revolution reads `2π` less whatever the tangent's own pitch
    /// keeps out of the turning plane. Accumulating is what
    /// distinguishes a whole turn from a straight path at all — their
    /// end-to-end normals are the same vector.
    pub fn total_turn(&self) -> f64 {
        self.planes
            .windows(2)
            .map(|w| w[0].1.dot(w[1].1).clamp(-1.0, 1.0).acos())
            .sum()
    }

    /// The level plane at an arbitrary `t`, oriented to agree with
    /// `reference` — used inside a bracket, where the two are one
    /// sample apart and the choice is not close.
    fn plane_at(&self, t: f64, reference: Vec3<f64>) -> (Point3<f64>, Vec3<f64>) {
        let (p, n) = level_ring_plane(self.lofted, t);
        let n = if n.dot(reference) < 0.0 { n * -1.0 } else { n };
        assert!(
            n.dot(reference) > CONTINUITY_COS,
            "the level plane at v-fraction {t} must agree with the bracketing \
             sample it is oriented from: cos = {}",
            n.dot(reference)
        );
        (p, n)
    }

    /// Is `q` inside the stacked solid? Every level whose plane holds
    /// `q`, tested for ring containment.
    pub fn contains(&self, q: Point3<f64>) -> SolidContainment {
        let h: Vec<f64> = self.planes.iter().map(|&(p, n)| (q - p).dot(n)).collect();
        let crossings = |step: usize| {
            (0..LEVELS / step)
                .filter(|i| (h[i * step] > 0.0) != (h[(i + 1) * step] > 0.0))
                .count()
        };
        let (fine, coarse) = (crossings(1), crossings(2));
        assert_eq!(
            fine, coarse,
            "the level-height roots for {q:?} must be resolved by the sampling: \
             {fine} at {LEVELS} levels but {coarse} at half that — the enumeration \
             below would be a scan artefact, not the body"
        );

        let mut claims = 0usize;
        for i in 0..LEVELS {
            if (h[i] > 0.0) == (h[i + 1] > 0.0) {
                continue;
            }
            #[allow(clippy::cast_precision_loss)]
            let (mut lo, mut hi) = (i as f64 / LEVELS as f64, (i + 1) as f64 / LEVELS as f64);
            let above = h[i] > 0.0;
            let reference = self.planes[i].1;
            for _ in 0..40 {
                let mid = (lo + hi) * 0.5;
                let (p, n) = self.plane_at(mid, reference);
                if ((q - p).dot(n) > 0.0) == above {
                    lo = mid;
                } else {
                    hi = mid;
                }
            }
            let t = (lo + hi) * 0.5;
            let plane = self.plane_at(t, reference);
            if level_set_contains(&level_ring(self.lofted, plane, t), plane, q) {
                claims += 1;
            }
        }
        assert!(
            claims <= 1,
            "{claims} level rings claim {q:?} — the body overlaps itself there and \
             containment is not a function of position, so no verdict here is one \
             about the solid"
        );
        if claims == 1 {
            SolidContainment::In
        } else {
            SolidContainment::Out
        }
    }
}

/// The least turn a `turns`-revolution path may put into its level
/// planes before a row that probes it is asserting nothing.
///
/// One law, one spelling. The level plane normal is the path tangent,
/// `T(a) ∝ (−R sin a, R cos a, k)`, so `|dT/da| = R/√(R² + k²)` and a
/// `turns`-revolution path turns it by `2π · turns · R/√(R² + k²)`;
/// nine tenths of that leaves room for the interpolated spine. A planar
/// arc is the `k = 0` case and reads `2π · turns`, so an elbow and a
/// helix take their bar from the same expression rather than deriving
/// it twice, one file apart.
pub fn min_roll_turn(turns: f64, r: f64, k: f64) -> f64 {
    0.9 * turns * core::f64::consts::TAU * r / (r * r + k * k).sqrt()
}
